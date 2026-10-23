//! Master copying, instance overrides and document-stencil authoring.
use crate::{Result, Session, edit::*, operation};
use misterdiagram_model::{Document, Shape, ShapeId, Sheet};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

fn next_master(doc: &Document) -> Result<u32> {
    if doc.masters.len() >= 10000 {
        return Err(operation("master count limit"));
    }
    doc.masters
        .iter()
        .map(|m| m.id)
        .max()
        .map_or(Some(2), |v| v.max(1).checked_add(1))
        .filter(|v| *v != u32::MAX)
        .ok_or_else(|| operation("master ID limit"))
}
fn part_name(doc: &Document) -> Result<String> {
    for i in 1..=10001 {
        let part = format!("visio/masters/master{i}.xml");
        if !doc.parts.contains_key(&part) && !doc.masters.iter().any(|m| m.part == part) {
            return Ok(part);
        }
    }
    Err(operation("master part count limit"))
}
fn cells_mut(sheet: &mut Sheet) -> impl Iterator<Item = &mut misterdiagram_model::Cell> {
    sheet.cells.iter_mut().chain(sheet.sections.iter_mut().flat_map(|s| s.cells.iter_mut().chain(s.rows.iter_mut().flat_map(|r| r.cells.iter_mut()))))
}
fn copy_fonts(doc: &mut Document, source: &Document) -> Result<BTreeMap<u32, u32>> {
    let mut map = BTreeMap::new();
    for (ordinal, name) in source.face_names.values().enumerate() {
        if !doc.face_names.values().any(|v| v.eq_ignore_ascii_case(name)) {
            if doc.face_names.len() >= 65536 {
                return Err(operation("font count limit"));
            }
            let id = doc.face_names.keys().max().map_or(Some(0), |v| v.checked_add(1)).ok_or_else(|| operation("font ID limit"))?;
            doc.face_names.insert(id, name.clone());
        }
        let destination = doc.face_names.values().position(|v| v.eq_ignore_ascii_case(name)).ok_or_else(|| operation("font mapping missing"))?;
        map.insert(u32::try_from(ordinal).map_err(operation)?, u32::try_from(destination).map_err(operation)?);
    }
    Ok(map)
}
fn map_fonts(sheet: &mut Sheet, fonts: &BTreeMap<u32, u32>) {
    for c in cells_mut(sheet) {
        if c.name == "Font"
            && let Some(new) = c.value.as_deref().and_then(|v| v.parse::<u32>().ok()).and_then(|id| fonts.get(&id))
        {
            c.value = Some(new.to_string());
            if c.formula.as_deref().is_some_and(|f| f.parse::<u32>().is_ok()) {
                c.formula = None;
            }
        }
    }
}
fn copy_styles(doc: &mut Document, source: &Document, fonts: &BTreeMap<u32, u32>) -> Result<BTreeMap<u32, u32>> {
    let mut map = BTreeMap::new();
    let mut copies = vec![];
    let mut next = doc.styles.iter().map(|s| s.id).max().unwrap_or(0).checked_add(1).ok_or_else(|| operation("style ID overflow"))?;
    for style in &source.styles {
        if let Some(existing) = doc.styles.iter().find(|s| s.name_u.eq_ignore_ascii_case(&style.name_u)) {
            map.insert(style.id, existing.id);
        } else {
            if doc.styles.len().saturating_add(copies.len()) >= 65536 {
                return Err(operation("style count limit"));
            }
            map.insert(style.id, next);
            copies.push(style.clone());
            next = next.checked_add(1).ok_or_else(|| operation("style ID overflow"))?;
        }
    }
    for mut style in copies {
        style.id = *map.get(&style.id).ok_or_else(|| operation("style mapping missing"))?;
        for id in [&mut style.line_style, &mut style.fill_style, &mut style.text_style] {
            if let Some(old) = *id {
                *id = Some(*map.get(&old).ok_or_else(|| operation("parent style missing"))?);
            }
        }
        map_fonts(&mut style.sheet, fonts);
        for cell in cells_mut(&mut style.sheet) {
            if let Some(rgb) =
                cell.value.as_deref().and_then(|v| v.strip_prefix('#')).filter(|v| v.len() == 6).and_then(|v| u32::from_str_radix(v, 16).ok())
            {
                let rgb = [(rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8];
                if cell.formula.is_none()
                    && let Some(index) = (0..24).find(|i| misterdiagram_sheet::functions::default_color(*i) == Some(rgb))
                {
                    cell.value = Some(index.to_string());
                }
            }
        }
        doc.styles.push(style);
    }
    Ok(map)
}
fn map_shape_ids(shapes: &[Shape], start: u32) -> Result<BTreeMap<u32, u32>> {
    let mut map = BTreeMap::new();
    let mut pending = shapes.iter().rev().map(|s| (s, 0usize)).collect::<Vec<_>>();
    let mut next = start;
    while let Some((shape, depth)) = pending.pop() {
        if depth > 128 || map.len() >= 100000 {
            return Err(operation("master shape count or depth limit"));
        }
        if shape.id == u32::MAX && shape.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true") {
            continue;
        }
        if map.insert(shape.id, next).is_some() {
            return Err(operation("duplicate master shape ID"));
        }
        next = next.checked_add(1).filter(|v| *v != u32::MAX).ok_or_else(|| operation("shape ID limit"))?;
        pending.extend(shape.shapes.iter().rev().map(|s| (s, depth + 1)));
    }
    Ok(map)
}
fn imported_ids(shapes: &[Shape]) -> Result<BTreeMap<u32, u32>> {
    let mapped = map_shape_ids(shapes, 5)?;
    if mapped.keys().all(|id| *id >= 5) { Ok(mapped.keys().map(|id| (*id, *id)).collect()) } else { Ok(mapped) }
}
fn rewrite_shapes(
    shapes: &mut [Shape],
    ids: &BTreeMap<u32, u32>,
    styles: &BTreeMap<u32, u32>,
    fonts: &BTreeMap<u32, u32>,
    depth: usize,
) -> Result<()> {
    if depth > 128 {
        return Err(operation("master shape nesting limit"));
    }
    for shape in shapes {
        if shape.id != u32::MAX {
            shape.id = *ids.get(&shape.id).ok_or_else(|| operation("shape ID mapping missing"))?;
        }
        for style in [&mut shape.line_style, &mut shape.fill_style, &mut shape.text_style] {
            if let Some(old) = *style {
                *style = Some(*styles.get(&old).ok_or_else(|| operation("shape style missing"))?);
            }
        }
        map_fonts(&mut shape.sheet, fonts);
        // Newly imported master rows follow the first-save form in the ten-master witness.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-connection-sectionvisio-xml
        for section in &mut shape.sheet.sections {
            if section.name == "Connection" {
                for row in &mut section.rows {
                    if !row.deleted && row.row_type.is_none() {
                        row.row_type = Some("Connection".into());
                    }
                }
            }
            if section.name == "User" {
                for cell in section.rows.iter_mut().flat_map(|r| &mut r.cells) {
                    if cell.units.as_deref() == Some("COLOR") {
                        cell.units = None;
                    }
                }
            }
        }
        for c in cells_mut(&mut shape.sheet) {
            c.ref_by.clear();
            if let Some(f) = &mut c.formula {
                *f = crate::arrange::remap(f, ids)?;
            }
        }
        rewrite_shapes(&mut shape.shapes, ids, styles, fonts, depth + 1)?;
    }
    Ok(())
}
fn copy_parts(
    doc: &mut Document,
    source: &Document,
    root: &str,
    destination: &str,
    master_id: u32,
    budget: &crate::retention::Reservation,
) -> Result<()> {
    if !source.parts.contains_key(root) {
        return Ok(());
    }
    let mut mapping = BTreeMap::from([(root.to_owned(), destination.to_owned())]);
    let mut pending = vec![root.to_owned()];
    while let Some(name) = pending.pop() {
        if mapping.len() > 10000 {
            return Err(operation("master resource count limit"));
        }
        let part = source.parts.get(&name).ok_or_else(|| operation("master resource is missing"))?;
        for rel in &part.relationships {
            if rel.external || mapping.contains_key(&rel.target) {
                continue;
            }
            if !source.parts.contains_key(&rel.target) {
                return Err(operation("master relationship target missing"));
            }
            let target = format!("visio/imported/master{master_id}/{}", rel.target);
            mapping.insert(rel.target.clone(), target);
            pending.push(rel.target.clone());
        }
    }
    let copied_nodes = budget.parts(doc, mapping.keys().filter_map(|name| source.parts.get(name)))?;
    doc.retained_xml_nodes = doc.retained_xml_nodes.saturating_add(copied_nodes);
    for (name, target) in &mapping {
        let mut part = source.parts.get(name).ok_or_else(|| operation("resource missing"))?.clone();
        for rel in &mut part.relationships {
            if !rel.external {
                rel.target = mapping.get(&rel.target).ok_or_else(|| operation("resource mapping missing"))?.clone();
            }
        }
        if doc.parts.contains_key(target) {
            return Err(operation("resource part collision"));
        }
        doc.parts.insert(target.clone(), part);
    }
    Ok(())
}
fn nested_references(
    shapes: &mut [Shape],
    active: Option<u32>,
    masters: &BTreeMap<u32, u32>,
    ids: &BTreeMap<u32, BTreeMap<u32, u32>>,
    depth: usize,
) -> Result<()> {
    if depth > 128 {
        return Err(operation("nested master depth limit"));
    }
    for shape in shapes {
        let source = shape.master.or(active);
        if let Some(old) = shape.master {
            shape.master = Some(*masters.get(&old).ok_or_else(|| operation("nested master mapping missing"))?);
        }
        if let (Some(master), Some(old)) = (source, shape.master_shape) {
            shape.master_shape =
                Some(*ids.get(&master).and_then(|map| map.get(&old)).ok_or_else(|| operation("nested master shape mapping missing"))?);
        }
        nested_references(&mut shape.shapes, source, masters, ids, depth + 1)?;
    }
    Ok(())
}
fn foreign_relationship_ids(shapes: &[Shape]) -> Result<BTreeSet<String>> {
    let mut ids = BTreeSet::new();
    let mut shapes = shapes.iter().collect::<Vec<_>>();
    let mut work = 0usize;
    while let Some(shape) = shapes.pop() {
        shapes.extend(&shape.shapes);
        let mut nodes = shape.foreign_data.iter().collect::<Vec<_>>();
        while let Some(node) = nodes.pop() {
            work += 1;
            if work > 100000 {
                return Err(operation("foreign resource node limit"));
            }
            for (key, value) in &node.attributes {
                if node.attribute_namespaces.get(key).is_some_and(|ns| ns == "http://schemas.openxmlformats.org/officeDocument/2006/relationships") {
                    ids.insert(value.clone());
                }
            }
            nodes.extend(node.elements());
        }
        work += 1;
        if work > 100000 {
            return Err(operation("master shape count limit"));
        }
    }
    Ok(ids)
}
/// Literal custom RGB colors need document table identities on first save.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/colors-element-visiodocument_type-complextypevisio-xml
fn collect_colors(sheet: &Sheet, colors: &mut BTreeSet<String>) {
    for cell in sheet.cells.iter().chain(sheet.sections.iter().flat_map(|s| s.cells.iter().chain(s.rows.iter().flat_map(|r| &r.cells)))) {
        if let Some(value) = cell.value.as_deref().filter(|v| v.len() == 7 && v.starts_with('#') && v.bytes().skip(1).all(|b| b.is_ascii_hexdigit()))
        {
            colors.insert(value.to_ascii_uppercase());
        }
    }
}
/// Native post-save type for an ordinary 1-D master is 1. Special connector
/// types and passive imported documents retain their source attributes.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
fn canonical_imported_type(master: &mut misterdiagram_model::Master, source: &Document) -> Result<()> {
    if master.attributes.get("MasterType").is_some_and(|v| v == "2") {
        let mut roots = master.shapes.iter().filter(|s| !s.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true"));
        if let Some(root) = roots.next().filter(|_| roots.next().is_none()) {
            let e = Evaluator::new(source).map_err(operation)?;
            if e.cell(Scope::MasterShape { master: master.id, shape: root.id }, "BeginX").is_some() {
                master.attributes.insert("MasterType".into(), "1".into());
            }
        }
    }
    Ok(())
}
fn import_master(
    doc: &mut Document,
    source: &Document,
    source_id: u32,
    visiting: &mut BTreeSet<u32>,
    budget: &crate::retention::Reservation,
) -> Result<u32> {
    let master = source.masters.iter().find(|m| m.id == source_id).ok_or_else(|| operation("master missing"))?;
    if let Some(existing) = doc.masters.iter().find(|m| {
        master.unique_id.is_some() && m.unique_id == master.unique_id
            || m.attributes.get("MatchByName").is_some_and(|v| v == "1") && m.name_u.eq_ignore_ascii_case(&master.name_u)
    }) {
        return Ok(existing.id);
    }
    if visiting.len() >= 128 || !visiting.insert(source_id) {
        return Err(operation("master dependency cycle or depth limit"));
    }
    let mut nested = BTreeSet::new();
    let mut pending = master.shapes.iter().collect::<Vec<_>>();
    while let Some(shape) = pending.pop() {
        if let Some(id) = shape.master {
            nested.insert(id);
        }
        pending.extend(&shape.shapes);
        if pending.len() > 100000 {
            return Err(operation("master shape count limit"));
        }
    }
    let mut nested_map = BTreeMap::new();
    let mut nested_ids = BTreeMap::new();
    for id in nested {
        nested_map.insert(id, import_master(doc, source, id, visiting, budget)?);
        let master = source.masters.iter().find(|m| m.id == id).ok_or_else(|| operation("nested master missing"))?;
        nested_ids.insert(id, imported_ids(&master.shapes)?);
    }
    let fonts = copy_fonts(doc, source)?;
    let styles = copy_styles(doc, source, &fonts)?;
    let id = next_master(doc)?;
    let part = part_name(doc)?;
    let mut copy = master.clone();
    canonical_imported_type(&mut copy, source)?;
    copy.id = id;
    copy.icon = None;
    copy.part = part.clone();
    let ids = imported_ids(&copy.shapes)?;
    rewrite_shapes(&mut copy.shapes, &ids, &styles, &fonts, 0)?;
    nested_references(&mut copy.shapes, None, &nested_map, &nested_ids, 0)?;
    for connect in &mut copy.connects {
        if let Some(id) = ids.get(&connect.from_sheet) {
            connect.from_sheet = *id;
        }
        if let Some(id) = ids.get(&connect.to_sheet) {
            connect.to_sheet = *id;
        }
    }
    copy_parts(doc, source, &master.part, &part, id, budget)?;
    if copy.unique_id.is_none() {
        copy.unique_id = misterdiagram_model::skeleton::dynamic_connector().unique_id;
    }
    doc.masters.push(copy);
    let mut colors = BTreeSet::new();
    for style in &doc.styles {
        collect_colors(&style.sheet, &mut colors);
    }
    let mut shapes = doc.masters.iter().flat_map(|m| &m.shapes).collect::<Vec<_>>();
    let mut count = 0usize;
    while let Some(shape) = shapes.pop() {
        count += 1;
        if count > 100000 {
            return Err(operation("master palette traversal limit"));
        }
        collect_colors(&shape.sheet, &mut colors);
        shapes.extend(&shape.shapes);
    }
    for color in colors {
        if !doc.colors.values().any(|v| v.eq_ignore_ascii_case(&color)) {
            let index = doc
                .colors
                .keys()
                .copied()
                .max()
                .unwrap_or(23)
                .checked_add(1)
                .filter(|v| *v < 65536)
                .ok_or_else(|| operation("palette count limit"))?;
            doc.colors.insert(index, color);
        }
    }
    visiting.remove(&source_id);
    Ok(id)
}
fn instance(source: &Shape, ids: &BTreeMap<u32, u32>, master: u32, root: bool, depth: usize) -> Result<Shape> {
    if depth > 128 {
        return Err(operation("instance nesting limit"));
    }
    let id = *ids.get(&source.id).ok_or_else(|| operation("instance ID mapping missing"))?;
    let mut shape = Shape {
        id,
        shape_type: source.shape_type.clone(),
        master: root.then_some(master),
        master_shape: (!root).then_some(source.id),
        ..Default::default()
    };
    for child in &source.shapes {
        if child.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true") {
            continue;
        }
        shape.shapes.push(instance(child, ids, master, false, depth + 1)?);
    }
    Ok(shape)
}
/// A new master omits inherited-only pp IX=0; the source shape is untouched.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pp-element-text_type-complextypevisio-xml
fn canonical_master_text(shapes: &mut [Shape], depth: usize) -> Result<()> {
    if depth > 128 {
        return Err(operation("master text nesting limit"));
    }
    for shape in shapes {
        let local = shape
            .sheet
            .sections
            .iter()
            .filter(|section| section.name == "Paragraph")
            .flat_map(|section| &section.rows)
            .any(|row| row.index.unwrap_or(0) == 0 && row.cells.iter().any(|c| c.formula.as_deref() != Some("Inh")));
        if !local && let Some(text) = &mut shape.text {
            let mut default_active = true;
            text.children.retain(|child| {
                if let misterdiagram_model::XmlNode::Element(e) = child
                    && e.is_visio("pp")
                {
                    let default = e.attr("IX") == Some("0");
                    let redundant = default && default_active && e.attributes.len() == 1;
                    default_active = default;
                    !redundant
                } else {
                    true
                }
            });
        }
        canonical_master_text(&mut shape.shapes, depth + 1)?;
    }
    Ok(())
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.drop
/// https://learn.microsoft.com/en-us/office/vba/api/visio.master.matchbyname
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-element-shapes_type-complextypevisio-xml
pub(crate) fn drop_master(s: &mut Session, p: &Value) -> Result<Value> {
    let page_index = page_index(s, p)?;
    let x = p.get("x").map(|_| number(p, "x")).transpose()?.unwrap_or(0.);
    let y = p.get("y").map(|_| number(p, "y")).transpose()?.unwrap_or(0.);
    let width = p.get("width").map(|_| number(p, "width")).transpose()?;
    let height = p.get("height").map(|_| number(p, "height")).transpose()?;
    if width.is_some_and(|v| v <= 0.) || height.is_some_and(|v| v <= 0.) {
        return Err(operation("master dimensions must be positive"));
    }
    let mut source = crate::stencils::document(s, p)?;
    if p.get("stencil").is_none() && p.get("master").is_some_and(Value::is_string) && crate::stencils::find(&source, p).is_err() {
        let mut matches = s.stencils.iter().filter(|stencil| crate::stencils::find(&stencil.document, p).is_ok());
        source = matches.next().map(|s| s.document.clone()).ok_or_else(|| operation("master not found"))?;
        if matches.next().is_some() {
            return Err(operation("ambiguous master name; supply stencil"));
        }
    }
    let source_id = crate::stencils::find(&source, p)?.id;
    let own = std::sync::Arc::ptr_eq(&source, &s.document);
    let budget = s.reservation();
    s.edit_document(|doc, selection| {
        let master_id = if own { source_id } else { import_master(doc, &source, source_id, &mut BTreeSet::new(), &budget)? };
        let master = doc.masters.iter().find(|m| m.id == master_id).ok_or_else(|| operation("master missing"))?;
        let roots = master.shapes.iter().filter(|s| !s.sheet.attributes.get("Del").is_some_and(|v| v == "1" || v == "true")).collect::<Vec<_>>();
        if roots.len() != 1 {
            return Err(operation("master must have one top-level shape"));
        }
        let root = *roots.first().ok_or_else(|| operation("master has no shape"))?;
        let first = next_id(doc.pages.get(page_index).ok_or_else(|| operation("page missing"))?)?;
        let ids = map_shape_ids(std::slice::from_ref(root), first)?;
        let mut shape = instance(root, &ids, master_id, true, 0)?;
        let page = doc.pages.get(page_index).ok_or_else(|| operation("page missing"))?;
        let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
        let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
        let scale = crate::master_scale::DrawingScale::read(&mut e, Scope::Master(master_id))?
            .factor(crate::master_scale::DrawingScale::read(&mut e, Scope::Page(page.id))?)?;
        let scope = Scope::MasterShape { master: master_id, shape: root.id };
        let line = e.cell(scope, "BeginX").is_some();
        let source_width = if (scale != 1. && width.is_none()) || (line && width.is_some()) {
            Some(e.evaluate(scope, "Width").numeric().map_err(operation)?)
        } else {
            None
        };
        let dropped_width = width.or(source_width.map(|v| v * scale));
        let dropped_height =
            if scale != 1. && height.is_none() { Some(e.evaluate(scope, "Height").numeric().map_err(operation)? * scale) } else { height };
        let mut existing = page.shapes.iter().collect::<Vec<_>>();
        let mut first_instance = true;
        while let Some(candidate) = existing.pop() {
            if candidate.master == Some(master_id) {
                first_instance = false;
                break;
            }
            existing.extend(&candidate.shapes);
        }
        if first_instance {
            shape.name = master.name.clone();
            shape.name_u = master.name_u.clone();
        }

        set_number(&mut shape.sheet, "PinX", x)?;
        set_number(&mut shape.sheet, "PinY", y)?;
        if let Some(width) = dropped_width.filter(|_| !line || p.get("width").is_some()) {
            set_number(&mut shape.sheet, "Width", width)?;
        }
        if let Some(height) = dropped_height {
            set_number(&mut shape.sheet, "Height", height)?;
        }
        if scale != 1. {
            set_number(&mut shape.sheet, "DropOnPageScale", scale)?;
        }
        for cell in &mut shape.sheet.cells {
            if matches!(cell.name.as_str(), "Width" | "Height") {
                cell.units = Some("IN".into());
            }
        }
        // A source master layer is not a layer assignment in the destination page.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/layermember-cell-layer-membership-section
        if e.cell(scope, "LayerMember").is_some_and(|c| c.value.as_deref().is_some_and(|v| !v.is_empty())) {
            set(&mut shape.sheet, "LayerMember", "", None)?;
        }
        if line {
            let px = e.evaluate(scope, "PinX").numeric().map_err(operation)?;
            let py = e.evaluate(scope, "PinY").numeric().map_err(operation)?;
            let endpoint_scale = if let (Some(width), Some(source_width)) = (width, source_width) {
                if source_width <= 0. {
                    return Err(operation("cannot resize a zero-length master line"));
                }
                width / source_width
            } else {
                scale
            };
            for (name, offset) in [("BeginX", x - px), ("BeginY", y - py), ("EndX", x - px), ("EndY", y - py)] {
                let original = e.evaluate(scope, name).numeric().map_err(operation)?;
                let pin = if name.ends_with('X') { px } else { py };
                let value = if endpoint_scale == 1. { original + offset } else { (original - pin) * endpoint_scale + pin + offset };
                set_number(&mut shape.sheet, name, value)?;
            }
        }
        let mut connects = vec![];
        for connect in &master.connects {
            let mut copy = connect.clone();
            copy.from_sheet = *ids.get(&connect.from_sheet).ok_or_else(|| operation("master connector source missing"))?;
            copy.to_sheet = *ids.get(&connect.to_sheet).ok_or_else(|| operation("master connector target missing"))?;
            connects.push(copy);
        }
        let id = shape.id;
        if line {
            let master = doc.masters.iter_mut().find(|m| m.id == master_id).ok_or_else(|| operation("master missing"))?;
            if master.attributes.get("MasterType").is_some_and(|v| v == "2") {
                master.attributes.insert("MasterType".into(), "1".into());
            }
        }
        let page = page_mut(doc, page_index)?;
        page.connects.extend(connects);
        page.shapes.push(shape);
        // Normal 1-D masters derive Width from their endpoints. Keep that formula;
        // a literal or fixed formula needs an override to represent the new width.
        if line && scale != 1. && width.is_none() {
            let expected = dropped_width.ok_or_else(|| operation("scaled master width missing"))?;
            let page_id = doc.pages.get(page_index).ok_or_else(|| operation("page missing"))?.id;
            let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
            let actual = e.evaluate(Scope::PageShape { page: page_id, shape: id }, "Width").numeric();
            if !actual.is_ok_and(|actual| (actual - expected).abs() <= expected.abs().max(1.) * 1e-12) {
                let shape = page_mut(doc, page_index)?.shapes.last_mut().ok_or_else(|| operation("dropped shape missing"))?;
                set_number(&mut shape.sheet, "Width", expected)?;
            }
        }
        selection.page = page_index;
        selection.shapes = vec![ShapeId(id)];
        Ok(json!({"id":id,"master":master_id}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.masters.drop
/// https://learn.microsoft.com/en-us/office/client-developer/visio/master-element-masters_type-complextypevisio-xml
pub(crate) fn new_from_shape(s: &mut Session, p: &Value) -> Result<Value> {
    let selected;
    let p = if p.get("id").is_none() {
        if s.selection.shapes.len() != 1 {
            return Err(operation("select exactly one shape or supply id"));
        }
        let id = s.selection.shapes.first().ok_or_else(|| operation("selection is empty"))?.0;
        selected = crate::cmd::with_param(p, "id", json!(id));
        &selected
    } else {
        p
    };
    let (index, _, scope) = shape_scope(s, p)?;
    let title = name(p, "name")?.to_owned();
    if s.document.masters.iter().any(|m| m.name_u.eq_ignore_ascii_case(&title)) {
        return Err(operation("master name already exists"));
    }
    let mut e = Evaluator::new(&s.document).map_err(operation)?;
    let mut shape = e.shape(scope).ok_or_else(|| operation("shape missing"))?.clone();
    let page_id = s.document.pages.get(index).ok_or_else(|| operation("page not found"))?.id;
    crate::reference_cleanup::isolate(&s.document, page_id, &mut shape)?;
    let one_d = e.cell(scope, "BeginX").is_some();
    let width = e.evaluate(scope, "Width").numeric().map_err(operation)?;
    let height = e.evaluate(scope, "Height").numeric().map_err(operation)?;
    let stroke = if e.evaluate(scope, "LinePattern").numeric().map_err(operation)? == 0. {
        0.
    } else {
        e.evaluate(scope, "LineWeight").numeric().map_err(operation)?
    };
    if !stroke.is_finite() || !(0. ..=1000.).contains(&stroke) {
        return Err(operation("master stroke extent limit"));
    }
    s.edit_document(|doc, _| {
        let mut master = misterdiagram_model::skeleton::dynamic_connector();
        master.id = next_master(doc)?;
        master.part = part_name(doc)?;
        master.name = title.clone();
        master.name_u = title;
        master.icon = None;
        master.attributes.insert("MasterType".into(), if one_d { "1" } else { "2" }.into());
        master.attributes.insert("Prompt".into(), String::new());
        master.attributes.insert("MatchByName".into(), "0".into());
        master.sheet.sections.clear();
        set_number(&mut master.sheet, "DrawingResizeType", 1.)?;
        master.shapes = vec![shape].into();
        canonical_master_text(&mut master.shapes, 0)?;
        master.connects.clear();
        let ids = map_shape_ids(&master.shapes, 5)?;
        let page = doc.pages.get(index).ok_or_else(|| operation("source page missing"))?;
        for connect in &page.connects {
            if let (Some(from), Some(to)) = (ids.get(&connect.from_sheet), ids.get(&connect.to_sheet)) {
                let mut copy = connect.clone();
                copy.from_sheet = *from;
                copy.to_sheet = *to;
                master.connects.push(copy);
            }
        }
        if let Some(source) = doc.parts.get(&page.part) {
            let referenced = foreign_relationship_ids(&master.shapes)?;
            let relationships = source.relationships.iter().filter(|rel| referenced.contains(&rel.id)).cloned().collect();
            doc.parts.insert(
                master.part.clone(),
                misterdiagram_model::Part {
                    relationships,
                    content_type: Some("application/vnd.ms-visio.master+xml".into()),
                    xml: Some(
                        misterdiagram_model::XmlElement {
                            name: "MasterContents".into(),
                            namespace: "http://schemas.microsoft.com/office/visio/2012/main".into(),
                            ..Default::default()
                        }
                        .into(),
                    ),
                    ..Default::default()
                },
            );
        }

        let styles = doc.styles.iter().map(|s| (s.id, s.id)).collect();
        rewrite_shapes(&mut master.shapes, &ids, &styles, &BTreeMap::new(), 0)?;
        let root = master.shapes.first_mut().ok_or_else(|| operation("master shape missing"))?;
        set_number(&mut root.sheet, "PinX", (width + stroke) / 2.)?;
        set_number(&mut root.sheet, "PinY", (height + stroke) / 2.)?;
        set_number(&mut master.sheet, "PageWidth", (width + stroke).max(1e-9))?;
        set_number(&mut master.sheet, "PageHeight", (height + stroke).max(1e-9))?;
        let id = master.id;
        doc.masters.push(master);
        Ok(json!({"master":id}))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
    #[test]
    fn nested_resource_copies_reserve_cumulative_nodes_before_allocation() {
        let mut destination = misterdiagram_model::skeleton::document(false);
        destination.retained_xml_nodes = misterdiagram_vsdx::MAX_RETAINED_XML_NODES - 2;
        let mut source = misterdiagram_model::skeleton::document(false);
        for (name, xml) in [("first.xml", "<Original/>"), ("second.xml", "<Original><Child/></Original>")] {
            source.parts.insert(
                name.into(),
                misterdiagram_model::Part { xml: Some(misterdiagram_vsdx::parse_xml(xml.as_bytes()).unwrap().into()), ..Default::default() },
            );
        }
        let budget = Session::new().reservation();
        copy_parts(&mut destination, &source, "first.xml", "copy1.xml", 2, &budget).unwrap();
        assert_eq!(destination.retained_xml_nodes, misterdiagram_vsdx::MAX_RETAINED_XML_NODES - 1);
        let before = destination.clone();
        let error = copy_parts(&mut destination, &source, "second.xml", "copy2.xml", 3, &budget).unwrap_err().to_string();
        assert!(error.contains("retained XML node limit"), "{error}");
        assert_eq!(destination, before, "refuse before allocating the second resource tree");
    }
    /// Returning from a local paragraph run to inherited row0 needs its pp marker.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/pp-element-text_type-complextypevisio-xml
    #[test]
    fn authored_master_keeps_paragraph_reset_to_inherited_formatting() {
        use misterdiagram_model::{XmlElement, XmlNode};
        let namespace = "http://schemas.microsoft.com/office/visio/2012/main";
        let mut text = XmlElement { name: "Text".into(), namespace: namespace.into(), ..Default::default() };
        for ix in [0, 1, 0] {
            text.children.push(XmlNode::Element(XmlElement {
                name: "pp".into(),
                namespace: namespace.into(),
                attributes: [("IX".into(), ix.to_string())].into(),
                ..Default::default()
            }));
            text.children.push(XmlNode::Text("paragraph\n".into()));
        }
        let mut shapes = vec![Shape { text: Some(text), ..Default::default() }];
        canonical_master_text(&mut shapes, 0).unwrap();
        let indices = shapes[0].text.as_ref().unwrap().elements().map(|e| e.attr("IX").unwrap()).collect::<Vec<_>>();
        assert_eq!(indices, vec!["1", "0"]);
    }
    #[test]
    fn nested_mastershape_ids_follow_reserved_id_remapping() {
        let mut source = misterdiagram_model::skeleton::document(false);
        source.masters[0].shapes[0].id = 4;
        let mut outer = source.masters[0].clone();
        outer.id = 3;
        outer.unique_id = Some("{OWN-OUTER}".into());
        outer.name_u = "Outer".into();
        outer.part.clear();
        outer.shapes = vec![Shape { id: 8, shape_type: Some("Shape".into()), master: Some(2), master_shape: Some(4), ..Default::default() }].into();
        source.masters.push(outer);
        let mut destination = misterdiagram_model::skeleton::document(false);
        let id = import_master(&mut destination, &source, 3, &mut BTreeSet::new(), &Session::new().reservation()).unwrap();
        let master = destination.masters.iter().find(|m| m.id == id).unwrap();
        assert_eq!(master.shapes[0].master_shape, Some(5));
        assert!(Evaluator::new(&destination).unwrap().evaluate(Scope::MasterShape { master: id, shape: 8 }, "Width").numeric().is_ok());
    }
    #[test]
    fn hostile_master_dependency_cycles_roll_back_at_the_command_boundary() {
        let mut source = misterdiagram_model::skeleton::document(false);
        source.masters[0].name_u = "Cyclic master".into();
        source.masters[0].shapes[0].master = Some(2);
        let mut destination = misterdiagram_model::skeleton::document(false);
        assert!(import_master(&mut destination, &source, 2, &mut BTreeSet::new(), &Session::new().reservation()).is_err());
        let mut s = Session::new();
        let before = s.document().clone();
        s.stencils.push(crate::stencils::Stencil { path: std::path::PathBuf::from("cycle.vssx"), document: std::sync::Arc::new(source) });
        assert!(s.execute("master.drop", &json!({"stencil":1,"master":2,"x":1,"y":1})).is_err());
        assert_eq!(s.document(), &before);
    }
}
