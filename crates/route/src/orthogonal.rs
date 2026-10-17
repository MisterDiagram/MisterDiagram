//! Bounded orthogonal paths from alignment boxes and visible obstacle edges.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/routestyle-cell-page-layout-section
use misterdiagram_geom::{Point, Rect};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub(crate) fn number(e: &mut Evaluator<'_>, scope: Scope, name: &str, default: f64) -> f64 {
    e.evaluate(scope, name).numeric().ok().filter(|n| n.is_finite() && n.abs() <= 1e6).unwrap_or(default)
}
/// Alignment boxes determine obstacle clearance, independent of fill or line paint.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/objtype-cell-miscellaneous-section
pub(crate) fn bounds(e: &mut Evaluator<'_>, scope: Scope) -> Result<Rect, String> {
    let width = number(e, scope, "Width", 0.);
    let height = number(e, scope, "Height", 0.);
    let t = e.shape_to_page(scope).map_err(|v| v.to_string())?;
    let b = t.transform_rect_bbox(Rect::new(0., 0., width, height));
    if [b.x0, b.y0, b.x1, b.y1].iter().any(|n| !n.is_finite() || n.abs() > 1e6) {
        return Err("route box limit".into());
    }
    Ok(b)
}
/// Only non-permeable placeable shapes block a connector.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shapepermeablex-cell-shape-layout-section
pub(crate) fn obstacles(e: &mut Evaluator<'_>, page: u32, excluded: &[u32]) -> Result<Vec<Rect>, String> {
    let scopes = e.scopes().filter(|s| matches!(s, Scope::PageShape {page:p,shape} if *p==page && !excluded.contains(shape))).collect::<Vec<_>>();
    let mut out = Vec::new();
    for scope in scopes {
        if number(e, scope, "ObjType", 0.) != 1. {
            continue;
        }
        if number(e, scope, "ShapePermeableX", 0.) != 0. && number(e, scope, "ShapePermeableY", 0.) != 0. {
            continue;
        }
        if out.len() >= 60 {
            return Err("route obstacle count limit".into());
        }
        out.push(bounds(e, scope)?);
    }
    Ok(out)
}
/// Connector-local style overrides the page default.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shaperoutestyle-cell-shape-layout-section
pub(crate) fn style(e: &mut Evaluator<'_>, scope: Scope, page: u32) -> f64 {
    let local = number(e, scope, "ShapeRouteStyle", 0.);
    if local == 0. { number(e, Scope::Page(page), "RouteStyle", 0.) } else { local }
}
fn center(b: Rect) -> Point {
    Point::new((b.x0 + b.x1) / 2., (b.y0 + b.y1) / 2.)
}
fn blocked(a: Point, b: Point, obs: &[Rect]) -> bool {
    obs.iter().any(|r| {
        if (a.y - b.y).abs() < 1e-9 {
            a.y > r.y0 + 1e-9 && a.y < r.y1 - 1e-9 && a.x.min(b.x) < r.x1 - 1e-9 && a.x.max(b.x) > r.x0 + 1e-9
        } else if (a.x - b.x).abs() < 1e-9 {
            a.x > r.x0 + 1e-9 && a.x < r.x1 - 1e-9 && a.y.min(b.y) < r.y1 - 1e-9 && a.y.max(b.y) > r.y0 + 1e-9
        } else {
            true
        }
    })
}
fn clear_y(a: Rect, b: Rect, obs: &[Rect]) -> Option<f64> {
    let low = a.y0.max(b.y0);
    let high = a.y1.min(b.y1);
    if low > high {
        return None;
    }
    let mut intervals = vec![(low, high)];
    for r in obs {
        if r.x0 >= a.x1.max(b.x1) || r.x1 <= a.x0.min(b.x0) {
            continue;
        }
        let mut next = Vec::new();
        for (l, h) in intervals {
            if r.y1 <= l || r.y0 >= h {
                next.push((l, h));
            } else {
                if r.y0 > l {
                    next.push((l, r.y0));
                }
                if r.y1 < h {
                    next.push((r.y1, h));
                }
            }
        }
        intervals = next;
    }
    intervals.into_iter().map(|(l, h)| (l + h) / 2.).min_by(|a, b| (a - (low + high) / 2.).abs().total_cmp(&(b - (low + high) / 2.).abs()))
}
/// Dynamic walking chooses the common free alignment interval before adding bends.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/walkpreference-cell-glue-info-section
pub(crate) fn endpoints(a: Rect, b: Rect, obs: &[Rect], style: f64, clearance: (f64, f64)) -> Result<[Point; 2], String> {
    let ac = center(a);
    let bc = center(b);
    if style == 16. {
        let ray = |r: Rect, c: Point, t: Point| {
            let d = t - c;
            let sx = if d.x.abs() > 1e-12 { r.width() / 2. / d.x.abs() } else { f64::INFINITY };
            let sy = if d.y.abs() > 1e-12 { r.height() / 2. / d.y.abs() } else { f64::INFINITY };
            if sx.min(sy).is_finite() { c + d * sx.min(sy) } else { c }
        };
        return Ok([ray(a, ac, bc), ray(b, bc, ac)]);
    }
    if style == 2. {
        return Ok([
            Point::new(if bc.x >= ac.x { a.x1 } else { a.x0 }, bc.y.clamp(a.y0, a.y1)),
            Point::new(if ac.x <= bc.x { b.x0 } else { b.x1 }, ac.y.clamp(b.y0, b.y1)),
        ]);
    }
    if a.x0.max(b.x0) < a.x1.min(b.x1) - 1e-9 && (a.y1 < b.y0 || b.y1 < a.y0) {
        let x = (a.x0.max(b.x0) + a.x1.min(b.x1)) / 2.;
        return Ok([Point::new(x, if bc.y > ac.y { a.y1 } else { a.y0 }), Point::new(x, if bc.y > ac.y { b.y0 } else { b.y1 })]);
    }
    if let Some(y) = clear_y(a, b, obs) {
        return Ok([Point::new(if bc.x >= ac.x { a.x1 } else { a.x0 }, y), Point::new(if bc.x >= ac.x { b.x0 } else { b.x1 }, y)]);
    }
    if a.y1 < b.y0 || b.y1 < a.y0 {
        return Ok([Point::new(ac.x, if bc.y > ac.y { a.y1 } else { a.y0 }), Point::new(if bc.x > ac.x { b.x0 } else { b.x1 }, bc.y)]);
    }
    let start = Point::new(if bc.x >= ac.x { a.x1 } else { a.x0 }, ac.y);
    let candidates = [Point::new(if bc.x >= ac.x { b.x0 } else { b.x1 }, bc.y), Point::new(bc.x, b.y1), Point::new(bc.x, b.y0)];
    let mut best = None;
    for (side, end) in candidates.into_iter().enumerate() {
        let inflated = self::clearance(obs, clearance, [start, end]);
        if let Ok(path) = path(start, end, &inflated, Some(0), Some(if side == 0 { 0 } else { 1 })) {
            let cost =
                path.windows(2).map(|s| s.first().zip(s.get(1)).map_or(0., |(a, b)| (*b - *a).hypot())).sum::<f64>() + path.len() as f64 * 0.01;
            if best.as_ref().is_none_or(|(v, _)| cost < *v - 1e-9) {
                best = Some((cost, end));
            }
        }
    }
    Ok([start, best.map(|(_, p)| p).unwrap_or(candidates[0])])
}
#[derive(Clone, Copy)]
struct Node {
    cost: f64,
    index: usize,
}
impl PartialEq for Node {
    fn eq(&self, o: &Self) -> bool {
        self.cost == o.cost && self.index == o.index
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Node {
    fn cmp(&self, o: &Self) -> Ordering {
        o.cost.total_cmp(&self.cost).then_with(|| o.index.cmp(&self.index))
    }
}
/// Rectilinear visibility grid, bounded before allocation and with bend-aware shortest paths.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.reroute
pub(crate) fn path(start: Point, end: Point, obs: &[Rect], first_dir: Option<usize>, last_dir: Option<usize>) -> Result<Vec<Point>, String> {
    if [start.x, start.y, end.x, end.y].iter().any(|n| !n.is_finite() || n.abs() > 1e6) || obs.len() > 60 {
        return Err("route input limit".into());
    }
    if ((start.x - end.x).abs() < 1e-9 || (start.y - end.y).abs() < 1e-9) && !blocked(start, end, obs) {
        return Ok(vec![start, end]);
    }
    let via = Point::new(start.x, end.y);
    if first_dir.is_none_or(|d| d == 1) && last_dir.is_none_or(|d| d == 0) && !blocked(start, via, obs) && !blocked(via, end, obs) {
        return Ok(vec![start, via, end]);
    }
    let mut xs = vec![start.x, end.x];
    let mut ys = vec![start.y, end.y];
    for r in obs {
        xs.extend([r.x0, r.x1]);
        ys.extend([r.y0, r.y1]);
    }
    for values in [&mut xs, &mut ys] {
        values.sort_by(f64::total_cmp);
        values.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    }
    let n = xs.len().checked_mul(ys.len()).filter(|n| *n <= 16384).ok_or("route grid limit")?;
    let locate = |p: Point| -> Result<usize, String> {
        let x = xs.iter().position(|x| (*x - p.x).abs() < 1e-9).ok_or("route x")?;
        let y = ys.iter().position(|y| (*y - p.y).abs() < 1e-9).ok_or("route y")?;
        Ok(y * xs.len() + x)
    };
    let position = |i: usize| -> Result<Point, String> {
        Ok(Point::new(*xs.get(i % xs.len()).ok_or("route x index")?, *ys.get(i / xs.len()).ok_or("route y index")?))
    };
    let first = locate(start)?;
    let last = locate(end)?;
    let mut dist = vec![f64::INFINITY; n * 2];
    let mut prev = vec![None; n * 2];
    let mut queue = BinaryHeap::new();
    for dir in 0..2 {
        let i = first * 2 + dir;
        *dist.get_mut(i).ok_or("route first")? = 0.;
        queue.push(Node { cost: 0., index: i });
    }
    let mut goal = None;
    while let Some(Node { cost, index }) = queue.pop() {
        if cost > *dist.get(index).ok_or("route distance")? {
            continue;
        }
        let cell = index / 2;
        if cell == last {
            goal = Some(index);
            break;
        }
        let x = cell % xs.len();
        let y = cell / xs.len();
        let p = position(cell)?;
        for (next, dir) in [
            (x.checked_sub(1).map(|x| y * xs.len() + x), 0),
            ((x + 1 < xs.len()).then_some(cell + 1), 0),
            (y.checked_sub(1).map(|y| y * xs.len() + x), 1),
            ((y + 1 < ys.len()).then_some(cell + xs.len()), 1),
        ] {
            let Some(next) = next else { continue };
            if cell == first && first_dir.is_some_and(|d| d != dir) || next == last && last_dir.is_some_and(|d| d != dir) {
                continue;
            }
            let q = position(next)?;
            if blocked(p, q, obs) {
                continue;
            }
            let next_index = next * 2 + dir;
            let distance = cost + (q - p).hypot() + if index % 2 == dir { 0. } else { 0.01 };
            if distance < *dist.get(next_index).ok_or("route next distance")? - 1e-9 {
                *dist.get_mut(next_index).ok_or("route distance slot")? = distance;
                *prev.get_mut(next_index).ok_or("route predecessor")? = Some(index);
                queue.push(Node { cost: distance, index: next_index });
                if queue.len() > n * 16 {
                    return Err("route queue limit".into());
                }
            }
        }
    }
    let mut index = goal.ok_or("no orthogonal route")?;
    let mut points = Vec::new();
    loop {
        points.push(position(index / 2)?);
        if points.len() > n * 2 {
            return Err("route predecessor cycle".into());
        }
        let Some(p) = *prev.get(index).ok_or("route predecessor index")? else { break };
        index = p;
    }
    points.reverse();
    let mut simplified: Vec<Point> = Vec::new();
    for p in points {
        while simplified.len() >= 2 {
            let a = *simplified.get(simplified.len() - 2).ok_or("route segment")?;
            let b = *simplified.last().ok_or("route last")?;
            if ((b - a).cross(p - b)).abs() > 1e-9 {
                break;
            }
            simplified.pop();
        }
        simplified.push(p);
    }
    Ok(simplified)
}

/// Transform, obstacle and layout dependencies that invalidate page routes.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/eventxfmod-cell-events-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/avoidpagebreaks-cell-page-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/blocksizex-cell-page-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linetonodex-cell-page-layout-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/plowcode-cell-page-layout-section
pub(crate) fn page_signature(e: &mut Evaluator<'_>, page: u32) -> Result<Vec<(u64, Vec<f64>)>, String> {
    let scopes = e.scopes().filter(|s| matches!(s,Scope::PageShape{page:p,..}if *p==page)).collect::<Vec<_>>();
    if scopes.len() > 100000 {
        return Err("route dependency count limit".into());
    }
    let layout = [
        "RouteStyle",
        "PlowCode",
        "BlockSizeX",
        "BlockSizeY",
        "LineToNodeX",
        "LineToNodeY",
        "LineToLineX",
        "LineToLineY",
        "AvoidPageBreaks",
        "PageWidth",
        "PageHeight",
        "LineJumpCode",
        "LineJumpStyle",
        "LineJumpFactorX",
        "LineJumpFactorY",
        "PageLineJumpDirX",
        "PageLineJumpDirY",
    ];
    let mut signature = vec![(u64::MAX, layout.map(|n| number(e, Scope::Page(page), n, 0.)).to_vec())];
    for scope in scopes {
        let Scope::PageShape { shape, .. } = scope else { continue };
        let mut values = [
            "Width",
            "Height",
            "ObjType",
            "ShapePermeableX",
            "ShapePermeableY",
            "ShapeRouteStyle",
            "ShapePlowCode",
            "ConFixedCode",
            "ConLineJumpCode",
            "ConLineJumpStyle",
            "ConLineJumpDirX",
            "ConLineJumpDirY",
        ]
        .map(|n| number(e, scope, n, 0.))
        .to_vec();
        values.extend(e.shape_to_page(scope).map_err(|v| v.to_string())?.as_coeffs());
        signature.push((u64::from(shape), values));
    }
    signature.sort_by_key(|(id, _)| *id);
    Ok(signature)
}

/// Reduce clearance only where a neighboring obstacle would engulf an attachment.
/// An endpoint already inside an obstacle can leave it; other obstacles retain clearance.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linetolinex-cell-page-layout-section
pub(crate) fn clearance(obstacles: &[Rect], distance: (f64, f64), ends: [Point; 2]) -> Vec<Rect> {
    obstacles
        .iter()
        .filter(|r| !ends.iter().any(|p| p.x > r.x0 + 1e-9 && p.x < r.x1 - 1e-9 && p.y > r.y0 + 1e-9 && p.y < r.y1 - 1e-9))
        .map(|r| {
            let (mut x, mut y) = distance;
            for p in ends {
                // A boundary attachment may leave along or away from that edge.
                // Keep the obstacle interior, but do not inflate across the attachment.
                if p.y >= r.y0 - 1e-9 && p.y <= r.y1 + 1e-9 && ((p.x - r.x0).abs() <= 1e-9 || (p.x - r.x1).abs() <= 1e-9) {
                    x = 0.;
                }
                if p.x >= r.x0 - 1e-9 && p.x <= r.x1 + 1e-9 && ((p.y - r.y0).abs() <= 1e-9 || (p.y - r.y1).abs() <= 1e-9) {
                    y = 0.;
                }
                let dx = (r.x0 - p.x).max(p.x - r.x1).max(0.);
                let dy = (r.y0 - p.y).max(p.y - r.y1).max(0.);
                if dx < x && dy < y && (dx > 0. || dy > 0.) {
                    if dx >= dy {
                        x = x.min(dx);
                    } else {
                        y = y.min(dy);
                    }
                }
            }
            r.inflate(x, y)
        })
        .collect()
}

/// The Dynamic connector inherits its initial zero coordinates from its master.
/// Only literal zero providers can be omitted safely across changes of frame size.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/moveto-row-geometry-section
pub(crate) fn inherited_zero(e: &Evaluator<'_>, scope: Scope) -> [[bool; 2]; 2] {
    [1, 2].map(|row| {
        ["X", "Y"].map(|column| {
            let name = format!("Geometry1.{column}{row}");
            e.master_source(scope).and_then(|m| e.cell(m, &name)).is_some_and(|c| {
                c.value.as_deref().and_then(|v| v.parse::<f64>().ok()) == Some(0.)
                    && c.formula.as_deref().is_none_or(|f| matches!(f.trim(), "0" | "0 in"))
            })
        })
    })
}
pub(crate) fn trim_inherited_start(rows: &mut Vec<misterdiagram_model::Row>, zero: [[bool; 2]; 2]) {
    for (i, zero) in zero.into_iter().enumerate() {
        let index = u32::try_from(i + 1).unwrap_or(0);
        if let Some(row) = rows.iter_mut().find(|r| r.index == Some(index)) {
            row.cells.retain(|c| {
                !(c.formula.is_none()
                    && c.value.as_deref().and_then(|v| v.parse::<f64>().ok()) == Some(0.)
                    && match c.name.as_str() {
                        "X" => zero[0],
                        "Y" => zero[1],
                        _ => false,
                    })
            });
        }
    }
    rows.retain(|r| r.index != Some(1) || !r.cells.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn covered_endpoints_leave_other_obstacles_and_boundary_interiors_enforced() {
        let start = Point::ORIGIN;
        let end = Point::new(6., 0.);
        let barrier = Rect::new(3., -1., 4., 1.);
        for host in [Rect::new(-1., -1., 1., 1.), Rect::new(-2., -1., 0., 1.)] {
            let obstacles = clearance(&[host, barrier], (0.25, 0.25), [start, end]);
            let inflated = barrier.inflate(0.25, 0.25);
            assert!(obstacles.contains(&inflated), "unrelated barrier lost its clearance");
            let route = path(start, end, &obstacles, Some(0), Some(0)).unwrap();
            assert!(route.len() > 2);
            assert!(route.windows(2).all(|s| !blocked(s[0], s[1], &[inflated])));
            if host.x1 == start.x {
                assert_eq!(obstacles.len(), 2);
                assert!(route.windows(2).all(|s| !blocked(s[0], s[1], &[host])));
            }
        }
    }
    #[test]
    fn obstacle_clearance_is_geometric_and_bounded() {
        let obstacle = Rect::new(2., -1., 3., 1.);
        let route = path(Point::new(0., 0.), Point::new(5., 0.), &[obstacle], Some(0), Some(0)).unwrap();
        assert!(route.len() > 2);
        assert!(route.windows(2).all(|s| !blocked(s[0], s[1], &[obstacle])));
        assert!(path(Point::new(f64::NAN, 0.), Point::new(5., 0.), &[], None, None).is_err());
        assert!(path(Point::ORIGIN, Point::new(5., 0.), &vec![obstacle; 61], None, None).is_err());
    }
}
