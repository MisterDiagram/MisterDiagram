//! Geometric topology checks, shared by command clients and acceptance tooling.
use super::*;
use misterdiagram_geom::{Affine, Rect};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
const EPS: f64 = 1e-8;
struct Node {
    bounds: Rect,
    pin: Point,
}
struct Segment {
    a: Point,
    b: Point,
    offset: f64,
}
struct Route {
    id: u32,
    source: Option<u32>,
    target: Option<u32>,
    segments: Vec<Segment>,
}
fn violation(out: &mut Vec<Value>, rule: &str, shapes: &[u32], connectors: &[u32], detail: &str) -> Result<()> {
    if out.len() >= 10000 {
        return Err(operation("layout checker violation limit"));
    }
    out.push(json!({"rule":rule,"shapes":shapes,"connectors":connectors,"detail":detail}));
    Ok(())
}
fn length(a: Point, b: Point) -> f64 {
    (b - a).hypot()
}
fn overlap(a: &Segment, b: &Segment) -> Option<(Point, Point)> {
    let d = a.b - a.a;
    let len = d.hypot();
    if len <= EPS {
        return None;
    }
    let unit = d / len;
    if (b.a - a.a).cross(unit).abs() > EPS || (b.b - a.a).cross(unit).abs() > EPS {
        return None;
    }
    let first = (b.a - a.a).dot(unit);
    let last = (b.b - a.a).dot(unit);
    let lo = first.min(last).max(0.);
    let hi = first.max(last).min(len);
    (hi - lo > EPS).then_some((a.a + unit * lo, a.a + unit * hi))
}
// Only a continuous shared prefix leaving the same source is a tree trunk.
fn trunk(a: &Route, b: &Route) -> f64 {
    if a.source.is_none() || a.source != b.source {
        return 0.;
    }
    let (Some(first), Some(second)) = (a.segments.first(), b.segments.first()) else {
        return 0.;
    };
    if length(first.a, second.a) > EPS {
        return 0.;
    }
    let (mut i, mut j, mut position, mut total) = (0, 0, first.a, 0.);
    while let (Some(x), Some(y)) = (a.segments.get(i), b.segments.get(j)) {
        if ((total - x.offset).abs() <= EPS && length(x.a, position) > EPS) || ((total - y.offset).abs() <= EPS && length(y.a, position) > EPS) {
            break;
        }
        let dx = x.b - position;
        let dy = y.b - position;
        let (lx, ly) = (dx.hypot(), dy.hypot());
        if lx <= EPS {
            i += 1;
            continue;
        }
        if ly <= EPS {
            j += 1;
            continue;
        }
        if dx.cross(dy).abs() > EPS * lx * ly || dx.dot(dy) <= 0. {
            break;
        }
        let step = lx.min(ly);
        position += dx / lx * step;
        total += step;
        if lx <= ly + EPS {
            i += 1;
        }
        if ly <= lx + EPS {
            j += 1;
        }
    }
    total
}
fn near_border(a: Point, b: Point, r: Rect) -> bool {
    let intersects = |a: f64, b: f64, c: f64, d: f64| a.max(c) + EPS < b.min(d);
    if (a.x - b.x).abs() <= EPS {
        intersects(a.y.min(b.y), a.y.max(b.y), r.y0, r.y1) && [r.x0, r.x1].iter().any(|x| (a.x - x).abs() <= 1. / 16. + EPS)
    } else if (a.y - b.y).abs() <= EPS {
        intersects(a.x.min(b.x), a.x.max(b.x), r.x0, r.x1) && [r.y0, r.y1].iter().any(|y| (a.y - y).abs() <= 1. / 16. + EPS)
    } else {
        false
    }
}
/// Inspect static glue, routed geometry, text rectangles, tiers and page margins.
/// Numerical comparisons use inches; the rule tolerances are fixed by Brief 25.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/text-transform-section
pub(crate) fn check(s: &mut Session, p: &Value) -> Result<Value> {
    if p.get("shared_only").and_then(Value::as_bool) == Some(true) {
        return super::shared::inspect(s, p).map(|r| r.value());
    }
    let index = page_index(s, p)?;
    let doc = s.document();
    let page = doc.pages.get(index).ok_or_else(|| operation("page missing"))?;
    if page.shapes.len() > 10000 || page.connects.len() > 20000 {
        return Err(operation("layout checker shape limit"));
    }
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
    let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    let mut nodes = BTreeMap::new();
    let mut routes = Vec::new();
    let mut violations = Vec::new();
    let mut segment_count = 0usize;
    let top_level = page.shapes.iter().map(|shape| shape.id).collect::<BTreeSet<_>>();
    let scopes = e.scopes().filter(|scope| matches!(scope,Scope::PageShape{page:id,..} if *id==page.id)).take(10001).collect::<Vec<_>>();
    if scopes.len() > 10000 {
        return Err(operation("layout checker shape limit"));
    }
    let mut labels = Vec::<(u32, Affine, f64, f64)>::new();
    for scope in scopes {
        let Scope::PageShape { shape: id, .. } = scope else { continue };
        let block = e.text_block(scope).map_err(operation)?;
        if !block.body.text().trim().is_empty() && number(&mut e, scope, "HideText")? == 0. {
            if block.to_page.determinant().abs() < 1e-12 || block.width <= 0. || block.height <= 0. {
                return Err(operation("invalid label transform"));
            }
            labels.push((id, block.to_page.inverse(), block.width, block.height));
        }
        if e.cell(scope, "BeginX").is_none() {
            let parent = e.parent_to_page(scope).map_err(operation)?;
            nodes.insert(
                id,
                Node {
                    bounds: crate::inspect::bounds(&mut e, scope)?,
                    pin: parent * Point::new(number(&mut e, scope, "PinX")?, number(&mut e, scope, "PinY")?),
                },
            );
            continue;
        }
        let mut ends = [None, None];
        for (i, from) in ["BeginX", "EndX"].iter().enumerate() {
            let connections = page.connects.iter().filter(|c| c.from_sheet == id && c.from_cell.as_deref() == Some(from)).collect::<Vec<_>>();
            let [c] = connections.as_slice() else {
                violation(&mut violations, "a", &[], &[id], "endpoint does not have exactly one glue record")?;
                continue;
            };
            if let Some(end) = ends.get_mut(i) {
                *end = Some(c.to_sheet);
            }
            let target = Scope::PageShape { page: page.id, shape: c.to_sheet };
            let all = e.resolved_named_cells(target);
            let row = all.iter().find(|n| {
                Some(n.name.as_str()) == c.to_cell.as_deref() && n.cell.name == "X" && n.section.is_some_and(|sec| sec.name == "Connection")
            });
            let y = row.and_then(|row| {
                all.iter().find(|n| {
                    n.cell.name == "Y"
                        && n.section.is_some_and(|sec| sec.name == "Connection")
                        && n.row.zip(row.row).is_some_and(|(x, y)| x.index == y.index && x.name == y.name)
                })
            });
            let (Some(x), Some(y)) = (row, y) else {
                violation(&mut violations, "a", &[c.to_sheet], &[id], "endpoint is not glued to a connection point")?;
                continue;
            };
            let (xn, yn) = (x.name.clone(), y.name.clone());
            let expected = e.shape_to_page(target).map_err(operation)? * Point::new(number(&mut e, target, &xn)?, number(&mut e, target, &yn)?);
            let actual = e.parent_to_page(scope).map_err(operation)?
                * Point::new(number(&mut e, scope, from)?, number(&mut e, scope, if i == 0 { "BeginY" } else { "EndY" })?);
            if length(expected, actual) > EPS {
                violation(&mut violations, "a", &[c.to_sheet], &[id], "endpoint is detached from its connection point")?;
            }
        }
        let transform = e.shape_to_page(scope).map_err(operation)?;
        let mut segments = Vec::new();
        let mut offset = 0.;
        for path in e.geometry_paths(scope, Rect::new(-1e6, -1e6, 1e6, 1e6)).map_err(operation)? {
            use kurbo::ParamCurve;
            for segment in path.path.segments() {
                let segment = transform * segment;
                let steps = curve_steps(segment)?;
                segment_count = segment_count.checked_add(steps).ok_or_else(|| operation("layout checker segment limit"))?;
                if segment_count > 100000 {
                    return Err(operation("layout checker segment limit"));
                }
                let mut a = segment.eval(0.);
                for step in 1..=steps {
                    let b = segment.eval(step as f64 / steps as f64);
                    let len = length(a, b);
                    if len > EPS {
                        segments.push(Segment { a, b, offset });
                        offset += len;
                    }
                    a = b;
                }
            }
        }
        if segments.is_empty() {
            violation(&mut violations, "a", &[], &[id], "connector has no route")?;
        }
        if let (Some(first), Some(last)) = (segments.first(), segments.last()) {
            for (i, actual, x, y) in [(0, first.a, "BeginX", "BeginY"), (1, last.b, "EndX", "EndY")] {
                let expected = e.parent_to_page(scope).map_err(operation)? * Point::new(number(&mut e, scope, x)?, number(&mut e, scope, y)?);
                if length(actual, expected) > EPS {
                    violation(
                        &mut violations,
                        "a",
                        &ends.get(i).copied().flatten().into_iter().collect::<Vec<_>>(),
                        &[id],
                        "drawn route does not reach its glued endpoint",
                    )?;
                }
            }
        }
        routes.push(Route { id, source: ends.first().copied().flatten(), target: ends.get(1).copied().flatten(), segments });
    }
    if segment_count.saturating_mul(nodes.len().saturating_add(labels.len())).saturating_add(segment_count.saturating_mul(segment_count)) > 2_000_000
    {
        return Err(operation("layout checker comparison limit"));
    }
    let mut extent = None::<Rect>;
    for node in nodes.values() {
        extent = Some(extent.map_or(node.bounds, |r| r.union(node.bounds)));
    }
    for &(_, inverse, width, height) in &labels {
        let bounds = inverse.inverse().transform_rect_bbox(Rect::new(0., 0., width, height));
        extent = Some(extent.map_or(bounds, |r| r.union(bounds)));
    }
    for route in &routes {
        let mut bad = BTreeSet::new();
        for segment in &route.segments {
            let box_ = Rect::from_points(segment.a, segment.b);
            extent = Some(extent.map_or(box_, |r| r.union(box_)));
            for (&id, node) in &nodes {
                if crosses(segment.a, segment.b, node.bounds) {
                    bad.insert(("b", id));
                }
                if near_border(segment.a, segment.b, node.bounds) {
                    bad.insert(("c", id));
                }
            }
            for &(id, inverse, w, h) in &labels {
                if crosses(inverse * segment.a, inverse * segment.b, Rect::new(0., 0., w, h)) {
                    bad.insert(("g", id));
                }
            }
        }
        for (rule, id) in bad {
            violation(
                &mut violations,
                rule,
                &[id],
                &[route.id],
                match rule {
                    "b" => "route crosses shape interior",
                    "c" => "parallel route is within 1/16 inch of border",
                    _ => "route crosses label",
                },
            )?;
        }
    }
    for (index, a) in routes.iter().enumerate() {
        for b in routes.iter().skip(index + 1) {
            let common = trunk(a, b);
            let mut bad = false;
            for x in &a.segments {
                for y in &b.segments {
                    if let Some((lo, hi)) = overlap(x, y)
                        && (x.offset + length(x.a, lo).max(length(x.a, hi)) > common + EPS
                            || y.offset + length(y.a, lo).max(length(y.a, hi)) > common + EPS)
                    {
                        bad = true;
                    }
                }
            }
            if bad {
                violation(
                    &mut violations,
                    "d",
                    &a.source.into_iter().chain(a.target).chain(b.source).chain(b.target).collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>(),
                    &[a.id, b.id],
                    "routes share a segment outside a source trunk",
                )?;
            }
        }
    }
    if p.get("root").is_some() {
        let nodes = nodes.iter().filter(|(id, _)| top_level.contains(id)).map(|(id, node)| (*id, node)).collect::<BTreeMap<_, _>>();
        let root = uint(p, "root")?;
        if !nodes.contains_key(&root) {
            return Err(operation("tree root missing"));
        }
        let mut children = BTreeMap::<u32, Vec<u32>>::new();
        for r in &routes {
            if let (Some(a), Some(b)) = (r.source, r.target) {
                children.entry(a).or_default().push(b);
            }
        }
        let mut depths = BTreeMap::from([(root, 0usize)]);
        let mut queue = VecDeque::from([root]);
        let mut invalid = false;
        while let Some(parent) = queue.pop_front() {
            let depth = depths.get(&parent).copied().ok_or_else(|| operation("tree depth missing"))?;
            let kids = children.get(&parent).map(Vec::as_slice).unwrap_or(&[]);
            let mut span = None::<Rect>;
            for child in kids {
                if let Some(node) = nodes.get(child) {
                    span = Some(span.map_or(node.bounds, |r| r.union(node.bounds)));
                } else {
                    invalid = true;
                }
                if depths.contains_key(child) {
                    invalid = true;
                } else {
                    depths.insert(*child, depth + 1);
                    queue.push_back(*child);
                }
            }
            if let (Some(span), Some(node)) = (span, nodes.get(&parent))
                && (node.pin.x - (span.x0 + span.x1) / 2.).abs() > 1. / 64. + EPS
            {
                violation(
                    &mut violations,
                    "e",
                    &std::iter::once(parent).chain(kids.iter().copied()).collect::<Vec<_>>(),
                    &[],
                    "parent is not centred over children",
                )?;
            }
        }
        if invalid || depths.len() != nodes.len() {
            violation(
                &mut violations,
                "e",
                &nodes.keys().copied().collect::<Vec<_>>(),
                &routes.iter().map(|r| r.id).collect::<Vec<_>>(),
                "drawing is not a directed tree from root",
            )?;
        }
        let mut tiers = BTreeMap::<usize, (u32, f64)>::new();
        for (id, depth) in depths {
            if let Some(node) = nodes.get(&id) {
                if let Some(&(peer, y)) = tiers.get(&depth) {
                    if (node.pin.y - y).abs() > EPS {
                        violation(&mut violations, "e", &[peer, id], &[], "tier pins do not share PinY")?;
                    }
                } else {
                    tiers.insert(depth, (id, node.pin.y));
                }
            }
        }
    }
    if p.get("fit_page").and_then(Value::as_bool) == Some(true)
        && let Some(extent) = extent
    {
        let width = number(&mut e, Scope::Page(page.id), "PageWidth")?;
        let height = number(&mut e, Scope::Page(page.id), "PageHeight")?;
        let margins = [extent.x0, extent.y0, width - extent.x1, height - extent.y1];
        let lo = margins.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = margins.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        if lo < -EPS || hi - lo > 1. / 64. + EPS {
            violation(
                &mut violations,
                "f",
                &nodes.keys().copied().collect::<Vec<_>>(),
                &routes.iter().map(|r| r.id).collect::<Vec<_>>(),
                "page margins differ by more than 1/64 inch",
            )?;
        }
    }
    let shared = super::shared::inspect(s, p)?;
    let stats = shared.stats();
    if violations.len().saturating_add(shared.violations.len()) > 10000 {
        return Err(operation("layout checker violation limit"));
    }
    violations.extend(shared.violations);
    Ok(
        json!({"page":index+1,"shapes":nodes.len(),"connectors":routes.len(),"violations":violations,"count":violations.len(),"shared_segments":stats}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn route(id: u32, pairs: &[((f64, f64), (f64, f64))]) -> Route {
        let mut offset = 0.;
        let segments = pairs
            .iter()
            .map(|&(a, b)| {
                let (a, b) = (Point::new(a.0, a.1), Point::new(b.0, b.1));
                let segment = Segment { a, b, offset };
                offset += length(a, b);
                segment
            })
            .collect();
        Route { id, source: Some(1), target: Some(id + 1), segments }
    }
    #[test]
    fn trunk_allows_subdivision_but_stops_at_gaps_divergence_and_reverse_overlap() {
        let a = route(2, &[((0., 0.), (0., -1.)), ((0., -1.), (1., -1.))]);
        let b = route(3, &[((0., 0.), (0., -0.5)), ((0., -0.5), (0., -1.)), ((0., -1.), (1., -1.))]);
        assert_eq!(trunk(&a, &b), 2.);
        let a = route(2, &[((0., 0.), (0., -0.25)), ((0., -0.5), (0., -0.75))]);
        let b = route(3, &[((0., 0.), (0., -0.25)), ((0., -0.5), (0., -0.75))]);
        assert_eq!(trunk(&a, &b), 0.25);
        let a = route(2, &[((0., 0.), (0., -1.)), ((0., -1.), (1., -1.)), ((1., -1.), (1., -2.)), ((1., -2.), (0., -2.)), ((0., -2.), (0., -3.))]);
        let b =
            route(3, &[((0., 0.), (0., -1.)), ((0., -1.), (-1., -1.)), ((-1., -1.), (-1., -2.)), ((-1., -2.), (0., -2.)), ((0., -2.), (0., -3.))]);
        assert_eq!(trunk(&a, &b), 1.);
        let reverse = route(3, &[((0., -1.), (0., 0.))]);
        assert_eq!(trunk(&a, &reverse), 0.);
        assert!(a.segments.first().zip(reverse.segments.first()).and_then(|(a, b)| overlap(a, b)).is_some());
    }
}
