//! Local path sampling for the Phase 1 reference functions, independent of dormant CAD geometry.
//! Row meanings: https://learn.microsoft.com/en-us/office/client-developer/visio/row-element-geometry-sectionvisio-xml
use super::{Error, Evaluator, Scope};
use std::f64::consts::TAU;

type Point = (f64, f64);
type Sample = (Point, Point);
const MAX_ROWS: usize = 4096;
const MAX_COORD: f64 = 1e12;
const MAX_SAMPLES: usize = 100_000;

#[derive(Clone, Copy)]
enum Segment {
    Line { start: Point, end: Point },
    Arc { center: Point, u: Point, v: Point, start: f64, sweep: f64 },
}
impl Segment {
    fn sample(self, t: f64) -> Result<Sample, Error> {
        let (point, derivative) = match self {
            Self::Line { start, end } => (add(start, mul(sub(end, start), t)), sub(end, start)),
            Self::Arc { center, u, v, start, sweep } => {
                let (s, c) = (start + sweep * t).sin_cos();
                (add(center, add(mul(u, c), mul(v, s))), mul(add(mul(u, -s), mul(v, c)), sweep))
            }
        };
        let length = norm(derivative);
        if !length.is_finite() || length == 0.0 {
            return Err(Error::Num);
        }
        Ok((point_checked(point)?, mul(derivative, 1.0 / length)))
    }
    fn speed(self, t: f64, budget: &mut usize) -> Result<f64, Error> {
        if *budget == 0 {
            return Err(Error::Limit);
        }
        *budget -= 1;
        let speed = match self {
            Self::Line { start, end } => norm(sub(end, start)),
            Self::Arc { u, v, start, sweep, .. } => {
                let (s, c) = (start + sweep * t).sin_cos();
                norm(add(mul(u, -s), mul(v, c))) * sweep.abs()
            }
        };
        if speed.is_finite() { Ok(speed) } else { Err(Error::Num) }
    }
    // Adaptive Simpson integration of the analytic speed. Both work and subdivision depth
    // are capped; failure to meet the tolerance is an error, never an approximate success.
    fn length(self, end: f64, budget: &mut usize) -> Result<f64, Error> {
        if end == 0.0 {
            return Ok(0.0);
        }
        if let Self::Line { start, end: finish } = self {
            return Ok(norm(sub(finish, start)) * end);
        }
        let a = self.speed(0.0, budget)?;
        let m = self.speed(end * 0.5, budget)?;
        let b = self.speed(end, budget)?;
        let whole = end * (a + 4.0 * m + b) / 6.0;
        let tolerance = 1e-11 * whole.abs().max(1.0);
        let mut pending = vec![(0.0, end, a, m, b, whole, tolerance, 0u8)];
        let mut total = 0.0;
        while let Some((lo, hi, a, m, b, old, tol, depth)) = pending.pop() {
            let middle = (lo + hi) * 0.5;
            let l = self.speed((lo + middle) * 0.5, budget)?;
            let r = self.speed((middle + hi) * 0.5, budget)?;
            let left = (middle - lo) * (a + 4.0 * l + m) / 6.0;
            let right = (hi - middle) * (m + 4.0 * r + b) / 6.0;
            let delta = left + right - old;
            // Subdivide at least twice: full ellipses can alias at the initial sample angles.
            if depth >= 2 && delta.abs() <= 15.0 * tol {
                total += left + right + delta / 15.0;
            } else {
                if depth >= 20 {
                    return Err(Error::Limit);
                }
                pending.push((middle, hi, m, r, b, right, tol * 0.5, depth + 1));
                pending.push((lo, middle, a, l, m, left, tol * 0.5, depth + 1));
            }
        }
        if total.is_finite() && total >= 0.0 { Ok(total) } else { Err(Error::Num) }
    }
    fn at_distance(self, distance: f64, length: f64, budget: &mut usize) -> Result<Sample, Error> {
        if length <= 0.0 {
            return Err(Error::Num);
        }
        if distance <= 0.0 {
            return self.sample(0.0);
        }
        if distance >= length {
            return self.sample(1.0);
        }
        if matches!(self, Self::Line { .. }) {
            return self.sample(distance / length);
        }
        let (mut lo, mut hi) = (0.0, 1.0);
        for _ in 0..44 {
            let middle = (lo + hi) * 0.5;
            if self.length(middle, budget)? < distance {
                lo = middle;
            } else {
                hi = middle;
            }
        }
        self.sample((lo + hi) * 0.5)
    }
}
fn add(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}
fn sub(a: Point, b: Point) -> Point {
    (a.0 - b.0, a.1 - b.1)
}
fn mul(a: Point, n: f64) -> Point {
    (a.0 * n, a.1 * n)
}
fn norm(a: Point) -> f64 {
    a.0.hypot(a.1)
}
fn cross(a: Point, b: Point) -> f64 {
    a.0 * b.1 - a.1 * b.0
}
fn point_checked(p: Point) -> Result<Point, Error> {
    if !p.0.is_finite() || !p.1.is_finite() {
        Err(Error::Num)
    } else if p.0.abs().max(p.1.abs()) > MAX_COORD {
        Err(Error::Limit)
    } else {
        Ok(p)
    }
}

/// Transform the three on-ellipse points to a circle using the documented axis ratio,
/// solve its circumcenter, then select the oriented sweep that contains the middle point.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/ellipticalarcto-row-geometry-section
fn elliptical_arc(start: Point, middle: Point, end: Point, angle: f64, ratio: f64) -> Result<Segment, Error> {
    if !angle.is_finite() || !ratio.is_finite() || ratio <= 0.0 || ratio > 1000.0 {
        return Err(Error::Num);
    }
    let (s, c) = angle.sin_cos();
    let rotate = |p: Point| (p.0 * c + p.1 * s, (-p.0 * s + p.1 * c) * ratio);
    let a = rotate(sub(middle, start));
    let b = rotate(sub(end, start));
    let scale = norm(a).max(norm(b));
    if !scale.is_finite() || scale == 0.0 {
        return Err(Error::Num);
    }
    let a = mul(a, 1.0 / scale);
    let b = mul(b, 1.0 / scale);
    let determinant = 2.0 * cross(a, b);
    if determinant.abs() < 1e-14 {
        return Err(Error::Unsupported("degenerate elliptical arc".into()));
    }
    let a2 = a.0 * a.0 + a.1 * a.1;
    let b2 = b.0 * b.0 + b.1 * b.1;
    let center = mul(((a2 * b.1 - b2 * a.1) / determinant, (a.0 * b2 - b.0 * a2) / determinant), scale);
    let radius = norm(center);
    let center_world = point_checked(add(start, (center.0 * c - center.1 / ratio * s, center.0 * s + center.1 / ratio * c)))?;
    let u = point_checked((radius * c, radius * s))?;
    let v = point_checked((-radius / ratio * s, radius / ratio * c))?;
    let first = (-center.1).atan2(-center.0);
    let am = sub(rotate(sub(middle, start)), center);
    let ae = sub(rotate(sub(end, start)), center);
    let middle_angle = (am.1.atan2(am.0) - first).rem_euclid(TAU);
    let finish = (ae.1.atan2(ae.0) - first).rem_euclid(TAU);
    let sweep = if middle_angle <= finish { finish } else { finish - TAU };
    if sweep.abs() < 1e-14 {
        return Err(Error::Num);
    }
    Ok(Segment::Arc { center: center_world, u, v, start: first, sweep })
}

impl Evaluator<'_> {
    /// Arc-length travel in a GeometryN.Path, optionally restricted to a 1-based stroked segment.
    /// Returns shape-local point and unit tangent; transforms and offsets belong to the caller.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/pointalongpath-function
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/anglealongpath-function
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/moveto-row-geometry-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/lineto-row-geometry-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/relmoveto-row-geometry-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/rellineto-row-geometry-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/ellipse-row-geometry-section
    pub fn path_sample(&mut self, scope: Scope, reference: &str, travel: f64, segment: Option<usize>) -> Result<Sample, Error> {
        if self.eval_depth == 0 {
            self.work = 0;
            self.eval_work = 0;
        }
        if !travel.is_finite() || !(0.0..=1.0).contains(&travel) {
            return Err(Error::Num);
        }
        if segment == Some(0) {
            return Err(Error::Ref);
        }
        let (scope, name) = self.reference_target(scope, reference)?;
        let lower = name.to_ascii_lowercase();
        let index = lower
            .strip_prefix("geometry")
            .and_then(|s| s.strip_suffix(".path"))
            .and_then(|s| s.parse::<u32>().ok())
            .and_then(|n| n.checked_sub(1))
            .ok_or(Error::Ref)?;
        let sheet = self.sheet(scope).ok_or(Error::Ref)?;
        let section = sheet.sections.iter().find(|s| s.name == "Geometry" && s.index.unwrap_or(0) == index && !s.deleted).ok_or(Error::Ref)?;
        if section.rows.len() > MAX_ROWS {
            return Err(Error::Limit);
        }
        let mut rows: Vec<_> = section.rows.iter().enumerate().filter(|(_, r)| !r.deleted).collect();
        rows.sort_by_key(|(position, row)| row.index.unwrap_or(u32::try_from(*position).unwrap_or(u32::MAX)));
        let mut segments = Vec::new();
        let mut current = None;
        let mut previous_index = None;
        for (position, row) in &rows {
            let row_index = row.index.unwrap_or(u32::try_from(*position).map_err(|_| Error::Limit)?);
            if previous_index == Some(row_index) {
                return Err(Error::Value);
            }
            previous_index = Some(row_index);
            let prefix = format!("Geometry{}.{{}}{row_index}", index.checked_add(1).ok_or(Error::Limit)?);
            let cell = |this: &mut Self, column: &str| -> Result<f64, Error> {
                if !row.cells.iter().any(|c| c.name == column) {
                    return Err(Error::Ref);
                }
                let name = prefix.replace("{}", column);
                this.resolve(scope, &name).numeric()
            };
            let row_type = row.row_type.as_deref().ok_or(Error::Ref)?;
            if !matches!(row_type, "MoveTo" | "RelMoveTo" | "LineTo" | "RelLineTo" | "Ellipse" | "EllipticalArcTo") {
                return Err(Error::Unsupported(format!("geometry row {row_type}")));
            }
            let mut endpoint = point_checked((cell(self, "X")?, cell(self, "Y")?))?;
            if matches!(row_type, "RelMoveTo" | "RelLineTo") {
                self.add_dependency(scope, "Width");
                self.add_dependency(scope, "Height");
                endpoint =
                    point_checked((endpoint.0 * self.resolve(scope, "Width").numeric()?, endpoint.1 * self.resolve(scope, "Height").numeric()?))?;
            }
            match row_type {
                "MoveTo" | "RelMoveTo" => current = Some(endpoint),
                "LineTo" | "RelLineTo" => {
                    segments.push(Segment::Line { start: current.ok_or(Error::Ref)?, end: endpoint });
                    current = Some(endpoint);
                }
                "Ellipse" => {
                    if rows.len() != 1 {
                        return Err(Error::Unsupported("ellipse mixed with other geometry rows".into()));
                    }
                    let u = point_checked(sub((cell(self, "A")?, cell(self, "B")?), endpoint))?;
                    let v = point_checked(sub((cell(self, "C")?, cell(self, "D")?), endpoint))?;
                    if norm(u) == 0.0 || norm(v) == 0.0 {
                        return Err(Error::Num);
                    }
                    if (u.0 * v.0 + u.1 * v.1).abs() > 1e-10 * norm(u) * norm(v) {
                        return Err(Error::Unsupported("ellipse points are not independent axis endpoints".into()));
                    }
                    segments.push(Segment::Arc { center: endpoint, u, v, start: 0.0, sweep: TAU });
                }
                "EllipticalArcTo" => {
                    let middle = point_checked((cell(self, "A")?, cell(self, "B")?))?;
                    let angle = cell(self, "C")?;
                    let ratio = cell(self, "D")?;
                    segments.push(elliptical_arc(current.ok_or(Error::Ref)?, middle, endpoint, angle, ratio)?);
                    current = Some(endpoint);
                }
                other => return Err(Error::Unsupported(format!("geometry row {other}"))),
            }
        }
        if segments.is_empty() {
            return Err(Error::Ref);
        }
        let available = MAX_SAMPLES.saturating_sub(self.eval_work);
        let mut budget = available;
        let result = (|| {
            if let Some(index) = segment {
                let chosen = *segments.get(index.checked_sub(1).ok_or(Error::Ref)?).ok_or(Error::Ref)?;
                let length = chosen.length(1.0, &mut budget)?;
                return chosen.at_distance(length * travel, length, &mut budget);
            }
            let mut lengths = Vec::with_capacity(segments.len());
            let mut total = 0.0;
            for item in &segments {
                let length = item.length(1.0, &mut budget)?;
                total += length;
                lengths.push(length);
            }
            if !total.is_finite() || total <= 0.0 {
                return Err(Error::Num);
            }
            let mut distance = total * travel;
            for (item, length) in segments.iter().zip(&lengths) {
                if *length > 0.0 && distance <= *length {
                    return item.at_distance(distance, *length, &mut budget);
                }
                distance -= length;
            }
            let (item, _) = segments.iter().zip(&lengths).rev().find(|(_, length)| **length > 0.0).ok_or(Error::Num)?;
            item.sample(1.0)
        })();
        self.eval_work = self.eval_work.saturating_add(available - budget);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diagramcraft_model::{Cell, Document, Row, Section, Sheet};
    fn row(index: u32, kind: &str, values: &[(&str, f64)]) -> Row {
        Row {
            index: Some(index),
            row_type: Some(kind.into()),
            cells: values.iter().map(|(name, n)| Cell { name: (*name).into(), value: Some(n.to_string()), ..Default::default() }).collect(),
            ..Default::default()
        }
    }
    fn document(rows: Vec<Row>) -> Document {
        Document {
            document_sheet: Sheet {
                cells: vec![
                    Cell { name: "Width".into(), value: Some("8".into()), ..Default::default() },
                    Cell { name: "Height".into(), value: Some("4".into()), ..Default::default() },
                ],
                sections: vec![Section { name: "Geometry".into(), index: Some(0), rows, ..Default::default() }],
                ..Default::default()
            },
            ..Default::default()
        }
    }
    fn near(a: Point, b: Point) {
        assert!(norm(sub(a, b)) < 1e-8, "{a:?} != {b:?}");
    }
    #[test]
    fn lines_use_length_segment_numbers_and_skip_move_gaps() {
        let d = document(vec![
            row(1, "MoveTo", &[("X", 0.), ("Y", 0.)]),
            row(2, "LineTo", &[("X", 3.), ("Y", 0.)]),
            row(3, "MoveTo", &[("X", 10.), ("Y", 10.)]),
            row(4, "LineTo", &[("X", 10.), ("Y", 14.)]),
        ]);
        let mut e = Evaluator::new(&d).unwrap();
        let (p, t) = e.path_sample(Scope::Document, "Geometry1.Path", 0.5, None).unwrap();
        near(p, (10., 10.5));
        near(t, (0., 1.));
        near(e.path_sample(Scope::Document, "Geometry1.Path", 0.5, Some(1)).unwrap().0, (1.5, 0.));
        near(e.path_sample(Scope::Document, "Geometry1.Path", 1., Some(2)).unwrap().0, (10., 14.));
        assert_eq!(e.path_sample(Scope::Document, "Geometry1.Path", 0., Some(3)), Err(Error::Ref));
    }
    #[test]
    fn relative_rows_scale_by_shape_dimensions_and_overlay_formulas() {
        let d = document(vec![row(1, "RelMoveTo", &[("X", 0.), ("Y", 0.5)]), row(2, "RelLineTo", &[("X", 1.), ("Y", 1.)])]);
        let mut e = Evaluator::new(&d).unwrap();
        near(e.path_sample(Scope::Document, "Geometry1.Path", 0.5, None).unwrap().0, (4., 3.));
        e.set_formula(Scope::Document, "Width", "16").unwrap();
        near(e.path_sample(Scope::Document, "Geometry1.Path", 0.5, None).unwrap().0, (8., 3.));
    }
    #[test]
    fn ellipse_quarters_close_and_nonuniform_arc_length_is_integrated() {
        let d = document(vec![row(1, "Ellipse", &[("X", 3.), ("Y", 4.), ("A", 5.), ("B", 4.), ("C", 3.), ("D", 5.)])]);
        let mut e = Evaluator::new(&d).unwrap();
        let (p, t) = e.path_sample(Scope::Document, "Geometry1.Path", 0.25, None).unwrap();
        near(p, (3., 5.));
        near(t, (-1., 0.));
        near(e.path_sample(Scope::Document, "Geometry1.Path", 1., None).unwrap().0, (5., 4.));
        let arc = Segment::Arc { center: (0., 0.), u: (2., 0.), v: (0., 1.), start: 0., sweep: TAU };
        let mut budget = MAX_SAMPLES;
        let length = arc.length(1., &mut budget).unwrap();
        // Independent fine polygon sum checks the adaptive analytic-speed integrator.
        let mut polygon = 0.;
        let mut previous = (2., 0.);
        for i in 1..=20000 {
            let angle = TAU * f64::from(i) / 20000.;
            let p = (2. * angle.cos(), angle.sin());
            polygon += norm(sub(p, previous));
            previous = p;
        }
        assert!((length - polygon).abs() < 1e-7);
        let (p, t) = arc.at_distance(length * 0.125, length, &mut budget).unwrap();
        assert!(p.1 > std::f64::consts::FRAC_1_SQRT_2); // half quadrant length is past half angle
        near((norm(t), 0.), (1., 0.));
    }
    #[test]
    fn elliptical_arc_recovers_both_sweep_directions_and_rotation() {
        for sign in [1., -1.] {
            let arc = elliptical_arc((2., 0.), (0., sign), (-2., 0.), 0., 2.).unwrap();
            let mut budget = MAX_SAMPLES;
            let length = arc.length(1., &mut budget).unwrap();
            let (p, t) = arc.at_distance(length * 0.5, length, &mut budget).unwrap();
            near(p, (0., sign));
            near(t, (-1., 0.));
            near(arc.sample(0.).unwrap().0, (2., 0.));
            near(arc.sample(1.).unwrap().0, (-2., 0.));
        }
        let arc = elliptical_arc((4., 7.), (3., 5.), (4., 3.), std::f64::consts::FRAC_PI_2, 2.).unwrap();
        let mut budget = MAX_SAMPLES;
        let length = arc.length(1., &mut budget).unwrap();
        near(arc.at_distance(length * 0.5, length, &mut budget).unwrap().0, (3., 5.));
    }
    #[test]
    fn invalid_hostile_and_unsupported_paths_return_errors() {
        let d = document(vec![row(1, "Ellipse", &[("X", 0.), ("Y", 0.), ("A", 1.), ("B", 0.), ("C", 0.), ("D", 1.)])]);
        let mut e = Evaluator::new(&d).unwrap();
        for t in [f64::NAN, f64::INFINITY, -0.1, 1.1] {
            assert_eq!(e.path_sample(Scope::Document, "Geometry1.Path", t, None), Err(Error::Num));
        }
        for name in ["Geometry0.Path", "Geometry4294967296.Path", "Geometry2.Path", "Width"] {
            assert_eq!(e.path_sample(Scope::Document, name, 0.5, None), Err(Error::Ref));
        }
        for s in [0, usize::MAX] {
            assert_eq!(e.path_sample(Scope::Document, "Geometry1.Path", 0.5, Some(s)), Err(Error::Ref));
        }
        assert!(elliptical_arc((0., 0.), (1., 0.), (2., 0.), 0., 1.).is_err());
        for ratio in [0., -1., 1001., f64::NAN] {
            assert!(elliptical_arc((1., 0.), (0., 1.), (-1., 0.), 0., ratio).is_err());
        }
        let d = document(vec![row(1, "NURBSTo", &[])]);
        let mut e = Evaluator::new(&d).unwrap();
        assert!(matches!(e.path_sample(Scope::Document, "Geometry1.Path", 0.5, None), Err(Error::Unsupported(_))));
        let d = document(vec![row(1, "MoveTo", &[("X", 0.), ("Y", 0.)]); MAX_ROWS + 1]);
        let mut e = Evaluator::new(&d).unwrap();
        assert_eq!(e.path_sample(Scope::Document, "Geometry1.Path", 0.5, None), Err(Error::Limit));
        assert_eq!(point_checked((f64::INFINITY, 0.)), Err(Error::Num));
        assert_eq!(point_checked((MAX_COORD * 2., 0.)), Err(Error::Limit));
    }
    #[test]
    fn quadrature_work_is_shared_with_reentrant_evaluation() {
        let d = document(vec![row(1, "Ellipse", &[("X", 0.), ("Y", 0.), ("A", 2.), ("B", 0.), ("C", 0.), ("D", 1.)])]);
        let mut e = Evaluator::new(&d).unwrap();
        e.eval_depth = 1;
        e.eval_work = MAX_SAMPLES - 20;
        assert_eq!(e.path_sample(Scope::Document, "Geometry1.Path", 0.125, None), Err(Error::Limit));
        assert_eq!(e.eval_work, MAX_SAMPLES);
        e.eval_depth = 0;
        assert!(e.path_sample(Scope::Document, "Geometry1.Path", 0.125, None).is_ok());
    }
}
