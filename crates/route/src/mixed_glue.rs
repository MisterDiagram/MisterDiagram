//! Whole-shape walking opposite a fixed connection point, Brief26b.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/walkpreference-cell-glue-info-section
use misterdiagram_geom::{Point, Rect};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};

pub(crate) fn endpoint(
    e: &mut Evaluator<'_>,
    scope: Scope,
    start: Point,
    boxes: [Rect; 2],
    obstacles: &[Rect],
    clearance: (f64, f64),
    preferred: Option<Point>,
) -> Result<Point, String> {
    let Scope::PageShape { page, shape: connector } = scope else { return Err("mixed glue scope".into()) };
    let [source, target] = boxes;
    let center = target.center();
    let normal = crate::stub::normal(source, start);
    let mut candidates = Vec::new();
    if start.x >= target.x0 && start.x <= target.x1 {
        candidates.push(Point::new(start.x, if start.y > center.y { target.y1 } else { target.y0 }));
    }
    if start.y >= target.y0 && start.y <= target.y1 {
        candidates.push(Point::new(if start.x > center.x { target.x1 } else { target.x0 }, start.y));
    }
    let aligned = candidates.len();
    if let Some(preferred) = preferred.filter(|p| crate::stub::normal(target, *p).is_some()) {
        candidates.push(preferred);
    }
    let retained = candidates.len() > aligned;
    let x = if start.x > center.x { target.x1 } else { target.x0 };
    let mut cuts = vec![target.y0, target.y1];
    let mut segments = 0usize;
    for scope in e.scopes().collect::<Vec<_>>() {
        if !matches!(scope, Scope::PageShape{page:p,shape} if p==page && shape!=connector) || crate::orthogonal::number(e, scope, "ObjType", 0.) != 2.
        {
            continue;
        }
        let transform = e.shape_to_page(scope).map_err(|v| v.to_string())?;
        let mut rows = std::collections::BTreeSet::new();
        for n in e.resolved_named_cells(scope) {
            if n.section.is_some_and(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0)
                && let Some(r) = n.row.filter(|r| !r.deleted)
            {
                rows.insert(r.index.unwrap_or(0));
            }
        }
        segments = segments.checked_add(rows.len()).ok_or("mixed glue segment limit")?;
        if segments > 100000 {
            return Err("mixed glue segment limit".into());
        }
        let mut previous: Option<Point> = None;
        for ix in rows {
            let p = transform
                * Point::new(
                    crate::orthogonal::number(e, scope, &format!("Geometry1.X{ix}"), 0.),
                    crate::orthogonal::number(e, scope, &format!("Geometry1.Y{ix}"), 0.),
                );
            if !p.x.is_finite() || !p.y.is_finite() || p.x.abs() > 1e6 || p.y.abs() > 1e6 {
                return Err("mixed glue coordinate limit".into());
            }
            if let Some(a) = previous
                && (a.y - p.y).abs() < 1e-6
                && p.y > target.y0
                && p.y < target.y1
                && a.x.min(p.x) < start.x.max(x)
                && a.x.max(p.x) > start.x.min(x)
            {
                cuts.push(p.y);
            }
            previous = Some(p);
        }
    }
    cuts.sort_by(f64::total_cmp);
    cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-6);
    let y = cuts
        .windows(2)
        .filter_map(|p| p.first().zip(p.get(1)).map(|(a, b)| (a + b) / 2.))
        .min_by(|a, b| (a - center.y).abs().total_cmp(&(b - center.y).abs()).then_with(|| (a - start.y).abs().total_cmp(&(b - start.y).abs())))
        .unwrap_or(center.y);
    candidates.extend([
        Point::new(x, y),
        Point::new(center.x, target.y1),
        Point::new(center.x, target.y0),
        Point::new(if x == target.x0 { target.x1 } else { target.x0 }, center.y),
    ]);
    let mut best = None;
    let mut limit = None;
    for (index, end) in candidates.into_iter().enumerate() {
        let end_normal = crate::stub::normal(target, end);
        let inflated = crate::orthogonal::clearance(obstacles, clearance, [start, end]);
        let axis = |v: Option<misterdiagram_geom::Vec2>| v.map(|n| usize::from(n.y != 0.));
        let path = crate::orthogonal::path(start, end, &inflated, axis(normal), axis(end_normal));
        let path = path.and_then(|path| crate::stub::path(start, end, [normal, end_normal], &inflated, &[], path));
        if let Err(error) = &path
            && crate::orthogonal::capacity(error)
        {
            limit = Some(error.clone());
        }
        if let Ok(path) = path {
            if index < aligned && path.len() == 2 || index == aligned && retained {
                return Ok(end);
            }
            let cost = path.windows(2).filter_map(|p| p.first().zip(p.get(1))).map(|(a, b)| a.distance(*b)).sum::<f64>() + path.len() as f64 * 0.01;
            if best.is_none_or(|(old, _)| cost < old - 1e-9) {
                best = Some((cost, end));
            }
        }
    }
    best.map(|(_, point)| point).ok_or_else(|| limit.unwrap_or_else(|| "no orthogonal mixed glue route".into()))
}
