//! Original layout commands built on public ShapeSheet cells and witnessed glue.
use crate::{Result, Session, edit::*, operation};
use misterdiagram_geom::Point;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Side {
    Left,
    Right,
    Bottom,
    Top,
}
fn number(e: &mut Evaluator<'_>, scope: Scope, name: &str) -> Result<f64> {
    let value = e.evaluate(scope, name).numeric().map_err(operation)?;
    if value.is_finite() && value.abs() <= 1e6 { Ok(value) } else { Err(operation(format!("invalid {name}"))) }
}
fn facing(doc: &misterdiagram_model::Document, page: usize, id: u32, target: u32) -> Result<Side> {
    let page = doc.pages.get(page).ok_or_else(|| operation("page missing"))?;
    let mut e = Evaluator::new(doc).map_err(operation)?;
    let scope = Scope::PageShape { page: page.id, shape: id };
    let other = Scope::PageShape { page: page.id, shape: target };
    for scope in [scope, other] {
        if e.shape(scope).is_none() || e.cell(scope, "BeginX").is_some() {
            return Err(operation("layout endpoints must be two-dimensional shapes"));
        }
    }
    let (w, h) = (number(&mut e, scope, "Width")?, number(&mut e, scope, "Height")?);
    if w <= 0. || h <= 0. {
        return Err(operation("layout endpoint must have positive bounds"));
    }
    let other_center =
        e.shape_to_page(other).map_err(operation)? * Point::new(number(&mut e, other, "Width")? / 2., number(&mut e, other, "Height")? / 2.);
    let transform = e.shape_to_page(scope).map_err(operation)?;
    if transform.determinant().abs() < 1e-12 {
        return Err(operation("singular shape transform"));
    }
    let local = transform.inverse() * other_center;
    let dx = (local.x - w / 2.) / w;
    let dy = (local.y - h / 2.) / h;
    if dx.abs() >= dy.abs() { Ok(if dx >= 0. { Side::Right } else { Side::Left }) } else { Ok(if dy >= 0. { Side::Top } else { Side::Bottom }) }
}
/// Reuse an inbound point at the requested side position, or append the seven
/// canonical cells observed in P5a route/side-glue and connector/static-glue.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/x-cell-connection-points-section
fn point(doc: &mut misterdiagram_model::Document, page: usize, id: u32, side: Side, fraction: f64) -> Result<String> {
    let page_id = doc.pages.get(page).ok_or_else(|| operation("page missing"))?.id;
    let scope = Scope::PageShape { page: page_id, shape: id };
    let mut e = Evaluator::new(doc).map_err(operation)?;
    let w = number(&mut e, scope, "Width")?;
    let h = number(&mut e, scope, "Height")?;
    let (x, y, xf, yf) = match side {
        Side::Left => (0., h * fraction, "0".to_string(), format!("Height*{fraction}")),
        Side::Right => (w, h * fraction, "Width".to_string(), format!("Height*{fraction}")),
        Side::Bottom => (w * fraction, 0., format!("Width*{fraction}"), "0".to_string()),
        Side::Top => (w * fraction, h, format!("Width*{fraction}"), "Height".to_string()),
    };
    let all = e.resolved_named_cells(scope);
    let points = all
        .iter()
        .filter(|n| n.cell.name == "X" && n.section.is_some_and(|s| s.name == "Connection"))
        .map(|n| {
            let companion = |column: &str| {
                all.iter()
                    .find(|v| {
                        v.cell.name == column
                            && v.section.is_some_and(|s| s.name == "Connection")
                            && v.row.zip(n.row).is_some_and(|(a, b)| a.index == b.index && a.name == b.name)
                    })
                    .map(|v| v.name.clone())
                    .ok_or_else(|| operation("incomplete connection row"))
            };
            Ok((n.name.clone(), companion("Y")?, companion("Type")?))
        })
        .collect::<Result<Vec<_>>>()?;
    for (name, y_name, type_name) in points {
        let kind = e.evaluate(scope, &type_name).numeric().map_err(operation)?;
        if kind != 0. && kind != 2. {
            continue;
        }
        if (number(&mut e, scope, &name)? - x).abs() < 1e-9 && (number(&mut e, scope, &y_name)? - y).abs() < 1e-9 {
            return Ok(name);
        }
    }
    drop(e);
    let coordinate = |name: &str, value: f64, formula: &str| cell(name, &value.to_string(), (formula != "0").then_some(formula));
    misterdiagram_route::add_connection(
        doc,
        page,
        id,
        misterdiagram_route::ConnectionPoint {
            name: None,
            x: coordinate("X", x, &xf),
            y: coordinate("Y", y, &yf),
            direction: [0., 0.],
            kind: 0,
            auto_gen: false,
            prompt: String::new(),
        },
    )
    .map_err(operation)
}
/// Glue both ends of an existing connector to facing side centers. Connection
/// rows and static PAR(PNT(...)) glue match the original route/side-glue witness.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
pub(crate) fn sides(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let source = uint(p, "source")?;
    let target = uint(p, "target")?;
    if source == target || id == source || id == target {
        return Err(operation("connector and endpoint shapes must be distinct"));
    }
    let begin = facing(&s.document, index, source, target)?;
    let end = facing(&s.document, index, target, source)?;
    s.edit_document(|doc, _| {
        let a = point(doc, index, source, begin, 0.5)?;
        let b = point(doc, index, target, end, 0.5)?;
        misterdiagram_route::connect_static(doc, index, id, misterdiagram_route::End::Begin, source, &a).map_err(operation)?;
        misterdiagram_route::connect_static(doc, index, id, misterdiagram_route::End::End, target, &b).map_err(operation)?;
        Ok(json!({"id":id,"begin":{"shape":source,"point":a},"end":{"shape":target,"point":b}}))
    })
}

fn ids(p: &Value, key: &str) -> Result<Vec<u32>> {
    let values = p
        .get(key)
        .and_then(Value::as_array)
        .filter(|v| !v.is_empty() && v.len() <= 10000)
        .ok_or_else(|| operation(format!("{key} must contain 1 through 10000 IDs")))?;
    let ids = values
        .iter()
        .map(|v| v.as_u64().and_then(|v| u32::try_from(v).ok()).ok_or_else(|| operation("invalid shape ID")))
        .collect::<Result<Vec<_>>>()?;
    if ids.iter().collect::<std::collections::BTreeSet<_>>().len() != ids.len() {
        return Err(operation("duplicate shape ID"));
    }
    Ok(ids)
}
fn rect_value(r: misterdiagram_geom::Rect) -> Value {
    json!({"x":r.x0,"y":r.y0,"width":r.width(),"height":r.height()})
}
/// Center ordered rows in a page-space region. Default gutters are the greater
/// of the block size and twice the connector-to-node clearance on each axis.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/blocksizex-cell-page-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/blocksizey-cell-page-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linetonodex-cell-page-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linetonodey-cell-page-layout-section
pub(crate) fn rows(s: &mut Session, p: &Value) -> Result<Value> {
    use misterdiagram_geom::Rect;
    let index = page_index(s, p)?;
    let selected = ids(p, "ids")?;
    let counts = p
        .get("counts")
        .and_then(Value::as_array)
        .filter(|v| !v.is_empty() && v.len() <= selected.len())
        .ok_or_else(|| operation("counts must specify nonempty rows"))?
        .iter()
        .map(|v| {
            v.as_u64().and_then(|v| usize::try_from(v).ok()).filter(|v| *v > 0 && *v <= selected.len()).ok_or_else(|| operation("invalid row count"))
        })
        .collect::<Result<Vec<_>>>()?;
    if counts.iter().try_fold(0usize, |n, v| n.checked_add(*v)) != Some(selected.len()) {
        return Err(operation("row counts must sum to the shape count"));
    }
    let page = s.document.pages.get(index).ok_or_else(|| operation("page missing"))?;
    let mut e = Evaluator::new(&s.document).map_err(operation)?;
    let ps = Scope::Page(page.id);
    let region = if let Some(region) = p.get("region") {
        if region.as_object().is_none_or(|v| v.keys().any(|k| !matches!(k.as_str(), "x" | "y" | "width" | "height"))) {
            return Err(operation("invalid region"));
        }
        let x = crate::edit::number(region, "x")?;
        let y = crate::edit::number(region, "y")?;
        let w = crate::edit::number(region, "width")?;
        let h = crate::edit::number(region, "height")?;
        Rect::new(x, y, x + w, y + h)
    } else {
        Rect::new(0., 0., number(&mut e, ps, "PageWidth")?, number(&mut e, ps, "PageHeight")?)
    };
    if region.width() <= 0. || region.height() <= 0. || [region.x0, region.y0, region.x1, region.y1].iter().any(|v| v.abs() > 1e6) {
        return Err(operation("invalid layout region"));
    }
    let mut gutter = |key: &str, block: &str, line: &str| -> Result<f64> {
        let v = if p.get(key).is_some() { crate::edit::number(p, key)? } else { number(&mut e, ps, block)?.max(2. * number(&mut e, ps, line)?) };
        if v < 0. || v > 1e6 { Err(operation("invalid gutter")) } else { Ok(v) }
    };
    let gx = gutter("gutter_x", "BlockSizeX", "LineToNodeX")?;
    let gy = gutter("gutter_y", "BlockSizeY", "LineToNodeY")?;
    let mut frames = Vec::new();
    for id in &selected {
        let scope = Scope::PageShape { page: page.id, shape: *id };
        let shape = e.shape(scope).ok_or_else(|| operation("shape not found"))?;
        if e.cell(scope, "BeginX").is_some() {
            return Err(operation("layout.rows requires two-dimensional shapes"));
        }
        let mut descendants = shape.shapes.iter().collect::<Vec<_>>();
        let mut visited = 0usize;
        while let Some(child) = descendants.pop() {
            visited += 1;
            if visited > 100000 {
                return Err(operation("shape traversal limit"));
            }
            if selected.contains(&child.id) {
                return Err(operation("cannot lay out a selected group and its child"));
            }
            descendants.extend(&child.shapes);
        }
        let bounds = crate::inspect::bounds(&mut e, scope)?;
        let pin = Point::new(number(&mut e, scope, "PinX")?, number(&mut e, scope, "PinY")?);
        let parent = e.parent_to_page(scope).map_err(operation)?;
        if parent.determinant().abs() < 1e-12 {
            return Err(operation("singular parent transform"));
        }
        frames.push((*id, bounds, pin, parent.inverse()));
    }
    let mut sizes = Vec::new();
    let mut offset = 0usize;
    for count in &counts {
        let row = frames.get(offset..offset + count).ok_or_else(|| operation("row range"))?;
        let width = row.iter().map(|(_, b, _, _)| b.width()).sum::<f64>() + gx * ((*count - 1) as f64);
        let height = row.iter().map(|(_, b, _, _)| b.height()).fold(0., f64::max);
        if width > region.width() + 1e-9 {
            return Err(operation("row exceeds region width"));
        }
        sizes.push((width, height));
        offset += count;
    }
    let height = sizes.iter().map(|(_, h)| h).sum::<f64>() + gy * ((counts.len() - 1) as f64);
    if !height.is_finite() || height > region.height() + 1e-9 {
        return Err(operation("rows exceed region height"));
    }
    let mut top = (region.y0 + region.y1 + height) / 2.;
    let mut offset = 0usize;
    let mut placements = Vec::new();
    for (count, (width, height)) in counts.iter().zip(sizes) {
        let mut x = (region.x0 + region.x1 - width) / 2.;
        for (id, bounds, pin, inverse) in frames.get(offset..offset + count).ok_or_else(|| operation("row range"))? {
            let y = top - height / 2. - bounds.height() / 2.;
            let delta = (*inverse * Point::new(x, y)) - (*inverse * Point::new(bounds.x0, bounds.y0));
            let placed = Rect::new(x, y, x + bounds.width(), y + bounds.height());
            placements.push((*id, *pin + delta, placed));
            x += bounds.width() + gx;
        }
        offset += count;
        top -= height + gy;
    }
    s.edit_document(|doc,_| {
        let page=page_mut(doc,index)?;
        for (id,pin,_) in &placements {
            let shape=shape_mut(&mut page.shapes,*id)?;
            set_number(&mut shape.sheet,"PinX",pin.x)?;set_number(&mut shape.sheet,"PinY",pin.y)?;
        }
        Ok(json!({"placed":placements.iter().map(|(id,_,r)|json!({"id":id,"bounds":rect_value(*r)})).collect::<Vec<_>>(),"gutter_x":gx,"gutter_y":gy}))
    })
}

/// Assign distinct positions along each facing hub side, preserving spoke order.
/// Existing connectors are selected by their two endpoint shape IDs; ambiguity
/// is an error. Routing still happens inside the ordinary strict transaction.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.reroute
pub(crate) fn fan(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let hub = uint(p, "hub")?;
    let spokes = ids(p, "spokes")?;
    if spokes.contains(&hub) {
        return Err(operation("hub cannot be a spoke"));
    }
    let page = s.document.pages.get(index).ok_or_else(|| operation("page missing"))?;
    let mut assignments = Vec::new();
    let mut totals = std::collections::BTreeMap::new();
    for spoke in &spokes {
        let side = facing(&s.document, index, hub, *spoke)?;
        let candidates = page
            .connects
            .iter()
            .filter(|c| c.to_sheet == hub)
            .filter(|c| {
                page.connects.iter().any(|other| other.from_sheet == c.from_sheet && other.to_sheet == *spoke && other.from_cell != c.from_cell)
            })
            .collect::<Vec<_>>();
        let [connection] = candidates.as_slice() else {
            return Err(operation("each spoke requires exactly one connector to the hub"));
        };
        let end = match connection.from_cell.as_deref() {
            Some("BeginX") => misterdiagram_route::End::Begin,
            Some("EndX") => misterdiagram_route::End::End,
            _ => return Err(operation("fan requires endpoint glue")),
        };
        *totals.entry(side).or_insert(0usize) += 1;
        assignments.push((*spoke, side, connection.from_sheet, end));
    }
    let placed = s.edit_document(|doc, _| {
        let mut positions = std::collections::BTreeMap::new();
        let mut placed = Vec::new();
        for (spoke, side, connector, end) in &assignments {
            let position = positions.entry(*side).or_insert(0usize);
            *position += 1;
            let total = totals.get(side).copied().ok_or_else(|| operation("missing side count"))?;
            let fraction = (*position as f64) / ((total + 1) as f64);
            let name = point(doc, index, hub, *side, fraction)?;
            misterdiagram_route::connect_static(doc, index, *connector, *end, hub, &name).map_err(operation)?;
            placed.push(json!({"spoke":spoke,"connector":connector,"point":name,"fraction":fraction}));
        }
        Ok(json!(placed))
    })?;
    crate::glue::reroute(s, &json!({"page":index+1}))?;
    let selected = std::iter::once(hub).chain(spokes.iter().copied()).collect::<Vec<_>>();
    let connectors = assignments.iter().map(|(_, _, id, _)| *id).collect::<Vec<_>>();
    let crossings = crossings(&s.document, index, &connectors, &selected)?;
    Ok(json!({"assignments":placed,"crossings":crossings}))
}

/// Test routed centerlines against open alignment-box interiors. Boundary contact
/// at a glued connection point is not a crossing. Curves use bounded flattening.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
fn crossings(doc: &misterdiagram_model::Document, index: usize, connectors: &[u32], selected: &[u32]) -> Result<Value> {
    let page = doc.pages.get(index).ok_or_else(|| operation("page missing"))?;
    let mut e = Evaluator::new(doc).map_err(operation)?;
    let bounds = selected
        .iter()
        .map(|id| Ok((*id, crate::inspect::bounds(&mut e, Scope::PageShape { page: page.id, shape: *id })?)))
        .collect::<Result<Vec<_>>>()?;
    let mut found = std::collections::BTreeSet::new();
    let mut segments = 0usize;
    for connector in connectors {
        let scope = Scope::PageShape { page: page.id, shape: *connector };
        let transform = e.shape_to_page(scope).map_err(operation)?;
        for path in e.geometry_paths(scope, misterdiagram_geom::Rect::new(-1e6, -1e6, 1e6, 1e6)).map_err(operation)? {
            let mut last = None;
            kurbo::flatten(path.path.iter(), 1e-6, |element| match element {
                kurbo::PathEl::MoveTo(p) => last = Some(transform * p),
                kurbo::PathEl::LineTo(p) => {
                    let p = transform * p;
                    if let Some(a) = last {
                        segments = segments.saturating_add(1);
                        if segments <= 100000 && segments.saturating_mul(bounds.len()) <= 2_000_000 {
                            for (id, bounds) in &bounds {
                                if crosses(a, p, *bounds) {
                                    found.insert((*connector, *id));
                                }
                            }
                        }
                    }
                    last = Some(p);
                }
                _ => last = None,
            });
            if segments > 100000 || segments.saturating_mul(bounds.len()) > 2_000_000 {
                return Err(operation("route inspection segment limit"));
            }
        }
    }
    Ok(json!(found.into_iter().map(|(connector, shape)| json!({"connector":connector,"shape":shape})).collect::<Vec<_>>()))
}
fn crosses(a: Point, b: Point, r: misterdiagram_geom::Rect) -> bool {
    let mut low = 0f64;
    let mut high = 1f64;
    for (a, b, min, max) in [(a.x, b.x, r.x0 + 1e-8, r.x1 - 1e-8), (a.y, b.y, r.y0 + 1e-8, r.y1 - 1e-8)] {
        if max <= min {
            return false;
        }
        if (a - b).abs() < 1e-12 {
            if a <= min || a >= max {
                return false;
            }
        } else {
            let p = (min - a) / (b - a);
            let q = (max - a) / (b - a);
            low = low.max(p.min(q));
            high = high.min(p.max(q));
        }
    }
    low < high && high > 0. && low < 1.
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crossings_detect_interiors_but_allow_glued_boundary_contacts() {
        let r = misterdiagram_geom::Rect::new(1., 1., 3., 3.);
        assert!(crosses(Point::new(0., 2.), Point::new(4., 2.), r));
        assert!(crosses(Point::new(2., 2.), Point::new(4., 2.), r));
        assert!(!crosses(Point::new(0., 1.), Point::new(4., 1.), r));
        assert!(!crosses(Point::new(0., 2.), Point::new(1., 2.), r));
        assert!(!crosses(Point::new(3., 2.), Point::new(4., 2.), r));
    }
}
