//! Shared orthogonal trunks: exact on the restricted family, approximation elsewhere.
//! The band/ordering is the contributor-generated M5.39 witness, adopted in Brief34b.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/shaperoutestyle-cell-shape-layout-section
use misterdiagram_geom::{Affine, Point, Rect};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use std::collections::{BTreeMap, BTreeSet};

const EPS: f64 = 1e-9;
const MIN_OVERLAP: f64 = 1e-6;
const MAX_WORK: usize = 4_000_000;
struct Budget(usize);
impl Budget {
    fn charge(&mut self) -> Result<(), String> {
        self.0 = self.0.checked_sub(1).ok_or("route lane work limit")?;
        Ok(())
    }
}

pub(crate) struct Route {
    pub page: u32,
    pub id: u32,
    pub frame: Affine,
    pub points: Vec<Point>,
    pub breaks: BTreeSet<usize>,
    pub eligible: bool,
    pub requested: bool,
    pub assigned: bool,
    pub glue: [Option<u32>; 2],
}
#[derive(Clone, Copy)]
struct Trunk {
    route: usize,
    segment: usize,
    horizontal: bool,
    coordinate: f64,
    lo: f64,
    hi: f64,
}
fn axes(p: Point, horizontal: bool) -> (f64, f64) {
    if horizontal { (p.x, p.y) } else { (p.y, p.x) }
}
fn segment(a: Point, b: Point) -> Option<(bool, f64, f64, f64)> {
    let horizontal = (a.y - b.y).abs() < EPS;
    if !horizontal && (a.x - b.x).abs() >= EPS {
        return None;
    }
    let (x, y) = axes(a, horizontal);
    let (end, _) = axes(b, horizontal);
    (x - end).abs().gt(&MIN_OVERLAP).then_some((horizontal, y, x.min(end), x.max(end)))
}
fn overlap(a: Point, b: Point, c: Point, d: Point) -> bool {
    segment(a, b)
        .zip(segment(c, d))
        .is_some_and(|((ah, ay, al, ar), (bh, by, bl, br))| ah == bh && (ay - by).abs() < EPS && ar.min(br) - al.max(bl) > MIN_OVERLAP)
}
fn touches(a: Point, b: Point, r: Rect) -> bool {
    if (a.y - b.y).abs() < EPS {
        a.y >= r.y0 && a.y <= r.y1 && a.x.min(b.x) <= r.x1 && a.x.max(b.x) >= r.x0
    } else if (a.x - b.x).abs() < EPS {
        a.x >= r.x0 && a.x <= r.x1 && a.y.min(b.y) <= r.y1 && a.y.max(b.y) >= r.y0
    } else {
        true
    }
}
/// Finite dyadic band. Skipping an occupied/blocked position never changes the
/// order of the remaining positions; the final scan is at 1/1024 inch.
fn offsets() -> Vec<f64> {
    let mut out = vec![192. / 1024.];
    let mut used = BTreeSet::from([192]);
    let mut step = 64;
    while step >= 1 {
        let mut value = 192 - step;
        while value > 0 {
            if used.insert(value) {
                out.push(f64::from(value) / 1024.);
            }
            value -= step;
        }
        step /= 2;
    }
    out
}
fn root(parents: &[usize], mut i: usize) -> Result<usize, String> {
    for _ in 0..parents.len() {
        let next = *parents.get(i).ok_or("lane group index")?;
        if next == i {
            return Ok(i);
        }
        i = next;
    }
    Err("lane group cycle".into())
}
fn warning(outcomes: &mut BTreeMap<(u32, u32), Option<String>>, route: &Route, reason: &str) {
    outcomes.insert((route.page, route.id), Some(format!("route lane {reason}: connector {}", route.id)));
}
/// Adjust only shared bend-to-bend segments. Work and retained input are bounded;
/// existing routing capacity fallbacks are never eligible for this pass.
pub(crate) fn apply(e: &mut Evaluator<'_>, routes: &mut [Route], outcomes: &mut BTreeMap<(u32, u32), Option<String>>) -> Result<(), String> {
    let count = routes.iter().try_fold(0usize, |n, r| n.checked_add(r.points.len())).ok_or("lane point count overflow")?;
    if count > 500000 {
        for route in routes.iter().filter(|r| r.eligible) {
            warning(outcomes, route, "point limit");
        }
        return Ok(());
    }
    let original = routes.iter().map(|r| r.points.clone()).collect::<Vec<_>>();
    let mut budget = Budget(MAX_WORK);
    let result = assign(e, routes, outcomes, &mut budget);
    if result.as_ref().is_err_and(|e| e == "route lane work limit") {
        for (route, points) in routes.iter_mut().zip(&original) {
            route.points = points.clone();
            route.assigned = false;
            if route.eligible {
                warning(outcomes, route, "work limit");
            }
        }
        return Ok(());
    }
    result?;
    for (route, points) in routes.iter_mut().zip(&original) {
        if route.eligible && outcomes.get(&(route.page, route.id)).is_some_and(Option::is_some) {
            route.points = points.clone();
            route.assigned = false;
        }
    }
    // A failed connector is restored before this bounded dependency check. No
    // surviving assignment may rely on that connector's unpublished position.
    for _ in 0..routes.len() {
        let mut rollback = None;
        for (own, route) in routes.iter().enumerate().filter(|(_, r)| r.assigned) {
            for i in 0..route.points.len().saturating_sub(1) {
                match terminal_conflict(&route.points, i, routes, own, route.page, &mut budget) {
                    Ok(false) => {}
                    Ok(true) => {
                        rollback = Some(own);
                        break;
                    }
                    Err(error) if error == "route lane work limit" => {
                        for (route, points) in routes.iter_mut().zip(&original) {
                            if route.assigned {
                                warning(outcomes, route, "work limit");
                                route.points = points.clone();
                                route.assigned = false;
                            }
                        }
                        return Ok(());
                    }
                    Err(error) => return Err(error),
                }
            }
            if rollback.is_some() {
                break;
            }
        }
        let Some(own) = rollback else { break };
        let route = routes.get_mut(own).ok_or("lane rollback index")?;
        route.points = original.get(own).ok_or("lane rollback points")?.clone();
        route.assigned = false;
        warning(outcomes, route, "dependent positions exhausted at 1/1024 inch");
    }
    Ok(())
}
fn assign(
    e: &mut Evaluator<'_>,
    routes: &mut [Route],
    outcomes: &mut BTreeMap<(u32, u32), Option<String>>,
    budget: &mut Budget,
) -> Result<(), String> {
    let pages = routes.iter().filter(|r| r.requested).map(|r| r.page).collect::<BTreeSet<_>>();
    for page in pages {
        let mut trunks = Vec::new();
        for (index, route) in routes.iter().enumerate().filter(|(_, r)| r.page == page && r.eligible) {
            for i in 1..route.points.len().saturating_sub(2) {
                let (Some(a), Some(b)) = (route.points.get(i), route.points.get(i + 1)) else { continue };
                let Some((horizontal, coordinate, lo, hi)) = segment(*a, *b) else { continue };
                if trunks.len() >= 16384 {
                    warning(outcomes, route, "work limit");
                    break;
                }
                trunks.push(Trunk { route: index, segment: i, horizontal, coordinate, lo, hi });
            }
        }
        if trunks.len() < 2 {
            continue;
        }
        trunks.sort_by(|a, b| a.horizontal.cmp(&b.horizontal).then(a.coordinate.total_cmp(&b.coordinate)).then(a.lo.total_cmp(&b.lo)));
        let mut parents = (0..trunks.len()).collect::<Vec<_>>();
        for (i, a) in trunks.iter().enumerate() {
            for (j, b) in trunks.iter().enumerate().skip(i + 1) {
                if a.horizontal != b.horizontal || (a.coordinate - b.coordinate).abs() >= EPS {
                    break;
                }
                budget.charge()?;
                if a.route != b.route && a.hi.min(b.hi) - a.lo.max(b.lo) > MIN_OVERLAP {
                    let left = root(&parents, i)?;
                    let right = root(&parents, j)?;
                    *parents.get_mut(right).ok_or("lane union index")? = left;
                }
            }
        }
        let mut groups = BTreeMap::<usize, Vec<Trunk>>::new();
        for (i, &t) in trunks.iter().enumerate() {
            groups.entry(root(&parents, i)?).or_default().push(t);
        }
        let mut groups = groups.into_values().filter(|g| g.len() > 1).collect::<Vec<_>>();
        if groups.is_empty() {
            continue;
        }
        // Only shared pages pay for physical obstacle collection. No inflated
        // clearance is changed for the original route solver or unshared routes.
        let scopes = e.scopes().filter(|s| matches!(s, Scope::PageShape {page:p,..} if *p==page)).collect::<Vec<_>>();
        let mut boxes = Vec::new();
        for scope in scopes {
            let Scope::PageShape { shape, .. } = scope else { continue };
            if crate::changes::placeable(e, scope)?
                && (crate::orthogonal::number(e, scope, "ShapePermeableX", 0.) == 0.
                    || crate::orthogonal::number(e, scope, "ShapePermeableY", 0.) == 0.)
            {
                if boxes.len() >= 100000 {
                    return Err("lane obstacle count limit".into());
                }
                boxes.push((shape, crate::orthogonal::bounds(e, scope)?));
            }
        }
        groups.sort_by_key(|g| g.iter().filter_map(|t| routes.get(t.route).map(|r| (r.id, t.segment))).min());
        for group in &mut groups {
            group.sort_by_key(|t| routes.get(t.route).map(|r| (r.id, t.segment)));
            let first = *group.first().ok_or("lane group empty")?;
            let lo = group.iter().map(|t| t.lo).fold(f64::INFINITY, f64::min);
            let hi = group.iter().map(|t| t.hi).fold(f64::NEG_INFINITY, f64::max);
            let mut lower = f64::NEG_INFINITY;
            let mut upper = f64::INFINITY;
            for &(_, rect) in &boxes {
                budget.charge()?;
                let (l, r, bottom, top) = if first.horizontal { (rect.x0, rect.x1, rect.y0, rect.y1) } else { (rect.y0, rect.y1, rect.x0, rect.x1) };
                if r < lo || l > hi {
                    continue;
                }
                if top <= first.coordinate {
                    lower = lower.max(top);
                }
                if bottom >= first.coordinate {
                    upper = upper.min(bottom);
                }
            }
            let members = group.iter().map(|t| (t.route, t.segment)).collect::<BTreeSet<_>>();
            let mut assigned = BTreeSet::new();
            for t in group.iter() {
                let route = routes.get(t.route).ok_or("lane route index")?;
                if outcomes.get(&(route.page, route.id)).is_some_and(Option::is_some) {
                    continue;
                }
                let begin = *route.points.first().ok_or("lane begin missing")?;
                let toward_lower = axes(begin, t.horizontal).1 <= t.coordinate;
                let edge = if toward_lower { lower } else { upper };
                let edge = if edge.is_finite() { edge } else { axes(begin, t.horizontal).1 };
                let sign = if toward_lower { 1. } else { -1. };
                let mut chosen = None;
                for offset in offsets() {
                    budget.charge()?;
                    let coordinate = edge + sign * offset;
                    if coordinate <= lower || coordinate >= upper || !coordinate.is_finite() {
                        continue;
                    }
                    let mut points = route.points.clone();
                    for i in [t.segment, t.segment + 1] {
                        let p = points.get_mut(i).ok_or("lane point index")?;
                        if t.horizontal {
                            p.y = coordinate;
                        } else {
                            p.x = coordinate;
                        }
                    }
                    let a = *points.get(t.segment).ok_or("lane point missing")?;
                    let b = *points.get(t.segment + 1).ok_or("lane point missing")?;
                    let mut valid = true;
                    for &(id, rect) in &boxes {
                        budget.charge()?;
                        if touches(a, b, rect) {
                            valid = false;
                            break;
                        }
                        for i in [t.segment - 1, t.segment + 1] {
                            let terminal = (i == 0 && route.glue[0] == Some(id)) || (i + 2 == points.len() && route.glue[1] == Some(id));
                            if !terminal && points.get(i).zip(points.get(i + 1)).is_some_and(|(a, b)| touches(*a, *b, rect)) {
                                valid = false;
                            }
                        }
                    }
                    if !valid {
                        continue;
                    }
                    for (other_index, other) in routes.iter().enumerate().filter(|(_, r)| r.page == page) {
                        if other_index == t.route {
                            continue;
                        }
                        for (i, pair) in other.points.windows(2).enumerate() {
                            if other.breaks.contains(&i) {
                                continue;
                            }
                            // Unassigned members will move off their provisional line.
                            if members.contains(&(other_index, i)) && !assigned.contains(&(other_index, i)) {
                                continue;
                            }
                            budget.charge()?;
                            if pair.first().zip(pair.get(1)).is_some_and(|(c, d)| overlap(a, b, *c, *d)) {
                                valid = false;
                                break;
                            }
                        }
                        if !valid {
                            break;
                        }
                    }
                    if valid && !self_overlap(&points, budget)? {
                        chosen = Some(points);
                        break;
                    }
                }
                if let Some(points) = chosen {
                    let route = routes.get_mut(t.route).ok_or("lane route index")?;
                    route.points = points;
                    route.assigned = true;
                    assigned.insert((t.route, t.segment));
                } else {
                    warning(outcomes, route, "positions exhausted at 1/1024 inch");
                }
            }
        }
        repair_terminals(routes, page, &boxes, outcomes, budget)?;
    }
    Ok(())
}

fn same_glue(a: &[Point], i: usize, ag: [Option<u32>; 2], b: &[Point], j: usize, bg: [Option<u32>; 2]) -> bool {
    let ends = |p: &[Point], k: usize, glue: [Option<u32>; 2]| {
        [if k == 0 { p.first().copied().zip(glue[0]) } else { None }, if k + 2 == p.len() { p.last().copied().zip(glue[1]) } else { None }]
    };
    ends(a, i, ag).into_iter().flatten().any(|(x, xt)| ends(b, j, bg).into_iter().flatten().any(|(y, yt)| xt == yt && x.distance(y) < MIN_OVERLAP))
}
fn terminal_conflict(points: &[Point], index: usize, routes: &[Route], own: usize, page: u32, budget: &mut Budget) -> Result<bool, String> {
    let Some((a, b)) = points.get(index).zip(points.get(index + 1)) else { return Ok(false) };
    for (_, other) in routes.iter().enumerate().filter(|(i, r)| *i != own && r.page == page) {
        for (j, w) in other.points.windows(2).enumerate() {
            if other.breaks.contains(&j) {
                continue;
            }
            budget.charge()?;
            if w.first().zip(w.get(1)).is_some_and(|(c, d)| overlap(*a, *b, *c, *d))
                && !same_glue(points, index, routes.get(own).ok_or("lane glue route")?.glue, &other.points, j, other.glue)
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
fn self_overlap(points: &[Point], budget: &mut Budget) -> Result<bool, String> {
    for (i, a) in points.windows(2).enumerate() {
        for b in points.windows(2).skip(i + 2) {
            budget.charge()?;
            if let Some(((a, b), (c, d))) = a.first().zip(a.get(1)).zip(b.first().zip(b.get(1)))
                && overlap(*a, *b, *c, *d)
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
// Translation can lengthen a terminal leg into another connector's leg. Keep
// its glue point and line, shortening it with a small internal jog if needed.
// This is part of the general-case approximation, never a change to glue.
fn repair_terminals(
    routes: &mut [Route],
    page: u32,
    boxes: &[(u32, Rect)],
    outcomes: &mut BTreeMap<(u32, u32), Option<String>>,
    budget: &mut Budget,
) -> Result<(), String> {
    let mut order = routes.iter().enumerate().filter(|(_, r)| r.page == page && r.assigned).map(|(i, r)| (r.id, i)).collect::<Vec<_>>();
    order.sort();
    for (_, own) in order {
        for reverse in [false, true] {
            let route = routes.get(own).ok_or("lane repair route")?;
            let index = if reverse { 0 } else { route.points.len().saturating_sub(2) };
            if !terminal_conflict(&route.points, index, routes, own, page, budget)? {
                continue;
            }
            let mut base = route.points.clone();
            if reverse {
                base.reverse();
            }
            let len = base.len();
            let Some(((p, b), end)) = len.checked_sub(3).and_then(|i| base.get(i)).zip(len.checked_sub(2).and_then(|i| base.get(i))).zip(base.last())
            else {
                continue;
            };
            let (p, b, end) = (*p, *b, *end);
            let incoming = b - p;
            let terminal = end - b;
            if incoming.hypot() < EPS || terminal.hypot() < EPS {
                continue;
            }
            let mut chosen = None;
            'candidate: for retreat in offsets() {
                if retreat >= incoming.hypot() {
                    continue;
                }
                'stub: for stub in offsets() {
                    budget.charge()?;
                    if stub >= terminal.hypot() {
                        continue;
                    }
                    let foot = end - terminal / terminal.hypot() * stub;
                    let a = b - incoming / incoming.hypot() * retreat;
                    let d = foot - incoming / incoming.hypot() * retreat;
                    let mut points = base.get(..len - 2).ok_or("lane repair prefix")?.to_vec();
                    points.extend([a, d, foot, end]);
                    for (k, w) in points.windows(2).enumerate().skip(len - 3) {
                        for &(id, r) in boxes {
                            budget.charge()?;
                            if k + 2 == points.len() && route.glue[usize::from(!reverse)] == Some(id) {
                                continue;
                            }
                            if w.first().zip(w.get(1)).is_some_and(|(a, b)| touches(*a, *b, r)) {
                                continue 'stub;
                            }
                        }
                    }
                    if reverse {
                        points.reverse();
                    }
                    let mut conflict = false;
                    for k in 0..points.len().saturating_sub(1) {
                        if terminal_conflict(&points, k, routes, own, page, budget)? {
                            conflict = true;
                            break;
                        }
                    }
                    if !conflict && !self_overlap(&points, budget)? {
                        chosen = Some(points);
                        break 'candidate;
                    }
                }
            }
            if let Some(points) = chosen {
                routes.get_mut(own).ok_or("lane repair route")?.points = points;
            } else {
                warning(outcomes, route, "terminal positions exhausted at 1/1024 inch");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use misterdiagram_model::{Cell, skeleton};
    fn drawing(height: f64) -> misterdiagram_model::Document {
        let mut doc = skeleton::document(false);
        for (id, y) in [(1, -0.5), (2, height)] {
            let mut shape = skeleton::rectangle(id, 0., y, 10., 0.5, "").unwrap();
            shape.sheet.cells.push(Cell { name: "ObjType".into(), value: Some("1".into()), ..Default::default() });
            doc.pages[0].shapes.push(shape);
        }
        doc
    }
    fn route(id: u32, y: f64) -> Route {
        let x = f64::from(id) * 0.001;
        Route {
            page: 0,
            id,
            frame: Affine::IDENTITY,
            points: vec![Point::new(x, 0.), Point::new(x, y), Point::new(9. - x, y), Point::new(9. - x, 0.)],
            breaks: BTreeSet::new(),
            eligible: true,
            requested: true,
            assigned: false,
            glue: [Some(1), Some(1)],
        }
    }
    #[test]
    fn near_parallel_unshared_trunks_keep_every_point() {
        for vertical in [false, true] {
            let doc = drawing(1.);
            let mut e = Evaluator::new(&doc).unwrap();
            let mut routes = vec![route(3, 0.5), route(4, 0.5000002)];
            if vertical {
                for route in &mut routes {
                    for point in &mut route.points {
                        *point = Point::new(point.y, point.x);
                    }
                }
            }
            let original = routes.iter().map(|r| r.points.clone()).collect::<Vec<_>>();
            let mut outcomes = BTreeMap::new();
            apply(&mut e, &mut routes, &mut outcomes).unwrap();
            for (route, points) in routes.iter().zip(&original) {
                assert_eq!(&route.points, points, "distinct parallel trunks must stay unchanged");
                assert!(!route.assigned);
            }
            assert!(outcomes.is_empty());
            assert!(!overlap(original[0][1], original[0][2], original[1][1], original[1][2]));
        }
    }
    #[test]
    fn narrow_channel_skips_touching_positions_and_warns_at_resolution_floor() {
        for height in [1. / 64., 1. / 1024.] {
            let doc = drawing(height);
            let mut e = Evaluator::new(&doc).unwrap();
            let mut routes = vec![route(3, height / 2.), route(4, height / 2.)];
            routes[1].requested = false;
            let original = routes.iter().map(|r| r.points.clone()).collect::<Vec<_>>();
            let mut outcomes = BTreeMap::new();
            apply(&mut e, &mut routes, &mut outcomes).unwrap();
            if height > 1. / 1024. {
                assert_eq!(routes[0].points[1].y, 1. / 128.);
                assert_eq!(routes[1].points[1].y, 3. / 256.);
                assert!(outcomes.is_empty());
                assert!(routes.iter().all(|r| r.points[1].y > 0. && r.points[1].y < height));
            } else {
                for (r, p) in routes.iter().zip(original) {
                    assert_eq!(r.points, p);
                    assert!(!r.assigned);
                    assert!(outcomes[&(0, r.id)].as_ref().unwrap().contains(&format!("connector {}", r.id)));
                }
            }
        }
    }
    #[test]
    fn dyadic_order_and_work_accounting_are_bounded() {
        let values = offsets();
        assert_eq!(&values[..10], &[0.1875, 0.125, 0.0625, 0.15625, 0.09375, 0.03125, 0.171875, 0.140625, 0.109375, 0.078125]);
        assert_eq!(values.len(), 192);
        assert!(values.iter().all(|v| *v >= 1. / 1024.));
        let doc = drawing(1. / 1024.);
        let mut e = Evaluator::new(&doc).unwrap();
        let mut routes = vec![route(3, 0.0005), route(4, 0.0005)];
        let mut budget = Budget(2);
        let result = assign(&mut e, &mut routes, &mut BTreeMap::new(), &mut budget);
        assert_eq!(result, Err("route lane work limit".into()));
        assert_eq!(budget.0, 0);
    }
    #[test]
    fn context_subpaths_do_not_occupy_their_moveto_gap() {
        let doc = drawing(1.);
        let mut e = Evaluator::new(&doc).unwrap();
        let mut context = route(9, 0.1875);
        context.eligible = false;
        context.requested = false;
        context.points = vec![Point::new(0., 0.1), Point::new(0., 0.1875), Point::new(9., 0.1875), Point::new(9., 0.1)];
        context.breaks.insert(1);
        let mut routes = vec![route(3, 0.5), route(4, 0.5), context];
        let mut outcomes = BTreeMap::new();
        apply(&mut e, &mut routes, &mut outcomes).unwrap();
        assert_eq!(routes[0].points[1].y, 0.1875);
        assert_eq!(routes[1].points[1].y, 0.125);
        assert!(outcomes.is_empty());
        routes[2].breaks.clear();
        routes[0] = route(3, 0.5);
        routes[1] = route(4, 0.5);
        apply(&mut e, &mut routes, &mut outcomes).unwrap();
        assert_eq!(routes[0].points[1].y, 0.125);
        assert_eq!(routes[1].points[1].y, 0.0625);
    }
    #[test]
    fn terminal_jog_rejects_self_overlap_and_preserves_glue() {
        let mut own = route(3, 0.5);
        own.points = [(8.5, 9.8125), (9.9, 9.8125), (9.9, 0.), (8., 0.), (8., 10.), (10., 10.)].map(|(x, y)| Point::new(x, y)).to_vec();
        own.assigned = true;
        let mut peer = route(4, 0.5);
        peer.points = vec![Point::new(8., 10.), Point::new(9.5, 10.)];
        peer.eligible = false;
        let ends = (*own.points.first().unwrap(), *own.points.last().unwrap());
        let mut routes = vec![own, peer];
        let mut outcomes = BTreeMap::new();
        let mut budget = Budget(MAX_WORK);
        repair_terminals(&mut routes, 0, &[], &mut outcomes, &mut budget).unwrap();
        assert!(outcomes.is_empty());
        assert_eq!((routes[0].points[0], *routes[0].points.last().unwrap()), ends);
        assert!(!self_overlap(&routes[0].points, &mut budget).unwrap());
        for i in 0..routes[0].points.len() - 1 {
            assert!(!terminal_conflict(&routes[0].points, i, &routes, 0, 0, &mut budget).unwrap());
        }
    }
    #[test]
    fn later_exhausted_trunk_restores_the_entire_connector() {
        let mut doc = skeleton::document(false);
        for (id, x, y, w, h) in [(1, 0.5, -0.5, 1., 0.5), (2, 0.5, 1., 1., 0.5), (3, 2.5, 1.5, 1., 0.5), (4, 2.5, 2. + 1. / 1024., 1., 0.5)] {
            let mut s = skeleton::rectangle(id, x, y, w, h, "").unwrap();
            s.sheet.cells.push(Cell { name: "ObjType".into(), value: Some("1".into()), ..Default::default() });
            doc.pages[0].shapes.push(s);
        }
        let mut e = Evaluator::new(&doc).unwrap();
        let mut routes = vec![route(5, 0.5), route(6, 0.5)];
        for (i, r) in routes.iter_mut().enumerate() {
            let d = i as f64 * 0.01;
            r.points = [(d, 0.), (d, 0.5), (2. - d, 0.5), (2. - d, 2. + 1. / 2048.), (4. - d, 2. + 1. / 2048.), (4. - d, 3.)]
                .map(|(x, y)| Point::new(x, y))
                .to_vec();
            r.glue = [None, None];
        }
        let original = routes.iter().map(|r| r.points.clone()).collect::<Vec<_>>();
        let mut outcomes = BTreeMap::new();
        apply(&mut e, &mut routes, &mut outcomes).unwrap();
        for (r, points) in routes.iter().zip(original) {
            assert_eq!(r.points, points);
            assert!(!r.assigned);
            assert!(outcomes[&(0, r.id)].as_ref().unwrap().contains(&format!("connector {}", r.id)));
        }
    }
    #[test]
    fn translation_skips_its_own_preexisting_segment() {
        let doc = skeleton::document(false);
        let mut e = Evaluator::new(&doc).unwrap();
        let mut own = route(3, 1.);
        own.points = [(0., 0.), (0., 1.), (4., 1.), (4., 0.1875), (1., 0.1875), (1., -1.)].map(|(x, y)| Point::new(x, y)).to_vec();
        let mut peer = route(4, 1.);
        peer.points = [(0.1, 0.), (0.1, 1.), (3.9, 1.), (3.9, 2.)].map(|(x, y)| Point::new(x, y)).to_vec();
        let mut routes = vec![own, peer];
        let mut outcomes = BTreeMap::new();
        apply(&mut e, &mut routes, &mut outcomes).unwrap();
        assert!(outcomes.is_empty(), "{outcomes:?}");
        assert_eq!(routes[0].points[1].y, 0.125);
        assert!(!self_overlap(&routes[0].points, &mut Budget(MAX_WORK)).unwrap());
    }
    #[test]
    fn coincident_terminal_points_need_the_same_actual_glue_target() {
        let a = [Point::new(0., 0.), Point::new(0., 1.)];
        let b = [Point::new(0., 0.), Point::new(0., 2.)];
        assert!(same_glue(&a, 0, [Some(1), Some(2)], &b, 0, [Some(1), Some(3)]));
        assert!(!same_glue(&a, 0, [Some(1), Some(2)], &b, 0, [Some(4), Some(3)]));
        assert!(!same_glue(&a, 0, [None, None], &b, 0, [None, None]));
    }
    #[test]
    fn hostile_points_are_rejected_by_geometry_validation() {
        let mut doc = drawing(1.);
        doc.pages[0].shapes[0].sheet.cells.retain(|c| c.name != "PinX");
        doc.pages[0].shapes[0].sheet.cells.push(Cell { name: "PinX".into(), value: Some("NaN".into()), ..Default::default() });
        let mut e = Evaluator::new(&doc).unwrap();
        let mut routes = vec![route(3, 0.5), route(4, 0.5)];
        assert!(apply(&mut e, &mut routes, &mut BTreeMap::new()).is_err());
    }
}
