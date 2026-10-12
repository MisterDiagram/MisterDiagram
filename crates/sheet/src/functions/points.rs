//! Point values, coordinate transforms and path queries delegated to the document host.
use crate::{Context, Error, Value, eval::eval, parser::Expr};

pub const NAMES: &[&str] = &["PNT", "PNTX", "PNTY", "PAR", "LOC", "LOCTOLOC", "LOCTOPAR", "ANG360", "POINTALONGPATH", "ANGLEALONGPATH"];
const LIMIT: usize = 65536;
#[derive(Clone, Debug, Default)]
struct Frame {
    reference: Option<String>,
    parent: bool,
    ambiguous: bool,
}
struct Point {
    xy: (f64, f64),
    frame: Frame,
}
impl Point {
    fn value(self) -> Value {
        Value::Point(self.xy.0, self.xy.1)
    }
}
pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if !NAMES.contains(&name) {
        return None;
    }
    Some(match dispatch(name, args, context, budget, depth) {
        Ok(value) => value,
        Err(error) => Value::Error(error),
    })
}
fn dispatch(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    match name {
        "PNTX" | "PNTY" => projection(name, args, context, budget, depth),
        "ANG360" => angle(args, context, budget, depth),
        "ANGLEALONGPATH" => path_angle(args, context, budget, depth),
        _ => point_call(name, args, context, budget, depth).map(Point::value),
    }
}
fn arity(args: &[Expr], min: usize, max: usize) -> Result<(), Error> {
    if (min..=max).contains(&args.len()) { Ok(()) } else { Err(Error::Value) }
}
fn item(args: &[Expr], index: usize) -> Result<&Expr, Error> {
    args.get(index).ok_or(Error::Value)
}
fn bound(budget: &mut usize, depth: usize) -> Result<(), Error> {
    if depth > 128 || *budget == 0 {
        return Err(Error::Limit);
    }
    *budget -= 1;
    Ok(())
}
fn xy(value: Value) -> Result<(f64, f64), Error> {
    match value {
        Value::Point(x, y) if x.is_finite() && y.is_finite() => Ok((x, y)),
        Value::Point(..) => Err(Error::Num),
        Value::Error(e) => Err(e),
        _ => Err(Error::Value),
    }
}
fn point_expr(expr: &Expr, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Point, Error> {
    bound(budget, depth)?;
    if !context.enter_eval() {
        return Err(Error::Limit);
    }
    let result = point_expr_node(expr, context, budget, depth);
    context.leave_eval();
    result
}
fn point_expr_node(expr: &Expr, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Point, Error> {
    if let Expr::Call { name, args } = expr {
        if matches!(name.as_str(), "PNT" | "PAR" | "LOC" | "LOCTOLOC" | "LOCTOPAR" | "POINTALONGPATH") {
            return point_call(name, args, context, budget, depth);
        }
        if name == "IF" {
            arity(args, 3, 3)?;
            let yes = eval(item(args, 0)?, context, budget, depth + 1).truth()?;
            return point_expr(item(args, if yes { 1 } else { 2 })?, context, budget, depth + 1);
        }
        if name == "GUARD" {
            arity(args, 1, 1)?;
            return point_expr(item(args, 0)?, context, budget, depth + 1);
        }
    }
    let frame = if let Expr::Reference(reference) = expr { reference_frame(reference)? } else { Frame::default() };
    Ok(Point { xy: xy(eval(expr, context, budget, depth + 1))?, frame })
}
fn point_call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Point, Error> {
    match name {
        "PNT" => point(args, context, budget, depth),
        "PAR" | "LOC" | "LOCTOLOC" | "LOCTOPAR" => transform(name, args, context, budget, depth),
        "POINTALONGPATH" => path_point(args, context, budget, depth),
        _ => Err(Error::Value),
    }
}
fn reference_frame(reference: &str) -> Result<Frame, Error> {
    if reference.len() > LIMIT {
        return Err(Error::Limit);
    }
    Ok(Frame { reference: reference.contains('!').then(|| reference.into()), parent: false, ambiguous: false })
}
fn owner(frame: &Frame) -> Option<&str> {
    frame.reference.as_deref().and_then(|r| r.rsplit_once('!').map(|(scope, _)| scope))
}
fn merge_frame(a: Option<Frame>, b: Option<Frame>) -> Result<Option<Frame>, Error> {
    match (a, b) {
        (Some(a), Some(b)) if a.ambiguous || b.ambiguous || owner(&a).map(str::to_ascii_lowercase) != owner(&b).map(str::to_ascii_lowercase) => {
            Ok(Some(Frame { ambiguous: true, ..Frame::default() }))
        }
        (Some(a), _) => Ok(Some(a)),
        (None, b) => Ok(b),
    }
}
// Extract only coordinate reference provenance. Mixed coordinate systems require an
// explicit LOCTOLOC/LOCTOPAR source rather than silently choosing one of the shapes.
fn coordinate_frame(expr: &Expr, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Option<Frame>, Error> {
    bound(budget, depth)?;
    if !context.enter_eval() {
        return Err(Error::Limit);
    }
    let result = coordinate_frame_node(expr, context, budget, depth);
    context.leave_eval();
    result
}
fn coordinate_frame_node(expr: &Expr, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Option<Frame>, Error> {
    match expr {
        Expr::Reference(reference) => reference_frame(reference).map(Some),
        Expr::Unary(_, expr) => coordinate_frame(expr, context, budget, depth + 1),
        Expr::Binary(_, left, right) => {
            merge_frame(coordinate_frame(left, context, budget, depth + 1)?, coordinate_frame(right, context, budget, depth + 1)?)
        }
        // A function's result can have a different coordinate frame from its inputs.
        // Only direct coordinate expressions carry inferred source provenance.
        _ => Ok(None),
    }
}
/// PNT retains both coordinates. The destination cell, or PNTX/PNTY, selects an axis.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pnt-function
fn point(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Point, Error> {
    arity(args, 2, 2)?;
    let a = item(args, 0)?;
    let b = item(args, 1)?;
    let x = eval(a, context, budget, depth + 1).numeric()?;
    let y = eval(b, context, budget, depth + 1).numeric()?;
    let frame = merge_frame(coordinate_frame(a, context, budget, depth + 1)?, coordinate_frame(b, context, budget, depth + 1)?)?.unwrap_or_default();
    Ok(Point { xy: (x, y), frame })
}
/// Explicit point projection is independent of the destination cell.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pntx-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pnty-function
fn projection(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    arity(args, 1, 1)?;
    let point = point_expr(item(args, 0)?, context, budget, depth + 1)?;
    Ok(Value::number(if name == "PNTX" { point.xy.0 } else { point.xy.1 }))
}
/// Implicit transforms infer the source from PNT references; explicit transforms use
/// their source/destination references. Nested transforms retain the resulting frame.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/par-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/loc-function-visioshapesheet
/// https://learn.microsoft.com/en-us/office/client-developer/visio/loctopar-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/loctoloc-function
fn transform(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Point, Error> {
    let explicit = matches!(name, "LOCTOLOC" | "LOCTOPAR");
    let count = if explicit { 3 } else { 1 };
    arity(args, count, count)?;
    let point = point_expr(item(args, 0)?, context, budget, depth + 1)?;
    let source = if explicit {
        Frame { reference: Some(reference(item(args, 1)?, context, budget, depth + 1)?), parent: false, ambiguous: false }
    } else {
        point.frame
    };
    if source.ambiguous {
        return Err(Error::Value);
    }
    let destination = Frame {
        reference: if explicit { Some(reference(item(args, 2)?, context, budget, depth + 1)?) } else { None },
        parent: matches!(name, "PAR" | "LOCTOPAR"),
        ambiguous: false,
    };
    let value = context.transform_point(point.xy, source.reference.as_deref(), source.parent, destination.reference.as_deref(), destination.parent);
    Ok(Point { xy: xy(value)?, frame: destination })
}
/// ANG360 uses radians when no unit was supplied, and rejects nonnumeric text.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/ang360-function
fn angle(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    arity(args, 1, 1)?;
    let value = eval(item(args, 0)?, context, budget, depth + 1);
    let n = if let Value::Text(s) = value {
        if s.len() > LIMIT {
            return Err(Error::Limit);
        }
        s.trim().parse::<f64>().map_err(|_| Error::Value)?
    } else {
        value.numeric()?
    };
    if !n.is_finite() {
        return Err(Error::Num);
    }
    let result = n.rem_euclid(std::f64::consts::TAU);
    // Floating-point rem_euclid can round a tiny negative remainder to exactly TAU.
    Ok(Value::number(if result >= std::f64::consts::TAU { 0. } else { result }))
}
// Path cells are references, not ordinary scalar values. IF chooses a reference
// lazily, preserving the same work/depth budget and avoiding nonexistent branches.
fn reference(expr: &Expr, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<String, Error> {
    bound(budget, depth)?;
    if !context.enter_eval() {
        return Err(Error::Limit);
    }
    let result = reference_node(expr, context, budget, depth);
    context.leave_eval();
    result
}
fn reference_node(expr: &Expr, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<String, Error> {
    match expr {
        Expr::Reference(s) | Expr::Text(s) => {
            if s.len() > LIMIT {
                Err(Error::Limit)
            } else if s.is_empty() {
                Err(Error::Ref)
            } else {
                Ok(s.clone())
            }
        }
        Expr::Call { name, args } if name == "IF" => {
            arity(args, 3, 3)?;
            let yes = eval(item(args, 0)?, context, budget, depth + 1).truth()?;
            reference(item(args, if yes { 1 } else { 2 })?, context, budget, depth + 1)
        }
        Expr::Qualified(base, suffix) => {
            let mut base = reference(base, context, budget, depth + 1)?;
            if base.len().saturating_add(suffix.len()) > LIMIT {
                return Err(Error::Limit);
            }
            base.push_str(suffix);
            Ok(base)
        }
        _ => match eval(expr, context, budget, depth + 1) {
            Value::Text(s) if s.len() > LIMIT => Err(Error::Limit),
            Value::Text(s) if s.is_empty() => Err(Error::Ref),
            Value::Text(s) => Ok(s),
            Value::Error(e) => Err(e),
            _ => Err(Error::Value),
        },
    }
}
fn travel(expr: &Expr, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<f64, Error> {
    let n = eval(expr, context, budget, depth + 1).numeric()?;
    if (0. ..=1.).contains(&n) { Ok(n) } else { Err(Error::Value) }
}
fn segment(expr: Option<&Expr>, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Option<usize>, Error> {
    let Some(expr) = expr.filter(|e| !matches!(e, Expr::Missing)) else { return Ok(None) };
    let n = eval(expr, context, budget, depth + 1).numeric()?;
    if n < 1. || n > f64::from(u32::MAX) {
        return Err(Error::Ref);
    }
    if n.fract() != 0. {
        return Err(Error::Value);
    }
    Ok(Some(n as usize))
}
/// Positive offsets are left of travel; the host handles length and segment geometry.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pointalongpath-function
fn path_point(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Point, Error> {
    arity(args, 2, 4)?;
    let path = reference(item(args, 0)?, context, budget, depth + 1)?;
    let travel = travel(item(args, 1)?, context, budget, depth)?;
    let offset = args.get(2).map(|e| eval(e, context, budget, depth + 1).numeric()).transpose()?.unwrap_or(0.);
    let segment = segment(args.get(3), context, budget, depth)?;
    let frame = reference_frame(&path)?;
    Ok(Point { xy: xy(context.path_point(&path, travel, offset, segment))?, frame })
}
/// The host returns the tangent angle, or #REF for a nonexistent section/segment.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/anglealongpath-function
fn path_angle(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    arity(args, 2, 3)?;
    let path = reference(item(args, 0)?, context, budget, depth + 1)?;
    let travel = travel(item(args, 1)?, context, budget, depth)?;
    let segment = segment(args.get(2), context, budget, depth)?;
    Ok(Value::number(context.path_angle(&path, travel, segment).numeric()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct Example {
        reads: Vec<String>,
        depth: usize,
        transforms: Vec<(Option<String>, bool, Option<String>, bool)>,
    }
    impl Context for Example {
        fn enter_eval(&mut self) -> bool {
            if self.depth >= 32 {
                return false;
            }
            self.depth += 1;
            true
        }
        fn leave_eval(&mut self) {
            self.depth = self.depth.saturating_sub(1);
        }
        fn resolve(&mut self, r: &str) -> Value {
            self.reads.push(r.into());
            match r {
                "PinX" | "LocPinX" => Value::Number(2.),
                "PinY" | "LocPinY" => Value::Number(3.),
                "Sheet.5!LocPinX" => Value::Number(1.),
                "Sheet.5!LocPinY" => Value::Number(4.),
                _ => Value::Error(Error::Ref),
            }
        }
        fn transform_point(
            &mut self,
            point: (f64, f64),
            source: Option<&str>,
            source_parent: bool,
            destination: Option<&str>,
            destination_parent: bool,
        ) -> Value {
            self.transforms.push((source.map(str::to_owned), source_parent, destination.map(str::to_owned), destination_parent));
            fn origin(r: Option<&str>, parent: bool) -> (f64, f64) {
                if parent || r.is_some_and(|r| r.starts_with("ThePage!")) {
                    (0., 0.)
                } else if r.is_some_and(|r| r.starts_with("Sheet.5!")) {
                    (20., 30.)
                } else if r.is_some_and(|r| r.starts_with("Sheet.4!")) {
                    (40., 50.)
                } else {
                    (10., 15.)
                }
            }
            let a = origin(source, source_parent);
            let b = origin(destination, destination_parent);
            Value::Point(point.0 + a.0 - b.0, point.1 + a.1 - b.1)
        }
        fn path_point(&mut self, r: &str, t: f64, offset: f64, segment: Option<usize>) -> Value {
            if r != "Geometry1.Path" || segment.is_some_and(|s| s != 1) { Value::Error(Error::Ref) } else { Value::Point(10. * t, offset) }
        }
        fn path_angle(&mut self, r: &str, _: f64, segment: Option<usize>) -> Value {
            if r != "Geometry1.Path" || segment.is_some_and(|s| s != 1) { Value::Error(Error::Ref) } else { Value::Number(0.) }
        }
    }
    #[test]
    fn microsoft_point_and_projection_examples() {
        let mut c = Example::default();
        assert_eq!(crate::formula("PNT(PinX,PinY)", &mut c), Value::Point(2., 3.));
        assert_eq!(crate::formula("PNT(PinX,Sheet.5!LocPinY)", &mut c), Value::Point(2., 4.));
        assert_eq!(crate::formula("PNTX(PNT(7,12))", &mut c), Value::Number(7.));
        assert_eq!(crate::formula("PNTY(PNT(7,12))", &mut c), Value::Number(12.));
    }
    #[test]
    fn microsoft_transform_examples_and_nested_frames() {
        let mut c = Example::default();
        assert_eq!(crate::formula("PAR(PNT(PinX,PinY))", &mut c), Value::Point(12., 18.));
        assert_eq!(crate::formula("LOC(PNT(Sheet.5!LocPinX,Sheet.5!LocPinY))", &mut c), Value::Point(11., 19.));
        assert_eq!(crate::formula("LOCTOPAR(PNT(LocPinX,LocPinY),Width,Sheet.4!Width)", &mut c), Value::Point(12., 18.));
        assert_eq!(crate::formula("LOCTOLOC(PNT(LocPinX,LocPinY),Width,ThePage!PageWidth)", &mut c), Value::Point(12., 18.));
        assert_eq!(crate::formula("LOC(PAR(PNT(2,3)))", &mut c), Value::Point(2., 3.));
        assert_eq!(crate::formula("PAR(LOC(PNT(Sheet.5!LocPinX,Sheet.5!LocPinY)))", &mut c), Value::Point(21., 34.));
        assert_eq!(crate::formula("LOC(IF(TRUE,PAR(PNT(2,3)),1/0))", &mut c), Value::Point(2., 3.));
        assert_eq!(crate::formula("LOCTOLOC(PNT(PinX,Sheet.5!LocPinY),Width,ThePage!PageWidth)", &mut c), Value::Point(12., 19.));
        assert!(!c.reads.iter().any(|s| s == "Width" || s == "Sheet.4!Width"));
    }
    #[test]
    fn microsoft_ang360_examples_and_strict_range() {
        let mut c = Example::default();
        for (formula, n, tolerance) in
            [("ANG360(395deg)", 35f64.to_radians(), 1e-12), ("ANG360(-9.8rad)", 2.7664, 0.0001), ("ANG360(45)", 1.0177, 0.0001)]
        {
            let value = crate::formula(formula, &mut c).numeric().unwrap();
            assert!((value - n).abs() < tolerance, "{formula}: {value}");
        }
        assert_eq!(crate::formula(r#"ANG360("not an angle")"#, &mut c), Value::Error(Error::Value));
        assert_eq!(crate::formula("ANG360(-1e-100)", &mut c), Value::Number(0.));
    }
    #[test]
    fn path_contract_lazy_reference_offset_and_segment() {
        let mut c = Example::default();
        assert_eq!(crate::formula("POINTALONGPATH(IF(TRUE,Geometry1.Path,Missing.Path),.25,2,1)", &mut c), Value::Point(2.5, 2.));
        assert_eq!(crate::formula("POINTALONGPATH(Geometry1.Path,.5,,1)", &mut c), Value::Point(5., 0.));
        assert_eq!(crate::formula("ANGLEALONGPATH(Geometry1.Path,.5,1)", &mut c), Value::Number(0.));
        assert!(c.reads.is_empty());
        assert_eq!(crate::formula("POINTALONGPATH(Geometry1.Path,.5,0,2)", &mut c), Value::Error(Error::Ref));
        assert_eq!(crate::formula("POINTALONGPATH(Missing.Path,.5)", &mut c), Value::Error(Error::Ref));
    }
    #[test]
    fn hostile_ast_and_invalid_point_arguments() {
        let mut c = Example::default();
        for formula in [
            "PNT()",
            "PNTY(4)",
            "LOCTOLOC(PNT(1,2))",
            "POINTALONGPATH(Geometry1.Path,2)",
            "POINTALONGPATH(Geometry1.Path,.5,0,1.5)",
            "ANGLEALONGPATH(Geometry1.Path,.5,0)",
        ] {
            assert!(matches!(crate::formula(formula, &mut c), Value::Error(_)), "{formula}");
        }
        let mut expression = Expr::Call { name: "PNT".into(), args: vec![Expr::Number(1., None), Expr::Number(2., None)] };
        for _ in 0..150 {
            expression = Expr::Call { name: "PAR".into(), args: vec![expression] };
        }
        assert_eq!(crate::evaluate(&expression, &mut c), Value::Error(Error::Limit));
        assert_eq!(c.depth, 0);
        assert_eq!(crate::formula("PNT(1,2)", &mut c), Value::Point(1., 2.));
        assert_eq!(c.depth, 0);
        let expression = Expr::Call { name: "POINTALONGPATH".into(), args: vec![Expr::Reference("λ".repeat(LIMIT)), Expr::Number(0., None)] };
        assert_eq!(crate::evaluate(&expression, &mut c), Value::Error(Error::Limit));
    }
}
