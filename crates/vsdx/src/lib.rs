//! Namespace-aware, bounded Visio reader. Original parts retain extension XML and unknown bytes.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
pub mod dump;
pub mod evaluate;
pub mod roundtrip;
pub mod write;
use misterdiagram_model::*;
use misterdiagram_opc::{Package, xml};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    io::Read,
    path::Path,
};
pub use write::{SaveKind, WriteError, Written, write, write_as, write_as_with_hosts, write_with_glue_host, write_with_hosts};
const REL_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const ATTR_REL_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const VIS_REL: &str = "http://schemas.microsoft.com/visio/2010/relationships/";
mod limits;

/// Shared logical-node admission limit, authorized by Charles on 2026-10-16.
pub const MAX_RETAINED_XML_NODES: usize = 4_000_000;
struct ReadPackage {
    package: Package,
    content_overrides: HashMap<String, String>,
    content_defaults: HashMap<String, String>,
    used: std::cell::Cell<usize>,
    already_open: usize,
    counted: std::cell::RefCell<HashSet<String>>,
}
impl std::ops::Deref for ReadPackage {
    type Target = Package;
    fn deref(&self) -> &Package {
        &self.package
    }
}
impl ReadPackage {
    fn content_type(&self, part: &str) -> Option<String> {
        self.content_overrides
            .get(&misterdiagram_opc::norm(part))
            .or_else(|| self.content_defaults.get(&part.rsplit('.').next().unwrap_or("").to_ascii_lowercase()))
            .cloned()
    }
    fn charge(&self, name: &str) -> Result<(), ReadError> {
        let name = misterdiagram_opc::norm(name);
        let mut counted = self.counted.try_borrow_mut().map_err(|_| bad("recursive XML budget access"))?;
        if counted.contains(&name) {
            return Ok(());
        }
        let bytes = self.get(&name).ok_or_else(|| bad(format!("missing part {name}")))?;
        let mut used = self.used.get();
        xml::charge_retained_nodes(bytes, &mut used, MAX_RETAINED_XML_NODES, self.already_open)?;
        self.used.set(used);
        counted.insert(name);
        Ok(())
    }
}
const MAX_FILE: u64 = 512 * 1024 * 1024;
#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("{0}")]
    Package(#[from] misterdiagram_opc::OpcError),
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
    read_with_open_nodes(path, 0)
}
/// Import while reserving the nodes already retained by a session.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.documents.open
pub fn read_with_open_nodes(path: impl AsRef<Path>, already_open: usize) -> Result<Document, ReadError> {
    let f = std::fs::File::open(path)?;
    if f.metadata()?.len() > MAX_FILE {
        return Err(bad("compressed file exceeds cap"));
    }
    let mut b = Vec::new();
    f.take(MAX_FILE + 1).read_to_end(&mut b)?;
    import_with_open_nodes(&b, already_open)
}
/// Import drawings, stencils and templates through their OPC relationships.
/// Macro payloads are discarded and `macros_removed` marks the document.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
pub fn import(bytes: &[u8]) -> Result<Document, ReadError> {
    import_with_open_nodes(bytes, 0)
}
/// Import against the session's remaining logical-node reservation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
pub fn import_with_open_nodes(bytes: &[u8], already_open: usize) -> Result<Document, ReadError> {
    if bytes.len() as u64 > MAX_FILE {
        return Err(bad("compressed file exceeds cap"));
    }
    let mut package = ReadPackage {
        package: Package::open(bytes)?,
        content_overrides: HashMap::new(),
        content_defaults: HashMap::new(),
        used: std::cell::Cell::new(already_open),
        already_open,
        counted: Default::default(),
    };
    // Validate the same package contract using trees already needed by this import.
    // Avoid reparsing root relationships through Package::validate_visio.
    let types = part_xml(&package, "[Content_Types].xml")?;
    if types.name != "Types" || types.namespace != "http://schemas.openxmlformats.org/package/2006/content-types" {
        return Err(bad("invalid content types root"));
    }
    // Populate the lookup from the admitted compact tree. Package::content_type
    // would construct another, noncompact DOM for the same part.
    for element in types.elements() {
        if element.name == "Override" {
            if let (Some(name), Some(kind)) = (element.attr("PartName"), element.attr("ContentType")) {
                package.content_overrides.insert(misterdiagram_opc::norm(name), kind.to_string());
            }
        } else if element.name == "Default"
            && let (Some(extension), Some(kind)) = (element.attr("Extension"), element.attr("ContentType"))
        {
            package.content_defaults.insert(extension.to_ascii_lowercase(), kind.to_string());
        }
    }
    drop(types);
    let root_rels = relationships(&package, "")?;
    let main = root_rels
        .iter()
        .find(|r| r.type_uri == format!("{VIS_REL}document") && !r.external)
        .ok_or_else(|| bad("missing document relationship"))?
        .target
        .clone();
    let kind = package.content_type(&main).ok_or_else(|| bad("missing main content type"))?;
    if !matches!(
        kind.as_str(),
        "application/vnd.ms-visio.drawing.main+xml"
            | "application/vnd.ms-visio.stencil.main+xml"
            | "application/vnd.ms-visio.template.main+xml"
            | "application/vnd.ms-visio.drawing.macroEnabled.main+xml"
            | "application/vnd.ms-visio.stencil.macroEnabled.main+xml"
            | "application/vnd.ms-visio.template.macroEnabled.main+xml"
    ) {
        return Err(bad("not a supported Visio package"));
    }
    let mut doc = Document { main_part: main.clone(), root_relationships: root_rels, ..Default::default() };
    for (name, _) in package.parts() {
        let content_type = package.content_type(name);
        if content_type.as_deref() == Some("application/vnd.ms-office.vbaProject")
            || name.rsplit('/').next().is_some_and(|s| s.eq_ignore_ascii_case("vbaProject.bin"))
        {
            doc.macros_removed = true;
            continue;
        }
        doc.parts.insert(name.to_string(), Part { content_type, ..Default::default() });
    }
    let mut tables = HashMap::new();
    for name in doc.parts.keys().filter(|n| !n.ends_with(".rels")) {
        tables.insert(misterdiagram_opc::norm(name), RelationshipTable::new(relationships(&package, name)?));
    }
    doc.macros_removed |= package.content_type(&main).is_some_and(|s| s.contains("macroEnabled"));
    let root = part_xml(&package, &main)?;
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
    retain_xml(&package, &mut doc, &main, root)?;
    let rels = tables.get(&misterdiagram_opc::norm(&main)).ok_or_else(|| bad("missing document relationship table"))?;
    let mut collections = HashSet::new();
    let mut pages = HashSet::new();
    let mut masters = HashSet::new();
    for r in rels.entries.iter().filter(|r| !r.external) {
        if !collections.insert((r.type_uri.as_str(), misterdiagram_opc::norm(&r.target))) {
            continue;
        }
        if r.type_uri == format!("{VIS_REL}pages") {
            parse_pages(&package, &mut doc, &r.target, &tables, &mut pages)?;
        } else if r.type_uri == format!("{VIS_REL}masters") {
            parse_masters(&package, &mut doc, &r.target, &tables, &mut masters)?;
        } else if r.type_uri == format!("{VIS_REL}windows") {
            let root = part_xml(&package, &r.target)?;
            retain_xml(&package, &mut doc, &r.target, root)?;
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
        let root = part_xml(&package, &name)?;
        retain_xml(&package, &mut doc, &name, root)?;
    }
    for (name, table) in tables {
        if let Some(p) = package.original_name(&name).and_then(|original| doc.parts.get_mut(original)) {
            p.relationships = table.entries;
        }
    }
    doc.retained_xml_nodes = package.used.get().saturating_sub(already_open);
    for (name, bytes) in package.package.into_parts() {
        if let Some(part) = doc.parts.get_mut(&name) {
            part.bytes = bytes;
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
#[derive(Default)]
struct VisioTree {
    namespaces: BTreeMap<String, XmlString>,
    namespace_frames: Vec<Vec<(String, Option<XmlString>)>>,
    strings: HashSet<std::sync::Arc<String>>,
    child_counts: std::vec::IntoIter<usize>,
}
impl VisioTree {
    fn intern(&mut self, s: String) -> std::sync::Arc<String> {
        if let Some(v) = self.strings.get(&s) {
            v.clone()
        } else {
            let v = std::sync::Arc::new(s);
            self.strings.insert(v.clone());
            v
        }
    }
}
impl xml::TreeBuilder for VisioTree {
    type Element = XmlElement;
    type Error = ReadError;
    fn start(&mut self, e: xml::El) -> Result<XmlElement, ReadError> {
        let mut changes = Vec::new();
        for (k, v) in &e.attrs {
            if k == "xmlns" {
                let value = self.intern(v.clone()).into();
                changes.push((String::new(), self.namespaces.insert(String::new(), value)));
            } else if let Some(p) = k.strip_prefix("xmlns:") {
                let value = self.intern(v.clone()).into();
                changes.push((p.to_string(), self.namespaces.insert(p.to_string(), value)));
            }
        }
        let (prefix, name) = e.name.split_once(':').unwrap_or(("", &e.name));
        let namespace = self.namespaces.get(prefix).cloned().unwrap_or_default();
        if !prefix.is_empty() && namespace.is_empty() {
            return Err(bad("unbound XML namespace prefix"));
        }
        let mut out = XmlElement {
            name: self.intern(name.to_string()).into(),
            namespace,
            children: Vec::with_capacity(self.child_counts.next().ok_or_else(|| bad("missing preflight child count"))?),
            ..Default::default()
        };
        let mut expanded = HashSet::new();
        let mut attributes = Vec::with_capacity(e.attrs.len());
        let mut attribute_namespaces = Vec::new();
        for (k, v) in e.attrs {
            if k == "xmlns" || k.starts_with("xmlns:") {
                attributes.push((self.intern(k), self.intern(v)));
                continue;
            }
            let (prefix, local) = k.split_once(':').unwrap_or(("", &k));
            let uri = if prefix == "xml" {
                self.intern("http://www.w3.org/XML/1998/namespace".to_string())
            } else if prefix.is_empty() {
                self.intern(String::new())
            } else {
                self.namespaces.get(prefix).map(XmlString::shared).ok_or_else(|| bad("unbound attribute namespace"))?
            };
            // URI handles come from this interner, so pointer identity also avoids
            // repeatedly hashing a long inherited namespace for every attribute.
            if !expanded.insert((std::sync::Arc::as_ptr(&uri), local.to_string())) {
                return Err(bad("duplicate expanded attribute"));
            }
            if !uri.is_empty() {
                attribute_namespaces.push((self.intern(k.clone()), uri));
            }
            attributes.push((self.intern(k), self.intern(v)));
        }
        out.attributes = Attributes::shared(attributes);
        out.attribute_namespaces = Attributes::shared(attribute_namespaces);
        self.namespace_frames.push(changes);
        Ok(out)
    }
    fn end(&mut self) {
        if let Some(changes) = self.namespace_frames.pop() {
            for (prefix, previous) in changes.into_iter().rev() {
                if let Some(uri) = previous {
                    self.namespaces.insert(prefix, uri);
                } else {
                    self.namespaces.remove(&prefix);
                }
            }
        }
    }
    fn element(parent: &mut XmlElement, mut child: XmlElement) {
        child.children.shrink_to_fit();
        parent.children.push(XmlNode::Element(child));
    }
    fn last_text(parent: &mut XmlElement) -> Option<&mut String> {
        if let Some(XmlNode::Text(text)) = parent.children.last_mut() { Some(text) } else { None }
    }
    fn text(parent: &mut XmlElement, text: String) {
        parent.children.push(XmlNode::Text(text));
    }
}
/// Parse one XML part with bounded nesting, strict namespaces and preserved text order.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/visiodocument-elementvisio-xml
pub fn parse_xml(bytes: &[u8]) -> Result<XmlElement, ReadError> {
    if bytes.len() > misterdiagram_opc::MAX_XML_PART {
        return Err(bad("XML part byte limit"));
    }
    let decoded = xml::decoded_xml(bytes)?;
    let bytes = decoded.as_ref();
    limits::preflight(bytes)?;
    xml::parse_with_builder(bytes, &mut VisioTree { child_counts: child_counts(bytes)?.into_iter(), ..Default::default() })
}

// Match TreeBuilder::last_text coalescing exactly, including entity, CDATA and
// comment boundaries. An event count is not a safe allocation size for text.
fn child_counts(bytes: &[u8]) -> Result<Vec<usize>, ReadError> {
    use quick_xml::{Reader, events::Event};
    let mut reader = Reader::from_reader(bytes);
    let mut counts: Vec<usize> = Vec::new();
    let mut stack: Vec<(usize, bool)> = Vec::new();
    loop {
        let event = reader.read_event().map_err(|e| bad(e.to_string()))?;
        let text = match &event {
            Event::Text(t) => !t.is_empty(),
            Event::CData(t) => !t.is_empty(),
            Event::GeneralRef(_) => true,
            _ => false,
        };
        if text
            && let Some((i, last_text)) = stack.last_mut()
            && !*last_text
        {
            let n = counts.get_mut(*i).ok_or_else(|| bad("invalid child count index"))?;
            *n = n.checked_add(1).ok_or_else(|| bad("child count overflow"))?;
            *last_text = true;
        }
        match event {
            event @ (Event::Start(_) | Event::Empty(_)) => {
                if stack.len() >= xml::MAX_DEPTH {
                    return Err(bad("XML depth limit"));
                }
                if let Some((i, last_text)) = stack.last_mut() {
                    let n = counts.get_mut(*i).ok_or_else(|| bad("invalid child count index"))?;
                    *n = n.checked_add(1).ok_or_else(|| bad("child count overflow"))?;
                    *last_text = false;
                }
                if counts.len() >= xml::MAX_NODES {
                    return Err(bad("XML node limit"));
                }
                let i = counts.len();
                counts.push(0);
                if matches!(event, Event::Start(_)) {
                    stack.push((i, false));
                }
            }
            Event::End(_) => {
                stack.pop();
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(counts)
}

fn part_xml(p: &ReadPackage, name: &str) -> Result<XmlElement, ReadError> {
    p.charge(name)?;
    let data = p.get(name).ok_or_else(|| bad(format!("missing part {name}")))?;
    parse_xml(data)
}
fn retain_xml(p: &ReadPackage, d: &mut Document, name: &str, root: XmlElement) -> Result<(), ReadError> {
    let original = p.original_name(name).ok_or_else(|| bad("part name missing"))?;
    if let Some(part) = d.parts.get_mut(original) {
        part.xml = Some(root);
    }
    Ok(())
}
fn relationships(p: &ReadPackage, name: &str) -> Result<Vec<Relationship>, ReadError> {
    let rel_part = misterdiagram_opc::rels_path(name);
    let Some(bytes) = p.get(&rel_part) else { return Ok(vec![]) };
    p.charge(&rel_part)?;
    #[cfg(test)]
    RELATIONSHIP_PARSES.with(|n| n.set(n.get() + 1));
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
        let external = r.attr("TargetMode").is_some_and(|mode| mode.eq_ignore_ascii_case("external"));
        out.push(Relationship {
            id: id.into(),
            type_uri: ty.into(),
            target: if external { target.into() } else { misterdiagram_opc::resolve(&misterdiagram_opc::dir_of(name), target) },
            external,
        });
    }
    Ok(out)
}
/// One indexed table per relationship part, retained for the duration of import.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/rel-element-page_type-complextypevisio-xml
struct RelationshipTable {
    entries: Vec<Relationship>,
    by_id: HashMap<String, usize>,
}
impl RelationshipTable {
    fn new(entries: Vec<Relationship>) -> Self {
        let by_id = entries.iter().enumerate().map(|(i, r)| (r.id.clone(), i)).collect();
        Self { entries, by_id }
    }
}
#[cfg(test)]
std::thread_local! { static RELATIONSHIP_PARSES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
fn related(tables: &HashMap<String, RelationshipTable>, part: &str, e: &XmlElement, kind: &str) -> Result<String, ReadError> {
    let rel = e.child("Rel").ok_or_else(|| bad("missing Rel element"))?;
    let id = rel
        .attributes
        .iter()
        .find(|(k, _)| k.rsplit(':').next() == Some("id") && rel.attribute_namespaces.get(k).is_some_and(|ns| ns == ATTR_REL_NS))
        .map(|(_, v)| v)
        .ok_or_else(|| bad("missing relationship id"))?;
    tables
        .get(&misterdiagram_opc::norm(part))
        .and_then(|t| t.by_id.get(id).and_then(|i| t.entries.get(*i)))
        .filter(|r| !r.external && r.type_uri == format!("{VIS_REL}{kind}"))
        .map(|r| r.target.clone())
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
    let mut sheet = Sheet {
        attributes: e.attributes.clone(),
        cells: Vec::with_capacity(e.elements().filter(|x| x.is_visio("Cell")).count()),
        sections: Vec::with_capacity(e.elements().filter(|x| x.is_visio("Section")).count()),
        ..Default::default()
    };
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
                cells: Vec::with_capacity(c.elements().filter(|x| x.is_visio("Cell")).count()),
                rows: Vec::with_capacity(c.elements().filter(|x| x.is_visio("Row")).count()),
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
                        cells: Vec::with_capacity(r.elements().filter(|x| x.is_visio("Cell")).count()),
                    };
                    for c in r.elements().filter(|e| e.is_visio("Cell")) {
                        row.cells.push(cell(c)?);
                    }
                    row.cells.shrink_to_fit();
                    s.rows.push(row);
                }
            }
            s.cells.shrink_to_fit();
            s.rows.shrink_to_fit();
            sheet.sections.push(s);
        }
    }
    sheet.cells.shrink_to_fit();
    sheet.sections.shrink_to_fit();
    Ok(sheet)
}
fn cell(e: &XmlElement) -> Result<Cell, ReadError> {
    Ok(Cell {
        name: e.attr("N").ok_or_else(|| bad("Cell missing N"))?.into(),
        value: e.attr("V").map(str::to_string),
        formula: e.attr("F").map(str::to_string),
        units: e.attr("U").map(str::to_string),
        error: e.attr("E").map(str::to_string),
        attributes: e
            .attributes
            .iter()
            .filter(|(k, _)| !matches!(k.as_str(), "N" | "V" | "F" | "U" | "E"))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
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
fn parse_pages(
    p: &ReadPackage,
    d: &mut Document,
    part: &str,
    tables: &HashMap<String, RelationshipTable>,
    seen: &mut HashSet<String>,
) -> Result<(), ReadError> {
    let root = part_xml(p, part)?;
    if !root.is_visio("Pages") {
        return Err(bad("invalid Pages root"));
    }
    for e in root.elements().filter(|e| e.is_visio("Page")) {
        let target = if e.child("Rel").is_some() { related(tables, part, e, "page")? } else { String::new() };
        if !target.is_empty() && !seen.insert(misterdiagram_opc::norm(&target)) {
            continue;
        }
        let content = if target.is_empty() {
            XmlElement { name: "PageContents".into(), namespace: e.namespace.clone(), ..Default::default() }
        } else {
            part_xml(p, &target)?
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
            part: target.clone(),
            attributes: e.attributes.clone(),
        });
        if !target.is_empty() {
            retain_xml(p, d, &target, content)?;
        }
    }
    retain_xml(p, d, part, root)?;
    Ok(())
}
fn parse_masters(
    p: &ReadPackage,
    d: &mut Document,
    part: &str,
    tables: &HashMap<String, RelationshipTable>,
    seen: &mut HashSet<String>,
) -> Result<(), ReadError> {
    let root = part_xml(p, part)?;
    if !root.is_visio("Masters") {
        return Err(bad("invalid Masters root"));
    }
    for e in root.elements() {
        if e.is_visio("MasterShortcut") {
            d.master_shortcuts.push(e.clone());
        } else if e.is_visio("Master") {
            let target = if e.child("Rel").is_some() { related(tables, part, e, "master")? } else { String::new() };
            if !target.is_empty() && !seen.insert(misterdiagram_opc::norm(&target)) {
                continue;
            }
            let content = if target.is_empty() {
                XmlElement { name: "MasterContents".into(), namespace: e.namespace.clone(), ..Default::default() }
            } else {
                part_xml(p, &target)?
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
                part: target.clone(),
                attributes: e.attributes.clone(),
            });
            if !target.is_empty() {
                retain_xml(p, d, &target, content)?;
            }
        }
    }
    retain_xml(p, d, part, root)?;
    Ok(())
}

#[cfg(test)]
mod relationship_tests;
