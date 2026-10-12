//! Namespace-aware, bounded Visio reader. Original parts retain extension XML and unknown bytes.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
pub mod dump;
pub mod evaluate;
use diagramcraft_model::*;
use diagramcraft_opc::{Package, xml};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::Path,
};
const REL_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const ATTR_REL_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const VIS_REL: &str = "http://schemas.microsoft.com/visio/2010/relationships/";
mod limits;

const MAX_FILE: u64 = 512 * 1024 * 1024;
#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("{0}")]
    Package(#[from] diagramcraft_opc::OpcError),
    #[error("invalid Visio file: {0}")]
    Invalid(String),
    #[error("cannot read file: {0}")]
    Io(#[from] std::io::Error),
}
fn bad(s: impl Into<String>) -> ReadError {
    ReadError::Invalid(s.into())
}
/// Read a bounded package from disk. Extensions do not override its content types.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
pub fn read(path: impl AsRef<Path>) -> Result<Document, ReadError> {
    let f = std::fs::File::open(path)?;
    if f.metadata()?.len() > MAX_FILE {
        return Err(bad("compressed file exceeds cap"));
    }
    let mut b = Vec::new();
    f.take(MAX_FILE + 1).read_to_end(&mut b)?;
    import(&b)
}
/// Import drawings, stencils and templates through their OPC relationships.
/// Macro payloads are discarded and `macros_removed` marks the document.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
pub fn import(bytes: &[u8]) -> Result<Document, ReadError> {
    if bytes.len() as u64 > MAX_FILE {
        return Err(bad("compressed file exceeds cap"));
    }
    let package = Package::open(bytes)?;
    package.validate_visio()?;
    let root_rels = relationships(&package, "")?;
    let main = root_rels
        .iter()
        .find(|r| r.type_uri == format!("{VIS_REL}document") && !r.external)
        .ok_or_else(|| bad("missing document relationship"))?
        .target
        .clone();
    let mut doc = Document { main_part: main.clone(), root_relationships: root_rels, ..Default::default() };
    for (name, bytes) in package.parts() {
        if name.rsplit('/').next().is_some_and(|s| s.eq_ignore_ascii_case("vbaProject.bin")) {
            doc.macros_removed = true;
            continue;
        }
        doc.parts.insert(name.to_string(), Part { bytes: bytes.to_vec(), content_type: package.content_type(name), ..Default::default() });
    }
    doc.macros_removed |= package.content_type(&main).is_some_and(|s| s.contains("macroEnabled"));
    let root = part_xml(&package, &mut doc, &main)?;
    if !root.is_visio("VisioDocument") {
        return Err(bad("invalid document namespace"));
    }
    doc.document_sheet = root.child("DocumentSheet").map(parse_sheet).transpose()?.unwrap_or_default();
    doc.document_settings = root.child("DocumentSettings").cloned();
    doc.event_list = root.child("EventList").cloned();
    if let Some(styles) = root.child("StyleSheets") {
        for s in styles.elements().filter(|e| e.is_visio("StyleSheet")) {
            doc.styles.push(Style {
                id: required_u32(s, "ID")?,
                name: string(s, "Name"),
                name_u: string(s, "NameU"),
                line_style: uint(s, "LineStyle")?,
                fill_style: uint(s, "FillStyle")?,
                text_style: uint(s, "TextStyle")?,
                sheet: parse_sheet(s)?,
            });
        }
    }
    if let Some(colors) = root.child("Colors") {
        for c in colors.elements().filter(|e| e.is_visio("ColorEntry")) {
            doc.colors.insert(required_u32(c, "IX")?, string(c, "RGB"));
        }
    }
    if let Some(fonts) = root.child("FaceNames") {
        for (i, f) in fonts.elements().filter(|e| e.is_visio("FaceName")).enumerate() {
            let index = uint(f, "ID")?.or(uint(f, "IX")?).unwrap_or(u32::try_from(i).map_err(|_| bad("font index overflow"))?);
            doc.face_names.insert(index, string(f, "NameU"));
        }
    }
    let rels = relationships(&package, &main)?;
    for r in rels.iter().filter(|r| !r.external) {
        if r.type_uri == format!("{VIS_REL}pages") {
            parse_pages(&package, &mut doc, &r.target)?;
        } else if r.type_uri == format!("{VIS_REL}masters") {
            parse_masters(&package, &mut doc, &r.target)?;
        } else if r.type_uri == format!("{VIS_REL}windows") {
            let _ = part_xml(&package, &mut doc, &r.target)?;
        }
    }
    // All relationship tables are parsed, so media/thumbnail and unknown links remain inspectable.
    // Known metadata/theme parts expose their namespace-aware XML tree, while custom
    // solution payloads remain opaque because they are not necessarily well-formed XML.
    let metadata: Vec<String> = doc
        .parts
        .iter()
        .filter(|(name, p)| {
            (name.starts_with("docProps/") && name.ends_with(".xml"))
                || p.content_type.as_deref() == Some("application/vnd.openxmlformats-officedocument.theme+xml")
        })
        .map(|(name, _)| name.clone())
        .collect();
    for name in metadata {
        let _ = part_xml(&package, &mut doc, &name)?;
    }
    let names: Vec<String> = doc.parts.keys().filter(|n| !n.ends_with(".rels")).cloned().collect();
    for name in names {
        let rels = relationships(&package, &name)?;
        if let Some(p) = doc.parts.get_mut(&name) {
            p.relationships = rels;
        }
    }
    Ok(doc)
}
fn string(e: &XmlElement, k: &str) -> String {
    e.attr(k).unwrap_or("").to_string()
}
fn uint(e: &XmlElement, k: &str) -> Result<Option<u32>, ReadError> {
    e.attr(k).map(|s| s.parse().map_err(|_| bad(format!("invalid {}.{k}", e.name)))).transpose()
}
fn required_u32(e: &XmlElement, k: &str) -> Result<u32, ReadError> {
    uint(e, k)?.ok_or_else(|| bad(format!("missing {}.{k}", e.name)))
}
fn boolean(e: &XmlElement, k: &str) -> Result<bool, ReadError> {
    match e.attr(k) {
        None | Some("0" | "false") => Ok(false),
        Some("1" | "true") => Ok(true),
        _ => Err(bad(format!("invalid {}.{k}", e.name))),
    }
}
fn normalize_xml(e: xml::El, parent: &BTreeMap<String, String>) -> Result<XmlElement, ReadError> {
    let mut ns = parent.clone();
    for (k, v) in &e.attrs {
        if k == "xmlns" {
            ns.insert(String::new(), v.clone());
        } else if let Some(p) = k.strip_prefix("xmlns:") {
            ns.insert(p.to_string(), v.clone());
        }
    }
    let (prefix, name) = e.name.split_once(':').unwrap_or(("", &e.name));
    let namespace = ns.get(prefix).cloned().unwrap_or_default();
    if !prefix.is_empty() && namespace.is_empty() {
        return Err(bad("unbound XML namespace prefix"));
    }
    let mut out = XmlElement { name: name.to_string(), namespace, ..Default::default() };
    let mut expanded = BTreeSet::new();
    for (k, v) in e.attrs {
        if k == "xmlns" || k.starts_with("xmlns:") {
            out.attributes.insert(k, v);
            continue;
        }
        let (prefix, local) = k.split_once(':').unwrap_or(("", &k));
        let uri = if prefix == "xml" {
            "http://www.w3.org/XML/1998/namespace".to_string()
        } else if prefix.is_empty() {
            String::new()
        } else {
            ns.get(prefix).cloned().ok_or_else(|| bad("unbound attribute namespace"))?
        };
        if !expanded.insert((uri.clone(), local.to_string())) {
            return Err(bad("duplicate expanded attribute"));
        }
        out.attribute_namespaces.insert(k.clone(), uri);
        out.attributes.insert(k, v);
    }
    for n in e.children {
        out.children.push(match n {
            xml::Node::Text(t) => XmlNode::Text(t),
            xml::Node::El(e) => XmlNode::Element(normalize_xml(e, &ns)?),
        });
    }
    Ok(out)
}
/// Parse one XML part with bounded nesting, strict namespaces and preserved text order.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
pub fn parse_xml(bytes: &[u8]) -> Result<XmlElement, ReadError> {
    limits::preflight(bytes)?;
    normalize_xml(xml::parse(bytes)?.root, &BTreeMap::new())
}
fn part_xml(p: &Package, d: &mut Document, name: &str) -> Result<XmlElement, ReadError> {
    let data = p.get(name).ok_or_else(|| bad(format!("missing part {name}")))?;
    let root = parse_xml(data)?;
    let original = p.original_name(name).ok_or_else(|| bad("part name missing"))?;
    if let Some(part) = d.parts.get_mut(original) {
        part.xml = Some(root.clone());
    }
    Ok(root)
}
fn relationships(p: &Package, name: &str) -> Result<Vec<Relationship>, ReadError> {
    let Some(bytes) = p.get(&diagramcraft_opc::rels_path(name)) else { return Ok(vec![]) };
    let root = parse_xml(bytes)?;
    if root.name != "Relationships" || root.namespace != REL_NS {
        return Err(bad("invalid relationship root"));
    }
    let mut out = vec![];
    let mut ids = BTreeSet::new();
    for r in root.elements().filter(|e| e.name == "Relationship" && e.namespace == REL_NS) {
        let id = r.attr("Id").ok_or_else(|| bad("relationship missing Id"))?;
        if !ids.insert(id) {
            return Err(bad("duplicate relationship Id"));
        }
        let ty = r.attr("Type").ok_or_else(|| bad("relationship missing Type"))?;
        let target = r.attr("Target").ok_or_else(|| bad("relationship missing Target"))?;
        let external = r.attr("TargetMode") == Some("External");
        out.push(Relationship {
            id: id.into(),
            type_uri: ty.into(),
            target: if external { target.into() } else { diagramcraft_opc::resolve(&diagramcraft_opc::dir_of(name), target) },
            external,
        });
    }
    Ok(out)
}
fn related(p: &Package, part: &str, e: &XmlElement, kind: &str) -> Result<String, ReadError> {
    let rel = e.child("Rel").ok_or_else(|| bad("missing Rel element"))?;
    let id = rel
        .attributes
        .iter()
        .find(|(k, _)| k.rsplit(':').next() == Some("id") && rel.attribute_namespaces.get(*k).is_some_and(|ns| ns == ATTR_REL_NS))
        .map(|(_, v)| v)
        .ok_or_else(|| bad("missing relationship id"))?;
    relationships(p, part)?
        .into_iter()
        .find(|r| r.id == *id && !r.external && r.type_uri == format!("{VIS_REL}{kind}"))
        .map(|r| r.target)
        .ok_or_else(|| bad(format!("missing {kind} relationship {id}")))
}
/// Preserve cells, deletion markers and arbitrary named sections without filtering cell names.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
pub fn parse_sheet(e: &XmlElement) -> Result<Sheet, ReadError> {
    // This entry point also accepts caller-built trees, which have not passed parse_xml.
    let mut pending = vec![(e, 0usize)];
    let mut nodes = 0usize;
    let mut bytes = 0usize;
    while let Some((node, depth)) = pending.pop() {
        nodes = nodes.saturating_add(1);
        if depth >= xml::MAX_DEPTH || nodes > xml::MAX_NODES {
            return Err(bad("sheet tree limit"));
        }
        bytes = bytes.saturating_add(node.name.len()).saturating_add(node.namespace.len());
        for (k, v) in &node.attributes {
            bytes = bytes.saturating_add(k.len()).saturating_add(v.len());
        }
        for child in &node.children {
            match child {
                XmlNode::Element(e) => pending.push((e, depth + 1)),
                XmlNode::Text(s) => bytes = bytes.saturating_add(s.len()),
            }
        }
        if bytes > limits::MAX_SHEET_BYTES {
            return Err(bad("sheet byte limit"));
        }
    }
    let mut sheet_cells = e.elements().filter(|c| c.is_visio("Cell")).count();
    if sheet_cells > limits::MAX_SHEET_CELLS {
        return Err(bad("sheet cell limit"));
    }
    for section in e.elements().filter(|s| s.is_visio("Section")) {
        let mut cells = 0usize;
        let mut rows = 0usize;
        for child in section.elements() {
            cells = cells.saturating_add(usize::from(child.is_visio("Cell")));
            if child.is_visio("Row") {
                rows += 1;
                cells = cells.saturating_add(child.elements().filter(|c| c.is_visio("Cell")).count());
            }
            if cells > limits::MAX_SECTION_CELLS || rows > limits::MAX_SECTION_ROWS {
                return Err(bad("section cell or row limit"));
            }
        }
        sheet_cells = sheet_cells.saturating_add(cells);
        if sheet_cells > limits::MAX_SHEET_CELLS {
            return Err(bad("sheet cell limit"));
        }
    }
    let mut sheet = Sheet { attributes: e.attributes.clone(), ..Default::default() };
    for c in e.elements() {
        if c.is_visio("Cell") {
            sheet.cells.push(cell(c)?);
        } else if c.is_visio("Trigger") {
            sheet.triggers.push(c.clone());
        } else if c.is_visio("Section") {
            let mut s = Section {
                name: string(c, "N"),
                index: uint(c, "IX")?,
                deleted: boolean(c, "Del")?,
                attributes: c.attributes.clone(),
                ..Default::default()
            };
            for r in c.elements() {
                if r.is_visio("Cell") {
                    s.cells.push(cell(r)?);
                } else if r.is_visio("Row") {
                    let mut row = Row {
                        name: r.attr("N").map(str::to_string),
                        index: uint(r, "IX")?,
                        row_type: r.attr("T").map(str::to_string),
                        deleted: boolean(r, "Del")?,
                        attributes: r.attributes.clone(),
                        ..Default::default()
                    };
                    for c in r.elements().filter(|e| e.is_visio("Cell")) {
                        row.cells.push(cell(c)?);
                    }
                    s.rows.push(row);
                }
            }
            sheet.sections.push(s);
        }
    }
    Ok(sheet)
}
fn cell(e: &XmlElement) -> Result<Cell, ReadError> {
    Ok(Cell {
        name: e.attr("N").ok_or_else(|| bad("Cell missing N"))?.into(),
        value: e.attr("V").map(str::to_string),
        formula: e.attr("F").map(str::to_string),
        units: e.attr("U").map(str::to_string),
        error: e.attr("E").map(str::to_string),
        attributes: e.attributes.clone(),
        ref_by: e.elements().filter(|e| e.is_visio("RefBy")).cloned().collect(),
    })
}
/// Shape attributes and nested groups; Text preserves cp/pp/tp/fld markers with IX and content.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/text-element-shapesheet_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cp-element-text_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pp-element-text_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/tp-element-text_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fld-element-text_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/foreigndata-element-shapesheet_type-complextypevisio-xml
fn shapes(e: &XmlElement, depth: usize) -> Result<Vec<Shape>, ReadError> {
    if depth > 128 {
        return Err(bad("shape nesting cap"));
    }
    let Some(container) = e.child("Shapes") else { return Ok(vec![]) };
    let mut result = vec![];
    for s in container.elements().filter(|e| e.is_visio("Shape")) {
        result.push(Shape {
            id: required_u32(s, "ID")?,
            name: string(s, "Name"),
            name_u: string(s, "NameU"),
            shape_type: s.attr("Type").map(str::to_string),
            master: uint(s, "Master")?,
            master_shape: uint(s, "MasterShape")?,
            line_style: uint(s, "LineStyle")?,
            fill_style: uint(s, "FillStyle")?,
            text_style: uint(s, "TextStyle")?,
            sheet: parse_sheet(s)?,
            shapes: shapes(s, depth + 1)?,
            text: s.child("Text").cloned(),
            foreign_data: s.child("ForeignData").cloned(),
        });
    }
    Ok(result)
}
fn connects(e: &XmlElement) -> Result<Vec<Connect>, ReadError> {
    let Some(container) = e.child("Connects") else { return Ok(vec![]) };
    container
        .elements()
        .filter(|e| e.is_visio("Connect"))
        .map(|c| {
            Ok(Connect {
                from_sheet: required_u32(c, "FromSheet")?,
                to_sheet: required_u32(c, "ToSheet")?,
                from_cell: c.attr("FromCell").map(str::to_string),
                to_cell: c.attr("ToCell").map(str::to_string),
                from_part: c.attr("FromPart").map(|s| s.parse().map_err(|_| bad("invalid FromPart"))).transpose()?,
                to_part: c.attr("ToPart").map(|s| s.parse().map_err(|_| bad("invalid ToPart"))).transpose()?,
                attributes: c.attributes.clone(),
            })
        })
        .collect()
}
fn parse_pages(p: &Package, d: &mut Document, part: &str) -> Result<(), ReadError> {
    let root = part_xml(p, d, part)?;
    if !root.is_visio("Pages") {
        return Err(bad("invalid Pages root"));
    }
    for e in root.elements().filter(|e| e.is_visio("Page")) {
        let target = if e.child("Rel").is_some() { related(p, part, e, "page")? } else { String::new() };
        let content = if target.is_empty() {
            XmlElement { name: "PageContents".into(), namespace: e.namespace.clone(), ..Default::default() }
        } else {
            part_xml(p, d, &target)?
        };
        if !content.is_visio("PageContents") {
            return Err(bad("invalid PageContents root"));
        }
        d.pages.push(Page {
            id: required_u32(e, "ID")?,
            name: string(e, "Name"),
            name_u: string(e, "NameU"),
            sheet: e.child("PageSheet").map(parse_sheet).transpose()?.unwrap_or_default(),
            shapes: shapes(&content, 0)?,
            connects: connects(&content)?,
            part: target,
            attributes: e.attributes.clone(),
        });
    }
    Ok(())
}
fn parse_masters(p: &Package, d: &mut Document, part: &str) -> Result<(), ReadError> {
    let root = part_xml(p, d, part)?;
    if !root.is_visio("Masters") {
        return Err(bad("invalid Masters root"));
    }
    for e in root.elements() {
        if e.is_visio("MasterShortcut") {
            d.master_shortcuts.push(e.clone());
        } else if e.is_visio("Master") {
            let target = if e.child("Rel").is_some() { related(p, part, e, "master")? } else { String::new() };
            let content = if target.is_empty() {
                XmlElement { name: "MasterContents".into(), namespace: e.namespace.clone(), ..Default::default() }
            } else {
                part_xml(p, d, &target)?
            };
            if !content.is_visio("MasterContents") {
                return Err(bad("invalid MasterContents root"));
            }
            d.masters.push(Master {
                id: required_u32(e, "ID")?,
                name: string(e, "Name"),
                name_u: string(e, "NameU"),
                unique_id: e.attr("UniqueID").map(str::to_string),
                base_id: e.attr("BaseID").map(str::to_string),
                icon: e.child("Icon").cloned(),
                sheet: e.child("PageSheet").map(parse_sheet).transpose()?.unwrap_or_default(),
                shapes: shapes(&content, 0)?,
                connects: connects(&content)?,
                part: target,
                attributes: e.attributes.clone(),
            });
        }
    }
    Ok(())
}
