//! Runtime glue from target transforms and Connects, never from a precomputed endpoint cache.
use crate::static_glue::{End, cell, set, shape_mut};
use diagramcraft_geom::{Affine, Point};
use diagramcraft_model::Document;
use diagramcraft_sheet::{HostHooks, Value};
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use std::collections::BTreeMap;

type Key = (u32, u32, String);
#[derive(Clone)]
struct BoxShape {
    transform: Affine,
    width: f64,
    height: f64,
    min: Point,
    max: Point,
    points: Vec<(Point, [f64; 2])>,
}
fn number(e: &mut Evaluator<'_>, scope: Scope, name: &str) -> Result<f64, String> {
    let n = e.evaluate(scope, name).numeric().map_err(|e| e.to_string())?;
    if n.is_finite() && n.abs() <= 1e6 { Ok(n) } else { Err("glue coordinate limit".into()) }
}
fn bounds(e: &mut Evaluator<'_>, page: u32, shape: u32) -> Result<BoxShape, String> {
    let scope = Scope::PageShape { page, shape };
    let width = number(e, scope, "Width")?;
    let height = number(e, scope, "Height")?;
    let transform = e.shape_to_page(scope).map_err(|e| e.to_string())?;
    let corners = [Point::new(0., 0.), Point::new(width, 0.), Point::new(width, height), Point::new(0., height)].map(|p| transform * p);
    let mut min = Point::new(f64::INFINITY, f64::INFINITY);
    let mut max = Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
    for p in corners {
        if !p.x.is_finite() || !p.y.is_finite() || p.x.abs() > 1e6 || p.y.abs() > 1e6 {
            return Err("glue transform limit".into());
        }
        min.x = min.x.min(p.x);
        min.y = min.y.min(p.y);
        max.x = max.x.max(p.x);
        max.y = max.y.max(p.y);
    }
    let mut points = vec![];
    for named in e.resolved_named_cells(scope) {
        if named.cell.name != "X" || !named.section.is_some_and(|s| s.name == "Connection") {
            continue;
        }
        if points.len() >= 16384 {
            return Err("connection point count limit".into());
        }
        let row = named.row.ok_or("connection row missing")?;
        let sec = named.section.ok_or("connection section missing")?;
        let col = |name: &str| diagramcraft_vsdx::evaluate::cell_name(sec, row, 0, &cell(name, 0, None)).map_err(|e| e.to_string());
        let x = number(e, scope, &named.name)?;
        let y = number(e, scope, &col("Y")?)?;
        let dx = number(e, scope, &col("DirX")?)?;
        let dy = number(e, scope, &col("DirY")?)?;
        let kind = number(e, scope, &col("Type")?)?;
        if kind == 0. || kind == 2. {
            let vector = transform * Point::new(dx, dy) - transform * Point::ORIGIN;
            points.push((transform * Point::new(x, y), [vector.x, vector.y]));
        }
    }
    Ok(BoxShape { transform, width, height, min, max, points })
}
fn center(b: &BoxShape) -> Point {
    Point::new((b.min.x + b.max.x) * 0.5, (b.min.y + b.max.y) * 0.5)
}
/// Intersect the facing ray with the transformed alignment rectangle.
fn facing(b: &BoxShape, aim: Point) -> Result<Point, String> {
    let determinant = b.transform.determinant();
    if !determinant.is_finite() || determinant.abs() < 1e-15 {
        return Err("singular glue target transform".into());
    }
    let inv = b.transform.inverse();
    let a = inv * aim;
    let c = Point::new(b.width / 2., b.height / 2.);
    let d = a - c;
    let tx = if d.x.abs() > 1e-12 { (b.width.abs() / 2.) / d.x.abs() } else { f64::INFINITY };
    let ty = if d.y.abs() > 1e-12 { (b.height.abs() / 2.) / d.y.abs() } else { f64::INFINITY };
    let t = tx.min(ty);
    if !t.is_finite() {
        return Ok(b.transform * c);
    }
    Ok(b.transform * (c + d * t))
}
fn endpoint(b: &BoxShape, other: &BoxShape, vertical: bool) -> Result<Point, String> {
    let c = center(b);
    let far = center(other);
    // The four straight-route witnesses align the ends inside the overlap of both boxes.
    let low_y = b.min.y.max(other.min.y);
    let high_y = b.max.y.min(other.max.y);
    let low_x = b.min.x.max(other.min.x);
    let high_x = b.max.x.min(other.max.x);
    let horizontal_overlap = low_y <= high_y;
    let vertical_overlap = low_x <= high_x;
    let preferred_vertical = vertical && vertical_overlap || !horizontal_overlap && vertical_overlap;
    let candidate = if preferred_vertical {
        Point::new((low_x + high_x) * 0.5, if far.y >= c.y { b.max.y } else { b.min.y })
    } else if horizontal_overlap {
        Point::new(if far.x >= c.x { b.max.x } else { b.min.x }, (low_y + high_y) * 0.5)
    } else {
        facing(b, far)?
    };
    let mut best = None;
    for (point, dir) in &b.points {
        let horizontal = dir[0].abs() >= dir[1].abs();
        if horizontal == preferred_vertical && dir != &[0., 0.] {
            continue;
        }
        let d = (*point - far).hypot2();
        if best.is_none_or(|(_, old)| d < old) {
            best = Some((*point, d));
        }
    }
    if let Some((p, _)) = best {
        return Ok(p);
    }
    // A nonrotated alignment rectangle accepts the common overlap coordinate directly.
    let coeff = b.transform.as_coeffs();
    if coeff.get(1).is_some_and(|v| v.abs() < 1e-12) && coeff.get(2).is_some_and(|v| v.abs() < 1e-12) { Ok(candidate) } else { facing(b, candidate) }
}
/// Glue hooks for the current edit, indexed by page, connector and calling cell.
/// `_WALKGLUE` and `_XFTRIGGER` are private formula names observed in the corpus.
/// The public contract describes the EventXFMod dependency and endpoint recomputation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/begtrigger-cell-glue-info-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/endtrigger-cell-glue-info-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/walkpreference-cell-glue-info-section
#[derive(Default)]
pub struct GlueHost {
    points: BTreeMap<Key, Point>,
    triggers: BTreeMap<Key, f64>,
}
impl HostHooks for GlueHost {
    fn supports(&self, name: &str) -> bool {
        matches!(name, "_WALKGLUE" | "_XFTRIGGER")
    }
    fn hook_at(&mut self, page: Option<u32>, shape: Option<u32>, cell: &str, name: &str, _args: &[Value], cached: &Value) -> Value {
        let (Some(page), Some(shape)) = (page, shape) else { return cached.clone() };
        let key = (page, shape, cell.to_ascii_lowercase());
        match name {
            "_WALKGLUE" => self.points.get(&key).map_or_else(|| cached.clone(), |p| Value::Point(p.x, p.y)),
            "_XFTRIGGER" => self.triggers.get(&key).map_or_else(|| cached.clone(), |v| Value::Number(*v)),
            _ => cached.clone(),
        }
    }
}
impl GlueHost {
    /// Compare transforms against the transaction snapshot and invalidate only affected glue triggers.
    /// Endpoint caches are not filled here: evaluation must call `_WALKGLUE` to obtain the new point.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/eventxfmod-cell-events-section
    pub fn prepare(before: &Document, doc: &mut Document) -> Result<Self, String> {
        let mut e = Evaluator::new(doc).map_err(|e| e.to_string())?;
        let mut old = Evaluator::new(before).map_err(|e| e.to_string())?;
        let mut host = Self::default();
        let mut changes = vec![];
        let mut affected = std::collections::BTreeSet::new();
        let mut count = 0usize;
        for (page_index, page) in doc.pages.iter().enumerate() {
            let mut links = BTreeMap::<u32, [Option<&diagramcraft_model::Connect>; 2]>::new();
            for c in &page.connects {
                count += 1;
                if count > 100000 {
                    return Err("glue connection count limit".into());
                }
                let side = match c.from_cell.as_deref() {
                    Some("BeginX") => 0,
                    Some("EndX") => 1,
                    _ => continue,
                };
                let slot = links.entry(c.from_sheet).or_default().get_mut(side).ok_or("endpoint index")?;
                if slot.is_some() {
                    return Err("ambiguous endpoint glue".into());
                }
                *slot = Some(c);
            }
            for (id, links) in links {
                let scope = Scope::PageShape { page: page.id, shape: id };
                let dynamic = |c: Option<&diagramcraft_model::Connect>| c.is_some_and(|c| matches!(c.to_cell.as_deref(), Some("PinX" | "PinY")));
                let boxes = links.map(|c| c.map(|c| bounds(&mut e, page.id, c.to_sheet)).transpose()).into_iter().collect::<Result<Vec<_>, _>>()?;
                let preference = number(&mut e, scope, "WalkPreference")?;
                let parent = e.parent_to_page(scope).map_err(|e| e.to_string())?;
                if !parent.determinant().is_finite() || parent.determinant().abs() < 1e-15 {
                    return Err("singular connector parent transform".into());
                }
                let mut inputs_changed =
                    old.parent_to_page(scope).ok() != Some(parent) || number(&mut old, scope, "WalkPreference").ok() != Some(preference);
                for (side, link) in links.iter().enumerate() {
                    if link.is_none() {
                        let prefix = if side == 0 { "Begin" } else { "End" };
                        for axis in ["X", "Y"] {
                            let name = format!("{prefix}{axis}");
                            inputs_changed |= number(&mut old, scope, &name).ok() != number(&mut e, scope, &name).ok();
                        }
                    }
                }
                if inputs_changed {
                    affected.insert((page_index, id));
                }
                for (side, link) in links.iter().enumerate() {
                    let Some(link) = link else { continue };
                    let end = if side == 0 { End::Begin } else { End::End };
                    let target = boxes.get(side).and_then(Option::as_ref).ok_or("glue target missing")?;
                    let old_target = bounds(&mut old, page.id, link.to_sheet).ok();
                    let changed = inputs_changed
                        || old_target.is_none_or(|o| {
                            o.transform != target.transform || o.width != target.width || o.height != target.height || o.points != target.points
                        })
                        || !before.pages.iter().find(|p| p.id == page.id).is_some_and(|p| p.connects.contains(link));
                    let prior = e.evaluate(scope, end.trigger()).numeric().map_err(|e| e.to_string())?;
                    let counter = if changed { prior + 1. } else { prior };
                    if !counter.is_finite() || counter > 9007199254740991. {
                        return Err("glue trigger counter limit".into());
                    }
                    host.triggers.insert((page.id, id, end.trigger().to_ascii_lowercase()), counter);
                    if changed {
                        changes.push((page_index, id, end.trigger(), counter));
                        affected.insert((page_index, id));
                    }
                    if dynamic(Some(link)) {
                        let opposite = boxes.get(1usize.saturating_sub(side)).and_then(Option::as_ref);
                        let fallback;
                        let other = if let Some(other) = opposite {
                            other
                        } else {
                            let prefix = if side == 0 { "End" } else { "Begin" };
                            let x = number(&mut e, scope, &format!("{prefix}X"))?;
                            let y = number(&mut e, scope, &format!("{prefix}Y"))?;
                            let point = parent * Point::new(x, y);
                            fallback = BoxShape { transform: Affine::IDENTITY, width: 0., height: 0., min: point, max: point, points: vec![] };
                            &fallback
                        };
                        let vertical = link.to_cell.as_deref() == Some("PinY")
                            || if side == 0 { preference == 1. || preference == 3. } else { preference == 2. || preference == 3. };
                        let point = parent.inverse() * endpoint(target, other, vertical)?;
                        for axis in ["x", "y"] {
                            host.points.insert((page.id, id, format!("{}{axis}", end.prefix().to_ascii_lowercase())), point);
                        }
                    }
                }
            }
        }
        drop(e);
        let active: std::collections::BTreeSet<_> =
            affected.iter().filter_map(|(index, id)| doc.pages.get(*index).map(|page| (page.id, *id))).collect();
        host.points.retain(|(page, shape, _), _| active.contains(&(*page, *shape)));
        for (page, id, name, value) in changes {
            let page = doc.pages.get_mut(page).ok_or("glue page missing")?;
            let shape = shape_mut(&mut page.shapes, id, 0).ok_or("glue shape missing")?;
            let formula = shape.sheet.cell(name).and_then(|c| c.formula.clone());
            set(&mut shape.sheet, cell(name, value, formula))?;
        }
        let mut centers = Vec::new();
        {
            let mut e = Evaluator::new(doc).map_err(|e| e.to_string())?.with_glue_host(&mut host);
            for (index, id) in affected {
                let page = doc.pages.get(index).ok_or("glue page missing")?;
                let scope = Scope::PageShape { page: page.id, shape: id };
                if e.resolved_named_cells(scope).iter().any(|n| n.name == "Controls.TextPosition") {
                    centers.push((index, id, number(&mut e, scope, "Width")? / 2., number(&mut e, scope, "Height")? / 2.));
                }
            }
        }
        for (index, id, x, y) in centers {
            let page = doc.pages.get_mut(index).ok_or("glue page missing")?;
            let shape = shape_mut(&mut page.shapes, id, 0).ok_or("glue shape missing")?;
            if !shape.sheet.sections.iter().any(|s| s.name == "Control" && !s.deleted) {
                shape.sheet.sections.push(diagramcraft_model::Section { name: "Control".into(), ..Default::default() });
            }
            let section = shape.sheet.sections.iter_mut().find(|s| s.name == "Control" && !s.deleted).ok_or("control section missing")?;
            if !section.rows.iter().any(|r| r.name.as_deref() == Some("TextPosition") && !r.deleted) {
                section.rows.push(diagramcraft_model::Row { name: Some("TextPosition".into()), ..Default::default() });
            }
            let row = section.rows.iter_mut().find(|r| r.name.as_deref() == Some("TextPosition") && !r.deleted).ok_or("text control row missing")?;
            for (name, value) in [("X", x), ("Y", y), ("XDyn", x), ("YDyn", y)] {
                let c = cell(name, value, name.ends_with("Dyn").then(|| "Inh".into()));
                if let Some(old) = row.cells.iter_mut().find(|c| c.name == name) {
                    *old = c;
                } else {
                    row.cells.push(c);
                }
            }
        }
        Ok(host)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn drawing() -> Document {
        let mut doc = diagramcraft_model::skeleton::document(false);
        let page = &mut doc.pages[0];
        page.shapes.push(diagramcraft_model::skeleton::rectangle(1, 1., 6., 1.5, 1.5, "").unwrap());
        page.shapes.push(diagramcraft_model::skeleton::rectangle(2, 5., 6., 1.5, 1.5, "").unwrap());
        diagramcraft_model::skeleton::glue_dynamic(page, 3, 2, [1, 2], [[2.5, 6.75], [5., 6.75]]).unwrap();
        doc
    }
    #[test]
    fn walkglue_is_computed_by_host_not_by_replacing_endpoint_cache() {
        let before = drawing();
        let mut after = before.clone();
        set(&mut after.pages[0].shapes[1].sheet, cell("PinX", 7.75, None)).unwrap();
        set(&mut after.pages[0].shapes[1].sheet, cell("PinY", 5.75, None)).unwrap();
        let mut host = GlueHost::prepare(&before, &mut after).unwrap();
        assert_eq!(after.pages[0].shapes[2].sheet.cell("BeginY").unwrap().value.as_deref(), Some("6.75"));
        assert_eq!(host.hook_at(Some(0), Some(3), "BeginY", "_WALKGLUE", &[], &Value::Number(6.75)), Value::Point(2.5, 6.25));
        assert_eq!(host.hook_at(Some(0), Some(3), "EndTrigger", "_XFTRIGGER", &[], &Value::Number(2.)), Value::Number(3.));
        assert_eq!(host.hook_at(Some(0), Some(3), "Other", "unknown", &[], &Value::Number(17.)), Value::Number(17.));
    }
    #[test]
    fn hostile_target_coordinates_and_counter_overflow_return_errors() {
        for value in ["NaN", "inf", "1000001"] {
            let before = drawing();
            let mut after = before.clone();
            set(&mut after.pages[0].shapes[1].sheet, cell("PinX", value, None)).unwrap();
            assert!(GlueHost::prepare(&before, &mut after).is_err());
        }
        let before = drawing();
        let mut after = before.clone();
        set(&mut after.pages[0].shapes[1].sheet, cell("PinX", 7.75, None)).unwrap();
        set(&mut after.pages[0].shapes[2].sheet, cell("EndTrigger", "9007199254740991", Some("_XFTRIGGER(Sheet.2!EventXFMod)".into()))).unwrap();
        assert!(GlueHost::prepare(&before, &mut after).is_err());
    }
}
