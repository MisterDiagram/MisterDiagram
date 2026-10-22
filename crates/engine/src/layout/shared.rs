//! Original straight-segment overlap check, independent of curve subdivision.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/routestyle-cell-page-layout-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/shaperoutestyle-cell-shape-layout-section
//! https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
use super::*;
use misterdiagram_geom::Rect;
const EPS: f64 = 1e-8;
const OVERLAP: f64 = 1e-6;
struct Line {
    a: Point,
    b: Point,
    start: Point,
    end: Point,
    ordinal: usize,
    glue: [Option<(u32, Point)>; 2],
}
struct Route {
    id: u32,
    bus: bool,
    lines: Vec<Line>,
}
pub(super) struct Report {
    page: usize,
    connectors: usize,
    raw: usize,
    style: usize,
    glue: usize,
    counted: usize,
    pub violations: Vec<Value>,
}
impl Report {
    pub fn stats(&self) -> Value {
        json!({"raw":self.raw,"style_exempt":self.style,"same_glue_exempt":self.glue,"violating":self.counted})
    }
    pub fn value(self) -> Value {
        json!({"page":self.page,"shared_only":true,"connectors":self.connectors,"shared_segments":self.stats(),"count":self.violations.len(),"violations":self.violations})
    }
}
// Native Flowchart/Tree enums plus the separately witnessed office SimpleNS bus.
// Other Simple and organization-chart styles are not inferred exemptions.
fn bus_style(style: f64) -> bool {
    [5., 6., 7., 8., 12., 13., 14., 15., 17.].contains(&style)
}
fn overlap(a: &Line, b: &Line) -> Option<(Point, Point)> {
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
    (hi - lo > OVERLAP).then_some((a.a + unit * lo, a.a + unit * hi))
}
fn straight_geometry(segment: kurbo::PathSeg) -> Option<(Point, Point)> {
    let points = match segment {
        kurbo::PathSeg::Line(l) => [l.p0, l.p1, l.p1, l.p1],
        kurbo::PathSeg::Quad(q) => [q.p0, q.p1, q.p2, q.p2],
        kurbo::PathSeg::Cubic(c) => [c.p0, c.p1, c.p2, c.p3],
    };
    let origin = *points.first()?;
    let other = *points.iter().find(|p| p.distance(origin) > EPS)?;
    let unit = (other - origin) / (other - origin).hypot();
    if points.iter().any(|p| (*p - origin).cross(unit).abs() > EPS) {
        return None;
    }
    // Collinear Beziers may backtrack. Their continuous image is the interval
    // between true curve extrema, not merely their two endpoint coordinates.
    let bbox = kurbo::ParamCurveExtrema::bounding_box(&segment);
    let lo = Point::new(if unit.x >= 0. { bbox.x0 } else { bbox.x1 }, if unit.y >= 0. { bbox.y0 } else { bbox.y1 });
    let hi = Point::new(if unit.x >= 0. { bbox.x1 } else { bbox.x0 }, if unit.y >= 0. { bbox.y1 } else { bbox.y0 });
    Some((origin + unit * (lo - origin).dot(unit), origin + unit * (hi - origin).dot(unit)))
}
fn same_glue(a: &Line, b: &Line) -> bool {
    a.glue.iter().flatten().any(|(target, point)| b.glue.iter().flatten().any(|(other, end)| target == other && point.distance(*end) <= EPS))
}
fn compare(page: usize, routes: &[Route], segments: usize) -> Result<Report> {
    if segments.saturating_mul(segments) > 2_000_000 {
        return Err(operation("layout shared-segment comparison limit"));
    }
    let mut report = Report { page, connectors: routes.len(), raw: 0, style: 0, glue: 0, counted: 0, violations: Vec::new() };
    for (index, a) in routes.iter().enumerate() {
        for b in routes.iter().skip(index + 1) {
            let mut bad = None;
            for x in &a.lines {
                for y in &b.lines {
                    if let Some((lo, hi)) = overlap(x, y) {
                        // The squared segment bound above caps every counter at two million.
                        report.raw += 1;
                        if a.bus && b.bus {
                            report.style += 1;
                        } else if same_glue(x, y) {
                            report.glue += 1;
                        } else {
                            report.counted += 1;
                            bad.get_or_insert((lo, hi));
                        }
                    }
                }
            }
            if let Some((lo, hi)) = bad {
                if report.violations.len() >= 10000 {
                    return Err(operation("layout checker violation limit"));
                }
                report.violations.push(json!({"rule":"h","shapes":[],"connectors":[a.id,b.id],"detail":format!("collinear segments overlap from ({:.9},{:.9}) to ({:.9},{:.9})",lo.x,lo.y,hi.x,hi.y)}));
            }
        }
    }
    Ok(report)
}
/// Check straight path segments, excluding genuine curves rather than treating
/// their flattening chords as straight connector trunks. Geometry subpath breaks
/// are retained by the path iterator. The original full-check limits stay fixed.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
pub(super) fn inspect(s: &Session, p: &Value) -> Result<Report> {
    let index = page_index(s, p)?;
    let doc = s.document();
    let page = doc.pages.get(index).ok_or_else(|| operation("page missing"))?;
    if page.shapes.len() > 10000 || page.connects.len() > 20000 {
        return Err(operation("layout checker shape limit"));
    }
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(operation)?;
    let mut e = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&misterdiagram_text::Host);
    let scopes = e.scopes().filter(|scope| matches!(scope,Scope::PageShape{page:id,..} if *id==page.id)).take(10001).collect::<Vec<_>>();
    if scopes.len() > 10000 {
        return Err(operation("layout checker shape limit"));
    }
    let targets = scopes
        .iter()
        .filter_map(|scope| match scope {
            Scope::PageShape { shape, .. } => Some(*shape),
            _ => None,
        })
        .collect::<std::collections::BTreeSet<_>>();
    let mut routes = Vec::new();
    let mut segment_count = 0usize;
    let mut path_count = 0usize;
    for scope in scopes {
        let Scope::PageShape { shape: id, .. } = scope else { continue };
        if e.cell(scope, "BeginX").is_none() {
            continue;
        }
        let parent = e.parent_to_page(scope).map_err(operation)?;
        let mut ends = [None, None];
        for (i, from, y) in [(0, "BeginX", "BeginY"), (1, "EndX", "EndY")] {
            let mut links = page.connects.iter().filter(|c| c.from_sheet == id && c.from_cell.as_deref() == Some(from));
            if let Some(link) = links.next()
                && links.next().is_none()
                && targets.contains(&link.to_sheet)
            {
                let point = parent * Point::new(number(&mut e, scope, from)?, number(&mut e, scope, y)?);
                *ends.get_mut(i).ok_or_else(|| operation("layout endpoint index"))? = Some((link.to_sheet, point));
            }
        }
        let style = if e.cell(scope, "ShapeRouteStyle").is_some() { number(&mut e, scope, "ShapeRouteStyle")? } else { 0. };
        let style = if style == 0. && e.cell(Scope::Page(page.id), "RouteStyle").is_some() {
            number(&mut e, Scope::Page(page.id), "RouteStyle")?
        } else {
            style
        };
        let transform = e.shape_to_page(scope).map_err(operation)?;
        let mut lines = Vec::new();
        let mut ordinal = 0usize;
        for path in e.geometry_paths(scope, Rect::new(-1e6, -1e6, 1e6, 1e6)).map_err(operation)? {
            for segment in path.path.segments() {
                path_count = path_count.checked_add(1).ok_or_else(|| operation("layout checker segment limit"))?;
                if path_count > 100000 {
                    return Err(operation("layout checker segment limit"));
                }
                let segment = transform * segment;
                let degenerate = match segment {
                    kurbo::PathSeg::Line(l) => l.p0.distance(l.p1) <= EPS,
                    kurbo::PathSeg::Quad(q) => [q.p1, q.p2].iter().all(|p| p.distance(q.p0) <= EPS),
                    kurbo::PathSeg::Cubic(c) => [c.p1, c.p2, c.p3].iter().all(|p| p.distance(c.p0) <= EPS),
                };
                if degenerate {
                    continue;
                }
                if let Some((a, b)) = straight_geometry(segment) {
                    use kurbo::ParamCurve;
                    if [a, b].iter().any(|p| !p.x.is_finite() || !p.y.is_finite() || p.x.abs() > 1e6 || p.y.abs() > 1e6) {
                        return Err(operation("layout route coordinate limit"));
                    }
                    if a.distance(b) > EPS {
                        lines.push(Line { a, b, start: segment.eval(0.), end: segment.eval(1.), ordinal, glue: [None, None] });
                        segment_count = segment_count.checked_add(1).ok_or_else(|| operation("layout checker segment limit"))?;
                    }
                }
                ordinal = ordinal.checked_add(1).ok_or_else(|| operation("layout checker segment limit"))?;
            }
        }
        if let Some(first) = lines.first_mut()
            && first.ordinal == 0
        {
            first.glue[0] = ends[0].filter(|(_, p)| p.distance(first.start) <= EPS);
        }
        if let Some(last) = lines.last_mut()
            && last.ordinal.checked_add(1) == Some(ordinal)
        {
            last.glue[1] = ends[1].filter(|(_, p)| p.distance(last.end) <= EPS);
        }
        routes.push(Route { id, bus: bus_style(style), lines });
    }
    compare(index + 1, &routes, segment_count)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn route(id: u32, a: (f64, f64), b: (f64, f64), style: f64) -> Route {
        Route {
            id,
            bus: bus_style(style),
            lines: vec![Line {
                a: Point::new(a.0, a.1),
                b: Point::new(b.0, b.1),
                start: Point::new(a.0, a.1),
                end: Point::new(b.0, b.1),
                ordinal: 0,
                glue: [None, None],
            }],
        }
    }
    #[test]
    fn overlap_rule_counts_reverse_diagonal_and_strict_tolerance() {
        for (len, count) in [(0.0000005, 0), (0.000001, 0), (0.000002, 1)] {
            let routes = [route(1, (0., 0.), (len, 0.), 1.), route(2, (-1., 0.), (2., 0.), 1.)];
            let report = compare(1, &routes, 2).unwrap();
            assert_eq!(report.violations.len(), count);
        }
        let routes = [route(1, (0., 0.), (2., 2.), 1.), route(2, (3., 3.), (1., 1.), 1.)];
        assert_eq!(compare(1, &routes, 2).unwrap().counted, 1);
        assert!(compare(1, &[], 2000).is_err());
    }
    #[test]
    fn collinear_bezier_encodings_include_backtracking_extrema() {
        let segment = kurbo::PathSeg::Cubic(kurbo::CubicBez::new((0., 0.), (1., 0.), (2., 0.), (3., 0.)));
        let (a, b) = straight_geometry(segment).unwrap();
        assert_eq!((a, b), (Point::new(0., 0.), Point::new(3., 0.)));
        let own = route(1, (a.x, a.y), (b.x, b.y), 1.);
        assert_eq!(compare(1, &[own, route(2, (1., 0.), (2., 0.), 1.)], 2).unwrap().counted, 1);
        let segment = kurbo::PathSeg::Quad(kurbo::QuadBez::new((0., 0.), (2., 0.), (0., 0.)));
        let (a, b) = straight_geometry(segment).unwrap();
        assert_eq!((a, b), (Point::new(0., 0.), Point::new(1., 0.)));
        assert!(straight_geometry(kurbo::PathSeg::Quad(kurbo::QuadBez::new((0., 0.), (2., 1.), (3., 0.)))).is_none());
    }
    #[test]
    fn dangling_targets_do_not_make_glue_exemptions() {
        let mut session = Session::new();
        for end in [2., 3.] {
            session.execute("shape.draw.line", &json!({"x1":0,"y1":0,"x2":end,"y2":0})).unwrap();
        }
        // Hostile in-memory input bypasses the writer, which correctly rejects
        // missing Connect targets before serializing a document.
        let doc = std::sync::Arc::make_mut(&mut session.document);
        for id in [1, 2] {
            doc.pages[0].connects.push(misterdiagram_model::Connect {
                from_sheet: id,
                from_cell: Some("BeginX".into()),
                to_sheet: 999999,
                ..Default::default()
            });
        }
        let report = inspect(&session, &json!({})).unwrap();
        assert_eq!((report.raw, report.glue, report.counted), (1, 0, 1));
    }
    #[test]
    fn exemptions_require_both_styles_or_the_same_terminal_glue() {
        for style in [5., 6., 7., 8., 12., 13., 14., 15., 17.] {
            let routes = [route(1, (0., 0.), (2., 0.), style), route(2, (1., 0.), (3., 0.), style)];
            let report = compare(1, &routes, 2).unwrap();
            assert_eq!((report.raw, report.style, report.counted), (1, 1, 0));
            let routes = [route(1, (0., 0.), (2., 0.), style), route(2, (1., 0.), (3., 0.), 1.)];
            assert_eq!(compare(1, &routes, 2).unwrap().counted, 1);
        }
        for style in [0., 1., 2., 3., 4., 9., 10., 11., 16., 18., 19., 20., 21., 22.] {
            let routes = [route(1, (0., 0.), (2., 0.), style), route(2, (1., 0.), (3., 0.), style)];
            assert_eq!(compare(1, &routes, 2).unwrap().counted, 1);
        }
        let mut routes = [route(1, (0., 0.), (2., 0.), 1.), route(2, (0., 0.), (3., 0.), 1.)];
        for r in &mut routes {
            r.lines[0].glue[0] = Some((10, Point::new(0., 0.)));
        }
        assert_eq!(compare(1, &routes, 2).unwrap().glue, 1);
        routes[1].lines[0].glue[0] = Some((11, Point::new(0., 0.)));
        assert_eq!(compare(1, &routes, 2).unwrap().counted, 1);
    }
}
