//! Numeric spatial constraints and nearest-path reference dispatch.
use crate::{Context, Error, Value, eval::eval, parser::Expr};

pub const NAMES: &[&str] = &["BOUND", "RECTSECT", "NEARESTPOINTONPATH", "INTERSECTX", "INTERSECTY"];
pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if !NAMES.contains(&name) {
        return None;
    }
    Some(match dispatch(name, args, context, budget, depth) {
        Ok(v) => v,
        Err(e) => Value::Error(e),
    })
}
fn number(args: &[Expr], index: usize, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<f64, Error> {
    let expr = args.get(index).ok_or(Error::Value)?;
    if matches!(expr, Expr::Missing) {
        return Err(Error::Value);
    }
    eval(expr, context, budget, depth + 1).numeric()
}
fn dispatch(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    match name {
        "BOUND" => bound(args, context, budget, depth),
        "INTERSECTX" | "INTERSECTY" => {
            if args.len() != 6 {
                return Err(Error::Value);
            }
            let mut v = [0.; 6];
            for (i, slot) in v.iter_mut().enumerate() {
                *slot = number(args, i, context, budget, depth)?;
            }
            let (x, y) = intersect(v)?;
            Ok(Value::Number(if name == "INTERSECTX" { x } else { y }))
        }
        "RECTSECT" => {
            if args.len() != 5 {
                return Err(Error::Value);
            }
            let width = number(args, 0, context, budget, depth)?;
            let height = number(args, 1, context, budget, depth)?;
            let x = number(args, 2, context, budget, depth)?;
            let y = number(args, 3, context, budget, depth)?;
            let option = number(args, 4, context, budget, depth)?;
            rectsect(width, height, x, y, option != 0.).map(Value::Number)
        }
        "NEARESTPOINTONPATH" => {
            if args.len() != 3 {
                return Err(Error::Value);
            }
            let reference = match args.first().ok_or(Error::Value)? {
                Expr::Reference(s) | Expr::Text(s) => {
                    if s.len() > 65536 {
                        return Err(Error::Limit);
                    }
                    s.clone()
                }
                expr => match eval(expr, context, budget, depth + 1) {
                    Value::Text(s) => s,
                    Value::Error(e) => return Err(e),
                    _ => return Err(Error::Value),
                },
            };
            if reference.len() > 65536 {
                return Err(Error::Limit);
            }
            if reference.is_empty() {
                return Err(Error::Ref);
            }
            let x = number(args, 1, context, budget, depth)?;
            let y = number(args, 2, context, budget, depth)?;
            Ok(context.nearest_point_on_path(&reference, x, y))
        }
        _ => Err(Error::Value),
    }
}
/// Read-time range constraints. Disabled and ignored ranges do not evaluate their endpoints.
/// Equal-distance candidates use the lower boundary; exclusive intervals permit their boundary.
/// These tie conventions remain explicit pending additional oracle edge-case witnesses.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/bound-function
fn bound(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    if args.len() < 5 || !(args.len() - 2).is_multiple_of(3) || args.len() > 1024 {
        return Err(Error::Value);
    }
    let value = number(args, 0, context, budget, depth)?;
    let kind = number(args, 1, context, budget, depth)?;
    if kind == 2. {
        return Ok(Value::Number(value));
    }
    if kind != 0. && kind != 1. {
        return Err(Error::Value);
    }
    let mut ranges = Vec::new();
    for chunk in args.get(2..).ok_or(Error::Value)?.as_chunks::<3>().0 {
        if eval(chunk.first().ok_or(Error::Value)?, context, budget, depth + 1).truth()? {
            continue;
        }
        let a = number(chunk, 1, context, budget, depth)?;
        let b = number(chunk, 2, context, budget, depth)?;
        ranges.push((a.min(b), a.max(b)));
    }
    if ranges.is_empty() {
        return Ok(Value::Number(value));
    }
    ranges.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut merged: Vec<(f64, f64)> = Vec::new();
    for (lo, hi) in ranges {
        if let Some(last) = merged.last_mut()
            && (lo < last.1 || kind == 0. && lo == last.1)
        {
            last.1 = last.1.max(hi);
        } else {
            merged.push((lo, hi));
        }
    }
    if kind == 1. {
        for (lo, hi) in merged {
            if value > lo && value < hi {
                return Ok(Value::Number(if (value * 0.5 - lo * 0.5).abs() <= (hi * 0.5 - value * 0.5).abs() { lo } else { hi }));
            }
        }
        return Ok(Value::Number(value));
    }
    let mut closest = None;
    for (lo, hi) in merged {
        if value >= lo && value <= hi {
            return Ok(Value::Number(value));
        }
        for candidate in [lo, hi] {
            let distance = (candidate * 0.5 - value * 0.5).abs();
            if closest.is_none_or(|(_, old)| distance < old) {
                closest = Some((candidate, distance));
            }
        }
    }
    closest.map(|(v, _)| Value::Number(v)).ok_or(Error::Value)
}
/// Sectors use coordinates relative to the center; scale each axis before comparing products.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/rectsect-function
fn rectsect(width: f64, height: f64, x: f64, y: f64, vertical_tie: bool) -> Result<f64, Error> {
    if ![width, height, x, y].iter().all(|v| v.is_finite()) || width <= 0. || height <= 0. {
        return Err(Error::Num);
    }
    if x == 0. && y == 0. {
        return Ok(0.);
    }
    if x == 0. {
        return Ok(if y > 0. { 2. } else { 4. });
    }
    if y == 0. {
        return Ok(if x > 0. { 1. } else { 3. });
    }
    let dimensions = width.max(height);
    let position = x.abs().max(y.abs());
    let horizontal = (x.abs() / position) * (height / dimensions);
    let vertical = (y.abs() / position) * (width / dimensions);
    let comparison = if horizontal == 0. && vertical == 0. {
        (x.abs().ln() + height.ln()).total_cmp(&(y.abs().ln() + width.ln()))
    } else {
        horizontal.total_cmp(&vertical)
    };
    if comparison.is_gt() || comparison.is_eq() && !vertical_tie { Ok(if x > 0. { 1. } else { 3. }) } else { Ok(if y > 0. { 2. } else { 4. }) }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Test;
    impl Context for Test {
        fn resolve(&mut self, r: &str) -> Value {
            match r {
                "Width" => Value::Number(8.),
                "Height" => Value::Number(4.),
                _ => Value::Error(Error::Ref),
            }
        }
        fn nearest_point_on_path(&mut self, r: &str, x: f64, y: f64) -> Value {
            if r == "Geometry1.Path" { Value::number((x + y) / 10.) } else { Value::Error(Error::Ref) }
        }
    }
    #[test]
    fn documented_bound_controls_and_interval_sets() {
        for (formula, want) in [
            ("BOUND(Width*.5,0,FALSE,Width*0,Width*1)", 4.),
            ("BOUND(9,0,FALSE,8,0)", 8.),
            ("BOUND(-3,0,FALSE,0,8)", 0.),
            ("BOUND(5,0,FALSE,2,2,FALSE,4,4,FALSE,6,6)", 4.),
            ("BOUND(5,1,FALSE,0,8)", 8.),
            ("BOUND(5,1,FALSE,0,6,FALSE,4,8)", 8.),
            ("BOUND(4,1,FALSE,0,4,FALSE,4,8)", 4.),
            ("BOUND(9,2,FALSE,1/0,Missing)", 9.),
            ("BOUND(9,0,TRUE,1/0,Missing)", 9.),
        ] {
            assert_eq!(crate::formula(formula, &mut Test), Value::Number(want), "{formula}");
        }
    }
    #[test]
    fn documented_rectsect_example_all_sectors_and_diagonals() {
        assert_eq!(crate::formula("RECTSECT(1in,2in,1in,-7in,0)", &mut Test), Value::Number(4.));
        for (x, y, want) in [(0., 0., 0.), (2., 0., 1.), (0., 2., 2.), (-2., 0., 3.), (0., -2., 4.)] {
            assert_eq!(rectsect(2., 2., x, y, false), Ok(want));
        }
        assert_eq!(rectsect(2., 4., 1., 2., false), Ok(1.));
        assert_eq!(rectsect(2., 4., 1., 2., true), Ok(2.));
        assert_eq!(rectsect(1e308, 1e308, -1e308, -1e308, true), Ok(4.));
        assert_eq!(rectsect(1e308, 1e-308, 0., 1., false), Ok(2.));
        assert_eq!(rectsect(1e-300, 1e300, 1e-301, 1e300, false), Ok(2.));
        assert_eq!(rectsect(1e-300, 1e300, 1e-300, 1e299, true), Ok(1.));
    }
    #[test]
    fn nearest_passes_reference_identity_and_rejects_bad_inputs() {
        assert_eq!(crate::formula("NEARESTPOINTONPATH(Geometry1.Path,2,3)", &mut Test), Value::Number(0.5));
        assert_eq!(crate::formula("NEARESTPOINTONPATH(Geometry2.Path,2,3)", &mut Test), Value::Error(Error::Ref));
        for f in [
            "BOUND(,0,FALSE,0,1)",
            "BOUND(1,3,FALSE,0,1)",
            "BOUND(1,0,FALSE,0)",
            "BOUND(1,0,FALSE,0,1,2)",
            "RECTSECT(0,1,2,3,0)",
            "RECTSECT(1,2,3,4)",
            "NEARESTPOINTONPATH(2,3,4)",
        ] {
            assert!(matches!(crate::formula(f, &mut Test), Value::Error(_)), "{f}");
        }
        assert_eq!(rectsect(f64::INFINITY, 1., 0., 0., false), Err(Error::Num));
    }
}

/// The point where the line through (x1, y1) at angle1 meets the line through (x2, y2) at
/// angle2, in the caller's coordinates. Angles are ShapeSheet angles (radians). Parallel lines
/// are a divide-by-zero, which Visio ignores and keeps the last value of the cell.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/intersectx-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/intersecty-function
fn intersect([x1, y1, a1, x2, y2, a2]: [f64; 6]) -> Result<(f64, f64), Error> {
    if ![x1, y1, a1, x2, y2, a2].iter().all(|v| v.is_finite()) {
        return Err(Error::Num);
    }
    let (s1, c1) = a1.sin_cos();
    let (s2, c2) = a2.sin_cos();
    let cross = c1 * s2 - s1 * c2;
    if cross.abs() < 1e-12 {
        return Err(Error::Div0);
    }
    let t = ((x2 - x1) * s2 - (y2 - y1) * c2) / cross;
    let (x, y) = (x1 + t * c1, y1 + t * s1);
    if !x.is_finite() || !y.is_finite() {
        return Err(Error::Num);
    }
    Ok((x, y))
}

#[cfg(test)]
mod intersect_tests {
    use super::*;
    #[test]
    fn two_guides_meet_where_geometry_says() {
        // A vertical guide through (1, 5) and a horizontal guide through (7, 2) meet at (1, 2).
        let (x, y) = intersect([1., 5., std::f64::consts::FRAC_PI_2, 7., 2., 0.]).unwrap();
        assert!((x - 1.).abs() < 1e-12 && (y - 2.).abs() < 1e-12);
        // A diagonal through the origin at 45 degrees and a vertical through (3, 0): (3, 3).
        let (x, y) = intersect([0., 0., std::f64::consts::FRAC_PI_4, 3., 0., std::f64::consts::FRAC_PI_2]).unwrap();
        assert!((x - 3.).abs() < 1e-12 && (y - 3.).abs() < 1e-12);
        // Order of the lines does not matter.
        let (x, y) = intersect([3., 0., std::f64::consts::FRAC_PI_2, 0., 0., std::f64::consts::FRAC_PI_4]).unwrap();
        assert!((x - 3.).abs() < 1e-12 && (y - 3.).abs() < 1e-12);
    }
    #[test]
    fn parallel_lines_and_bad_numbers_are_errors() {
        assert_eq!(intersect([0., 0., 0., 0., 1., 0.]).unwrap_err(), Error::Div0);
        assert_eq!(intersect([0., 0., 0., 0., 1., std::f64::consts::PI]).unwrap_err(), Error::Div0);
        assert_eq!(intersect([f64::NAN, 0., 0., 0., 1., 1.]).unwrap_err(), Error::Num);
        assert_eq!(intersect([0., 0., 0., 1e308, 1e308, 1.]).map(|_| ()).unwrap_or(()), ());
    }
}
