//! Grouping and arrangement preserve local coordinates in their common parent.
use crate::{Result, Session, edit::*, operation};
use diagramcraft_geom::{Affine, Point, Rect};
use diagramcraft_model::{Shape, ShapeId, Sheet};
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

fn ids(p: &Value) -> Result<Vec<u32>> {
    let values = p
        .get("ids")
        .and_then(Value::as_array)
        .filter(|v| !v.is_empty() && v.len() <= 10000)
        .ok_or_else(|| operation("ids must contain 1 through 10000 shapes"))?;
    let ids = values
        .iter()
        .map(|v| v.as_u64().and_then(|v| u32::try_from(v).ok()).ok_or_else(|| operation("invalid shape ID")))
        .collect::<Result<Vec<_>>>()?;
    if ids.iter().collect::<BTreeSet<_>>().len() != ids.len() {
        return Err(operation("duplicate selection ID"));
    }
    Ok(ids)
}
fn parent(shapes: &[Shape], id: u32, owner: Option<u32>, depth: usize) -> Result<Option<Option<u32>>> {
    if depth > 128 {
        return Err(operation("shape nesting limit"));
    }
    for shape in shapes {
        if shape.id == id {
            return Ok(Some(owner));
        }
        if let Some(found) = parent(&shape.shapes, id, Some(shape.id), depth + 1)? {
            return Ok(Some(found));
        }
    }
    Ok(None)
}
fn siblings(shapes: &mut Vec<Shape>, owner: Option<u32>) -> Result<&mut Vec<Shape>> {
    match owner {
        None => Ok(shapes),
        Some(id) => Ok(&mut shape_mut(shapes, id)?.shapes),
    }
}
struct Frame {
    pin: Point,
    local: Point,
    width: f64,
    height: f64,
    transform: Affine,
    ends: Option<[Point; 2]>,
}
fn frame(e: &mut Evaluator<'_>, scope: Scope) -> Result<Frame> {
    let mut n = |name: &str| -> Result<f64> {
        match e.evaluate(scope, name).numeric() {
            Ok(n) if n.is_finite() && n.abs() <= 1e6 => Ok(n),
            _ => Err(operation(format!("invalid {name}"))),
        }
    };
    let pin = Point::new(n("PinX")?, n("PinY")?);
    let local = Point::new(n("LocPinX")?, n("LocPinY")?);
    let width = n("Width")?;
    let height = n("Height")?;
    let transform = diagramcraft_geom::shape_transform(pin, local, n("Angle")?, n("FlipX")? != 0., n("FlipY")? != 0.).map_err(operation)?;
    let ends = if e.cell(scope, "BeginX").is_some() {
        Some([
            Point::new(e.evaluate(scope, "BeginX").numeric().map_err(operation)?, e.evaluate(scope, "BeginY").numeric().map_err(operation)?),
            Point::new(e.evaluate(scope, "EndX").numeric().map_err(operation)?, e.evaluate(scope, "EndY").numeric().map_err(operation)?),
        ])
    } else {
        None
    };
    Ok(Frame { pin, local, width, height, transform, ends })
}
fn translate(sheet: &mut Sheet, f: &Frame, dx: f64, dy: f64) -> Result<()> {
    if let Some([a, b]) = f.ends {
        for (n, v) in [("BeginX", a.x + dx), ("BeginY", a.y + dy), ("EndX", b.x + dx), ("EndY", b.y + dy)] {
            set_number(sheet, n, v)?;
        }
    } else {
        set_number(sheet, "PinX", f.pin.x + dx)?;
        set_number(sheet, "PinY", f.pin.y + dy)?;
    }
    Ok(())
}
fn translate_detached(page: &mut diagramcraft_model::Page, id: u32, f: &Frame, dx: f64, dy: f64) -> Result<()> {
    if dx == 0. && dy == 0. {
        return Ok(());
    }
    if let Some([a, b]) = f.ends {
        crate::reference_cleanup::move_ends(page, id, [a.x, a.y, b.x, b.y], [a.x + dx, a.y + dy, b.x + dx, b.y + dy])
    } else {
        translate(&mut shape_mut(&mut page.shapes, id)?.sheet, f, dx, dy)
    }
}
fn frames(s: &Session, index: usize, ids: &[u32]) -> Result<(Option<u32>, BTreeMap<u32, Frame>)> {
    let page = s.document.pages.get(index).ok_or_else(|| operation("page not found"))?;
    let first = *ids.first().ok_or_else(|| operation("empty selection"))?;
    let owner = parent(&page.shapes, first, None, 0)?.ok_or_else(|| operation("shape not found"))?;
    let mut evaluator = Evaluator::new(&s.document).map_err(operation)?;
    let mut result = BTreeMap::new();
    for id in ids {
        if parent(&page.shapes, *id, None, 0)? != Some(owner) {
            return Err(operation("selected shapes must share a parent"));
        }
        result.insert(*id, frame(&mut evaluator, Scope::PageShape { page: page.id, shape: *id })?);
    }
    Ok((owner, result))
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.selection.group
pub(crate) fn group(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let ids = ids(p)?;
    if ids.len() < 2 {
        return Err(operation("group needs at least two shapes"));
    }
    let (owner, frames) = frames(s, index, &ids)?;
    let mut bounds: Option<Rect> = None;
    for f in frames.values() {
        let r = f.transform.transform_rect_bbox(Rect::new(0., 0., f.width, f.height));
        bounds = Some(bounds.map_or(r, |b| b.union(r)));
    }
    let bounds = bounds.ok_or_else(|| operation("empty bounds"))?;
    s.edit_document(|doc, selection| {
        let page = page_mut(doc, index)?;
        let id = next_id(page)?;
        let mut group = diagramcraft_model::skeleton::rectangle(id, bounds.x0, bounds.y0, bounds.width().max(1e-9), bounds.height().max(1e-9), "")
            .map_err(operation)?;
        group.shape_type = Some("Group".into());
        group.sheet.sections.clear();
        let glued = page
            .connects
            .iter()
            .filter_map(|c| c.from_cell.as_deref().filter(|n| matches!(*n, "BeginX" | "EndX")).map(|n| (c.from_sheet, n.to_owned())))
            .collect::<BTreeSet<_>>();
        let shapes = siblings(&mut page.shapes, owner)?;
        let position = shapes.iter().position(|s| ids.contains(&s.id)).ok_or_else(|| operation("shape missing"))?;
        let mut i = 0;
        while i < shapes.len() {
            let selected = shapes.get(i).is_some_and(|s| ids.contains(&s.id));
            if selected {
                let mut child = shapes.remove(i);
                let glue_formulas = child
                    .sheet
                    .cells
                    .iter()
                    .filter_map(|c| {
                        let key = if c.name.starts_with("Begin") {
                            "BeginX"
                        } else if c.name.starts_with("End") {
                            "EndX"
                        } else {
                            ""
                        };
                        glued.contains(&(child.id, key.to_owned())).then(|| (c.name.clone(), c.formula.clone()))
                    })
                    .collect::<BTreeMap<_, _>>();
                translate(&mut child.sheet, frames.get(&child.id).ok_or_else(|| operation("frame missing"))?, -bounds.x0, -bounds.y0)?;
                let frame = frames.get(&child.id).ok_or_else(|| operation("frame missing"))?;
                let width = bounds.width().max(1e-9);
                let height = bounds.height().max(1e-9);
                let values = if let Some([a, b]) = frame.ends {
                    vec![
                        ("BeginX", a.x - bounds.x0, "Width", width),
                        ("BeginY", a.y - bounds.y0, "Height", height),
                        ("EndX", b.x - bounds.x0, "Width", width),
                        ("EndY", b.y - bounds.y0, "Height", height),
                    ]
                } else {
                    vec![
                        ("PinX", frame.pin.x - bounds.x0, "Width", width),
                        ("PinY", frame.pin.y - bounds.y0, "Height", height),
                        ("Width", frame.width, "Width", width),
                        ("Height", frame.height, "Height", height),
                    ]
                };
                for (name, value, axis, extent) in values {
                    let formula = format!("Sheet.{id}!{axis}*{}", value / extent);
                    let formula = glue_formulas.get(name).map_or(Some(formula.as_str()), |f| f.as_deref());
                    set(&mut child.sheet, name, &value.to_string(), formula)?;
                }
                group.shapes.push(child);
            } else {
                i += 1;
            }
        }
        shapes.insert(position, group);
        selection.page = index;
        selection.shapes = vec![ShapeId(id)];
        Ok(json!({"id":id}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.ungroup
pub(crate) fn ungroup(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, scope) = shape_scope(s, p)?;
    let page = s.document.pages.get(index).ok_or_else(|| operation("page not found"))?;
    let owner = parent(&page.shapes, id, None, 0)?.ok_or_else(|| operation("shape not found"))?;
    let mut evaluator = Evaluator::new(&s.document).map_err(operation)?;
    let shape = evaluator.shape(scope).ok_or_else(|| operation("shape not found"))?;
    if shape.shapes.is_empty() {
        return Err(operation("shape is not a nonempty group"));
    }
    let children = shape.shapes.iter().map(|s| s.id).collect::<Vec<_>>();
    let group = frame(&mut evaluator, scope)?;
    let mut child_frames = BTreeMap::new();
    for child in &children {
        child_frames.insert(*child, frame(&mut evaluator, Scope::PageShape { page: page.id, shape: *child })?);
    }
    let detached = crate::reference_cleanup::removing(&s.document, page.id, id)?;
    s.edit_document(|doc, selection| {
        for (scope, cell) in detached {
            crate::reference_cleanup::apply(crate::sheet::sheet_mut(doc, scope)?, cell)?;
        }
        let page = page_mut(doc, index)?;
        page.connects.retain(|c| c.from_sheet != id && c.to_sheet != id);
        let glued = page
            .connects
            .iter()
            .filter_map(|c| c.from_cell.as_deref().filter(|n| matches!(*n, "BeginX" | "EndX")).map(|n| (c.from_sheet, n.to_owned())))
            .collect::<BTreeSet<_>>();
        let shapes = siblings(&mut page.shapes, owner)?;
        let position = shapes.iter().position(|s| s.id == id).ok_or_else(|| operation("group not found"))?;
        let group_shape = shapes.remove(position);
        for (offset, mut child) in group_shape.shapes.into_iter().enumerate() {
            let f = child_frames.get(&child.id).ok_or_else(|| operation("frame missing"))?;
            if let Some([a, b]) = f.ends {
                let a = group.transform * a;
                let b = group.transform * b;
                for (n, v) in [("BeginX", a.x), ("BeginY", a.y), ("EndX", b.x), ("EndY", b.y)] {
                    let key = if n.starts_with("Begin") { "BeginX" } else { "EndX" };
                    let formula = glued.contains(&(child.id, key.to_owned())).then(|| child.sheet.cell(n).and_then(|c| c.formula.clone())).flatten();
                    set_number(&mut child.sheet, n, v)?;
                    if let Some(formula) = formula {
                        set(&mut child.sheet, n, &v.to_string(), Some(&formula))?;
                    }
                }
            } else {
                set_number(&mut child.sheet, "Width", f.width)?;
                set_number(&mut child.sheet, "Height", f.height)?;
                let transform = group.transform * f.transform;
                let pin = transform * f.local;
                let [a, b, _, _, _, _] = transform.as_coeffs();
                for (n, v) in [
                    ("PinX", pin.x),
                    ("PinY", pin.y),
                    ("Angle", b.atan2(a)),
                    ("FlipX", 0.),
                    ("FlipY", if transform.determinant() < 0. { 1. } else { 0. }),
                ] {
                    set_number(&mut child.sheet, n, v)?;
                }
            }
            shapes.insert(position + offset, child);
        }
        page.connects.retain(|c| c.from_sheet != id && c.to_sheet != id);
        selection.page = index;
        selection.shapes = children.iter().copied().map(ShapeId).collect();
        Ok(json!({"ids":children}))
    })
}
/// Rewrite numeric Sheet references without changing quoted string contents.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
pub(crate) fn remap(formula: &str, ids: &BTreeMap<u32, u32>) -> Result<String> {
    if formula.len() > 65536 {
        return Err(operation("formula byte limit"));
    }
    let mut out = String::new();
    let mut at = 0;
    let mut quoted = false;
    while let Some(tail) = formula.get(at..).filter(|s| !s.is_empty()) {
        let c = tail.chars().next().ok_or_else(|| operation("formula character missing"))?;
        if c == '"' {
            quoted = !quoted;
        }
        if !quoted && tail.get(..6).is_some_and(|s| s.eq_ignore_ascii_case("Sheet.")) {
            let digits = tail.get(6..).ok_or_else(|| operation("formula range"))?.bytes().take_while(u8::is_ascii_digit).count();
            let end = 6 + digits;
            if digits > 0 && tail.get(end..end + 1) == Some("!") {
                let old = tail.get(6..end).ok_or_else(|| operation("formula reference range"))?.parse::<u32>().map_err(operation)?;
                if let Some(new) = ids.get(&old) {
                    out.push_str(&format!("Sheet.{new}!"));
                    at += end + 1;
                    if out.len() > 65536 {
                        return Err(operation("rewritten formula limit"));
                    }
                    continue;
                }
            }
        }
        out.push(c);
        at += c.len_utf8();
        if out.len() > 65536 {
            return Err(operation("rewritten formula limit"));
        }
    }
    Ok(out)
}
fn visit(shape: &mut Shape, ids: &BTreeMap<u32, u32>, depth: usize) -> Result<()> {
    if depth > 128 {
        return Err(operation("shape nesting limit"));
    }
    shape.id = *ids.get(&shape.id).ok_or_else(|| operation("shape ID mapping missing"))?;
    shape.name.clear();
    shape.name_u.clear();
    shape.sheet.attributes.remove("UniqueID");
    for cell in shape
        .sheet
        .cells
        .iter_mut()
        .chain(shape.sheet.sections.iter_mut().flat_map(|s| s.cells.iter_mut().chain(s.rows.iter_mut().flat_map(|r| r.cells.iter_mut()))))
    {
        cell.ref_by.clear();
        if let Some(f) = &mut cell.formula {
            *f = remap(f, ids)?;
        }
    }
    for child in &mut shape.shapes {
        visit(child, ids, depth + 1)?;
    }
    Ok(())
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.duplicate
pub(crate) fn duplicate(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, scope) = shape_scope(s, p)?;
    let dx = p.get("dx").map(|_| number(p, "dx")).transpose()?.unwrap_or(0.25);
    let dy = p.get("dy").map(|_| number(p, "dy")).transpose()?.unwrap_or(-0.25);
    let page = s.document.pages.get(index).ok_or_else(|| operation("page not found"))?;
    let owner = parent(&page.shapes, id, None, 0)?.ok_or_else(|| operation("shape missing"))?;
    let mut e = Evaluator::new(&s.document).map_err(operation)?;
    let f = frame(&mut e, scope)?;
    let mut copy = e.shape(scope).ok_or_else(|| operation("shape missing"))?.clone();
    let mut map = BTreeMap::new();
    let mut next = next_id(page)?;
    let mut pending = vec![(&copy, 0usize)];
    while let Some((shape, depth)) = pending.pop() {
        if depth > 128 || map.len() >= 100000 {
            return Err(operation("duplicate shape limit"));
        }
        map.insert(shape.id, next);
        next = next.checked_add(1).filter(|v| *v != u32::MAX).ok_or_else(|| operation("shape ID limit"))?;
        pending.extend(shape.shapes.iter().map(|s| (s, depth + 1)));
    }
    crate::reference_cleanup::isolate(&s.document, page.id, &mut copy)?;
    visit(&mut copy, &map, 0)?;
    translate(&mut copy.sheet, &f, dx, dy)?;
    let new_id = copy.id;
    s.edit_document(|doc, selection| {
        let page = page_mut(doc, index)?;
        let copies = page
            .connects
            .iter()
            .filter_map(|c| {
                let from = map.get(&c.from_sheet)?;
                let to = map.get(&c.to_sheet)?;
                let mut clone = c.clone();
                clone.from_sheet = *from;
                clone.to_sheet = *to;
                Some(clone)
            })
            .collect::<Vec<_>>();
        page.connects.extend(copies);
        siblings(&mut page.shapes, owner)?.push(copy);
        selection.page = index;
        selection.shapes = vec![ShapeId(new_id)];
        Ok(json!({"id":new_id}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.selection.align
pub(crate) fn align(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let ids = ids(p)?;
    let (_, frames) = frames(s, index, &ids)?;
    let mode = name(p, "mode")?;
    let reference = frames.get(ids.first().ok_or_else(|| operation("empty selection"))?).ok_or_else(|| operation("frame missing"))?;
    let target = reference.transform.transform_rect_bbox(Rect::new(0., 0., reference.width, reference.height));
    let mut shifts = vec![];
    for (id, f) in &frames {
        let b = f.transform.transform_rect_bbox(Rect::new(0., 0., f.width, f.height));
        let delta = match mode {
            "left" => (target.x0 - b.x0, 0.),
            "right" => (target.x1 - b.x1, 0.),
            "center" => (target.center().x - b.center().x, 0.),
            "bottom" => (0., target.y0 - b.y0),
            "top" => (0., target.y1 - b.y1),
            "middle" => (0., target.center().y - b.center().y),
            _ => return Err(operation("invalid alignment")),
        };
        shifts.push((*id, delta));
    }
    s.edit_document(|doc, _| {
        for (id, (dx, dy)) in shifts {
            translate_detached(page_mut(doc, index)?, id, frames.get(&id).ok_or_else(|| operation("frame missing"))?, dx, dy)?;
        }
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.selection.distribute
pub(crate) fn distribute(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let ids = ids(p)?;
    if ids.len() < 3 {
        return Err(operation("distribution needs at least three shapes"));
    }
    let (_, frames) = frames(s, index, &ids)?;
    let axis = name(p, "axis")?;
    if !matches!(axis, "x" | "y") {
        return Err(operation("invalid axis"));
    }
    let mut centers = frames
        .iter()
        .map(|(id, f)| {
            let b = f.transform.transform_rect_bbox(Rect::new(0., 0., f.width, f.height));
            (*id, if axis == "x" { b.center().x } else { b.center().y })
        })
        .collect::<Vec<_>>();
    centers.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
    let start = centers.first().ok_or_else(|| operation("empty selection"))?.1;
    let end = centers.last().ok_or_else(|| operation("empty selection"))?.1;
    let step = (end - start) / (centers.len() - 1) as f64;
    s.edit_document(|doc, _| {
        for (i, (id, old)) in centers.iter().enumerate() {
            let delta = start + step * i as f64 - old;
            translate_detached(
                page_mut(doc, index)?,
                *id,
                frames.get(id).ok_or_else(|| operation("frame missing"))?,
                if axis == "x" { delta } else { 0. },
                if axis == "y" { delta } else { 0. },
            )?;
        }
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/width-cell-shape-transform-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/height-cell-shape-transform-section
pub(crate) fn same_size(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let ids = ids(p)?;
    let (_, frames) = frames(s, index, &ids)?;
    let axis = p.get("axis").and_then(Value::as_str).unwrap_or("both");
    if !matches!(axis, "width" | "height" | "both") {
        return Err(operation("invalid size axis"));
    }
    let reference = frames.get(ids.first().ok_or_else(|| operation("empty selection"))?).ok_or_else(|| operation("frame missing"))?;
    s.edit_document(|doc, _| {
        for id in &ids {
            let f = frames.get(id).ok_or_else(|| operation("frame missing"))?;
            let page = page_mut(doc, index)?;
            if axis != "height" && f.width != reference.width {
                if let Some([a, b]) = f.ends {
                    let angle = (b.y - a.y).atan2(b.x - a.x);
                    let center = Point::new((a.x + b.x) / 2., (a.y + b.y) / 2.);
                    let dx = reference.width * angle.cos() / 2.;
                    let dy = reference.width * angle.sin() / 2.;
                    crate::reference_cleanup::move_ends(
                        page,
                        *id,
                        [a.x, a.y, b.x, b.y],
                        [center.x - dx, center.y - dy, center.x + dx, center.y + dy],
                    )?;
                } else {
                    set_number(&mut shape_mut(&mut page.shapes, *id)?.sheet, "Width", reference.width)?;
                }
            }
            if axis != "width" && f.height != reference.height {
                set_number(&mut shape_mut(&mut page.shapes, *id)?.sheet, "Height", reference.height)?;
            }
        }
        Ok(json!({}))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remapping_preserves_strings_and_rejects_hostile_formula_lengths() {
        let ids = BTreeMap::from([(1, 42)]);
        assert_eq!(remap("Sheet.1!Width+LEN(\"Sheet.1!Width\")", &ids).unwrap(), "Sheet.42!Width+LEN(\"Sheet.1!Width\")");
        assert_eq!(remap("LEN(\"a\"\"Sheet.1!Width\")+Sheet.1!Width", &ids).unwrap(), "LEN(\"a\"\"Sheet.1!Width\")+Sheet.42!Width");
        assert!(remap(&"x".repeat(65537), &ids).is_err());
        assert!(remap("Sheet.999999999999999999999999!Width", &ids).is_err());
    }
}
