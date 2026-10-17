//! Procedural ShapeSheet paints and line decorations, in inches.
use kurbo::{Cap, Circle, Join, ParamCurve, ParamCurveArclen, PathEl, RoundedRect, Shape, Stroke};
use misterdiagram_color::Rgba;
use misterdiagram_geom::{Affine, BezPath, Point, Rect};

#[derive(Clone, Debug)]
pub struct Fill {
    pub pattern: u32,
    pub foreground: Rgba,
    pub background: Rgba,
}
impl Fill {
    /// Procedural patterns use an eight-pixel repeat at 96 dpi. Gradient positions
    /// are normalized shape coordinates with Y increasing downward.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/fillpattern-cell-fill-format-section
    pub fn sample(&self, x: f64, y: f64, px: i32, py: i32) -> Rgba {
        if self.pattern == 0 || !x.is_finite() || !y.is_finite() {
            return Rgba::TRANSPARENT;
        }
        let a = px.rem_euclid(8);
        let b = py.rem_euclid(8);
        let x = x.clamp(0., 1.);
        let y = y.clamp(0., 1.);
        // Exact pixel coverage of a unit-width 45-degree hatch line.
        if matches!(self.pattern, 2 | 5) {
            let d = if self.pattern == 2 { (a + b - 7).rem_euclid(8) } else { (a - b).rem_euclid(8) };
            let coverage = match d {
                0 => std::f64::consts::SQRT_2 - 0.5,
                1 | 7 => 0.25,
                _ => 0.,
            };
            let mix = |a: u8, b: u8| (f64::from(a) * coverage + f64::from(b) * (1. - coverage)).round() as u8;
            return Rgba::rgba(
                mix(self.foreground.r, self.background.r),
                mix(self.foreground.g, self.background.g),
                mix(self.foreground.b, self.background.b),
                mix(self.foreground.a, self.background.a),
            );
        }
        let fg = match self.pattern {
            1 => true,
            2 => (a + b) % 8 == 7,
            3 => a == 0 || b == 0,
            4 => (a - b).rem_euclid(8) == 0 || (a + b) % 8 == 6,
            5 => a == b,
            6 => b == 0,
            7 => a == 0,
            8 => (a + 2 * b) % 4 != 0,
            9 => (a + b) % 2 == 1,
            10 => (a + 2 * b) % 4 == 0,
            11 => b % 2 == 0 && (a + b) % 4 == 0,
            12 => b % 2 == 0 && (a + 2 * b) % 8 == 3,
            13 => b % 4 < 2,
            14 => a % 4 < 2,
            15 => (a - b).rem_euclid(4) < 2,
            16 => (a + b) % 4 >= 2,
            17 => matches!(b % 4, 0 | 3) == matches!(a % 4, 0 | 3),
            18 => b % 2 == 0 || matches!(b % 4, 0 | 3) == matches!(a % 4, 0 | 3),
            19 => b % 4 == 0,
            20 => a % 4 == 0,
            21 => (a - b).rem_euclid(4) == 0,
            22 => (a + b) % 4 == 3,
            23 => a % 4 == 0 || b % 4 == 0,
            24 => {
                if b % 2 == 0 {
                    (a + b) % 4 != 3
                } else {
                    a % 2 == 1
                }
            }
            _ => false,
        };
        if self.pattern <= 24 {
            return if fg { self.foreground } else { self.background };
        }
        let distance = match self.pattern {
            25 => x,
            26 => (2. * x - 1.).abs(),
            27 => 1. - x,
            28 => y,
            29 => (2. * y - 1.).abs(),
            30 => 1. - y,
            31 => x.max(y),
            32 => (1. - x).max(y),
            33 => x.max(1. - y),
            34 => (1. - x).max(1. - y),
            35 => (2. * x - 1.).abs().max((2. * y - 1.).abs()),
            36 => x.hypot(y),
            37 => (1. - x).hypot(y),
            38 => x.hypot(1. - y),
            39 => (1. - x).hypot(1. - y),
            40 => (x - 0.5).hypot(y - 0.5) * 2.,
            _ => 0.,
        }
        .clamp(0., 1.);
        // The one-way legacy gradients use a normalized Gaussian blend and
        // gamma-2.2 colour interpolation, observed in fill/pattern-25 and -35.
        let gamma = matches!(self.pattern, 25 | 27 | 28 | 30..=35);
        let distance = if gamma {
            let cdf = |x: f64| {
                let z = x / std::f64::consts::SQRT_2;
                let t = 1. / (1. + 0.3275911 * z.abs());
                let erf = 1. - (((((1.061405429 * t - 1.453152027) * t) + 1.421413741) * t - 0.284496736) * t + 0.254829592) * t * (-z * z).exp();
                (1. + erf.copysign(z)) * 0.5
            };
            ((cdf(4. * distance - 2.) - cdf(-2.)) / (cdf(2.) - cdf(-2.))).clamp(0., 1.)
        } else {
            distance
        };
        let mix = |a: u8, b: u8| {
            if gamma {
                (((f64::from(a) / 255.).powf(2.2) * (1. - distance) + (f64::from(b) / 255.).powf(2.2) * distance).powf(1. / 2.2) * 255.).round() as u8
            } else {
                (f64::from(a) * (1. - distance) + f64::from(b) * distance).round() as u8
            }
        };
        Rgba::rgba(
            mix(self.foreground.r, self.background.r),
            mix(self.foreground.g, self.background.g),
            mix(self.foreground.b, self.background.b),
            mix(self.foreground.a, self.background.a),
        )
    }
}
/// Gradient stops carry independent transparency, sorted and bounded by the host.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradient-section
#[derive(Clone, Debug)]
pub struct Gradient {
    pub stops: Vec<(f64, Rgba)>,
    pub radial: bool,
    pub angle: f64,
}
impl Gradient {
    pub fn sample(&self, x: f64, y: f64) -> Rgba {
        if self.stops.is_empty()
            || self.stops.len() > 4096
            || self.stops.iter().any(|(position, _)| !position.is_finite())
            || !x.is_finite()
            || !y.is_finite()
            || !self.angle.is_finite()
        {
            return Rgba::TRANSPARENT;
        }
        let t = if self.radial {
            (x - 0.5).hypot(y - 0.5) * 2.
        } else {
            let (sin, cos) = self.angle.sin_cos();
            0.5 + ((x - 0.5) * cos + (0.5 - y) * sin) / (cos.abs() + sin.abs()).max(1e-12)
        }
        .clamp(0., 1.);
        self.at(t)
    }
    pub fn at(&self, t: f64) -> Rgba {
        if !t.is_finite() {
            return Rgba::TRANSPARENT;
        }
        let t = t.clamp(0., 1.);
        let Some(&(first, color)) = self.stops.first() else {
            return Rgba::TRANSPARENT;
        };
        if t <= first {
            return color;
        }
        for pair in self.stops.windows(2) {
            let Some(&(a, ca)) = pair.first() else {
                continue;
            };
            let Some(&(b, cb)) = pair.get(1) else {
                continue;
            };
            if t <= b && b > a {
                let f = (t - a) / (b - a);
                let mix = |a: u8, b: u8| (f64::from(a) * (1. - f) + f64::from(b) * f).round() as u8;
                return Rgba::rgba(mix(ca.r, cb.r), mix(ca.g, cb.g), mix(ca.b, cb.b), mix(ca.a, cb.a));
            }
        }
        self.stops.last().map(|(_, c)| *c).unwrap_or(Rgba::TRANSPARENT)
    }
}
/// A uniform line weight controls dash lengths. The three seven-pattern families
/// differ in their short/long segment lengths, measured in the synthetic line group.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/linepattern-cell-line-format-section
pub fn dash_pattern(pattern: u32, weight: f64) -> Vec<f64> {
    if !weight.is_finite() || weight <= 0. {
        return vec![];
    }
    if pattern == 23 {
        return vec![weight, 2. * weight];
    }
    let (base, short, long, gap) = match pattern {
        2..=8 => (2, 7., 19., 5.),
        9..=15 => (9, 3., 9., 3.),
        16..=22 => (16, 15., 39., 9.),
        _ => return vec![],
    };
    let values = match pattern - base {
        0 => vec![short, gap],
        1 => vec![0., gap],
        2 => vec![short, gap, 0., gap],
        3 => vec![short, gap, 0., gap, 0., gap],
        4 => vec![short, gap, short, gap, 0., gap],
        5 => vec![long, gap, short, gap],
        _ => vec![long, gap, short, gap, short, gap],
    };
    values.into_iter().map(|v| v * weight).collect()
}
#[derive(Clone, Debug)]
pub struct Line {
    pub color: Rgba,
    pub weight: f64,
    pub pattern: u32,
    pub cap: u32,
    pub join: u32,
    pub rounding: f64,
    pub compound: u32,
    pub begin_arrow: u32,
    pub end_arrow: u32,
    pub begin_size: u32,
    pub end_size: u32,
}
impl Line {
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/linecap-cell-line-format-section
    pub fn stroke(&self) -> Stroke {
        Stroke::new(self.weight)
            .with_caps(match self.cap {
                0 => Cap::Round,
                2 => Cap::Square,
                _ => Cap::Butt,
            })
            .with_join(match self.join {
                1 => Join::Miter,
                2 => Join::Bevel,
                _ => Join::Round,
            })
            .with_dashes(0., dash_pattern(self.pattern, self.weight))
    }
    /// Compound bands are fractions of the nominal line width.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/compoundtype-cell-line-format-section
    pub fn compound_bands(&self) -> Vec<(f64, f64)> {
        match self.compound {
            1 => vec![(-1. / 3., 1. / 3.), (1. / 3., 1. / 3.)],
            2 => vec![(-0.4, 0.2), (0.2, 0.6)],
            3 => vec![(-0.2, 0.6), (0.4, 0.2)],
            4 => vec![(-0.4, 0.2), (0., 0.2), (0.4, 0.2)],
            _ => vec![(0., 1.)],
        }
    }
}
/// Round straight-segment corners with tangent circular arcs. Existing curves retain their geometry.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/rounding-cell-line-format-section
pub fn round_rectangle(path: &BezPath, radius: f64) -> BezPath {
    if !radius.is_finite() || radius <= 0. {
        return path.clone();
    }
    let points: Vec<_> = path
        .elements()
        .iter()
        .filter_map(|el| match el {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => Some(*p),
            _ => None,
        })
        .collect();
    if points.len() == 5
        && points.first() == points.last()
        && path.elements().iter().all(|el| matches!(el, PathEl::MoveTo(_) | PathEl::LineTo(_) | PathEl::ClosePath))
        && points.windows(2).all(|p| p.first().zip(p.get(1)).is_some_and(|(a, b)| a.x == b.x || a.y == b.y))
    {
        let b = path.bounding_box();
        return RoundedRect::from_rect(b, radius.min(b.width() * 0.5).min(b.height() * 0.5)).to_path(1e-6);
    }
    if path.elements().len() > 100_000 || path.elements().iter().any(|e| matches!(e, PathEl::QuadTo(..) | PathEl::CurveTo(..))) {
        return path.clone();
    }
    fn append(out: &mut BezPath, points: &mut Vec<Point>, closed: bool, radius: f64) {
        if points.len() > 1 && points.first() == points.last() {
            points.pop();
        }
        let n = points.len();
        if n == 0 {
            return;
        }
        for (i, &p) in points.iter().enumerate() {
            let previous = if i > 0 {
                points.get(i - 1)
            } else if closed {
                points.last()
            } else {
                None
            };
            let next = points.get(i + 1).or_else(|| if closed { points.first() } else { None });
            let rounded = previous.zip(next).and_then(|(previous, next)| {
                let a = p - *previous;
                let b = *next - p;
                let la = a.hypot();
                let lb = b.hypot();
                if la < 1e-12 || lb < 1e-12 {
                    return None;
                }
                let u = a / la;
                let v = b / lb;
                let turn = u.cross(v).atan2(u.dot(v)).abs();
                if turn < 1e-6 || turn > std::f64::consts::PI - 1e-6 {
                    return None;
                }
                let tan = (turn * 0.5).tan();
                let d = (radius * tan).min(la * 0.5).min(lb * 0.5);
                let k = 4. / 3. * (turn * 0.25).tan() * d / tan;
                Some((p - u * d, p + v * d, u * k, v * k))
            });
            if let Some((before, after, cu, cv)) = rounded {
                if i == 0 {
                    out.move_to(before);
                } else {
                    out.line_to(before);
                }
                out.curve_to(before + cu, after - cv, after);
            } else if i == 0 {
                out.move_to(p);
            } else {
                out.line_to(p);
            }
        }
        if closed {
            out.close_path();
        }
        points.clear();
    }
    let mut out = BezPath::new();
    let mut points = Vec::new();
    for element in path.elements() {
        match element {
            PathEl::MoveTo(p) => {
                append(&mut out, &mut points, false, radius);
                points.push(*p);
            }
            PathEl::LineTo(p) => points.push(*p),
            PathEl::ClosePath => append(&mut out, &mut points, true, radius),
            _ => {}
        }
    }
    append(&mut out, &mut points, false, radius);
    out
}
fn marker_scale(size: u32, weight: f64) -> f64 {
    // SVG arrow group, 2pt pens: sizes1/2/4 use1.90/2.26/2.98 stroke units.
    weight * (1.54 + 0.36 * f64::from(size))
}
/// Shaft endpoints stop at the filled marker's rear intersection.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/endarrow-cell-line-format-section
pub fn marker_setback(kind: u32, size: u32, weight: f64) -> f64 {
    marker_scale(size, weight)
        * match kind {
            5 | 8 => 1.75,
            39 => 4.,
            _ => 0.,
        }
}
/// Trim a single open contour by physical arc length, leaving closed contours intact.
pub fn marker_shaft(path: &BezPath, begin: f64, end: f64) -> BezPath {
    if begin <= 0. && end <= 0.
        || path.elements().iter().filter(|e| matches!(e, PathEl::MoveTo(_))).count() != 1
        || path.elements().iter().any(|e| matches!(e, PathEl::ClosePath))
    {
        return path.clone();
    }
    let segments: Vec<_> = path
        .segments()
        .map(|s| {
            let length = s.arclen(1e-8);
            (s, length)
        })
        .collect();
    let total: f64 = segments.iter().map(|(_, n)| n).sum();
    let limit = total - end;
    let mut out = BezPath::new();
    let mut offset = 0.;
    for (segment, length) in segments {
        let a = (begin - offset).max(0.);
        let b = (limit - offset).min(length);
        if b > a && length > 0. {
            let part = segment.subsegment(segment.inv_arclen(a, 1e-8)..segment.inv_arclen(b, 1e-8));
            if out.elements().is_empty() {
                out.move_to(part.start());
            }
            out.push(part.as_path_el());
        }
        offset += length;
    }
    out
}
#[derive(Clone, Debug)]
pub struct ArrowPart {
    pub path: BezPath,
    pub filled: bool,
    pub stroke: bool,
}
/// Arrow geometry uses circles, bars, triangles and diamonds. Measurements come
/// from arrow/{begin,end}-00..45 at three sizes, not an embedded vendor asset.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/beginarrow-cell-line-format-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/beginarrowsize-cell-line-format-section
pub fn arrow(kind: u32, size: u32, weight: f64, tip: Point, direction: f64) -> Vec<ArrowPart> {
    if kind == 0 || kind > 45 || size > 6 || !weight.is_finite() || weight <= 0. || !direction.is_finite() || !tip.x.is_finite() || !tip.y.is_finite()
    {
        return vec![];
    }
    let scale = marker_scale(size, weight);
    let matrix = Affine::translate(tip.to_vec2()) * Affine::rotate(direction) * Affine::scale_non_uniform(-scale, scale);
    let mut parts = Vec::new();
    let mut add = |path: BezPath, filled: bool, stroke: bool| parts.push(ArrowPart { path: matrix * path, filled, stroke });
    let line = |pts: &[(f64, f64)], closed: bool| {
        let mut p = BezPath::new();
        for (i, point) in pts.iter().enumerate() {
            if i == 0 { p.move_to(*point) } else { p.line_to(*point) }
        }
        if closed {
            p.close_path();
        }
        p
    };
    let triangle = |offset: f64, len: f64, closed: bool| line(&[(offset + len, -1.), (offset, 0.), (offset + len, 1.)], closed);
    let bar = |x: f64| line(&[(x, -0.75), (x, 0.75)], false);
    let circle = |x: f64| Circle::new((x, 0.), 0.75).to_path(1e-4);
    let diamond = |x: f64| line(&[(x, 0.), (x + 2.25, -1.125), (x + 4.5, 0.), (x + 2.25, 1.125)], true);
    match kind {
        1..=4 | 12..=16 => {
            let len = match kind {
                1 | 2 | 15 => 1.,
                12..=14 => 3.,
                _ => 2.,
            };
            let filled = matches!(kind, 2 | 4 | 13);
            add(triangle(0., len, filled || kind >= 14), filled, !filled);
        }
        5 | 8 => {
            // The SVG exports alias type8 to type5. Its rear edge is concave.
            let mut p = line(&[(2., 1.), (0., 0.), (1.98117, -0.993387)], false);
            p.curve_to((1.67173, -0.364515), (1.67301, 0.372641), (1.98465, 1.00043));
            p.close_path();
            add(p, true, false);
        }
        6 | 17 | 18 => {
            let mut p = triangle(0., 2., false);
            let bend = if matches!(kind, 6 | 18) { 2.45 } else { 1.55 };
            p.curve_to((bend, 0.33), (bend, -0.33), (2., -1.));
            p.close_path();
            let filled = kind < 17;
            add(p, filled, !filled);
        }
        7 | 19 => {
            let mut p = BezPath::new();
            p.move_to((2., -1.));
            p.curve_to((1.5, -0.3), (0.7, 0.), (0., 0.));
            p.curve_to((0.7, 0.), (1.5, 0.3), (2., 1.));
            if kind == 19 {
                p.curve_to((1.55, 0.3), (1.55, -0.3), (2., -1.));
                p.close_path();
            }
            add(p, false, true);
        }
        9 => add(line(&[(1., -1.), (-1., 1.)], false), false, true),
        10 | 20 => add(circle(0.), kind != 20, kind == 20),
        11 | 21 => add(Rect::new(-0.75, -0.75, 0.75, 0.75).to_path(1e-4), kind == 11, kind == 21),
        22 => add(diamond(0.), false, true),
        23 => add(line(&[(2.5, 1.), (1.5, -1.)], false), false, true),
        24..=26 => {
            for i in 0..kind - 23 {
                add(bar(1.5 + 0.75 * f64::from(i)), false, true);
            }
        }
        27..=29 => {
            add(line(&[(0., -0.75), (1.5, 0.), (0., 0.75)], false), false, true);
            if kind == 28 {
                add(bar(2.), false, true);
            }
            if kind == 29 {
                add(circle(2.25), false, true);
            }
        }
        30 => {
            add(circle(2.25), false, true);
            add(bar(0.75), false, true);
        }
        31..=38 => {
            let filled = kind >= 35;
            add(circle(0.75), filled, !filled);
            let base = if filled { kind - 4 } else { kind };
            if base == 34 {
                add(diamond(1.5), false, true);
            } else {
                for i in 0..base - 30 {
                    add(bar(2.25 + 0.75 * f64::from(i)), false, true);
                }
            }
        }
        39 | 40 => {
            add(triangle(0., 2., true), kind == 39, kind == 40);
            add(triangle(2., 2., true), kind == 39, kind == 40);
        }
        41 => add(circle(0.75), false, true),
        42 => add(circle(0.75), true, false),
        43..=45 => {
            add(triangle(0., 2., false), false, true);
            if kind != 44 {
                add(triangle(2., 2., false), false, true);
            }
            if kind != 43 {
                add(bar(if kind == 44 { 3. } else { 5. }), false, true);
            }
        }
        _ => {}
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn svg_marker_scales_aliases_and_shaft_setbacks() {
        // generated/arrow/end-08.svg, begin-08.svg and end-39.svg, 2pt pen.
        for (size, scale) in [(1, 1.9), (2, 2.26), (4, 2.98)] {
            assert!((marker_scale(size, 2. / 72.) * 72. - 2. * scale).abs() < 1e-10);
            assert!((marker_setback(8, size, 2. / 72.) * 72. - 3.5 * scale).abs() < 1e-10);
            assert!((marker_setback(39, size, 2. / 72.) * 72. - 8. * scale).abs() < 1e-10);
            let a = arrow(8, size, 2. / 72., Point::ZERO, 0.);
            let b = arrow(5, size, 2. / 72., Point::ZERO, 0.);
            assert_eq!(a[0].path, b[0].path);
        }
        let mut path = BezPath::new();
        path.move_to((0., 0.));
        path.line_to((5., 0.));
        let trimmed = marker_shaft(&path, 0., marker_setback(39, 1, 2. / 72.));
        assert!((trimmed.bounding_box().x1 * 72. - 344.8).abs() < 1e-8);
        assert!(marker_shaft(&path, 6., 0.).elements().is_empty());
        path.close_path();
        assert_eq!(marker_shaft(&path, 1., 1.), path);
    }
    #[test]
    fn observed_diagonal_coverage_and_legacy_gradient_ramp() {
        let fill = Fill { pattern: 2, foreground: Rgba::rgb(0, 32, 96), background: Rgba::rgb(255, 255, 192) };
        let center = fill.sample(0., 0., 0, 7);
        assert!((i16::from(center.r) - 21).abs() <= 1);
        assert_eq!(fill.sample(0., 0., 0, 0).r, 191);
        let ramp = Fill { pattern: 25, ..fill };
        assert!((i16::from(ramp.sample(0.5, 0.5, 0, 0).r) - 186).abs() <= 1);
    }
    #[test]
    fn all_patterns_and_arrow_sizes_are_finite_and_bounded() {
        for pattern in 0..=40 {
            let f = Fill { pattern, foreground: Rgba::BLACK, background: Rgba::WHITE };
            for y in 0..8 {
                for x in 0..8 {
                    let _ = f.sample(f64::from(x) / 7., f64::from(y) / 7., x, y);
                }
            }
        }
        for kind in 1..=45 {
            for size in 0..=6 {
                let parts = arrow(kind, size, 2. / 72., Point::ZERO, 0.);
                assert!(!parts.is_empty(), "{kind}");
                for p in parts {
                    let b = p.path.bounding_box();
                    assert!(b.width().is_finite() && b.width() < 3.);
                }
            }
        }
        assert_eq!(Fill { pattern: 25, foreground: Rgba::BLACK, background: Rgba::WHITE }.sample(f64::NAN, 0., 0, 0), Rgba::TRANSPARENT);
        assert_eq!(Gradient { stops: vec![(f64::NAN, Rgba::BLACK)], radial: false, angle: 0. }.sample(0., 0.), Rgba::TRANSPARENT);
        assert!(arrow(u32::MAX, 0, 1., Point::ZERO, 0.).is_empty());
        assert!(arrow(1, 0, f64::NAN, Point::ZERO, 0.).is_empty());
    }
    #[test]
    fn measured_line_families_and_fill_density() {
        assert_eq!(dash_pattern(2, 0.75), vec![5.25, 3.75]);
        assert_eq!(dash_pattern(23, 0.75), vec![0.75, 1.5]);
        for (pattern, density) in [(3, 15), (9, 32), (10, 16), (11, 8), (12, 4), (13, 32), (23, 28)] {
            let f = Fill { pattern, foreground: Rgba::BLACK, background: Rgba::WHITE };
            let count = (0..8).flat_map(|y| (0..8).map(move |x| (x, y))).filter(|(x, y)| f.sample(0., 0., *x, *y) == Rgba::BLACK).count();
            assert_eq!(count, density, "pattern {pattern}");
        }
    }
}
