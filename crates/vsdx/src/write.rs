//! Model-driven Visio OPC writing, preserving opaque part payloads byte-for-byte.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
use crate::evaluate::{Evaluator, Scope};
use diagramcraft_model::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Cursor, Write},
};
mod model;
mod validate;
mod xml;
use model::{node, replace, set};
#[derive(Debug, thiserror::Error)]
#[error("cannot write Visio package: {0}")]
pub struct WriteError(String);
fn error(message: impl Into<String>) -> WriteError {
    WriteError(message.into())
}
#[derive(Debug)]
pub struct Written {
    pub bytes: Vec<u8>,
    pub warnings: Vec<String>,
    pub warning_count: usize,
}
/// Recalculate concrete local caches; unavailable formulas retain their saved values with warnings.
/// Inject theme/text hosts at the application layer to preserve the L2 dependency contract.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
pub fn write(doc: &Document) -> Result<Written, WriteError> {
    write_with_hosts(doc, None, None)
}
pub fn write_with_hosts(
    doc: &Document,
    theme: Option<&dyn diagramcraft_sheet::host::ThemeHost>,
    text: Option<&dyn diagramcraft_model::text::TextHost>,
) -> Result<Written, WriteError> {
    validate::document(doc)?;
    let mut evaluator = Evaluator::new(doc).map_err(|e| error(e.to_string()))?;
    if let Some(h) = theme {
        evaluator = evaluator.with_theme_host(h);
    }
    if let Some(h) = text {
        evaluator = evaluator.with_text_host(h);
    }
    let mut w = model::Writer { evaluator, warnings: vec![], warning_count: 0 };
    let mut parts = doc.parts.clone();
    let mut main = source(doc, &doc.main_part).cloned().unwrap_or_else(|| node("VisioDocument"));
    replace(&mut main, "DocumentSettings", doc.document_settings.clone().into_iter().collect());
    let palette = colors(doc, main.child("Colors"));
    let face_names = fonts(doc, main.child("FaceNames"));
    replace(&mut main, "Colors", vec![palette]);
    replace(&mut main, "FaceNames", vec![face_names]);
    let mut styles = main.child("StyleSheets").cloned().unwrap_or_else(|| node("StyleSheets"));
    let mut items = vec![];
    for s in &doc.styles {
        let old = styles.elements().find(|e| e.is_visio("StyleSheet") && e.attr("ID").and_then(|n| n.parse::<u32>().ok()) == Some(s.id));
        let mut e = w.sheet(&s.sheet, Scope::Style(s.id), "StyleSheet", old)?;
        for (k, v) in [
            ("ID", Some(s.id.to_string())),
            ("Name", (!s.name.is_empty()).then(|| s.name.clone())),
            ("NameU", (!s.name_u.is_empty()).then(|| s.name_u.clone())),
            ("LineStyle", s.line_style.map(|v| v.to_string())),
            ("FillStyle", s.fill_style.map(|v| v.to_string())),
            ("TextStyle", s.text_style.map(|v| v.to_string())),
        ] {
            set(&mut e, k, v);
        }
        items.push(e);
    }
    replace(&mut styles, "StyleSheet", items);
    replace(&mut main, "StyleSheets", vec![styles]);
    let document_sheet = w.sheet(&doc.document_sheet, Scope::Document, "DocumentSheet", main.child("DocumentSheet"))?;
    replace(&mut main, "DocumentSheet", vec![document_sheet]);
    replace(&mut main, "EventList", doc.event_list.clone().into_iter().collect());
    put(&mut parts, &doc.main_part, main, "application/vnd.ms-visio.drawing.main+xml");
    let mut main_rels = parts.get(&doc.main_part).map(|p| p.relationships.clone()).unwrap_or_default();
    let pages_path = related(&main_rels, "pages").unwrap_or_else(|| "visio/pages/pages.xml".into());
    let masters_path = related(&main_rels, "masters").unwrap_or_else(|| "visio/masters/masters.xml".into());
    if !doc.pages.is_empty() || parts.contains_key(&pages_path) {
        link(&mut main_rels, "pages", &pages_path)?;
        let mut root = source(doc, &pages_path).cloned().unwrap_or_else(|| node("Pages"));
        let mut relations = parts.get(&pages_path).map(|p| p.relationships.clone()).unwrap_or_default();
        let mut children = vec![];
        for (i, p) in doc.pages.iter().enumerate() {
            let path = if p.part.is_empty() { format!("visio/pages/page{}.xml", i + 1) } else { p.part.clone() };
            let old = root.elements().find(|e| e.is_visio("Page") && e.attr("ID").and_then(|s| s.parse::<u32>().ok()) == Some(p.id));
            let mut e = old.cloned().unwrap_or_else(|| node("Page"));
            e.attributes = p.attributes.clone();
            set(&mut e, "ID", Some(p.id.to_string()));
            set(&mut e, "Name", Some(p.name.clone()));
            set(&mut e, "NameU", Some(p.name_u.clone()));
            let sheet = w.sheet(&p.sheet, Scope::Page(p.id), "PageSheet", old.and_then(|e| e.child("PageSheet")))?;
            replace(&mut e, "PageSheet", vec![sheet]);
            // Stencils can store a PageSheet-only entry without a PageContents part.
            if !sheet_only(doc, p) {
                let id = link(&mut relations, "page", &path)?;
                replace(&mut e, "Rel", vec![rel_node(&id)]);
                let contents = w.contents(&p.shapes, &p.connects, Scope::Page(p.id), "PageContents", source(doc, &path))?;
                put(&mut parts, &path, contents, "application/vnd.ms-visio.page+xml");
                page_master_links(&mut parts, &path, &p.shapes, doc)?;
            }
            children.push(e);
        }
        replace(&mut root, "Page", children);
        put(&mut parts, &pages_path, root, "application/vnd.ms-visio.pages+xml");
        if let Some(p) = parts.get_mut(&pages_path) {
            p.relationships = relations;
        }
    }
    if !doc.masters.is_empty() || parts.contains_key(&masters_path) {
        link(&mut main_rels, "masters", &masters_path)?;
        let mut root = source(doc, &masters_path).cloned().unwrap_or_else(|| node("Masters"));
        let mut relations = parts.get(&masters_path).map(|p| p.relationships.clone()).unwrap_or_default();
        let mut children = vec![];
        for (i, m) in doc.masters.iter().enumerate() {
            let path = if m.part.is_empty() { format!("visio/masters/master{}.xml", i + 1) } else { m.part.clone() };
            let old = root.elements().find(|e| e.is_visio("Master") && e.attr("ID").and_then(|s| s.parse::<u32>().ok()) == Some(m.id));
            let mut e = old.cloned().unwrap_or_else(|| node("Master"));
            e.attributes = m.attributes.clone();
            for (k, v) in [
                ("ID", Some(m.id.to_string())),
                ("Name", Some(m.name.clone())),
                ("NameU", Some(m.name_u.clone())),
                ("UniqueID", m.unique_id.clone()),
                ("BaseID", m.base_id.clone()),
            ] {
                set(&mut e, k, v);
            }
            let sheet = w.sheet(&m.sheet, Scope::Master(m.id), "PageSheet", old.and_then(|e| e.child("PageSheet")))?;
            replace(&mut e, "PageSheet", vec![sheet]);
            replace(&mut e, "Icon", m.icon.clone().into_iter().collect());
            let id = link(&mut relations, "master", &path)?;
            replace(&mut e, "Rel", vec![rel_node(&id)]);
            let contents = w.contents(&m.shapes, &m.connects, Scope::Master(m.id), "MasterContents", source(doc, &path))?;
            put(&mut parts, &path, contents, "application/vnd.ms-visio.master+xml");
            children.push(e);
        }
        replace(&mut root, "Master", children);
        replace(&mut root, "MasterShortcut", doc.master_shortcuts.clone());
        put(&mut parts, &masters_path, root, "application/vnd.ms-visio.masters+xml");
        if let Some(p) = parts.get_mut(&masters_path) {
            p.relationships = relations;
        }
    }
    if let Some(p) = parts.get_mut(&doc.main_part) {
        p.relationships = main_rels;
    }
    let mut root_rels = doc.root_relationships.clone();
    link(&mut root_rels, "document", &doc.main_part)?;
    if doc.macros_removed {
        parts.retain(|n, _| !n.rsplit('/').next().is_some_and(|n| n.eq_ignore_ascii_case("vbaProject.bin")));
        for p in parts.values_mut() {
            if let Some(ct) = &mut p.content_type {
                *ct = ct.replace(".macroEnabled", "");
            }
            p.relationships.retain(|r| !r.type_uri.ends_with("/vbaProject"));
        }
        root_rels.retain(|r| !r.type_uri.ends_with("/vbaProject"));
    }
    // AWSANALYTICS_M has an optional thumbnail relationship but no thumbnail part.
    // Do not fabricate vendor preview bytes or emit a dangling link in a new package.
    root_rels.retain(|r| {
        let missing_thumbnail = !r.external
            && r.type_uri == "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
            && !parts.contains_key(&r.target);
        if missing_thumbnail {
            w.warning_count = w.warning_count.saturating_add(1);
            if w.warnings.len() < 100 {
                w.warnings.push(format!("omitted missing optional thumbnail relationship: {}", r.target));
            }
        }
        !missing_thumbnail
    });
    let bytes = package(parts, root_rels)?;
    Ok(Written { bytes, warnings: w.warnings, warning_count: w.warning_count })
}
fn sheet_only(doc: &Document, p: &Page) -> bool {
    if !p.part.is_empty() || !p.shapes.is_empty() || !p.connects.is_empty() {
        return false;
    }
    let path = doc.parts.get(&doc.main_part).and_then(|p| related(&p.relationships, "pages")).unwrap_or_else(|| "visio/pages/pages.xml".into());
    source(doc, &path).is_some_and(|root| {
        root.elements().any(|e| e.is_visio("Page") && e.attr("ID").and_then(|n| n.parse::<u32>().ok()) == Some(p.id) && e.child("Rel").is_none())
    })
}
fn source<'a>(d: &'a Document, path: &str) -> Option<&'a XmlElement> {
    d.parts.get(path).and_then(|p| p.xml.as_ref())
}
fn put(parts: &mut BTreeMap<String, Part>, path: &str, xml: XmlElement, kind: &str) {
    let p = parts.entry(path.into()).or_default();
    if p.content_type.is_none() {
        p.content_type = Some(kind.into());
    }
    p.xml = Some(xml);
}
fn related(rels: &[Relationship], kind: &str) -> Option<String> {
    rels.iter().find(|r| !r.external && r.type_uri == format!("{}{kind}", crate::VIS_REL)).map(|r| r.target.clone())
}
fn link(rels: &mut Vec<Relationship>, kind: &str, target: &str) -> Result<String, WriteError> {
    let ty = format!("{}{kind}", crate::VIS_REL);
    if let Some(r) = rels.iter().find(|r| !r.external && r.type_uri == ty && r.target == target) {
        return Ok(r.id.clone());
    }
    if rels.len() >= 65536 {
        return Err(error("relationship limit"));
    }
    let ids: BTreeSet<_> = rels.iter().map(|r| r.id.as_str()).collect();
    let id = (1..=65537).map(|n| format!("rId{n}")).find(|id| !ids.contains(id.as_str())).ok_or_else(|| error("relationship ID limit"))?;
    rels.push(Relationship { id: id.clone(), type_uri: ty, target: target.into(), external: false });
    Ok(id)
}
fn rel_node(id: &str) -> XmlElement {
    let mut e = node("Rel");
    e.attributes.insert("xmlns:r".into(), crate::ATTR_REL_NS.into());
    e.attributes.insert("r:id".into(), id.into());
    e
}
fn page_master_links(parts: &mut BTreeMap<String, Part>, path: &str, shapes: &[Shape], doc: &Document) -> Result<(), WriteError> {
    let mut pending: Vec<_> = shapes.iter().collect();
    let mut ids = BTreeSet::new();
    while let Some(s) = pending.pop() {
        if let Some(m) = s.master {
            ids.insert(m);
        }
        pending.extend(&s.shapes);
    }
    let Some(p) = parts.get_mut(path) else {
        return Err(error("missing page part"));
    };
    for id in ids {
        let (i, m) = doc.masters.iter().enumerate().find(|(_, m)| m.id == id).ok_or_else(|| error("shape references missing master"))?;
        let target = if m.part.is_empty() { format!("visio/masters/master{}.xml", i + 1) } else { m.part.clone() };
        link(&mut p.relationships, "master", &target)?;
    }
    Ok(())
}
/// Palette and face-name entries retain their unmodelled metadata where available.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/facenames-element-visiodocument_type-complextypevisio-xml
fn colors(d: &Document, old: Option<&XmlElement>) -> XmlElement {
    let mut e = old.cloned().unwrap_or_else(|| node("Colors"));
    let entries = d
        .colors
        .iter()
        .map(|(i, rgb)| {
            let mut c = old
                .and_then(|e| e.elements().find(|c| c.attr("IX").and_then(|s| s.parse::<u32>().ok()) == Some(*i)))
                .cloned()
                .unwrap_or_else(|| node("ColorEntry"));
            set(&mut c, "IX", Some(i.to_string()));
            set(&mut c, "RGB", Some(rgb.clone()));
            c
        })
        .collect();
    replace(&mut e, "ColorEntry", entries);
    e
}
fn fonts(d: &Document, old: Option<&XmlElement>) -> XmlElement {
    let mut e = old.cloned().unwrap_or_else(|| node("FaceNames"));
    let entries = d
        .face_names
        .iter()
        .map(|(id, name)| {
            let old = old.and_then(|e| {
                e.elements()
                    .filter(|e| e.is_visio("FaceName"))
                    .enumerate()
                    .find(|(i, e)| e.attr("ID").or(e.attr("IX")).and_then(|s| s.parse::<u32>().ok()).unwrap_or(*i as u32) == *id)
                    .map(|(_, e)| e)
            });
            let mut f = old.cloned().unwrap_or_else(|| node("FaceName"));
            if old.is_none() {
                set(&mut f, "ID", Some(id.to_string()));
            }
            set(&mut f, "NameU", Some(name.clone()));
            f
        })
        .collect();
    replace(&mut e, "FaceName", entries);
    e
}
fn relationships(path: &str, rels: &[Relationship]) -> Result<Vec<u8>, WriteError> {
    let mut e = XmlElement { name: "Relationships".into(), namespace: crate::REL_NS.into(), ..Default::default() };
    for r in rels {
        let mut x = XmlElement { name: "Relationship".into(), namespace: crate::REL_NS.into(), ..Default::default() };
        x.attributes = BTreeMap::from([
            ("Id".into(), r.id.clone()),
            ("Type".into(), r.type_uri.clone()),
            ("Target".into(), if r.external { r.target.clone() } else { diagramcraft_opc::relative(path, &r.target) }),
        ]);
        if r.external {
            x.attributes.insert("TargetMode".into(), "External".into());
        }
        e.children.push(XmlNode::Element(x));
    }
    xml::tree(&e)
}
fn package(parts: BTreeMap<String, Part>, root_rels: Vec<Relationship>) -> Result<Vec<u8>, WriteError> {
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let mut types = BTreeMap::new();
    let mut output_bytes = 0usize;
    let all: BTreeSet<_> = parts.keys().map(String::as_str).collect();
    for (name, p) in &parts {
        if name == "[Content_Types].xml" || name.ends_with(".rels") {
            continue;
        }
        validate::valid_path(name)?;
        for r in &p.relationships {
            if !r.external && !all.contains(r.target.as_str()) {
                return Err(error(format!("missing relationship target {}", r.target)));
            }
        }
        let bytes = if let Some(e) = &p.xml { xml::tree(e)? } else { p.bytes.clone() };
        if bytes.len() as u64 > diagramcraft_opc::MAX_PART {
            return Err(error("written part byte limit"));
        }
        types.insert(name.clone(), p.content_type.clone().ok_or_else(|| error(format!("missing content type {name}")))?);
        add_file(&mut files, &mut output_bytes, name.clone(), bytes)?;
        let rel_path = diagramcraft_opc::rels_path(name);
        if !p.relationships.is_empty() || parts.contains_key(&rel_path) {
            add_file(&mut files, &mut output_bytes, rel_path, relationships(name, &p.relationships)?)?;
        }
    }
    for r in &root_rels {
        if !r.external && !all.contains(r.target.as_str()) {
            return Err(error("missing root relationship target"));
        }
    }
    add_file(&mut files, &mut output_bytes, "_rels/.rels".into(), relationships("", &root_rels)?)?;
    let ns = "http://schemas.openxmlformats.org/package/2006/content-types";
    let mut ct = if let Some(p) = parts.get("[Content_Types].xml") {
        crate::parse_xml(&p.bytes).map_err(|e| error(e.to_string()))?
    } else {
        XmlElement { name: "Types".into(), namespace: ns.into(), ..Default::default() }
    };
    // Keep observed default/override order. Remove only absent entries; append genuinely new parts.
    ct.children.retain(
        |n| !matches!(n,XmlNode::Element(e) if e.name=="Override" && !files.contains_key(e.attr("PartName").unwrap_or("").trim_start_matches('/'))),
    );
    let mut covered = BTreeSet::new();
    for n in &mut ct.children {
        if let XmlNode::Element(e) = n
            && e.name == "Override"
        {
            let p = e.attr("PartName").unwrap_or("").trim_start_matches('/').to_string();
            if let Some(t) = types.get(&p) {
                e.attributes.insert("ContentType".into(), t.clone());
                covered.insert(p);
            }
        }
    }
    let defaults: BTreeMap<_, _> = ct
        .elements()
        .filter(|e| e.name == "Default")
        .filter_map(|e| Some((e.attr("Extension")?.to_ascii_lowercase(), e.attr("ContentType")?.to_string())))
        .collect();
    if !defaults.contains_key("rels") {
        ct.children.push(XmlNode::Element(XmlElement {
            name: "Default".into(),
            namespace: ns.into(),
            attributes: BTreeMap::from([
                ("Extension".into(), "rels".into()),
                ("ContentType".into(), "application/vnd.openxmlformats-package.relationships+xml".into()),
            ]),
            ..Default::default()
        }));
    }
    for (name, ty) in types {
        if covered.contains(&name) || defaults.get(&name.rsplit('.').next().unwrap_or("").to_ascii_lowercase()) == Some(&ty) {
            continue;
        }
        ct.children.push(XmlNode::Element(XmlElement {
            name: "Override".into(),
            namespace: ns.into(),
            attributes: BTreeMap::from([("PartName".into(), format!("/{name}")), ("ContentType".into(), ty)]),
            ..Default::default()
        }));
    }
    add_file(&mut files, &mut output_bytes, "[Content_Types].xml".into(), xml::tree(&ct)?)?;
    if files.len() > diagramcraft_opc::MAX_ENTRIES {
        return Err(error("written entry limit"));
    }
    let total = files.values().try_fold(0usize, |n, b| n.checked_add(b.len()).ok_or_else(|| error("written byte overflow")))?;
    if total as u64 > diagramcraft_opc::MAX_TOTAL {
        return Err(error("written package byte limit"));
    }
    let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let opts = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in files {
        zip.start_file(name, opts).map_err(|e| error(e.to_string()))?;
        zip.write_all(&bytes).map_err(|e| error(e.to_string()))?;
    }
    let bytes = zip.finish().map_err(|e| error(e.to_string()))?.into_inner();
    if bytes.len() as u64 > crate::MAX_FILE {
        return Err(error("written compressed file limit"));
    }
    Ok(bytes)
}

fn add_file(files: &mut BTreeMap<String, Vec<u8>>, total: &mut usize, name: String, bytes: Vec<u8>) -> Result<(), WriteError> {
    let next = total.checked_add(bytes.len()).ok_or_else(|| error("written byte overflow"))?;
    if next as u64 > diagramcraft_opc::MAX_TOTAL {
        return Err(error("written package byte limit"));
    }
    if files.contains_key(&name) {
        return Err(error("duplicate serialized part"));
    }
    *total = next;
    files.insert(name, bytes);
    Ok(())
}
