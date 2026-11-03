//! Connector intent and page-qualified attachment, without evaluating cells.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
use super::Substitution;
use kurbo::{BezPath, PathEl, Point, Rect};
use misterdiagram_render::scene::{ConnectionSite, EvaluatedConnect, EvaluatedShape, SceneBudget};
use misterdiagram_vsdx::evaluate::Scope;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct Attachment {
    pub target: Scope,
    pub index: usize,
}
#[derive(Clone, Debug)]
pub struct Connector {
    /// Intended route in page inches, before PowerPoint's preset solver runs.
    pub points: Vec<Point>,
    pub curved: bool,
    pub fitted: bool,
    pub jumps: bool,
    pub begin: Option<Attachment>,
    pub end: Option<Attachment>,
}
#[derive(Default)]
pub struct Connections {
    pub ids: BTreeMap<Scope, u32>,
    pub sites: BTreeMap<Scope, Vec<ConnectionSite>>,
    pub targets: BTreeMap<Scope, u32>,
    pub boxes: BTreeMap<Scope, Rect>,
    pub routes: BTreeMap<Scope, Connector>,
    pub attached: BTreeSet<Scope>,
    names: BTreeMap<Scope, BTreeMap<String, usize>>,
}
impl Connections {
    pub(super) fn prepare(
        shapes: &mut [EvaluatedShape],
        connects: &[EvaluatedConnect],
        substitutions: &mut BTreeSet<Substitution>,
        warnings: &mut Vec<String>,
        budget: &mut SceneBudget,
    ) -> Result<Self, String> {
        let mut result = Self::default();
        result.collect(shapes, substitutions, warnings, budget, 0)?;
        let mut next_carrier = u32::try_from(result.ids.len()).map_err(|_| "PPTX id limit")?.checked_add(2).ok_or("PPTX id limit")?;
        for c in connects {
            result.attached.insert(c.to);
            if result.routes.contains_key(&c.to) && result.targets.get(&c.to) == result.ids.get(&c.to) {
                result.targets.insert(c.to, next_carrier);
                next_carrier = next_carrier.checked_add(1).ok_or("PPTX carrier id limit")?;
                warn(warnings, "PPTX: glue to a 1-D target uses a transparent site carrier grouped with the connector");
            }
            let endpoint = result.routes.get(&c.from).ok_or("PPTX glue source is not a visible connector")?;
            let point = if c.end { endpoint.points.last() } else { endpoint.points.first() }.copied().ok_or("PPTX empty connector")?;
            let sites = result.sites.get_mut(&c.to).ok_or("PPTX glue target is not a visible shape")?;
            let explicit = result.names.get(&c.to).and_then(|names| names.get(&c.to_cell.to_ascii_lowercase())).copied();
            let index = if let Some(index) = explicit {
                index
            } else {
                if sites.len() >= 16384 {
                    return Err("PPTX connection site limit".into());
                }
                budget.charge_bytes(std::mem::size_of::<ConnectionSite>() * 2)?;
                sites.push(ConnectionSite { name: String::new(), position: point, normal: kurbo::Vec2::ZERO });
                sites.len() - 1
            };
            let site = sites.get(index).ok_or("PPTX connection site index")?.position;
            let route = result.routes.get_mut(&c.from).ok_or("PPTX connector missing")?;
            let slot = if c.end { &mut route.end } else { &mut route.begin };
            if slot.is_some() {
                return Err("PPTX duplicate endpoint glue".into());
            }
            *slot = Some(Attachment { target: c.to, index });
            let end = if c.end { route.points.last_mut() } else { route.points.first_mut() }.ok_or("PPTX empty route")?;
            if (*end - site).hypot() > 1e-8 {
                // A stale saved route must still open attached to its declared site.
                route.fitted = true;
                substitutions.insert(Substitution::Route);
                warn(warnings, "PPTX: route fitted to a preset (saved endpoint adjusted to its connection site)");
            }
            *end = site;
        }
        let fitted = result.routes.values().filter(|r| r.fitted).count();
        let jumps = result.routes.values().filter(|r| r.jumps).count();
        if fitted > 0 {
            warn(warnings, &format!("PPTX: route fitted to a preset on {fitted} connectors"));
        }
        if jumps > 0 {
            warn(warnings, &format!("PPTX: line jumps dropped on {jumps} connectors"));
        }
        result.apply(shapes, 0)?;
        Ok(result)
    }
    fn collect(
        &mut self,
        shapes: &[EvaluatedShape],
        substitutions: &mut BTreeSet<Substitution>,
        warnings: &mut Vec<String>,
        budget: &mut SceneBudget,
        depth: usize,
    ) -> Result<(), String> {
        if depth > 128 {
            return Err("PPTX connector tree depth limit".into());
        }
        for n in shapes {
            if self.ids.len() >= 100_000 {
                return Err("PPTX shape count limit".into());
            }
            let id = u32::try_from(self.ids.len()).map_err(|_| "PPTX id overflow")?.checked_add(2).ok_or("PPTX id overflow")?;
            if self.ids.insert(n.scope, id).is_some() {
                return Err("PPTX duplicate shape identity".into());
            }
            self.targets.insert(n.scope, id);
            budget.charge_bytes(256)?;
            if n.one_d {
                let (route, jump) = route(n)?;
                budget.charge_bytes(route.points.capacity().saturating_mul(std::mem::size_of::<Point>()).saturating_add(256))?;
                if jump {
                    substitutions.insert(Substitution::Jumps);
                    warn(warnings, "PPTX: line jumps dropped");
                }
                if route.fitted {
                    substitutions.insert(Substitution::Route);
                    warn(warnings, "PPTX: route fitted to a preset");
                }
                self.routes.insert(n.scope, route);
            }
            {
                let b = bounds(n, 0)?;
                self.boxes.insert(n.scope, b);
                let mut sites = vec![
                    site(Point::new(b.center().x, b.y1), kurbo::Vec2::new(0., 1.)),
                    site(Point::new(b.x0, b.center().y), kurbo::Vec2::new(-1., 0.)),
                    site(Point::new(b.center().x, b.y0), kurbo::Vec2::new(0., -1.)),
                    site(Point::new(b.x1, b.center().y), kurbo::Vec2::new(1., 0.)),
                ];
                for s in &n.connections {
                    if sites.len() >= 16384 {
                        return Err("PPTX connection site limit".into());
                    }
                    finite(s.position)?;
                    if !s.normal.x.is_finite() || !s.normal.y.is_finite() {
                        return Err("PPTX invalid site normal".into());
                    }
                    budget.charge_bytes(std::mem::size_of::<ConnectionSite>() * 2 + s.name.len())?;
                    sites.push(s.clone());
                }
                budget.charge_bytes(4 * std::mem::size_of::<ConnectionSite>() * 2 + 256)?;
                let mut names = BTreeMap::new();
                for (index, site) in sites.iter().enumerate().skip(4) {
                    budget.charge_bytes(site.name.len().saturating_add(128))?;
                    names.entry(site.name.to_ascii_lowercase()).or_insert(index);
                }
                self.names.insert(n.scope, names);
                self.sites.insert(n.scope, sites);
            }
            self.collect(&n.children, substitutions, warnings, budget, depth + 1)?;
        }
        Ok(())
    }
    fn apply(&self, shapes: &mut [EvaluatedShape], depth: usize) -> Result<(), String> {
        if depth > 128 {
            return Err("PPTX connector tree depth limit".into());
        }
        for n in shapes {
            if let Some(r) = self.routes.get(&n.scope) {
                let mut path = BezPath::new();
                if let Some(p) = r.points.first() {
                    path.move_to(*p);
                }
                for p in r.points.iter().skip(1) {
                    path.line_to(*p);
                }
                let mut flags = n.sections.iter().find(|s| s.stroked()).map(|s| s.flags).unwrap_or(misterdiagram_geom::GeometryFlags {
                    no_fill: true,
                    no_line: true,
                    ..Default::default()
                });
                flags.no_fill = true;
                n.paint = None;
                n.sections = vec![misterdiagram_geom::GeometryPath { path, flags }];
            }
            self.apply(&mut n.children, depth + 1)?;
        }
        Ok(())
    }
}
fn site(position: Point, normal: kurbo::Vec2) -> ConnectionSite {
    ConnectionSite { name: String::new(), position, normal }
}
fn finite(p: Point) -> Result<(), String> {
    if !p.x.is_finite() || !p.y.is_finite() || p.x.abs() > 1e6 || p.y.abs() > 1e6 { Err("PPTX route coordinate limit".into()) } else { Ok(()) }
}
fn warn(w: &mut Vec<String>, s: &str) {
    if w.len() < 1000 && !w.iter().any(|x| x == s) {
        w.push(s.into());
    }
}
/// The same unrotated box is used by custom geometry, sites and the shape xfrm.
pub fn bounds(n: &EvaluatedShape, depth: usize) -> Result<Rect, String> {
    use kurbo::Shape as _;
    if depth > 128 {
        return Err("PPTX shape bounds depth limit".into());
    }
    let mut b = None;
    if !n.hidden_own {
        for s in n.sections.iter().filter(|s| !s.flags.no_show) {
            let r = s.path.bounding_box();
            b = Some(b.map_or(r, |old: Rect| old.union(r)));
        }
        if b.is_none() {
            b = Some(n.bounds);
        }
    } else {
        for child in &n.children {
            let r = bounds(child, depth + 1)?;
            b = Some(b.map_or(r, |old: Rect| old.union(r)));
        }
    }
    let b = b.unwrap_or(Rect::ZERO);
    finite(Point::new(b.x0, b.y0))?;
    finite(Point::new(b.x1, b.y1))?;
    Ok(b)
}
/// Remove only a curved excursion between collinear, same-direction straight
/// segments. This is the stored ArcTo jump in connector/line-jumps.vsdx.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linejumpcode-cell-page-layout-section
fn route(n: &EvaluatedShape) -> Result<(Connector, bool), String> {
    let sections: Vec<_> = n.sections.iter().filter(|s| s.stroked()).collect();
    let source = sections.first().copied().or_else(|| n.sections.first());
    let mut points: Vec<Point> = Vec::new();
    let mut curves = false;
    let mut jumps = false;
    let mut fitted = sections.len() > 1 || (n.paint.is_some() && n.sections.iter().any(|s| s.filled()));
    if let Some(section) = source {
        let els = section.path.elements();
        if els.len() > 65536 {
            return Err("PPTX route element limit".into());
        }
        let mut i = 0;
        while let Some(el) = els.get(i) {
            match *el {
                PathEl::MoveTo(p) | PathEl::LineTo(p) => {
                    if matches!(el, PathEl::MoveTo(_)) && !points.is_empty() {
                        let before = points.get(points.len().saturating_sub(2)).zip(points.last());
                        let after = match els.get(i + 1) {
                            Some(PathEl::LineTo(q)) => Some(*q),
                            _ => None,
                        };
                        let gap = before.zip(after).is_some_and(|((a, b), c)| {
                            let axis = p - *b;
                            let incoming = *b - *a;
                            let outgoing = c - p;
                            axis.hypot() > 1e-12
                                && incoming.cross(axis).abs() < 1e-9
                                && outgoing.cross(axis).abs() < 1e-9
                                && incoming.dot(axis) > 0.
                                && outgoing.dot(axis) > 0.
                        });
                        if gap {
                            jumps = true;
                        } else {
                            fitted = true;
                        }
                    }
                    points.push(p);
                }
                PathEl::CurveTo(_, _, _) | PathEl::QuadTo(_, _) => {
                    let begin = points.last().copied().ok_or("PPTX route starts with curve")?;
                    let mut j = i;
                    let mut ends = Vec::new();
                    while let Some(el) = els.get(j) {
                        let p = match *el {
                            PathEl::CurveTo(_, _, p) | PathEl::QuadTo(_, p) => p,
                            _ => break,
                        };
                        ends.push(p);
                        j += 1;
                    }
                    let end = ends.last().copied().ok_or("PPTX empty curve")?;
                    let prev = points.get(points.len().saturating_sub(2)).copied();
                    let after = match els.get(j) {
                        Some(PathEl::LineTo(p)) => Some(*p),
                        _ => None,
                    };
                    let jump = prev.zip(after).is_some_and(|(a, b)| {
                        let axis = end - begin;
                        let before = begin - a;
                        let after = b - end;
                        axis.hypot() > 1e-12
                            && before.cross(axis).abs() < 1e-9
                            && after.cross(axis).abs() < 1e-9
                            && before.dot(axis) > 0.
                            && after.dot(axis) > 0.
                    });
                    if jump {
                        jumps = true;
                        points.push(end);
                    } else {
                        curves = true;
                        fitted = true;
                        points.extend(ends);
                    }
                    i = j;
                    continue;
                }
                PathEl::ClosePath => {
                    fitted = true;
                    if let Some(p) = points.first().copied() {
                        points.push(p);
                    }
                }
            }
            i += 1;
        }
    }
    if points.len() < 2 {
        points = vec![Point::new(n.bounds.x0, n.bounds.y0), Point::new(n.bounds.x1, n.bounds.y1)];
        fitted = true;
    }
    for p in &points {
        finite(*p)?;
    }
    // Redundant collinear vertices do not change the intended route.
    let mut clean: Vec<Point> = Vec::new();
    for p in points {
        if clean.last().is_some_and(|q| (*q - p).hypot() < 1e-12) {
            continue;
        }
        while clean.len() >= 2 {
            let a = *clean.get(clean.len() - 2).ok_or("route point")?;
            let b = *clean.last().ok_or("route point")?;
            if (b - a).cross(p - b).abs() > 1e-12 || (b - a).dot(p - b) < 0. {
                break;
            }
            clean.pop();
        }
        clean.push(p);
    }
    if clean.len() == 1 {
        clean.push(*clean.first().ok_or("empty route")?);
    }
    let orthogonal = clean.windows(2).all(|p| p.first().zip(p.get(1)).is_some_and(|(a, b)| (a.x - b.x).abs() < 1e-8 || (a.y - b.y).abs() < 1e-8));
    if clean.len() > 6 || (!orthogonal && clean.len() > 2 && !curves) {
        fitted = true;
        clean = fit(&clean)?;
    }
    Ok((Connector { points: clean, curved: curves, fitted, jumps, begin: None, end: None }, jumps))
}
/// Preserve endpoint directions and select the three interior extremes in route
/// order. PowerPoint's preset solver owns its final drawing (Part F).
fn fit(points: &[Point]) -> Result<Vec<Point>, String> {
    let first = *points.first().ok_or("PPTX empty route")?;
    let last = *points.last().ok_or("PPTX empty route")?;
    let interior = points.get(1..points.len().saturating_sub(1)).ok_or("PPTX short route")?;
    let mut ranked: Vec<_> = interior.iter().enumerate().collect();
    ranked.sort_by(|(ia, a), (ib, b)| {
        let score = |p: &Point| {
            let v = last - first;
            let d = p.to_vec2() - first.to_vec2();
            if v.hypot2() > 1e-20 { d.cross(v).abs() / v.hypot() } else { d.hypot() }
        };
        score(b).total_cmp(&score(a)).then(ia.cmp(ib))
    });
    let mut selected: Vec<_> = ranked.into_iter().take(3).map(|(i, p)| (i, *p)).collect();
    selected.sort_by_key(|(i, _)| *i);
    let second = *points.get(1).ok_or("PPTX short route")?;
    let penultimate = *points.get(points.len().saturating_sub(2)).ok_or("PPTX short route")?;
    let vertical = (second.y - first.y).abs() > (second.x - first.x).abs();
    let local = |p: Point| if vertical { Point::new(p.y, p.x) } else { p };
    let global = local;
    let a = local(first);
    let b = local(last);
    let initial = local(second);
    let terminal = local(penultimate);
    let mut near = selected.first().map(|(_, p)| local(*p)).unwrap_or(initial);
    let mut far = selected.last().map(|(_, p)| local(*p)).unwrap_or(terminal);
    if (near.x - a.x) * (initial.x - a.x) <= 0. {
        near.x = initial.x;
    }
    if (b.x - far.x) * (b.x - terminal.x) <= 0. {
        far.x = terminal.x;
    }
    if (terminal.x - b.x).abs() < 1e-12 {
        far.x = b.x;
    }
    let middle = selected.get(1).or_else(|| selected.first()).map(|(_, p)| local(*p).y).unwrap_or((a.y + b.y) / 2.);
    Ok(vec![
        first,
        global(Point::new(near.x, a.y)),
        global(Point::new(near.x, middle)),
        global(Point::new(far.x, middle)),
        global(Point::new(far.x, b.y)),
        last,
    ])
}
