//! Evaluated Visio geometry rows in y-up shape-local inches.
use kurbo::{Affine, BezPath, Point, Rect, Vec2};
use std::f64::consts::{FRAC_PI_2, TAU};

const MAX_ROWS: usize = 100_000;
const MAX_ELEMENTS: usize = 1_000_000;
const MAX_COORD: f64 = 1e9;
const TOLERANCE: f64 = 1e-5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryError {
    Invalid,
    Limit,
}
impl std::fmt::Display for GeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "geometry {:?}", self)
    }
}
impl std::error::Error for GeometryError {}
type Result<T> = std::result::Result<T, GeometryError>;

/// Every row is resolved by the document host before conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowKind {
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/moveto-row-geometry-section
    MoveTo,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/lineto-row-geometry-section
    LineTo,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/arcto-row-geometry-section
    ArcTo,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/ellipticalarcto-row-geometry-section
    EllipticalArcTo,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/ellipse-row-geometry-section
    Ellipse,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/infiniteline-row-geometry-section
    InfiniteLine,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/polylineto-row-geometry-section
    PolylineTo,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/nurbsto-row-geometry-section
    NURBSTo,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/splinestart-row-geometry-section
    SplineStart,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/splineknot-row-geometry-section
    SplineKnot,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/relmoveto-row-geometry-section
    RelMoveTo,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/rellineto-row-geometry-section
    RelLineTo,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/relcubbezto-row-geometry-section
    RelCubBezTo,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/relquadbezto-row-geometry-section
    RelQuadBezTo,
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/relellipticalarcto-row-geometry-section
    RelEllipticalArcTo,
}
impl std::str::FromStr for RowKind {
    type Err = GeometryError;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "MoveTo" => Self::MoveTo,
            "LineTo" => Self::LineTo,
            "ArcTo" => Self::ArcTo,
            "EllipticalArcTo" => Self::EllipticalArcTo,
            "Ellipse" => Self::Ellipse,
            "InfiniteLine" => Self::InfiniteLine,
            "PolylineTo" => Self::PolylineTo,
            "NURBSTo" => Self::NURBSTo,
            "SplineStart" => Self::SplineStart,
            "SplineKnot" => Self::SplineKnot,
            "RelMoveTo" => Self::RelMoveTo,
            "RelLineTo" => Self::RelLineTo,
            "RelCubBezTo" => Self::RelCubBezTo,
            "RelQuadBezTo" => Self::RelQuadBezTo,
            "RelEllipticalArcTo" => Self::RelEllipticalArcTo,
            _ => return Err(GeometryError::Invalid),
        })
    }
}
#[derive(Debug, Clone)]
pub struct GeometryRow {
    pub kind: RowKind,
    pub x: f64,
    pub y: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    /// Evaluated POLYLINE or NURBS constructor, not an executable expression.
    pub data: String,
}
/// Section flags are retained independently: NoSnap never hides a painted path.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/nofill-cell-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/noline-cell-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/noshow-cell-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/nosnap-cell-geometry-section
#[derive(Debug, Default, Clone, Copy)]
pub struct GeometryFlags {
    pub no_fill: bool,
    pub no_line: bool,
    pub no_show: bool,
    pub no_snap: bool,
}
#[derive(Debug, Clone)]
pub struct GeometryPath {
    pub path: BezPath,
    pub flags: GeometryFlags,
}
impl GeometryPath {
    pub fn filled(&self) -> bool {
        !self.flags.no_show && !self.flags.no_fill
    }
    pub fn stroked(&self) -> bool {
        !self.flags.no_show && !self.flags.no_line
    }
    pub fn snappable(&self) -> bool {
        !self.flags.no_show && !self.flags.no_snap
    }
}
fn point(x: f64, y: f64) -> Result<Point> {
    if !x.is_finite() || !y.is_finite() {
        return Err(GeometryError::Invalid);
    }
    if x.abs() > MAX_COORD || y.abs() > MAX_COORD {
        return Err(GeometryError::Limit);
    }
    Ok(Point::new(x, y))
}
fn checked(p: Point) -> Result<Point> {
    point(p.x, p.y)
}
fn scalar(n: f64) -> Result<f64> {
    if n.is_finite() { Ok(n) } else { Err(GeometryError::Invalid) }
}
/// Parse only serialized finite numeric constructor arguments, with bounded input.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/polyline-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/nurbs-function
pub fn constructor(text: &str, name: &str) -> Result<Vec<f64>> {
    if text.len() > 65536 {
        return Err(GeometryError::Limit);
    }
    let body =
        text.trim().strip_prefix(name).and_then(|s| s.trim().strip_prefix('(')).and_then(|s| s.strip_suffix(')')).ok_or(GeometryError::Invalid)?;
    let mut values = Vec::new();
    for v in body.split(',') {
        if values.len() >= 8192 {
            return Err(GeometryError::Limit);
        }
        values.push(scalar(v.trim().parse().map_err(|_| GeometryError::Invalid)?)?);
    }
    Ok(values)
}
fn at(values: &[f64], i: usize) -> Result<f64> {
    values.get(i).copied().ok_or(GeometryError::Invalid)
}
fn scale_flag(flag: f64, size: f64) -> Result<f64> {
    if flag == 0. {
        Ok(size)
    } else if flag == 1. {
        Ok(1.)
    } else {
        Err(GeometryError::Invalid)
    }
}

/// Convert evaluated rows, clipping infinite lines to the supplied local page bounds.
/// A repeated endpoint closes a subpath so stroke joins match the closed outline.
pub fn geometry(rows: &[GeometryRow], width: f64, height: f64, clip: Rect, flags: GeometryFlags) -> Result<GeometryPath> {
    if rows.len() > MAX_ROWS {
        return Err(GeometryError::Limit);
    }
    point(width, height)?;
    checked(clip.origin())?;
    point(clip.x1, clip.y1)?;
    let mut path = BezPath::new();
    let mut current = None;
    let mut start = None;
    let mut i = 0;
    while let Some(row) = rows.get(i) {
        for n in [row.x, row.y, row.a, row.b, row.c, row.d] {
            scalar(n)?;
        }
        let relative =
            matches!(row.kind, RowKind::RelMoveTo | RowKind::RelLineTo | RowKind::RelCubBezTo | RowKind::RelQuadBezTo | RowKind::RelEllipticalArcTo);
        let (sx, sy) = if relative { (width, height) } else { (1., 1.) };
        let end = point(row.x * sx, row.y * sy)?;
        match row.kind {
            RowKind::MoveTo | RowKind::RelMoveTo => {
                path.move_to(end);
                start = Some(end);
            }
            RowKind::LineTo | RowKind::RelLineTo => {
                current.ok_or(GeometryError::Invalid)?;
                path.line_to(end);
            }
            RowKind::RelCubBezTo => {
                current.ok_or(GeometryError::Invalid)?;
                path.curve_to(point(row.a * width, row.b * height)?, point(row.c * width, row.d * height)?, end);
            }
            RowKind::RelQuadBezTo => {
                current.ok_or(GeometryError::Invalid)?;
                path.quad_to(point(row.a * width, row.b * height)?, end);
            }
            RowKind::ArcTo => {
                let from: Point = current.ok_or(GeometryError::Invalid)?;
                let chord = end - from;
                let length = chord.hypot();
                if row.a.abs() < 1e-12 || length < 1e-12 {
                    path.line_to(end);
                } else {
                    // Bow is a distance. Convert to dimensionless tan(sweep/4).
                    let bulge = 2. * row.a / length;
                    let center = checked(from + chord * 0.5 + Vec2::new(-chord.y, chord.x) * ((1. - bulge * bulge) / (4. * bulge)))?;
                    let radius = (from - center).hypot();
                    let angle = (from - center).atan2();
                    arc(&mut path, center, Vec2::new(radius, 0.), Vec2::new(0., radius), angle, 4. * bulge.atan())?;
                }
            }
            RowKind::EllipticalArcTo | RowKind::RelEllipticalArcTo => {
                let from = current.ok_or(GeometryError::Invalid)?;
                ellipse_arc(&mut path, from, point(row.a * sx, row.b * sy)?, end, row.c, row.d)?;
            }
            RowKind::Ellipse => {
                let u = point(row.a, row.b)? - end;
                let v = point(row.c, row.d)? - end;
                let first = checked(end + u)?;
                path.move_to(first);
                arc(&mut path, end, u, v, 0., TAU)?;
                path.close_path();
                current = Some(first);
                start = Some(first);
                i += 1;
                continue;
            }
            RowKind::InfiniteLine => {
                if let Some((a, b)) = clip_line(end, point(row.a, row.b)?, clip)? {
                    path.move_to(a);
                    path.line_to(b);
                }
                current = None;
                start = None;
                i += 1;
                continue;
            }
            RowKind::PolylineTo => {
                current.ok_or(GeometryError::Invalid)?;
                let data = constructor(&row.data, "POLYLINE")?;
                if data.len() < 4 || !data.len().is_multiple_of(2) {
                    return Err(GeometryError::Invalid);
                }
                let x = scale_flag(at(&data, 0)?, width)?;
                let y = scale_flag(at(&data, 1)?, height)?;
                for pair in data.get(2..).ok_or(GeometryError::Invalid)?.as_chunks::<2>().0 {
                    path.line_to(point(at(pair, 0)? * x, at(pair, 1)? * y)?);
                }
                path.line_to(end);
            }
            RowKind::NURBSTo => {
                let from = current.ok_or(GeometryError::Invalid)?;
                let data = constructor(&row.data, "NURBS")?;
                if data.len() < 8 || !data.len().is_multiple_of(4) {
                    return Err(GeometryError::Invalid);
                }
                let degree = degree(at(&data, 1)?)?;
                let x = scale_flag(at(&data, 2)?, width)?;
                let y = scale_flag(at(&data, 3)?, height)?;
                let mut control = vec![from];
                let mut weights = vec![row.d];
                let mut knots = vec![row.c];
                for p in data.get(4..).ok_or(GeometryError::Invalid)?.as_chunks::<4>().0 {
                    control.push(point(at(p, 0)? * x, at(p, 1)? * y)?);
                    knots.push(at(p, 2)?);
                    weights.push(at(p, 3)?);
                }
                control.push(end);
                weights.push(row.b);
                knots.push(row.a);
                knots.extend(std::iter::repeat_n(at(&data, 0)?, degree + 1));
                spline(&mut path, &control, &weights, &knots, degree)?;
            }
            RowKind::SplineStart => {
                let from = current.ok_or(GeometryError::Invalid)?;
                let degree = degree(row.d)?;
                let mut control = vec![from, end];
                let mut knots = vec![row.b, row.a];
                while let Some(next) = rows.get(i + 1).filter(|r| r.kind == RowKind::SplineKnot) {
                    control.push(point(next.x, next.y)?);
                    knots.push(scalar(next.a)?);
                    i += 1;
                }
                knots.extend(std::iter::repeat_n(row.c, degree + 1));
                spline(&mut path, &control, &vec![1.; control.len()], &knots, degree)?;
                current = control.last().copied();
                i += 1;
                continue;
            }
            RowKind::SplineKnot => return Err(GeometryError::Invalid),
        }
        if start == Some(end) && current.is_some() && !matches!(row.kind, RowKind::MoveTo | RowKind::RelMoveTo) {
            path.close_path();
        }
        current = Some(end);
        i += 1;
        if path.elements().len() > MAX_ELEMENTS {
            return Err(GeometryError::Limit);
        }
    }
    Ok(GeometryPath { path, flags })
}
fn degree(value: f64) -> Result<usize> {
    if value.fract() != 0. || !(1.0..=25.0).contains(&value) { Err(GeometryError::Invalid) } else { Ok(value as usize) }
}
fn arc(path: &mut BezPath, center: Point, u: Vec2, v: Vec2, start: f64, sweep: f64) -> Result<()> {
    if !sweep.is_finite() || sweep.abs() > TAU + 1e-9 {
        return Err(GeometryError::Invalid);
    }
    let pieces = (sweep.abs() / FRAC_PI_2).ceil().max(1.) as u32;
    for i in 0..pieces {
        let a = start + sweep * f64::from(i) / f64::from(pieces);
        let b = start + sweep * f64::from(i + 1) / f64::from(pieces);
        let k = 4. / 3. * ((b - a) / 4.).tan();
        let pa = center + u * a.cos() + v * a.sin();
        let pb = center + u * b.cos() + v * b.sin();
        let da = u * (-a.sin()) + v * a.cos();
        let db = u * (-b.sin()) + v * b.cos();
        path.curve_to(checked(pa + da * k)?, checked(pb - db * k)?, checked(pb)?);
    }
    Ok(())
}
fn ellipse_arc(path: &mut BezPath, start: Point, middle: Point, end: Point, angle: f64, ratio: f64) -> Result<()> {
    if !(0.0..=1000.).contains(&ratio) || ratio == 0. {
        return Err(GeometryError::Invalid);
    }
    let (s, c) = angle.sin_cos();
    let map = |p: Point| Vec2::new(c * p.x + s * p.y, (-s * p.x + c * p.y) * ratio);
    let a = map(start);
    let b = map(middle);
    let d = map(end);
    let ab = b - a;
    let ad = d - a;
    let det = 2. * ab.cross(ad);
    if det.abs() < 1e-14 {
        path.line_to(end);
        return Ok(());
    }
    let offset = Vec2::new((ab.hypot2() * ad.y - ad.hypot2() * ab.y) / det, (ab.x * ad.hypot2() - ad.x * ab.hypot2()) / det);
    let center = a + offset;
    let radius = offset.hypot();
    let first = (a - center).atan2();
    let mid = (b - center).atan2();
    let last = (d - center).atan2();
    let ccw = (last - first).rem_euclid(TAU);
    let sweep = if (mid - first).rem_euclid(TAU) <= ccw { ccw } else { ccw - TAU };
    let unmap = |p: Vec2| Vec2::new(c * p.x - s * p.y / ratio, s * p.x + c * p.y / ratio);
    let center = unmap(center);
    arc(path, point(center.x, center.y)?, unmap(Vec2::new(radius, 0.)), unmap(Vec2::new(0., radius)), first, sweep)
}
fn clip_line(a: Point, b: Point, rect: Rect) -> Result<Option<(Point, Point)>> {
    let d = b - a;
    if d.hypot2() < 1e-24 {
        return Ok(None);
    }
    let mut lo = f64::NEG_INFINITY;
    let mut hi = f64::INFINITY;
    for (p, v, min, max) in [(a.x, d.x, rect.x0, rect.x1), (a.y, d.y, rect.y0, rect.y1)] {
        if v.abs() < 1e-15 {
            if p < min || p > max {
                return Ok(None);
            }
        } else {
            let x = (min - p) / v;
            let y = (max - p) / v;
            lo = lo.max(x.min(y));
            hi = hi.min(x.max(y));
        }
    }
    if lo > hi { Ok(None) } else { Ok(Some((checked(a + d * lo)?, checked(a + d * hi)?))) }
}
/// Cox-de Boor evaluation in homogeneous coordinates, capped to degree 25.
fn spline(path: &mut BezPath, control: &[Point], weights: &[f64], knots: &[f64], degree: usize) -> Result<()> {
    if control.len() > 8192 || degree == 0 || degree > 25 {
        return Err(GeometryError::Limit);
    }
    if control.len() <= degree || weights.len() != control.len() || knots.len() != control.len() + degree + 1 {
        return Err(GeometryError::Invalid);
    }
    if weights.iter().any(|w| !w.is_finite() || *w <= 0.) || knots.iter().any(|k| !k.is_finite()) {
        return Err(GeometryError::Invalid);
    }
    // A/E cells serialize repeated NURBS knots with different decimal precision.
    // NETSME_U Lock: 0.7138452034029296 in A3, 0.71384520340293 in E3.
    // Absorb floating representation noise only; genuinely descending knots remain invalid.
    let mut adjusted = knots.to_vec();
    let mut previous = *adjusted.first().ok_or(GeometryError::Invalid)?;
    for knot in &mut adjusted {
        if *knot < previous {
            if previous - *knot > 16. * f64::EPSILON * previous.abs().max(knot.abs()).max(1.) {
                return Err(GeometryError::Invalid);
            }
            *knot = previous;
        }
        previous = *knot;
    }
    let knots = adjusted.as_slice();
    let lo = at(knots, degree)?;
    let hi = at(knots, control.len())?;
    if hi <= lo {
        return Err(GeometryError::Invalid);
    }
    let sample = |t: f64| -> Result<Point> {
        let span = if t >= hi { control.len() - 1 } else { knots.partition_point(|k| *k <= t).saturating_sub(1).clamp(degree, control.len() - 1) };
        let mut work = Vec::with_capacity(degree + 1);
        for j in 0..=degree {
            let i = span - degree + j;
            let p = control.get(i).ok_or(GeometryError::Invalid)?;
            let w = at(weights, i)?;
            work.push((p.x * w, p.y * w, w));
        }
        for r in 1..=degree {
            for j in (r..=degree).rev() {
                let i = span - degree + j;
                let a = at(knots, i)?;
                let b = at(knots, i + degree + 1 - r)?;
                let alpha = if b == a { 0. } else { (t - a) / (b - a) };
                let left = *work.get(j - 1).ok_or(GeometryError::Invalid)?;
                let right = *work.get(j).ok_or(GeometryError::Invalid)?;
                *work.get_mut(j).ok_or(GeometryError::Invalid)? =
                    (left.0 * (1. - alpha) + right.0 * alpha, left.1 * (1. - alpha) + right.1 * alpha, left.2 * (1. - alpha) + right.2 * alpha);
            }
        }
        let (x, y, w) = *work.last().ok_or(GeometryError::Invalid)?;
        if w.abs() < 1e-300 {
            return Err(GeometryError::Invalid);
        }
        point(x / w, y / w)
    };
    let mut budget = 65536usize;
    for interval in knots.windows(2) {
        let a = at(interval, 0)?.max(lo);
        let b = at(interval, 1)?.min(hi);
        if b <= a {
            continue;
        }
        let mut stack = vec![(a, sample(a)?, b, sample(b)?, 0u32)];
        while let Some((a, pa, b, pb, depth)) = stack.pop() {
            budget = budget.checked_sub(1).ok_or(GeometryError::Limit)?;
            let mid = (a + b) * 0.5;
            let pm = sample(mid)?;
            let q1 = sample(a + (b - a) * 0.25)?;
            let q3 = sample(a + (b - a) * 0.75)?;
            let error = pm.distance(pa.lerp(pb, 0.5)).max(q1.distance(pa.lerp(pb, 0.25))).max(q3.distance(pa.lerp(pb, 0.75)));
            if error <= TOLERANCE {
                path.line_to(pb);
            } else if depth >= 20 {
                return Err(GeometryError::Limit);
            } else {
                stack.push((mid, pm, b, pb, depth + 1));
                stack.push((a, pa, mid, pm, depth + 1));
            }
            if path.elements().len() > MAX_ELEMENTS {
                return Err(GeometryError::Limit);
            }
        }
    }
    Ok(())
}
/// Compose local pin translation, local flips and rotation in y-up coordinates.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shape-transform-section
pub fn shape_transform(pin: Point, local_pin: Point, angle: f64, flip_x: bool, flip_y: bool) -> Result<Affine> {
    checked(pin)?;
    checked(local_pin)?;
    scalar(angle)?;
    Ok(Affine::translate(pin.to_vec2())
        * Affine::rotate(angle)
        * Affine::scale_non_uniform(if flip_x { -1. } else { 1. }, if flip_y { -1. } else { 1. })
        * Affine::translate(-local_pin.to_vec2()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::{PathEl, Shape};
    fn row(kind: RowKind, x: f64, y: f64) -> GeometryRow {
        GeometryRow { kind, x, y, a: 0., b: 0., c: 0., d: 0., data: String::new() }
    }
    fn build(rows: &[GeometryRow]) -> Result<GeometryPath> {
        geometry(rows, 4., 2., Rect::new(-10., -10., 10., 10.), GeometryFlags::default())
    }
    #[test]
    fn relative_beziers_scale_controls_and_close_subpaths() {
        let mut cubic = row(RowKind::RelCubBezTo, 1., 1.);
        cubic.a = 0.;
        cubic.b = 1.;
        cubic.c = 1.;
        cubic.d = 0.;
        let mut quad = row(RowKind::RelQuadBezTo, 0., 0.);
        quad.a = 0.5;
        quad.b = 1.;
        let p = build(&[row(RowKind::RelMoveTo, 0., 0.), cubic, quad]).unwrap();
        assert_eq!(
            p.path.elements(),
            &[
                PathEl::MoveTo((0., 0.).into()),
                PathEl::CurveTo((0., 2.).into(), (4., 0.).into(), (4., 2.).into()),
                PathEl::QuadTo((2., 2.).into(), (0., 0.).into()),
                PathEl::ClosePath
            ]
        );
    }
    #[test]
    fn bow_distance_and_ellipse_geometry_are_in_inches() {
        let mut a = row(RowKind::ArcTo, 2., 0.);
        a.a = 1.;
        let p = build(&[row(RowKind::MoveTo, 0., 0.), a]).unwrap();
        let b = p.path.bounding_box();
        assert!((b.y0 + 1.).abs() < 1e-10);
        assert!((b.x1 - 2.).abs() < 1e-10);
        let mut e = row(RowKind::Ellipse, 2., 1.);
        e.a = 4.;
        e.b = 1.;
        e.c = 2.;
        e.d = 2.;
        let b = build(&[e]).unwrap().path.bounding_box();
        assert!(b.origin().distance((0., 0.).into()) < 1e-12);
        assert!((b.x1 - 4.).abs() < 1e-12 && (b.y1 - 2.).abs() < 1e-12);
    }
    #[test]
    fn ellipse_arc_passes_through_control_point_in_both_directions() {
        for sign in [-1., 1.] {
            let mut a = row(RowKind::EllipticalArcTo, -2., 0.);
            a.a = 0.;
            a.b = sign;
            a.d = 2.;
            let b = build(&[row(RowKind::MoveTo, 2., 0.), a]).unwrap().path.bounding_box();
            assert!((if sign < 0. { b.y0 + 1. } else { b.y1 - 1. }).abs() < 1e-9);
        }
    }
    #[test]
    fn polyline_reference_example_and_section_flags() {
        let mut r = row(RowKind::PolylineTo, 0., 0.);
        r.data = "POLYLINE(0,0,0,0,0,1,1,1,1,0,0,0)".into();
        let p = build(&[row(RowKind::MoveTo, 0., 0.), r]).unwrap();
        assert_eq!(p.path.bounding_box(), Rect::new(0., 0., 4., 2.));
        for bits in 0..16 {
            let p = geometry(
                &[],
                1.,
                1.,
                Rect::ZERO,
                GeometryFlags { no_fill: bits & 1 != 0, no_line: bits & 2 != 0, no_show: bits & 4 != 0, no_snap: bits & 8 != 0 },
            )
            .unwrap();
            assert_eq!(p.filled(), bits & 5 == 0);
            assert_eq!(p.stroked(), bits & 6 == 0);
            assert_eq!(p.snappable(), bits & 12 == 0);
        }
    }
    #[test]
    fn cubic_nurbs_matches_bezier_midpoint_and_spline_rows() {
        let mut n = row(RowKind::NURBSTo, 1., 0.);
        n.b = 1.;
        n.d = 1.;
        n.data = "NURBS(1,3,1,1,0,1,0,1,1,1,0,1)".into();
        let p = build(&[row(RowKind::MoveTo, 0., 0.), n]).unwrap();
        assert!((p.path.bounding_box().y1 - 0.75).abs() < 1e-8);
        let mut s = row(RowKind::SplineStart, 0., 1.);
        s.c = 1.;
        s.d = 3.;
        let k = row(RowKind::SplineKnot, 1., 1.);
        let last = row(RowKind::SplineKnot, 1., 0.);
        let q = build(&[row(RowKind::MoveTo, 0., 0.), s, k, last]).unwrap();
        assert_eq!(p.path, q.path);
    }
    #[test]
    fn repeated_knots_accept_serialization_noise_but_reject_descending_values() {
        let points = [Point::new(0., 0.), Point::new(1., 1.), Point::new(2., 0.), Point::new(3., 1.)];
        let mut path = BezPath::new();
        path.move_to(points[0]);
        let knots = [0., 0., 0.71384520340293, 0.7138452034029296, 1., 1., 1.];
        assert!(spline(&mut path, &points, &[1.; 4], &knots, 2).is_ok());
        let knots = [0., 0., 0.71384520340293, 0.71384, 1., 1., 1.];
        assert!(spline(&mut path, &points, &[1.; 4], &knots, 2).is_err());
    }
    #[test]
    fn infinite_line_clips_and_group_composition_stays_y_up() {
        let mut line = row(RowKind::InfiniteLine, 0., 0.);
        line.a = 2.;
        line.b = 1.;
        let p = build(&[line]).unwrap();
        assert_eq!(p.path.elements(), &[PathEl::MoveTo((-10., -5.).into()), PathEl::LineTo((10., 5.).into())]);
        let parent = shape_transform((10., 20.).into(), (1., 1.).into(), FRAC_PI_2, false, false).unwrap();
        let child = shape_transform((4., 5.).into(), (1., 1.).into(), 0., true, false).unwrap();
        assert!((parent * child * Point::new(1., 1.)).distance((6., 23.).into()) < 1e-12);
    }
    #[test]
    fn hostile_constructors_and_numeric_rows_return_errors() {
        for s in ["", "NURBS()", "POLYLINE(0,0,NaN,1)", "POLYLINE(0,0,1/0,1)", "POLYLINE(0,0,1,1))"] {
            assert!(constructor(s, "POLYLINE").is_err());
        }
        assert!(constructor(&"x".repeat(65537), "POLYLINE").is_err());
        assert!(build(&[row(RowKind::MoveTo, f64::NAN, 0.)]).is_err());
        assert!(build(&[row(RowKind::LineTo, 1., 1.)]).is_err());
        assert!(build(&[row(RowKind::SplineKnot, 1., 1.)]).is_err());
        let mut n = row(RowKind::NURBSTo, 1., 0.);
        n.data = "NURBS(1,999999999999,1,1,0,1,0,1)".into();
        assert!(build(&[row(RowKind::MoveTo, 0., 0.), n]).is_err());
        assert!(shape_transform((0., 0.).into(), (0., 0.).into(), f64::INFINITY, false, false).is_err());
    }
}
