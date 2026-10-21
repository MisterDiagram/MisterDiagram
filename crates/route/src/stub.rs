//! Perpendicular static-port entry observed in Brief26b (#66).
//! https://learn.microsoft.com/en-us/office/client-developer/visio/shaperoutestyle-cell-shape-layout-section
use misterdiagram_geom::{Point, Rect, Vec2};
const STUB: f64 = 0.1875;
pub(crate) fn normal(bounds: Rect, p: Point) -> Option<Vec2> {
    if p.x < bounds.x0 - 1e-6 || p.x > bounds.x1 + 1e-6 || p.y < bounds.y0 - 1e-6 || p.y > bounds.y1 + 1e-6 {
        return None;
    }
    [
        (p.x - bounds.x0, Vec2::new(-1., 0.)),
        (p.x - bounds.x1, Vec2::new(1., 0.)),
        (p.y - bounds.y0, Vec2::new(0., -1.)),
        (p.y - bounds.y1, Vec2::new(0., 1.)),
    ]
    .into_iter()
    .find(|(d, _)| d.abs() <= 1e-6)
    .map(|(_, n)| n)
}
fn clear(a: Point, b: Point, obstacles: &[Rect]) -> bool {
    obstacles.iter().all(|r| {
        if (a.y - b.y).abs() < 1e-9 {
            !(a.y > r.y0 + 1e-9 && a.y < r.y1 - 1e-9 && a.x.min(b.x) < r.x1 - 1e-9 && a.x.max(b.x) > r.x0 + 1e-9)
        } else if (a.x - b.x).abs() < 1e-9 {
            !(a.x > r.x0 + 1e-9 && a.x < r.x1 - 1e-9 && a.y.min(b.y) < r.y1 - 1e-9 && a.y.max(b.y) > r.y0 + 1e-9)
        } else {
            false
        }
    })
}
fn valid(points: &[Point], normals: [Option<Vec2>; 2], obs: &[Rect]) -> bool {
    let Some(a) = points.first().zip(points.get(1)) else {
        return false;
    };
    let Some(b) = points.len().checked_sub(2).and_then(|i| points.get(i)).zip(points.last()) else {
        return false;
    };
    let matches = |d: Vec2, n: Option<Vec2>| n.is_none_or(|n| d.cross(n).abs() < 1e-6 && d.dot(n) > 0.);
    matches(*a.1 - *a.0, normals[0])
        && matches(*b.0 - *b.1, normals[1])
        && points.windows(2).all(|w| w.first().zip(w.get(1)).is_some_and(|(a, b)| clear(*a, *b, obs)))
}
pub(crate) fn path(
    start: Point,
    end: Point,
    normals: [Option<Vec2>; 2],
    obstacles: &[Rect],
    horizontal_runs: &[(Point, Point)],
    old: Vec<Point>,
) -> Result<Vec<Point>, String> {
    if valid(&old, normals, obstacles) {
        return Ok(old);
    }
    let a = normals[0].map_or(start, |n| start + n * STUB);
    let b = normals[1].map_or(end, |n| end + n * STUB);
    let vertical = normals[0].is_some_and(|n| n.y != 0.);
    let mut candidates = if vertical {
        vec![vec![start, a, Point::new(end.x, a.y), end], vec![start, Point::new(start.x, b.y), b, end]]
    } else {
        vec![vec![start, a, Point::new(a.x, end.y), end], vec![start, Point::new(b.x, start.y), b, end]]
    };
    let overlaps = |points: &[Point]| {
        points.windows(2).any(|w| {
            w.first().zip(w.get(1)).is_some_and(|(a, b)| {
                (a.y - b.y).abs() < 1e-9
                    && horizontal_runs
                        .iter()
                        .any(|(c, d)| (a.y - c.y).abs() < 1e-6 && a.x.max(b.x).min(c.x.max(d.x)) - a.x.min(b.x).max(c.x.min(d.x)) > STUB)
            })
        })
    };
    for points in &mut candidates {
        points.dedup_by(|a, b| a.distance(*b) < 1e-9);
    }
    for avoid in [true, false] {
        for points in &candidates {
            if valid(points, normals, obstacles) && (!avoid || !overlaps(points)) {
                return Ok(points.clone());
            }
        }
    }
    let mut middle = crate::orthogonal::path(a, b, obstacles, None, None)?;
    middle.insert(0, start);
    middle.push(end);
    middle.dedup_by(|a, b| a.distance(*b) < 1e-9);
    if valid(&middle, normals, obstacles) { Ok(middle) } else { Err("no orthogonal port route".into()) }
}
