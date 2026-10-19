//! Original layout commands built on public ShapeSheet cells and witnessed glue.
use crate::{Result, Session, edit::*, operation};
use misterdiagram_geom::Point;
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};
mod tree;
pub(crate) use tree::tree;

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
/// Authored layout coordinates use a 1/64 inch grid. Ties round away from zero.
/// This is an original layout policy, not an evaluator arithmetic tolerance.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pinx-cell-shape-transform-section
fn grid(value: f64) -> f64 {
    (value * 64.).round() / 64.
}
/// Use the smallest binary denominator admitting ordered interior points.
/// With at most 10,000 spokes all integer arithmetic is bounded and exact.
fn fraction(position: usize, total: usize) -> Result<f64> {
    let count = total.checked_add(1).ok_or_else(|| operation("fan point count"))?;
    let denominator = count.checked_next_power_of_two().ok_or_else(|| operation("fan point denominator"))?;
    let numerator = position.checked_mul(denominator).and_then(|v| v.checked_add(count / 2)).ok_or_else(|| operation("fan point numerator"))? / count;
    Ok(numerator as f64 / denominator as f64)
}
/// Normalize only shapes explicitly selected by a layout command. Formula-driven
/// dimensions retain their expression, quantized with ROUND, so dependency changes
/// are still caught by the placement validation. Imported, unselected cells stay intact.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/round-function
fn grid_shapes(doc: &mut misterdiagram_model::Document, index: usize, ids: &[u32]) -> Result<()> {
    let page_id = doc.pages.get(index).ok_or_else(|| operation("page missing"))?.id;
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
    let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    let mut edits = Vec::new();
    for id in ids {
        let scope = Scope::PageShape { page: page_id, shape: *id };
        if e.shape(scope).is_none() || e.cell(scope, "BeginX").is_some() {
            return Err(operation("layout requires two-dimensional shapes"));
        }
        for name in ["Width", "Height", "PinX", "PinY"] {
            let value = number(&mut e, scope, name)?;
            let dimension = matches!(name, "Width" | "Height");
            if dimension && value <= 0. {
                return Err(operation("layout endpoint must have positive bounds"));
            }
            let snapped = if dimension { grid(value).max(1. / 64.) } else { grid(value) };
            let original = e.effective_formula(scope, name);
            let formula = if dimension {
                original.clone().filter(|f| !f.eq_ignore_ascii_case("No Formula") && f != "Inh").map(|f| {
                    if f.starts_with("MAX(0.015625,ROUND((") && f.ends_with(")*64,0)/64)") {
                        f
                    } else {
                        format!("MAX(0.015625,ROUND(({f})*64,0)/64)")
                    }
                })
            } else {
                None
            };
            if snapped == value && formula == original {
                continue;
            }
            if formula.as_ref().is_some_and(|f| f.len() > 65536) {
                return Err(operation("layout dimension formula limit"));
            }
            edits.push((*id, name, snapped, formula));
        }
    }
    drop(e);
    let page = page_mut(doc, index)?;
    for (id, name, value, formula) in edits {
        set(&mut shape_mut(&mut page.shapes, id)?.sheet, name, &value.to_string(), formula.as_deref())?;
    }
    Ok(())
}
fn check_grid(e: &mut Evaluator<'_>, page: u32, ids: &[u32]) -> Result<()> {
    for id in ids {
        for name in ["Width", "Height", "PinX", "PinY"] {
            let value = number(e, Scope::PageShape { page, shape: *id }, name)?;
            if (value * 64.).fract() != 0. {
                return Err(operation("dependent shape values invalidate binary layout grid"));
            }
        }
    }
    Ok(())
}
// Even-grid slots keep centered shapes clear when their pins snap to 1/64.
fn slot(size: f64) -> f64 {
    (size * 32.).ceil() / 32.
}
fn facing(doc: &misterdiagram_model::Document, page: usize, id: u32, target: u32) -> Result<Side> {
    let page = doc.pages.get(page).ok_or_else(|| operation("page missing"))?;
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
    let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
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
fn point(
    doc: &mut misterdiagram_model::Document,
    page: usize,
    id: u32,
    side: Side,
    fraction: f64,
    used: &std::collections::BTreeSet<String>,
) -> Result<String> {
    let page_id = doc.pages.get(page).ok_or_else(|| operation("page missing"))?.id;
    let scope = Scope::PageShape { page: page_id, shape: id };
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
    let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
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
        if used.contains(&name) || (kind != 0. && kind != 2.) {
            continue;
        }
        if number(&mut e, scope, &name)? == x && number(&mut e, scope, &y_name)? == y {
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
    let mut normalized = s.document().clone();
    grid_shapes(&mut normalized, index, &[source, target])?;
    let begin = facing(&normalized, index, source, target)?;
    let end = facing(&normalized, index, target, source)?;
    s.edit_document(|doc, _| {
        *doc = normalized;
        let a = point(doc, index, source, begin, 0.5, &Default::default())?;
        let b = point(doc, index, target, end, 0.5, &Default::default())?;
        misterdiagram_route::connect_static(doc, index, id, misterdiagram_route::End::Begin, source, &a).map_err(operation)?;
        misterdiagram_route::connect_static(doc, index, id, misterdiagram_route::End::End, target, &b).map_err(operation)?;
        let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
        let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
        check_grid(&mut e, doc.pages.get(index).ok_or_else(|| operation("page missing"))?.id, &[source, target])?;
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
    let mut normalized = s.document().clone();
    grid_shapes(&mut normalized, index, &selected)?;
    let page = normalized.pages.get(index).ok_or_else(|| operation("page missing"))?;
    let themes = misterdiagram_theme::Themes::from_document(&normalized).map_err(operation)?;
    let mut e = Evaluator::new(&normalized).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
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
        if v < 0. || v > 1e6 { Err(operation("invalid gutter")) } else { Ok(grid(v)) }
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
        let width = row.iter().map(|(_, b, _, _)| slot(b.width())).sum::<f64>() + gx * ((*count - 1) as f64);
        let height = row.iter().map(|(_, b, _, _)| slot(b.height())).fold(0., f64::max);
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
        let mut previous_right = None;
        for (id, bounds, pin, inverse) in frames.get(offset..offset + count).ok_or_else(|| operation("row range"))? {
            let y = top - height / 2. - bounds.height() / 2.;
            let left = x + (slot(bounds.width()) - bounds.width()) / 2.;
            let delta = (*inverse * Point::new(left, y)) - (*inverse * Point::new(bounds.x0, bounds.y0));
            let raw_pin = *pin + delta;
            let placed_pin = Point::new(grid(raw_pin.x), grid(raw_pin.y));
            let parent = inverse.inverse();
            let correction = (parent * placed_pin) - (parent * raw_pin);
            let placed = Rect::new(left + correction.x, y + correction.y, left + correction.x + bounds.width(), y + correction.y + bounds.height());
            if placed.x0 < region.x0 || placed.y0 < region.y0 || placed.x1 > region.x1 || placed.y1 > region.y1 {
                return Err(operation("grid placement exceeds region"));
            }
            if previous_right.is_some_and(|right| placed.x0 < right + gx) {
                return Err(operation("grid placement overlaps row gutter"));
            }
            previous_right = Some(placed.x1);
            placements.push((*id, placed_pin, placed));
            x += slot(bounds.width()) + gx;
        }
        offset += count;
        top -= height + gy;
    }
    let mut offset = 0;
    let mut previous_bottom = None;
    for count in &counts {
        let row = placements.get(offset..offset + count).ok_or_else(|| operation("row range"))?;
        let bottom = row.iter().map(|(_, _, r)| r.y0).fold(f64::INFINITY, f64::min);
        let top = row.iter().map(|(_, _, r)| r.y1).fold(f64::NEG_INFINITY, f64::max);
        if previous_bottom.is_some_and(|previous| top + gy > previous) {
            return Err(operation("grid placement overlaps row gutter"));
        }
        previous_bottom = Some(bottom);
        offset += count;
    }
    drop(e);
    s.edit_document(|doc,_| {
        *doc = normalized;
        let page=page_mut(doc,index)?;
        for (id,pin,_) in &placements {
            let shape=shape_mut(&mut page.shapes,*id)?;
            set_number(&mut shape.sheet,"PinX",pin.x)?;set_number(&mut shape.sheet,"PinY",pin.y)?;
        }
        // Moving a shape may change formula-driven dimensions or parent transforms.
        // Refuse the entire command if those dependencies invalidate the planned layout.
        let page_id = page.id;
        let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
        let mut evaluated = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
        check_grid(&mut evaluated,page_id,&selected)?;
        for (id, _, planned) in &placements {
            let actual = crate::inspect::bounds(&mut evaluated, Scope::PageShape { page: page_id, shape: *id })?;
            if [actual.x0-planned.x0, actual.y0-planned.y0, actual.x1-planned.x1, actual.y1-planned.y1].iter().any(|delta| delta.abs() > 1e-9) {
                return Err(operation("position-dependent shape bounds invalidate row layout"));
            }
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
    let mut normalized = s.document().clone();
    grid_shapes(&mut normalized, index, &std::iter::once(hub).chain(spokes.iter().copied()).collect::<Vec<_>>())?;
    let page = normalized.pages.get(index).ok_or_else(|| operation("page missing"))?;
    let mut assignments = Vec::new();
    let mut totals = std::collections::BTreeMap::new();
    for spoke in &spokes {
        let side = facing(&normalized, index, hub, *spoke)?;
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
        *doc = normalized;
        let mut positions = std::collections::BTreeMap::new();
        let mut placed = Vec::new();
        let mut used = std::collections::BTreeSet::new();
        for (spoke, side, connector, end) in &assignments {
            let position = positions.entry(*side).or_insert(0usize);
            *position += 1;
            let total = totals.get(side).copied().ok_or_else(|| operation("missing side count"))?;
            let fraction = fraction(*position, total)?;
            let name = point(doc, index, hub, *side, fraction, &used)?;
            used.insert(name.clone());
            misterdiagram_route::connect_static(doc, index, *connector, *end, hub, &name).map_err(operation)?;
            placed.push(json!({"spoke":spoke,"connector":connector,"point":name,"fraction":fraction}));
        }
        Ok(json!(placed))
    })?;
    crate::glue::reroute(s, &json!({"page":index+1}))?;
    let selected = std::iter::once(hub).chain(spokes.iter().copied()).collect::<Vec<_>>();
    let connectors = assignments.iter().map(|(_, _, id, _)| *id).collect::<Vec<_>>();
    let themes = misterdiagram_theme::Themes::from_document(&s.document).map_err(operation)?;
    let mut e = Evaluator::new(&s.document).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    check_grid(&mut e, s.document.pages.get(index).ok_or_else(|| operation("page missing"))?.id, &selected)?;
    let crossings = crossings(&s.document, index, &connectors, &selected)?;
    Ok(json!({"assignments":placed,"crossings":crossings}))
}

/// Test routed centerlines against open alignment-box interiors. Boundary contact
/// at a glued connection point is not a crossing. Curves use bounded flattening.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
fn crossings(doc: &misterdiagram_model::Document, index: usize, connectors: &[u32], selected: &[u32]) -> Result<Value> {
    let page = doc.pages.get(index).ok_or_else(|| operation("page missing"))?;
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
    let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
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
            use kurbo::ParamCurve;
            for segment in path.path.segments() {
                let segment = transform * segment;
                let steps = curve_steps(segment)?;
                segments = segments.checked_add(steps).ok_or_else(|| operation("route inspection segment limit"))?;
                if segments > 100000 || segments.saturating_mul(bounds.len()) > 2_000_000 {
                    return Err(operation("route inspection segment limit"));
                }
                let mut start = segment.eval(0.);
                for step in 1..=steps {
                    let end = segment.eval(step as f64 / steps as f64);
                    for (id, bounds) in &bounds {
                        if crosses(start, end, *bounds) {
                            found.insert((*connector, *id));
                        }
                    }
                    start = end;
                }
            }
        }
    }
    Ok(json!(found.into_iter().map(|(connector, shape)| json!({"connector":connector,"shape":shape})).collect::<Vec<_>>()))
}
/// Uniform interpolation error is at most max(|B''|)/(8*n*n). Bound the
/// subdivision count before iteration, including hostile imported control points.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
fn curve_steps(segment: kurbo::PathSeg) -> Result<usize> {
    use kurbo::PathSeg;
    let (points, error) = match segment {
        PathSeg::Line(line) => ([line.p0, line.p1, line.p0, line.p1], 0.),
        PathSeg::Quad(q) => {
            let second = q.p0.to_vec2() - 2. * q.p1.to_vec2() + q.p2.to_vec2();
            ([q.p0, q.p1, q.p2, q.p2], second.hypot() / 4.)
        }
        PathSeg::Cubic(c) => {
            let a = c.p0.to_vec2() - 2. * c.p1.to_vec2() + c.p2.to_vec2();
            let b = c.p1.to_vec2() - 2. * c.p2.to_vec2() + c.p3.to_vec2();
            ([c.p0, c.p1, c.p2, c.p3], 0.75 * a.hypot().max(b.hypot()))
        }
    };
    if points.iter().any(|p| !p.is_finite() || p.x.abs() > 1e9 || p.y.abs() > 1e9) {
        return Err(operation("route inspection coordinate limit"));
    }
    let steps = (error / 1e-6).sqrt().ceil().max(1.);
    if !steps.is_finite() || steps > 100000. {
        return Err(operation("route inspection segment limit"));
    }
    Ok(steps as usize)
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
    fn hostile_curve_work_is_rejected_before_subdivision() {
        let huge = kurbo::CubicBez::new((0., 0.), (1e9, 1e9), (-1e9, 1e9), (0., 0.));
        assert!(curve_steps(huge.into()).unwrap_err().to_string().contains("segment limit"));
        let nonfinite = kurbo::Line::new((f64::INFINITY, 0.), (0., 0.));
        assert!(curve_steps(nonfinite.into()).is_err());
        let normal = kurbo::CubicBez::new((0., 0.), (0., 1.), (1., 1.), (1., 0.));
        let steps = curve_steps(normal.into()).unwrap();
        assert!((2..100000).contains(&steps));
        use kurbo::ParamCurve;
        // The maximum interpolation error on every sampled chord stays below tolerance.
        for i in 0..steps {
            let a = normal.eval(i as f64 / steps as f64);
            let b = normal.eval((i + 1) as f64 / steps as f64);
            let middle = normal.eval((i as f64 + 0.5) / steps as f64);
            assert!(middle.distance(a.midpoint(b)) <= 1e-6);
        }
    }
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
