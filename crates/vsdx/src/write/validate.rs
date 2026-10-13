//! Validate caller-built models before recursive cloning or XML serialization.
use super::*;
struct Budget {
    bytes: usize,
    nodes: usize,
}
impl Budget {
    fn nodes(&mut self, count: usize) -> Result<(), WriteError> {
        self.nodes = self.nodes.checked_add(count).ok_or_else(|| error("model node overflow"))?;
        if self.nodes > 4_000_000 {
            return Err(error("model node limit"));
        }
        Ok(())
    }
    fn string(&mut self, s: &str) -> Result<(), WriteError> {
        self.bytes = self.bytes.checked_add(s.len()).ok_or_else(|| error("model byte overflow"))?;
        if self.bytes > diagramcraft_opc::MAX_TOTAL as usize {
            return Err(error("model byte limit"));
        }
        Ok(())
    }
    fn attrs(&mut self, a: &BTreeMap<String, String>) -> Result<(), WriteError> {
        self.nodes(a.len())?;
        for (k, v) in a {
            self.string(k)?;
            self.string(v)?;
        }
        Ok(())
    }
    fn xml(&mut self, e: &XmlElement) -> Result<(), WriteError> {
        let mut stack = vec![(e, 0usize)];
        while let Some((e, depth)) = stack.pop() {
            self.nodes = self.nodes.saturating_add(1);
            if depth >= 256 || self.nodes > 4_000_000 {
                return Err(error("model XML node/depth limit"));
            }
            self.string(&e.name)?;
            self.string(&e.namespace)?;
            self.attrs(&e.attributes)?;
            self.attrs(&e.attribute_namespaces)?;
            if self.nodes.saturating_add(stack.len()).saturating_add(e.children.len()) > 4_000_000 {
                return Err(error("model XML node limit"));
            }
            for n in &e.children {
                match n {
                    XmlNode::Text(s) => {
                        self.nodes(1)?;
                        self.string(s)?;
                    }
                    XmlNode::Element(e) => stack.push((e, depth + 1)),
                }
            }
        }
        Ok(())
    }
    fn cell(&mut self, c: &Cell) -> Result<(), WriteError> {
        self.nodes(1)?;
        self.string(&c.name)?;
        for s in [&c.value, &c.formula, &c.units, &c.error].into_iter().flatten() {
            self.string(s)?;
        }
        self.attrs(&c.attributes)?;
        for e in &c.ref_by {
            self.xml(e)?;
        }
        Ok(())
    }
    fn sheet(&mut self, s: &Sheet) -> Result<(), WriteError> {
        let start = self.bytes;
        if s.sections.len() > 16_384 {
            return Err(error("sheet section limit"));
        }
        self.nodes(s.sections.len())?;
        let mut count = s.cells.len();
        if count > crate::limits::MAX_SHEET_CELLS {
            return Err(error("sheet cell limit"));
        }
        self.attrs(&s.attributes)?;
        for c in &s.cells {
            self.cell(c)?;
        }
        for section in &s.sections {
            let mut cells = section.cells.len();
            for row in &section.rows {
                cells = cells.saturating_add(row.cells.len());
            }
            if cells > crate::limits::MAX_SECTION_CELLS || section.rows.len() > crate::limits::MAX_SECTION_ROWS {
                return Err(error("section cell/row limit"));
            }
            count = count.saturating_add(cells);
            if count > crate::limits::MAX_SHEET_CELLS {
                return Err(error("sheet cell limit"));
            }
            self.nodes(section.rows.len())?;
            self.string(&section.name)?;
            self.attrs(&section.attributes)?;
            for c in &section.cells {
                self.cell(c)?;
            }
            for r in &section.rows {
                self.attrs(&r.attributes)?;
                for s in [&r.name, &r.row_type].into_iter().flatten() {
                    self.string(s)?;
                }
                for c in &r.cells {
                    self.cell(c)?;
                }
            }
        }
        for t in &s.triggers {
            self.xml(t)?;
        }
        if self.bytes - start > crate::limits::MAX_SHEET_BYTES {
            return Err(error("sheet byte limit"));
        }
        Ok(())
    }
    fn shapes(&mut self, shapes: &[Shape]) -> Result<(), WriteError> {
        if shapes.len() > 100_000 {
            return Err(error("shape count limit"));
        }
        let mut stack: Vec<_> = shapes.iter().map(|s| (s, 0usize)).collect();
        let mut ids = BTreeSet::new();
        let mut count = 0usize;
        while let Some((s, depth)) = stack.pop() {
            count += 1;
            // The UML witness uses repeated UINT_MAX IDs for deleted inherited members.
            // Their MasterShape, not the sentinel ID, identifies the tombstone.
            let deleted_member =
                s.id == u32::MAX && s.master_shape.is_some() && s.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true");
            if depth > 128 || count > 100_000 || (!deleted_member && !ids.insert(s.id)) {
                return Err(error("shape IDs or nesting limit"));
            }
            self.string(&s.name)?;
            self.string(&s.name_u)?;
            if let Some(kind) = &s.shape_type {
                self.string(kind)?;
            }
            self.sheet(&s.sheet)?;
            for x in [&s.text, &s.foreign_data].into_iter().flatten() {
                self.xml(x)?;
            }
            if count.saturating_add(stack.len()).saturating_add(s.shapes.len()) > 100_000 {
                return Err(error("shape count limit"));
            }
            for c in &s.shapes {
                stack.push((c, depth + 1));
            }
        }
        Ok(())
    }
}
pub(crate) fn document(d: &Document) -> Result<(), WriteError> {
    if d.parts.len() > diagramcraft_opc::MAX_ENTRIES || d.pages.len() > 100_000 || d.masters.len() > 100_000 || d.styles.len() > 65536 {
        return Err(error("model collection limit"));
    }
    let mut b = Budget { bytes: 0, nodes: 0 };
    if d.colors.len() > 65536 || d.face_names.len() > 65536 {
        return Err(error("palette/font count limit"));
    }
    for value in d.colors.values().chain(d.face_names.values()) {
        b.string(value)?;
    }
    let mut destinations = BTreeSet::new();
    for (i, p) in d.pages.iter().enumerate() {
        // Empty imported stencil entries have no content destination.
        if super::sheet_only(d, p) {
            continue;
        }
        let path = if p.part.is_empty() { format!("visio/pages/page{}.xml", i + 1) } else { p.part.clone() };
        destination(d, &mut destinations, &path, "PageContents")?;
    }
    for (i, m) in d.masters.iter().enumerate() {
        let path = if m.part.is_empty() { format!("visio/masters/master{}.xml", i + 1) } else { m.part.clone() };
        destination(d, &mut destinations, &path, "MasterContents")?;
    }
    let mut names = BTreeSet::new();
    for (name, p) in &d.parts {
        valid_path(name)?;
        if !names.insert(name.to_ascii_lowercase()) || p.bytes.len() as u64 > diagramcraft_opc::MAX_PART {
            return Err(error("duplicate part or part byte limit"));
        }
        b.bytes = b.bytes.checked_add(p.bytes.len()).ok_or_else(|| error("part byte overflow"))?;
        b.string(name)?;
        if let Some(e) = &p.xml {
            b.xml(e)?;
        }
        b.rels(&p.relationships)?;
        if let Some(kind) = &p.content_type {
            b.string(kind)?;
        }
    }
    valid_path(&d.main_part)?;
    b.rels(&d.root_relationships)?;
    b.sheet(&d.document_sheet)?;
    for e in [&d.document_settings, &d.event_list].into_iter().flatten() {
        b.xml(e)?;
    }
    for e in &d.master_shortcuts {
        b.xml(e)?;
    }
    let mut ids = BTreeSet::new();
    for s in &d.styles {
        if !ids.insert(s.id) {
            return Err(error("duplicate style ID"));
        }
        b.string(&s.name)?;
        b.string(&s.name_u)?;
        b.sheet(&s.sheet)?;
    }
    let mut ids = BTreeSet::new();
    for p in &d.pages {
        if !ids.insert(p.id) {
            return Err(error("duplicate page ID"));
        }
        if !p.part.is_empty() {
            valid_path(&p.part)?;
        }
        b.string(&p.name)?;
        b.string(&p.name_u)?;
        b.attrs(&p.attributes)?;
        b.connects(&p.connects)?;
        b.sheet(&p.sheet)?;
        b.shapes(&p.shapes)?;
        connect_targets(&p.connects, &p.shapes)?;
    }
    background_chains(d)?;
    let mut ids = BTreeSet::new();
    for m in &d.masters {
        if !ids.insert(m.id) {
            return Err(error("duplicate master ID"));
        }
        if !m.part.is_empty() {
            valid_path(&m.part)?;
        }
        b.string(&m.name)?;
        b.string(&m.name_u)?;
        for value in [&m.unique_id, &m.base_id].into_iter().flatten() {
            b.string(value)?;
        }
        b.attrs(&m.attributes)?;
        b.connects(&m.connects)?;
        b.sheet(&m.sheet)?;
        b.shapes(&m.shapes)?;
        connect_targets(&m.connects, &m.shapes)?;
        master_ids(d, m)?;
        if let Some(i) = &m.icon {
            b.xml(i)?;
        }
    }
    Ok(())
}
pub(super) fn valid_path(name: &str) -> Result<(), WriteError> {
    if name.is_empty()
        || name.len() > 4096
        || name.starts_with('/')
        || name.contains(['\\', '\0', ':'])
        || name.split('/').any(|c| matches!(c, "" | "." | ".."))
    {
        return Err(error("invalid OPC part path"));
    }
    Ok(())
}
fn rels(rs: &[Relationship]) -> Result<(), WriteError> {
    if rs.len() > 65536 {
        return Err(error("relationship limit"));
    }
    let mut ids = BTreeSet::new();
    for r in rs {
        if r.id.is_empty() || r.id.len() > 4096 || r.target.len() > 65536 || r.type_uri.len() > 65536 || !ids.insert(&r.id) {
            return Err(error("invalid relationship"));
        }
        if !r.external {
            valid_path(&r.target)?;
        }
    }
    Ok(())
}

fn destination(d: &Document, seen: &mut BTreeSet<String>, path: &str, root: &str) -> Result<(), WriteError> {
    valid_path(path)?;
    if !seen.insert(path.to_ascii_lowercase()) {
        return Err(error("duplicate modelled part destination"));
    }
    if d.parts.keys().any(|name| name != path && name.eq_ignore_ascii_case(path)) {
        return Err(error("case-colliding part destination"));
    }
    if let Some(p) = d.parts.get(path)
        && !p.xml.as_ref().is_some_and(|e| e.is_visio(root))
    {
        return Err(error("modelled destination would overwrite another part"));
    }
    Ok(())
}
impl Budget {
    fn rels(&mut self, rs: &[Relationship]) -> Result<(), WriteError> {
        rels(rs)?;
        self.nodes(rs.len())?;
        for r in rs {
            self.string(&r.id)?;
            self.string(&r.type_uri)?;
            self.string(&r.target)?;
        }
        Ok(())
    }
    fn connects(&mut self, cs: &[Connect]) -> Result<(), WriteError> {
        if cs.len() > 100_000 {
            return Err(error("connect count limit"));
        }
        self.nodes(cs.len())?;
        for c in cs {
            self.attrs(&c.attributes)?;
            for value in [&c.from_cell, &c.to_cell].into_iter().flatten() {
                self.string(value)?;
            }
        }
        Ok(())
    }
}

/// Revalidate bounded instance caches after inherited values have been materialized.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
pub(super) fn sheet(sheet: &Sheet) -> Result<usize, WriteError> {
    let mut budget = Budget { bytes: 0, nodes: 0 };
    budget.sheet(sheet)?;
    Ok(budget.bytes)
}

/// Authored master content starts at ID 5. Preserve only witnessed source ID 4.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
/// Observed: corpus/_vm/out/diag-masters/probe-log.txt, tm-v0 and tm-v1-shapeid5.
fn master_ids(d: &Document, m: &Master) -> Result<(), WriteError> {
    let mut pending: Vec<_> = m.shapes.iter().collect();
    let mut reserved = false;
    while let Some(shape) = pending.pop() {
        reserved |= shape.id < 5;
        pending.extend(&shape.shapes);
    }
    if !reserved {
        return Ok(());
    }
    let mut original = BTreeSet::new();
    if let Some(part) = d.parts.get(&m.part).filter(|p| !p.bytes.is_empty()) {
        let root = crate::parse_xml(&part.bytes).map_err(|e| error(e.to_string()))?;
        let mut pending: Vec<_> = root.child("Shapes").into_iter().flat_map(|s| s.elements()).filter(|s| s.is_visio("Shape")).collect();
        while let Some(node) = pending.pop() {
            if node.is_visio("Shape") && node.attr("ID").and_then(|id| id.parse::<u32>().ok()) == Some(4) {
                original.insert(4);
            }
            pending.extend(node.child("Shapes").into_iter().flat_map(|s| s.elements()).filter(|s| s.is_visio("Shape")));
        }
    }
    let mut pending: Vec<_> = m.shapes.iter().collect();
    while let Some(shape) = pending.pop() {
        if shape.id < 5 && !original.contains(&shape.id) {
            return Err(error(format!("master {} content shape ID {} must start at 5", m.id, shape.id)));
        }
        pending.extend(&shape.shapes);
    }
    Ok(())
}

/// Connect endpoints must name existing shapes in their page or master contents.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
fn connect_targets(connects: &[Connect], shapes: &[Shape]) -> Result<(), WriteError> {
    // Sheet 0 is the enclosing PageSheet, not a missing drawing shape.
    // Witness: BLOCKP_M.vssx master2.xml, Controls.Row_1 -> Sheet.0!User.VPX.
    let mut ids = BTreeSet::from([0]);
    let mut pending: Vec<_> = shapes.iter().collect();
    while let Some(shape) = pending.pop() {
        if shape.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true") {
            continue;
        }
        ids.insert(shape.id);
        pending.extend(&shape.shapes);
    }
    if connects.iter().any(|c| !ids.contains(&c.from_sheet) || !ids.contains(&c.to_sheet)) {
        return Err(error("Connect references missing sheet"));
    }
    Ok(())
}

/// Validate the bounded BackPage graph before writing page relationships.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/page-element-pages_type-complextypevisio-xml
fn background_chains(d: &Document) -> Result<(), WriteError> {
    let pages: BTreeMap<_, _> = d.pages.iter().map(|p| (p.id, p)).collect();
    let mut complete = BTreeSet::new();
    for page in &d.pages {
        let mut path = BTreeSet::new();
        let mut next = Some(page.id);
        while let Some(id) = next {
            if complete.contains(&id) {
                break;
            }
            if !path.insert(id) {
                return Err(error("background page cycle"));
            }
            let page = pages.get(&id).ok_or_else(|| error("missing background page"))?;
            next = page.attributes.get("BackPage").map(|v| v.parse::<u32>().map_err(|_| error("invalid background page ID"))).transpose()?;
        }
        complete.extend(path);
    }
    Ok(())
}
