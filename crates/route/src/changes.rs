//! Transaction-local placement evidence, independent of page-wide routing policy.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/objtype-cell-miscellaneous-section
use crate::orthogonal::number;
use misterdiagram_geom::{Affine, Point, Rect};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use std::collections::BTreeMap;

#[derive(Clone, PartialEq)]
struct Placement {
    transform: Affine,
    width: f64,
    height: f64,
    bounds: Rect,
    placeable: bool,
}
#[derive(Default)]
pub(crate) struct PageChanges {
    moved: BTreeMap<u32, (Option<Placement>, Placement)>,
}
/// Zero-valued shapes are promoted to one by the existing glue context rule.
/// Group eight contributes its contained placeables, including nested groups.
pub(crate) fn placeable(e: &mut Evaluator<'_>, scope: Scope) -> Result<bool, String> {
    let value = number(e, scope, "ObjType", 0.);
    if value == 1. {
        return Ok(true);
    }
    if value != 8. && value != 12. {
        return Ok(false);
    }
    let Scope::PageShape { page, .. } = scope else { return Ok(false) };
    let mut pending = e.shape(scope).into_iter().flat_map(|s| s.shapes.iter().map(|c| (c.id, 1usize))).collect::<Vec<_>>();
    let mut count = 0usize;
    while let Some((shape, depth)) = pending.pop() {
        count = count.checked_add(1).ok_or("route dependency count limit")?;
        if count > 100000 || depth >= 128 {
            return Err("route dependency count limit".into());
        }
        let scope = Scope::PageShape { page, shape };
        let value = number(e, scope, "ObjType", 0.);
        if value == 1. {
            return Ok(true);
        }
        if value == 8. || value == 12. {
            let next = depth.checked_add(1).ok_or("route dependency count limit")?;
            if let Some(s) = e.shape(scope) {
                if pending.len().checked_add(s.shapes.len()).is_none_or(|n| n > 100000) {
                    return Err("route dependency count limit".into());
                }
                pending.extend(s.shapes.iter().map(|c| (c.id, next)));
            }
        }
    }
    Ok(false)
}
fn placement(e: &mut Evaluator<'_>, scope: Scope) -> Result<Placement, String> {
    let transform = e.shape_to_page(scope).map_err(|v| v.to_string())?;
    let width = number(e, scope, "Width", 0.);
    let height = number(e, scope, "Height", 0.);
    let bounds = transform.transform_rect_bbox(Rect::new(0., 0., width, height));
    if [bounds.x0, bounds.y0, bounds.x1, bounds.y1].iter().any(|v| !v.is_finite() || v.abs() > 1e6) {
        return Err("route box limit".into());
    }
    Ok(Placement { transform, width, height, bounds, placeable: placeable(e, scope)? })
}
impl PageChanges {
    pub(crate) fn collect(e: &mut Evaluator<'_>, old: &mut Evaluator<'_>, page: u32) -> Result<Self, String> {
        let scopes = e.scopes().filter(|s| matches!(s, Scope::PageShape {page:p,..} if *p==page)).collect::<Vec<_>>();
        if scopes.len() > 100000 {
            return Err("route dependency count limit".into());
        }
        let mut out = Self::default();
        for scope in scopes {
            let Scope::PageShape { shape, .. } = scope else { continue };
            let after = placement(e, scope)?;
            let before = if old.shape(scope).is_some() { Some(placement(old, scope)?) } else { None };
            if before.as_ref() != Some(&after) {
                out.moved.insert(shape, (before, after));
            }
        }
        Ok(out)
    }
    pub(crate) fn crosses(&self, e: &mut Evaluator<'_>, scope: Scope, work: &mut usize) -> Result<bool, String> {
        if !self.moved.values().any(|(a, b)| b.placeable && a.as_ref().is_none_or(|a| a.bounds != b.bounds)) {
            return Ok(false);
        }
        let Scope::PageShape { page, shape } = scope else { return Ok(false) };
        let segments = segments(e, scope)?;
        // SimpleNS uses the obstacle bounds without inflation in the solver.
        let distance = if crate::orthogonal::style(e, scope, page) == 17. {
            (0., 0.)
        } else {
            (number(e, Scope::Page(page), "LineToLineX", 0.125) * 1.5, number(e, Scope::Page(page), "LineToLineY", 0.125) * 1.5)
        };
        for (id, (before, after)) in &self.moved {
            if *id == shape || !after.placeable || before.as_ref().is_some_and(|b| b.bounds == after.bounds) {
                continue;
            }
            let b = Rect::new(after.bounds.x0 - distance.0, after.bounds.y0 - distance.1, after.bounds.x1 + distance.0, after.bounds.y1 + distance.1);
            for &(a, z) in &segments {
                *work = work.checked_add(1).ok_or("route scope work limit")?;
                if *work > 4_000_000 {
                    return Err("route scope work limit".into());
                }
                if intersects(a, z, b) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
/// Saved route polyline in page space. MoveTo starts a new component; jump arcs
/// contribute their endpoint chord without changing any serialized row.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
pub(crate) fn segments(e: &mut Evaluator<'_>, scope: Scope) -> Result<Vec<(Point, Point)>, String> {
    let transform = e.shape_to_page(scope).map_err(|v| v.to_string())?;
    let mut rows = BTreeMap::new();
    for n in e.resolved_named_cells(scope) {
        if n.section.is_some_and(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0)
            && let Some(row) = n.row.filter(|r| !r.deleted)
        {
            rows.insert(row.index.unwrap_or(0), row.row_type.clone());
            if rows.len() > 16384 {
                return Err("route row count limit".into());
            }
        }
    }
    let mut out = Vec::new();
    let mut previous = None;
    for (ix, kind) in rows {
        let mut x = number(e, scope, &format!("Geometry1.X{ix}"), 0.);
        let mut y = number(e, scope, &format!("Geometry1.Y{ix}"), 0.);
        if kind.as_deref().is_some_and(|k| k.starts_with("Rel")) {
            x *= number(e, scope, "Width", 0.);
            y *= number(e, scope, "Height", 0.);
        }
        let p = transform * Point::new(x, y);
        if !p.x.is_finite() || !p.y.is_finite() || p.x.abs() > 1e6 || p.y.abs() > 1e6 {
            return Err("route coordinate limit".into());
        }
        if !matches!(kind.as_deref(), Some("MoveTo" | "RelMoveTo"))
            && let Some(a) = previous
        {
            out.push((a, p));
        }
        previous = Some(p);
    }
    Ok(out)
}
fn intersects(a: Point, b: Point, r: Rect) -> bool {
    let mut low: f64 = 0.;
    let mut high: f64 = 1.;
    for (start, delta, min, max) in [(a.x, b.x - a.x, r.x0, r.x1), (a.y, b.y - a.y, r.y0, r.y1)] {
        if delta.abs() < 1e-12 {
            if start < min || start > max {
                return false;
            }
        } else {
            let x = (min - start) / delta;
            let y = (max - start) / delta;
            low = low.max(x.min(y));
            high = high.min(x.max(y));
            if low > high {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use misterdiagram_model::{Cell, skeleton};
    #[test]
    fn segment_intersection_does_not_use_the_whole_route_box() {
        let b = Rect::new(1., 1., 2., 2.);
        assert!(!intersects(Point::new(0., 0.), Point::new(3., 0.), b));
        assert!(intersects(Point::new(0., 0.), Point::new(3., 3.), b));
        assert!(!intersects(Point::new(0., 0.), Point::new(0., 3.), b));
    }
    #[test]
    fn hostile_coordinate_and_scope_work_have_named_errors() {
        let mut d = skeleton::document(false);
        d.pages[0].shapes.push(skeleton::rectangle(1, 0., 0., 1., 1., "").unwrap());
        let shape = &mut d.pages[0].shapes[0];
        shape.sheet.sections[0].rows[0].cells = vec![
            Cell { name: "X".into(), value: Some("1000000".into()), ..Default::default() },
            Cell { name: "Y".into(), value: Some("0".into()), ..Default::default() },
        ];
        shape.sheet.cells.iter_mut().find(|c| c.name == "PinX").unwrap().value = Some("999999".into());
        let scope = Scope::PageShape { page: 0, shape: 1 };
        assert_eq!(segments(&mut Evaluator::new(&d).unwrap(), scope).unwrap_err(), "route coordinate limit");
        let mut d = skeleton::document(false);
        d.pages[0].shapes.push(skeleton::rectangle(1, 0., 0., 1., 1., "").unwrap());
        let mut after = d.clone();
        let mover = &mut after.pages[0].shapes[0];
        mover.sheet.cells.push(Cell { name: "ObjType".into(), value: Some("1".into()), ..Default::default() });
        mover.sheet.cells.iter_mut().find(|c| c.name == "PinX").unwrap().value = Some("2".into());
        let mut e = Evaluator::new(&after).unwrap();
        let mut old = Evaluator::new(&d).unwrap();
        let mut changes = PageChanges::collect(&mut e, &mut old, 0).unwrap();
        let moved = changes.moved.remove(&1).unwrap();
        changes.moved.insert(2, moved);
        assert_eq!(changes.crosses(&mut old, scope, &mut 4_000_000).unwrap_err(), "route scope work limit");
    }
}
