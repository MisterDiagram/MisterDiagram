//! Model-driven Visio OPC writing, preserving opaque part payloads byte-for-byte.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
use crate::evaluate::{Evaluator, Scope};
use misterdiagram_model::*;
use std::collections::{BTreeMap, BTreeSet};
mod format;
pub use format::{SaveKind, write_as, write_as_with_hosts};
mod instances;
mod model;
mod source;
mod validate;
pub(crate) use validate::document as validate_document;
pub(crate) use validate::dump_bounds;
mod xml;
mod zip_parts;
use model::{node, replace, set};
#[derive(Debug, thiserror::Error)]
#[error("cannot write Visio package: {0}")]
pub struct WriteError(String);
fn error(message: impl Into<String>) -> WriteError {
    WriteError(message.into())
}
/// An admitted internal package. Its fields cannot be forged by callers.
#[derive(Debug)]
pub struct WrittenParts {
    pub(crate) files: BTreeMap<String, Shared<Vec<u8>>>,
    pub(crate) memos: BTreeMap<String, PartMemo>,
    pub warnings: Vec<String>,
    pub warning_count: usize,
}
#[derive(Debug)]
pub struct Written {
    pub bytes: Vec<u8>,
    pub warnings: Vec<String>,
    pub warning_count: usize,
}
/// Preserve unchanged source caches and recalculate authored or dependency-affected cells.
/// Inject theme/text hosts at the application layer to preserve the L2 dependency contract.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
pub fn write(doc: &Document) -> Result<Written, WriteError> {
    write_with_hosts(doc, None, None)
}
pub fn write_with_hosts(
    doc: &Document,
    theme: Option<&dyn misterdiagram_sheet::host::ThemeHost>,
    text: Option<&dyn misterdiagram_model::text::TextHost>,
) -> Result<Written, WriteError> {
    write_with_glue_host(doc, theme, text, None)
}
/// Application-injected glue hooks preserve layer direction and ordinary save cache behavior.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/begtrigger-cell-glue-info-section
pub fn write_with_glue_host<'a>(
    doc: &'a Document,
    theme: Option<&'a dyn misterdiagram_sheet::host::ThemeHost>,
    text: Option<&'a dyn misterdiagram_model::text::TextHost>,
    glue: Option<&'a mut dyn misterdiagram_sheet::HostHooks>,
) -> Result<Written, WriteError> {
    let parts = build_parts_with_glue_host(doc, theme, text, glue)?;
    let bytes = zip_parts::bytes(&parts.files)?;
    Ok(Written { bytes, warnings: parts.warnings, warning_count: parts.warning_count })
}
/// Publish complete serialized parts while preserving immutable source allocations.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
pub fn write_parts_with_glue_host<'a>(
    doc: &'a Document,
    theme: Option<&'a dyn misterdiagram_sheet::host::ThemeHost>,
    text: Option<&'a dyn misterdiagram_model::text::TextHost>,
    glue: Option<&'a mut dyn misterdiagram_sheet::HostHooks>,
) -> Result<WrittenParts, WriteError> {
    let parts = build_parts_with_glue_host(doc, theme, text, glue)?;
    zip_parts::admit(&parts.files)?;
    Ok(parts)
}
fn build_parts_with_glue_host<'a>(
    doc: &'a Document,
    theme: Option<&'a dyn misterdiagram_sheet::host::ThemeHost>,
    text: Option<&'a dyn misterdiagram_model::text::TextHost>,
    glue: Option<&'a mut dyn misterdiagram_sheet::HostHooks>,
) -> Result<WrittenParts, WriteError> {
    validate::document(doc)?;
    let mut evaluator = Evaluator::new(doc).map_err(|e| error(e.to_string()))?.with_internal_cache_units();
    if let Some(h) = theme {
        evaluator = evaluator.with_theme_host(h);
    }
    if let Some(h) = text {
        evaluator = evaluator.with_text_host(h);
    }
    if let Some(host) = glue {
        evaluator = evaluator.with_glue_host(host);
    }
    let source_state = source::SourceState::new(doc, &evaluator)?;
    let mut w = model::Writer { evaluator, source_state, warnings: vec![], warning_count: 0 };
    let mut parts = doc.parts.clone();
    if let Some(p) = parts.get_mut("docProps/app.xml").filter(|p| p.bytes.is_empty())
        && let Some(app) = &mut p.xml
    {
        misterdiagram_model::skeleton::update_summary(doc, app).map_err(error)?;
    }
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
        let desired: BTreeSet<_> = doc
            .pages
            .iter()
            .enumerate()
            .filter(|(_, p)| !sheet_only(doc, p))
            .map(|(i, p)| if p.part.is_empty() { format!("visio/pages/page{}.xml", i + 1) } else { p.part.clone() })
            .collect();
        let read_index = doc.parts.get(&pages_path).is_some_and(|p| !p.bytes.is_empty());
        // Only a relationship owned by a deleted or retargeted modeled Page is removed.
        // An unreferenced source relationship is not an authoring edit.
        let owned: BTreeSet<_> = root.elements().filter(|e| e.is_visio("Page")).filter_map(page_relation_id).collect();
        let removed: Vec<_> = relations
            .iter()
            .filter(|r| {
                !r.external
                    && r.type_uri == format!("{}page", crate::VIS_REL)
                    && !desired.contains(&r.target)
                    && (!read_index || owned.contains(r.id.as_str()))
            })
            .map(|r| (r.id.clone(), r.target.clone()))
            .collect();
        relations.retain(|r| !removed.iter().any(|(id, _)| id == &r.id));
        for (_, path) in removed {
            parts.remove(&path);
            parts.remove(&misterdiagram_opc::rels_path(&path));
        }
        let mut children = vec![];
        for (i, p) in doc.pages.iter().enumerate() {
            let path = if p.part.is_empty() { format!("visio/pages/page{}.xml", i + 1) } else { p.part.clone() };
            let old = root.elements().find(|e| e.is_visio("Page") && e.attr("ID").and_then(|s| s.parse::<u32>().ok()) == Some(p.id));
            let mut e = old.cloned().unwrap_or_else(|| node("Page"));
            e.attributes = p.attributes.clone().into_iter().collect();
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
                page_background_link(&mut parts, &path, p, doc)?;
            }
            children.push(e);
        }
        replace(&mut root, "Page", children);
        put(&mut parts, &pages_path, root, "application/vnd.ms-visio.pages+xml");
        if relations.is_empty() {
            parts.remove(&misterdiagram_opc::rels_path(&pages_path));
        }
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
            e.attributes = m.attributes.clone().into_iter().collect();
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
    let mut written = package(parts, root_rels)?;
    written.warnings = w.warnings;
    written.warning_count = w.warning_count;
    Ok(written)
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
    d.parts.get(path).and_then(|p| p.xml.as_ref()).map(|root| &**root)
}
fn put(parts: &mut BTreeMap<String, Part>, path: &str, xml: XmlElement, kind: &str) {
    let p = parts.entry(path.into()).or_default();
    if p.content_type.is_none() {
        p.content_type = Some(kind.into());
    }
    p.xml = Some(xml.into());
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
            f.attributes.remove("ID");
            set(&mut f, "NameU", Some(name.clone()));
            f
        })
        .collect();
    replace(&mut e, "FaceName", entries);
    e
}
fn relationships(path: &str, rels: &[Relationship]) -> Result<Vec<u8>, WriteError> {
    let mut e = XmlElement { name: "Relationships".into(), namespace: crate::REL_NS.into(), ..Default::default() };
    let mut ordered: Vec<_> = rels.iter().collect();
    if !path.is_empty() {
        ordered.sort_by_key(|r| std::cmp::Reverse(r.id.strip_prefix("rId").and_then(|n| n.parse::<u32>().ok()).unwrap_or(0)));
    }
    for r in ordered {
        let mut x = XmlElement { name: "Relationship".into(), namespace: crate::REL_NS.into(), ..Default::default() };
        x.attributes = misterdiagram_model::Attributes::from([
            ("Id".into(), r.id.clone()),
            ("Type".into(), r.type_uri.clone()),
            ("Target".into(), if r.external { r.target.clone() } else { misterdiagram_opc::relative(path, &r.target) }),
        ]);
        if r.external {
            x.attributes.insert("TargetMode".into(), "External".into());
        }
        e.children.push(XmlNode::Element(x));
    }
    xml::tree(&e)
}
/// Preserve an unchanged relationship part, including its original serialization.
/// Witness: the probe's six unmatched OPC hashes in the M3.24 baseline comparison.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
fn relationship_bytes(parts: &BTreeMap<String, Part>, path: &str, rels: &[Relationship]) -> Result<Shared<Vec<u8>>, WriteError> {
    if source_relationships(parts, path)?.as_deref() == Some(rels)
        && let Some(p) = parts.get(&misterdiagram_opc::rels_path(path))
    {
        return Ok(p.bytes.clone());
    }
    relationships(path, rels).map(Into::into)
}
/// Source relationships survive an unchanged save even if their target is absent.
/// Observed: AZURECOMPUTE_M.vssx root thumbnail relationship and its Visio save.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
fn source_relationships(parts: &BTreeMap<String, Part>, path: &str) -> Result<Option<Vec<Relationship>>, WriteError> {
    let Some(p) = parts.get(&misterdiagram_opc::rels_path(path)).filter(|p| !p.bytes.is_empty()) else {
        return Ok(None);
    };
    let root = source_xml(p, &misterdiagram_opc::rels_path(path))?;
    if root.name != "Relationships" || root.namespace != crate::REL_NS {
        return Err(error("invalid source relationship root"));
    }
    let original: Result<Vec<_>, WriteError> = root
        .elements()
        .filter(|e| e.name == "Relationship" && e.namespace == crate::REL_NS)
        .map(|e| {
            let external = e.attr("TargetMode") == Some("External");
            let target = e.attr("Target").ok_or_else(|| error("source relationship target missing"))?;
            Ok(Relationship {
                id: e.attr("Id").ok_or_else(|| error("source relationship ID missing"))?.into(),
                type_uri: e.attr("Type").ok_or_else(|| error("source relationship type missing"))?.into(),
                target: if external { target.into() } else { misterdiagram_opc::resolve(&misterdiagram_opc::dir_of(path), target) },
                external,
            })
        })
        .collect();
    Ok(Some(original?))
}

fn source_xml(p: &Part, name: &str) -> Result<Shared<XmlElement>, WriteError> {
    if let Some(cached) = crate::cache::admitted(p, name) {
        return Ok(cached.xml.clone());
    }
    crate::parse_xml(&p.bytes).map(Into::into).map_err(|e| error(e.to_string()))
}
fn package(parts: BTreeMap<String, Part>, root_rels: Vec<Relationship>) -> Result<WrittenParts, WriteError> {
    let mut files: BTreeMap<String, Shared<Vec<u8>>> = BTreeMap::new();
    let mut types = BTreeMap::new();
    let mut output_bytes = 0usize;
    let all: BTreeSet<_> = parts.keys().map(String::as_str).collect();
    for (name, p) in &parts {
        if name == "[Content_Types].xml" || name.ends_with(".rels") {
            continue;
        }
        validate::valid_path(name)?;
        let read_relationships = source_relationships(&parts, name)?.unwrap_or_default();
        for r in &p.relationships {
            if !r.external && !all.contains(r.target.as_str()) && !read_relationships.contains(r) {
                return Err(error(format!("missing relationship target {}", r.target)));
            }
        }
        let bytes = if let Some(e) = &p.xml {
            if !p.bytes.is_empty() && source_xml(p, name)? == *e {
                p.bytes.clone()
            } else if p.bytes.is_empty() && name == "docProps/app.xml" {
                xml::authored_app(e)?.into()
            } else {
                xml::tree(e)?.into()
            }
        } else {
            p.bytes.clone()
        };
        if bytes.len() as u64 > misterdiagram_opc::MAX_PART {
            return Err(error("written part byte limit"));
        }
        types.insert(name.clone(), p.content_type.clone().ok_or_else(|| error(format!("missing content type {name}")))?);
        add_file(&mut files, &mut output_bytes, name.clone(), bytes)?;
        let rel_path = misterdiagram_opc::rels_path(name);
        if !p.relationships.is_empty() || parts.contains_key(&rel_path) {
            add_file(&mut files, &mut output_bytes, rel_path, relationship_bytes(&parts, name, &p.relationships)?)?;
        }
    }
    let read_relationships = source_relationships(&parts, "")?.unwrap_or_default();
    for r in &root_rels {
        if !r.external && !all.contains(r.target.as_str()) && !read_relationships.contains(r) {
            return Err(error("missing root relationship target"));
        }
    }
    add_file(&mut files, &mut output_bytes, "_rels/.rels".into(), relationship_bytes(&parts, "", &root_rels)?)?;
    let ns = "http://schemas.openxmlformats.org/package/2006/content-types";
    let original_ct = if let Some(p) = parts.get("[Content_Types].xml") {
        source_xml(p, "[Content_Types].xml")?
    } else {
        Shared::from(XmlElement { name: "Types".into(), namespace: ns.into(), ..Default::default() })
    };
    let mut ct = (*original_ct).clone();
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
            attributes: misterdiagram_model::Attributes::from([
                ("Extension".into(), "rels".into()),
                ("ContentType".into(), "application/vnd.openxmlformats-package.relationships+xml".into()),
            ]),
            ..Default::default()
        }));
    }
    if !parts.contains_key("[Content_Types].xml") && !defaults.contains_key("xml") {
        ct.children.push(XmlNode::Element(XmlElement {
            name: "Default".into(),
            namespace: ns.into(),
            attributes: misterdiagram_model::Attributes::from([("Extension".into(), "xml".into()), ("ContentType".into(), "application/xml".into())]),
            ..Default::default()
        }));
    }
    let mut types: Vec<_> = types.into_iter().collect();
    types.sort_by_key(|(name, _)| {
        let rank = match name.as_str() {
            "visio/document.xml" => 0,
            "visio/masters/masters.xml" => 1,
            n if n.starts_with("visio/masters/master") => 2,
            "visio/pages/pages.xml" => 3,
            n if n.starts_with("visio/pages/page") => 4,
            "visio/windows.xml" => 5,
            "docProps/core.xml" => 6,
            "docProps/app.xml" => 7,
            "docProps/custom.xml" => 8,
            _ => 9,
        };
        let number = name
            .rsplit('/')
            .next()
            .unwrap_or("")
            .trim_end_matches(".xml")
            .trim_start_matches(|c: char| !c.is_ascii_digit())
            .parse::<u32>()
            .unwrap_or(0);
        (rank, number, name.clone())
    });
    for (name, ty) in types {
        if covered.contains(&name) || defaults.get(&name.rsplit('.').next().unwrap_or("").to_ascii_lowercase()) == Some(&ty) {
            continue;
        }
        ct.children.push(XmlNode::Element(XmlElement {
            name: "Override".into(),
            namespace: ns.into(),
            attributes: misterdiagram_model::Attributes::from([("PartName".into(), format!("/{name}")), ("ContentType".into(), ty)]),
            ..Default::default()
        }));
    }
    let ct_bytes = if ct == *original_ct {
        parts.get("[Content_Types].xml").map(|p| p.bytes.clone()).map(Ok).unwrap_or_else(|| xml::tree(&ct).map(Into::into))?
    } else {
        xml::tree(&ct)?.into()
    };
    add_file(&mut files, &mut output_bytes, "[Content_Types].xml".into(), ct_bytes)?;
    if files.len() > misterdiagram_opc::MAX_ENTRIES {
        return Err(error("written entry limit"));
    }
    let total = files.values().try_fold(0usize, |n, b| n.checked_add(b.len()).ok_or_else(|| error("written byte overflow")))?;
    if total as u64 > misterdiagram_opc::MAX_TOTAL {
        return Err(error("written package byte limit"));
    }
    // Namespace declarations can differ in the in-memory candidate tree while
    // serializing to identical bytes. Exact whole-part byte equality is also a
    // valid source proof, after the ordinary serializer has completed its work.
    for (name, bytes) in &mut files {
        if let Some(part) = parts.get(name)
            && *bytes == part.bytes
        {
            *bytes = part.bytes.clone();
        }
    }
    let memos = files
        .iter()
        .filter_map(|(name, bytes)| {
            let part = parts.get(name)?;
            (part.bytes.ptr_eq(bytes) && crate::cache::admitted(part, name).is_some()).then(|| (name.clone(), part.memo.clone()))
        })
        .collect();
    Ok(WrittenParts { files, memos, warnings: vec![], warning_count: 0 })
}

fn add_file(files: &mut BTreeMap<String, Shared<Vec<u8>>>, total: &mut usize, name: String, bytes: Shared<Vec<u8>>) -> Result<(), WriteError> {
    let next = total.checked_add(bytes.len()).ok_or_else(|| error("written byte overflow"))?;
    if next as u64 > misterdiagram_opc::MAX_TOTAL {
        return Err(error("written package byte limit"));
    }
    if files.contains_key(&name) {
        return Err(error("duplicate serialized part"));
    }
    *total = next;
    files.insert(name, bytes);
    Ok(())
}

/// Background pages require a page relationship from the foreground content part.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
fn page_background_link(parts: &mut BTreeMap<String, Part>, path: &str, page: &Page, doc: &Document) -> Result<(), WriteError> {
    let target = page
        .attributes
        .get("BackPage")
        .map(|id| -> Result<String, WriteError> {
            let id = id.parse::<u32>().map_err(|_| error("invalid background page ID"))?;
            let (i, background) = doc.pages.iter().enumerate().find(|(_, p)| p.id == id).ok_or_else(|| error("missing background page"))?;
            if id == page.id || sheet_only(doc, background) {
                return Err(error("invalid background page"));
            }
            Ok(if background.part.is_empty() { format!("visio/pages/page{}.xml", i + 1) } else { background.part.clone() })
        })
        .transpose()?;
    let pages_path = doc.parts.get(&doc.main_part).and_then(|p| related(&p.relationships, "pages")).unwrap_or_else(|| "visio/pages/pages.xml".into());
    let old_page = source(doc, &pages_path)
        .and_then(|root| root.elements().find(|e| e.is_visio("Page") && e.attr("ID").and_then(|n| n.parse::<u32>().ok()) == Some(page.id)));
    let is_read = doc.parts.get(path).is_some_and(|p| !p.bytes.is_empty());
    let old_target = old_page
        .and_then(|e| e.attr("BackPage"))
        .and_then(|id| source(doc, &pages_path)?.elements().find(|e| e.is_visio("Page") && e.attr("ID") == Some(id)))
        .and_then(page_relation_id)
        .and_then(|id| doc.parts.get(&pages_path)?.relationships.iter().find(|r| r.id == id))
        .map(|r| r.target.clone());
    if is_read && old_target == target && old_page.is_some_and(|e| e.attr("BackPage") == page.attributes.get("BackPage").map(String::as_str)) {
        return Ok(());
    }
    let p = parts.get_mut(path).ok_or_else(|| error("missing foreground part"))?;
    p.relationships.retain(|r| {
        r.external
            || r.type_uri != format!("{}page", crate::VIS_REL)
            || target.as_ref() == Some(&r.target)
            || (is_read && old_target.as_ref() != Some(&r.target))
    });
    if let Some(target) = target {
        link(&mut p.relationships, "page", &target)?;
    }
    if p.relationships.is_empty() {
        parts.remove(&misterdiagram_opc::rels_path(path));
    }
    Ok(())
}

// Resolve only the namespace-qualified Rel reference owned by a modeled Page.
fn page_relation_id(page: &XmlElement) -> Option<&str> {
    let rel = page.child("Rel")?;
    rel.attributes
        .iter()
        .find(|(key, _)| key.rsplit(':').next() == Some("id") && rel.attribute_namespaces.get(key).is_some_and(|ns| ns == crate::ATTR_REL_NS))
        .map(|(_, value)| value.as_str())
}
