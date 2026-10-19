//! Rooted top-to-bottom placement using native, statically glued connectors.
use super::*;
use misterdiagram_geom::Rect;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[derive(Clone, Copy)]
struct Frame {
    left: f64,
    right: f64,
    below: f64,
    above: f64,
    flip_y: bool,
}
#[derive(Clone, Copy)]
struct Span {
    width: f64,
    pin: f64,
}
fn get<T>(map: &BTreeMap<u32, T>, id: u32) -> Result<&T> {
    map.get(&id).ok_or_else(|| operation("tree node missing"))
}
fn id(value: &Value) -> Result<u32> {
    value.as_u64().and_then(|n| u32::try_from(n).ok()).filter(|n| *n != u32::MAX).ok_or_else(|| operation("invalid tree shape ID"))
}
/// Place rooted edges in depth tiers and glue bottom/top connection points.
/// Style 17 midpoint geometry is observed in B25 office-style-17, before this code.
/// Half-sixteenth dimensions and thirty-second pins leave route midpoints on 1/64.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shaperoutestyle-cell-shape-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/blocksizex-cell-page-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linetonodey-cell-page-layout-section
pub(crate) fn tree(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let root = uint(p, "root")?;
    let page = s.document().pages.get(index).ok_or_else(|| operation("page missing"))?;
    let mut links = BTreeMap::<(u32, u32), Vec<u32>>::new();
    let mut ends = BTreeMap::<u32, (Option<u32>, Option<u32>)>::new();
    for c in &page.connects {
        let entry = ends.entry(c.from_sheet).or_default();
        match c.from_cell.as_deref() {
            Some("BeginX") => entry.0 = Some(c.to_sheet),
            Some("EndX") => entry.1 = Some(c.to_sheet),
            _ => {}
        }
    }
    for (connector, (a, b)) in ends {
        if let (Some(a), Some(b)) = (a, b) {
            links.entry((a.min(b), a.max(b))).or_default().push(connector);
        }
    }
    let mut edges = Vec::new();
    let mut selected = BTreeSet::from([root]);
    if let Some(values) = p.get("edges") {
        let values = values.as_array().filter(|v| v.len() < 10000).ok_or_else(|| operation("tree edges limit"))?;
        for value in values {
            let pair = value.as_array().filter(|v| v.len() == 2).ok_or_else(|| operation("tree edges must be parent-child pairs"))?;
            let a = id(pair.first().ok_or_else(|| operation("parent missing"))?)?;
            let b = id(pair.get(1).ok_or_else(|| operation("child missing"))?)?;
            selected.extend([a, b]);
            edges.push((a, b));
        }
        if p.get("ids").is_some() {
            return Err(operation("choose edges or ids"));
        }
    } else {
        selected = ids(p, "ids")?.into_iter().collect();
        if !selected.contains(&root) {
            return Err(operation("tree root must be selected"));
        }
        let mut adjacency = BTreeMap::<u32, Vec<u32>>::new();
        for (&(a, b), connectors) in &links {
            if selected.contains(&a) && selected.contains(&b) {
                if connectors.len() != 1 {
                    return Err(operation("parallel tree connectors"));
                }
                adjacency.entry(a).or_default().push(b);
                adjacency.entry(b).or_default().push(a);
            }
        }
        let mut seen = BTreeSet::from([root]);
        let mut queue = VecDeque::from([(root, None)]);
        while let Some((node, parent)) = queue.pop_front() {
            for child in adjacency.get(&node).into_iter().flatten() {
                if Some(*child) == parent {
                    continue;
                }
                if !seen.insert(*child) {
                    return Err(operation("tree cycle"));
                }
                edges.push((node, *child));
                queue.push_back((*child, Some(node)));
            }
        }
    }
    if selected.len() > 10000 || edges.len() + 1 != selected.len() {
        return Err(operation("tree must be connected and acyclic"));
    }
    let mut children = BTreeMap::<u32, Vec<u32>>::new();
    let mut parents = BTreeMap::new();
    for &(a, b) in &edges {
        if a == b || b == root || parents.insert(b, a).is_some() {
            return Err(operation("tree child must have one parent"));
        }
        children.entry(a).or_default().push(b);
    }
    let mut order = Vec::new();
    let mut depths = BTreeMap::from([(root, 0usize)]);
    let mut queue = VecDeque::from([root]);
    while let Some(node) = queue.pop_front() {
        order.push(node);
        let depth = *get(&depths, node)?;
        for child in children.get(&node).into_iter().flatten() {
            if depths.insert(*child, depth + 1).is_some() {
                return Err(operation("tree cycle"));
            }
            queue.push_back(*child);
        }
    }
    if order.len() != selected.len() {
        return Err(operation("tree has disconnected nodes"));
    }
    if selected.len().saturating_mul(edges.len()) > 2_000_000 {
        return Err(operation("tree placement work limit"));
    }
    let ids = selected.iter().copied().collect::<Vec<_>>();
    let mut doc = s.document().clone();
    grid_shapes(&mut doc, index, &ids)?;
    // Dimension normalization leaves half-sizes on 1/32 and route midpoints on 1/64.
    let page_id = doc.pages.get(index).ok_or_else(|| operation("page missing"))?.id;
    let themes = misterdiagram_theme::Themes::from_document(&doc).map_err(operation)?;
    let mut e = Evaluator::new(&doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    let mut dimensions = Vec::new();
    for node in &ids {
        let scope = Scope::PageShape { page: page_id, shape: *node };
        if !doc.pages.get(index).is_some_and(|p| p.shapes.iter().any(|s| s.id == *node)) || number(&mut e, scope, "Angle")? != 0. {
            return Err(operation("tree requires unrotated top-level shapes"));
        }
        for name in ["Width", "Height"] {
            let value = number(&mut e, scope, name)?;
            let snapped = (value * 16.).ceil() / 16.;
            if snapped != value {
                let formula = e.effective_formula(scope, name).filter(|f| f != "Inh" && !f.eq_ignore_ascii_case("No Formula"));
                dimensions.push((*node, name, snapped, formula.map(|f| format!("CEILING(({f}),0.0625)"))));
            }
        }
    }
    drop(e);
    for (node, name, value, formula) in dimensions {
        set(&mut shape_mut(&mut page_mut(&mut doc, index)?.shapes, node)?.sheet, name, &value.to_string(), formula.as_deref())?;
    }
    let themes = misterdiagram_theme::Themes::from_document(&doc).map_err(operation)?;
    let mut e = Evaluator::new(&doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    let ps = Scope::Page(page_id);
    let mut gap = |key: &str, block: &str, line: &str| -> Result<f64> {
        let v = if p.get(key).is_some() { crate::edit::number(p, key)? } else { number(&mut e, ps, block)?.max(2. * number(&mut e, ps, line)?) };
        if !(0. ..=1e6).contains(&v) {
            return Err(operation("invalid tree gutter"));
        }
        Ok(slot(v))
    };
    let gx = gap("gutter_x", "BlockSizeX", "LineToNodeX")?;
    let gy = gap("gutter_y", "BlockSizeY", "LineToNodeY")?;
    let mut frames = BTreeMap::new();
    let mut tiers = BTreeMap::<usize, (f64, f64)>::new();
    for node in &ids {
        let scope = Scope::PageShape { page: page_id, shape: *node };
        let b = crate::inspect::bounds(&mut e, scope)?;
        let x = number(&mut e, scope, "PinX")?;
        let y = number(&mut e, scope, "PinY")?;
        let f = Frame { left: b.x0 - x, right: b.x1 - x, below: y - b.y0, above: b.y1 - y, flip_y: number(&mut e, scope, "FlipY")? != 0. };
        if [f.left, f.right, f.below, f.above].iter().any(|v| (v * 32.).fract() != 0.) {
            return Err(operation("tree alignment bounds must have thirty-second-inch half sizes"));
        }
        frames.insert(*node, f);
        let t = tiers.entry(*get(&depths, *node)?).or_default();
        t.0 = t.0.max(f.below);
        t.1 = t.1.max(f.above);
    }
    let mut spans = BTreeMap::<u32, Span>::new();
    let mut offsets = BTreeMap::new();
    for node in order.iter().rev() {
        let f = get(&frames, *node)?;
        let kids = children.get(node).map(Vec::as_slice).unwrap_or(&[]);
        if kids.is_empty() {
            spans.insert(*node, Span { width: f.right - f.left, pin: -f.left });
            continue;
        }
        let mut x = 0.;
        let mut low = f64::INFINITY;
        let mut high = f64::NEG_INFINITY;
        for child in kids {
            let span = get(&spans, *child)?;
            let cf = get(&frames, *child)?;
            let pin = x + span.pin;
            offsets.insert(*child, pin);
            low = low.min(pin + cf.left);
            high = high.max(pin + cf.right);
            x += span.width + gx;
        }
        let pin = ((low + high) * 16.).round() / 32.;
        let left = 0f64.min(pin + f.left);
        let right = (x - gx).max(pin + f.right);
        for child in kids {
            let value = offsets.get_mut(child).ok_or_else(|| operation("tree offset missing"))?;
            *value -= pin;
        }
        spans.insert(*node, Span { width: right - left, pin: pin - left });
    }
    let mut y = BTreeMap::new();
    let mut top = 0.;
    for (&depth, &(below, above)) in &tiers {
        let pin = top - above;
        y.insert(depth, pin);
        top = pin - below - gy;
    }
    let mut positions = BTreeMap::from([(root, 0.)]);
    let mut bounds = Rect::new(f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for node in &order {
        let x = *get(&positions, *node)?;
        let py = *y.get(get(&depths, *node)?).ok_or_else(|| operation("tree tier missing"))?;
        let f = get(&frames, *node)?;
        bounds = Rect::new(bounds.x0.min(x + f.left), bounds.y0.min(py - f.below), bounds.x1.max(x + f.right), bounds.y1.max(py + f.above));
        for child in children.get(node).into_iter().flatten() {
            positions.insert(*child, x + get(&offsets, *child)?);
        }
    }
    let fit = p.get("fit_page").and_then(Value::as_bool).unwrap_or(false);
    let margin = p.get("margin").map(|_| crate::edit::number(p, "margin")).transpose()?.unwrap_or(0.5);
    if !(0. ..=1e6).contains(&margin) {
        return Err(operation("invalid page margin"));
    }
    let margin = slot(margin);
    let region = if let Some(r) = p.get("region") {
        let x = crate::edit::number(r, "x")?;
        let y = crate::edit::number(r, "y")?;
        Rect::new(x, y, x + crate::edit::number(r, "width")?, y + crate::edit::number(r, "height")?)
    } else {
        Rect::new(0., 0., number(&mut e, ps, "PageWidth")?, number(&mut e, ps, "PageHeight")?)
    };
    let region = if fit { Rect::new(0., 0., bounds.width() + 2. * margin, bounds.height() + 2. * margin) } else { region };
    if [region.x0, region.y0, region.x1, region.y1].iter().any(|v| !v.is_finite() || v.abs() > 1e6)
        || bounds.width() > region.width()
        || bounds.height() > region.height()
    {
        return Err(operation("tree exceeds page region"));
    }
    let dx = ((region.x0 + region.x1 - bounds.x0 - bounds.x1) * 16.).round() / 32.;
    let dy = ((region.y0 + region.y1 - bounds.y0 - bounds.y1) * 16.).round() / 32.;
    if bounds.x0 + dx < region.x0 || bounds.x1 + dx > region.x1 || bounds.y0 + dy < region.y0 || bounds.y1 + dy > region.y1 {
        return Err(operation("binary grid placement exceeds tree region"));
    }
    drop(e);
    s.edit_document(|current, _| {
        *current = doc;
        let page = page_mut(current, index)?;
        if fit {
            set_number(&mut page.sheet, "PageWidth", region.width())?;
            set_number(&mut page.sheet, "PageHeight", region.height())?;
        }
        for node in &order {
            let shape = shape_mut(&mut page.shapes, *node)?;
            set_number(&mut shape.sheet, "PinX", get(&positions, *node)? + dx)?;
            set_number(&mut shape.sheet, "PinY", y.get(get(&depths, *node)?).ok_or_else(|| operation("tier missing"))? + dy)?;
        }
        Ok(())
    })?;
    let themes = misterdiagram_theme::Themes::from_document(s.document()).map_err(operation)?;
    let mut evaluated = Evaluator::new(s.document()).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    check_grid(&mut evaluated, page_id, &ids)?;
    for node in &order {
        let scope = Scope::PageShape { page: page_id, shape: *node };
        if number(&mut evaluated, scope, "Angle")? != 0. || (number(&mut evaluated, scope, "FlipY")? != 0.) != get(&frames, *node)?.flip_y {
            return Err(operation("position-dependent transform invalidates tree layout"));
        }
        let actual = crate::inspect::bounds(&mut evaluated, scope)?;
        let f = get(&frames, *node)?;
        let x = get(&positions, *node)? + dx;
        let y = y.get(get(&depths, *node)?).ok_or_else(|| operation("tier missing"))? + dy;
        if [actual.x0 - x - f.left, actual.x1 - x - f.right, actual.y0 - y + f.below, actual.y1 - y - f.above].iter().any(|v| v.abs() > 1e-9) {
            return Err(operation("position-dependent bounds invalidate tree layout"));
        }
    }
    if fit {
        let tree_bounds = Rect::new(bounds.x0 + dx, bounds.y0 + dy, bounds.x1 + dx, bounds.y1 + dy);
        let tree_connectors =
            edges.iter().flat_map(|(a, b)| links.get(&((*a).min(*b), (*a).max(*b))).into_iter().flatten().copied()).collect::<BTreeSet<_>>();
        for shape in &s.document().pages.get(index).ok_or_else(|| operation("page missing"))?.shapes {
            let scope = Scope::PageShape { page: page_id, shape: shape.id };
            if selected.contains(&shape.id) || tree_connectors.contains(&shape.id) {
                continue;
            }
            let other = crate::inspect::bounds(&mut evaluated, scope)?;
            if other.x0 < tree_bounds.x0 || other.y0 < tree_bounds.y0 || other.x1 > tree_bounds.x1 || other.y1 > tree_bounds.y1 {
                return Err(operation("fit_page would exclude unrelated drawing content"));
            }
        }
    }
    drop(evaluated);
    let mut connectors = Vec::new();
    for &(a, b) in &edges {
        let existing = links.get(&(a.min(b), a.max(b))).map(Vec::as_slice).unwrap_or(&[]);
        let connector = match existing {
            [] => {
                let result = crate::masters::drop_master(s, &json!({"page":index+1,"master":"Dynamic connector"}))?;
                uint(&result, "id")?
            }
            [id] => *id,
            _ => return Err(operation("parallel tree connectors")),
        };
        s.edit_document(|doc, _| {
            let begin = point(doc, index, a, if get(&frames, a)?.flip_y { Side::Top } else { Side::Bottom }, 0.5, &Default::default())?;
            let end = point(doc, index, b, if get(&frames, b)?.flip_y { Side::Bottom } else { Side::Top }, 0.5, &Default::default())?;
            set_number(&mut shape_mut(&mut page_mut(doc, index)?.shapes, connector)?.sheet, "ShapeRouteStyle", 17.)?;
            misterdiagram_route::connect_static(doc, index, connector, misterdiagram_route::End::Begin, a, &begin).map_err(operation)?;
            misterdiagram_route::connect_static(doc, index, connector, misterdiagram_route::End::End, b, &end).map_err(operation)?;
            Ok(())
        })?;
        connectors.push(json!({"parent":a,"child":b,"connector":connector}));
    }
    crate::glue::reroute(s, &json!({"page":index+1}))?;
    // Existing obstacles may force the generic router away from the witnessed
    // midpoint path. Refuse that placement rather than publish off-grid detours.
    let themes = misterdiagram_theme::Themes::from_document(s.document()).map_err(operation)?;
    let mut evaluated = Evaluator::new(s.document()).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    for link in &connectors {
        let connector = uint(link, "connector")?;
        let scope = Scope::PageShape { page: page_id, shape: connector };
        let middle = (number(&mut evaluated, scope, "BeginY")? + number(&mut evaluated, scope, "EndY")?) / 2.;
        let transform = evaluated.shape_to_page(scope).map_err(operation)?;
        for geometry in evaluated.geometry_paths(scope, Rect::new(-1e6, -1e6, 1e6, 1e6)).map_err(operation)? {
            if geometry.path.elements().len() > 64 {
                return Err(operation("tree route inspection limit"));
            }
            let mut previous = None;
            for element in geometry.path.elements() {
                let (point, line) = match element {
                    kurbo::PathEl::MoveTo(p) => (*p, false),
                    kurbo::PathEl::LineTo(p) => (*p, true),
                    _ => return Err(operation("tree route requires unobstructed midpoint segments")),
                };
                let point = transform * point;
                if !point.is_finite() || [point.x, point.y].iter().any(|v| (v * 64.).fract() != 0.) {
                    return Err(operation("tree route leaves binary grid"));
                }
                if let Some(old) = previous {
                    let old: Point = old;
                    if line && old.x != point.x && (old.y != point.y || (point.y - middle).abs() > 1e-9) {
                        return Err(operation(format!(
                            "obstacle prevents midpoint tree route: connector {connector}, {old:?} to {point:?}, middle {middle}"
                        )));
                    }
                }
                previous = Some(point);
            }
        }
    }
    Ok(
        json!({"root":root,"placed":order.iter().map(|node|json!({"id":node,"depth":depths.get(node),"x":positions.get(node).map(|x|x+dx),"y":depths.get(node).and_then(|d|y.get(d)).map(|y|y+dy)})).collect::<Vec<_>>(),"links":connectors,"fit_page":fit,"margin":margin,"gutter_x":gx,"gutter_y":gy}),
    )
}
