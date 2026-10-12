//! ShapeSheet functions with explicit arity, bounded evaluation and reference examples.
use crate::{Context, Error, Value, eval::eval, parser::Expr};
mod color;
mod numeric;
mod points;
mod references;
mod spatial;
mod text;
pub use color::default_color;
pub const IMPLEMENTED: &[&str] = &[
    "LANGUAGE",
    "BOUND",
    "RECTSECT",
    "NEARESTPOINTONPATH",
    "PNT",
    "PNTX",
    "PNTY",
    "PAR",
    "LOC",
    "LOCTOPAR",
    "LOCTOLOC",
    "ANG360",
    "POINTALONGPATH",
    "ANGLEALONGPATH",
    "SETATREF",
    "SETATREFEXPR",
    "SETATREFEVAL",
    "DEPENDSON",
    "SETF",
    "GETREF",
    "EVALCELL",
    "ARG",
    "EVALTEXT",
    "FORMAT",
    "FORMATEX",
    "STRSAME",
    "LEN",
    "LEFT",
    "RIGHT",
    "MID",
    "INDEX",
    "LOOKUP",
    "LISTSEP",
    "RED",
    "GREEN",
    "BLUE",
    "RGB",
    "HSL",
    "HUE",
    "SAT",
    "LUM",
    "FONT",
    "FONTTOID",
    "ABS",
    "ACOS",
    "ASIN",
    "ATAN",
    "ATAN2",
    "COS",
    "COSH",
    "SIN",
    "SINH",
    "TAN",
    "TANH",
    "PI",
    "POW",
    "SQRT",
    "GUARD",
    "IF",
    "AND",
    "OR",
    "NOT",
    "MIN",
    "MAX",
    "INT",
    "ROUND",
    "MODULUS",
    "TRUNC",
    "CEILING",
    "FLOOR",
];
pub(crate) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Value {
    if let Some(value) = spatial::call(name, args, context, budget, depth) {
        return value;
    }
    if let Some(value) = points::call(name, args, context, budget, depth) {
        return value;
    }
    if let Some(value) = references::call(name, args, context, budget, depth) {
        return value;
    }
    if let Some(value) = text::call(name, args, context, budget, depth) {
        return value;
    }
    match name {
        "LANGUAGE" => language(args, context, budget, depth),
        "GUARD" => guard(args, context, budget, depth),
        "IF" => conditional(args, context, budget, depth),
        "AND" | "OR" => logical(name, args, context, budget, depth),
        _ => {
            let values: Vec<_> = args.iter().map(|arg| eval(arg, context, budget, depth + 1)).collect();
            color::call(name, &values, context).unwrap_or_else(|| numeric::call(name, &values))
        }
    }
}
/// Convert BCP47 strings or numeric LCID representations to a language identifier.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/language-function
fn language(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Value {
    let [arg] = args else { return Value::Error(Error::Value) };
    let value = eval(arg, context, budget, depth + 1);
    if let Some(e) = value.error() {
        return Value::Error(e);
    }
    match crate::language::lcid(&value.text()) {
        Ok(id) => Value::Number(f64::from(id)),
        Err(e) => Value::Error(e),
    }
}
/// Protection affects editing; reading GUARD evaluates its expression unchanged.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/guard-function
fn guard(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Value {
    if args.len() != 1 {
        return Value::Error(Error::Value);
    }
    args.first().map(|e| eval(e, context, budget, depth + 1)).unwrap_or(Value::Error(Error::Value))
}
/// Evaluate only the branch selected by the logical expression.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/if-function
fn conditional(args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Value {
    if args.len() != 3 {
        return Value::Error(Error::Value);
    }
    let Some(test) = args.first() else { return Value::Error(Error::Value) };
    match eval(test, context, budget, depth + 1).truth() {
        Ok(b) => args.get(if b { 1 } else { 2 }).map(|e| eval(e, context, budget, depth + 1)).unwrap_or(Value::Error(Error::Value)),
        Err(e) => Value::Error(e),
    }
}
/// AND returns true only for all true arguments; OR for any true argument.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/and-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/or-function
fn logical(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Value {
    if args.is_empty() {
        return Value::Error(Error::Value);
    }
    let and = name == "AND";
    for e in args {
        match eval(e, context, budget, depth + 1).truth() {
            Ok(b) if b != and => return Value::Bool(!and),
            Ok(_) => {}
            Err(e) => return Value::Error(e),
        }
    }
    Value::Bool(and)
}

#[cfg(test)]
#[allow(clippy::approx_constant)] // Retain the decimal examples printed in Microsoft reference pages.
mod tests {
    use super::*;
    struct Example;
    impl Context for Example {
        fn resolve(&mut self, r: &str) -> Value {
            match r {
                "Height" => Value::Number(2.0),
                "PinX" => Value::Number(0.5),
                "TheText" => Value::Text("Example".into()),
                _ => Value::Error(Error::Ref),
            }
        }
        fn cached(&self) -> Value {
            Value::Number(3.0)
        }
    }
    fn result(f: &str) -> Value {
        crate::formula(f, &mut Example)
    }
    fn near(f: &str, expected: f64, tolerance: f64) {
        let got = result(f).numeric().unwrap();
        assert!((got - expected).abs() <= tolerance, "{f}: {got} != {expected}");
    }
    #[test]
    fn microsoft_arithmetic_and_trig_examples() {
        assert_eq!(result("LANGUAGE(\"en-us\")"), Value::Number(1033.));
        assert_eq!(result("LANGUAGE(\"es-es\")"), Value::Number(3082.));
        for (f, n, t) in [
            ("ABS(-1.24)", 1.24, 1e-12),
            ("ACOS(0)", std::f64::consts::FRAC_PI_2, 1e-12),
            ("ASIN(1)", std::f64::consts::FRAC_PI_2, 1e-12),
            ("ATAN(1)", std::f64::consts::FRAC_PI_4, 1e-12),
            // The page prints 29.0456 deg, a digit transposition: the defined angle is 29.0546041 deg.
            // ADS_M.vssx master2 shape6 Angle independently agrees with standard atan2 to 1e-15.
            ("ATAN2(1.25,2.25)", 1.25_f64.atan2(2.25), 1e-12),
            ("ATAN2(1,SQRT(3))", std::f64::consts::PI / 6., 1e-12),
            ("ATAN2(1,1)", std::f64::consts::FRAC_PI_4, 1e-12),
            ("COS(45deg)", 0.7071, 0.00005),
            ("COSH(45deg)", 1.3246, 0.00005),
            ("SIN(45deg)", 0.7071, 0.00005),
            ("SINH(45deg)", 0.8687, 0.00005),
            ("TAN(45deg)", 1., 1e-12),
            ("(14in/2)^2*PI()", 153.9380, 0.00005),
            ("POW(5,2)", 25., 1e-12),
            ("SQRT(2)", 1.4142, 0.00005),
        ] {
            near(f, n, t)
        }
        // The TANH page's example uses a degree result despite its stated radians default.
        // Test both readings explicitly; this is not a measured Visio-runtime discrepancy.
        near("TANH(-45)", -1.0, 1e-12);
        near("TANH(-45deg)", -0.6558, 0.00005);
    }
    #[test]
    fn microsoft_guard_logic_and_extrema_examples() {
        near("GUARD(TEXTWIDTH(TheText)+0.5in)", 3.5, 1e-12);
        near("IF(Height>1.25in,5,7)", 5., 1e-12);
        assert_eq!(result("AND(Height>1,PinX>1)"), Value::Bool(false));
        assert_eq!(result("OR(Height>1,PinX>1)"), Value::Bool(true));
        assert_eq!(result("NOT(Height>0.75in)"), Value::Bool(false));
        near("MIN(13in,1ft,20cm)", 20. / 2.54, 1e-12);
        near("MAX(13in,1ft,20cm)", 13., 1e-12);
        near("IF(1,7,1/0)", 7., 1e-12);
        near("IF(0,1/0,8)", 8., 1e-12);
    }
    #[test]
    fn microsoft_rounding_and_modulus_examples() {
        for (f, n) in [
            ("INT(7.2)", 7.),
            ("INT(-7.2)", -8.),
            ("ROUND(123.654,2)", 123.65),
            ("ROUND(123.654,0)", 124.),
            ("ROUND(123.654,-1)", 120.),
            ("TRUNC(123.654,2)", 123.65),
            ("TRUNC(123.654,0)", 123.),
            ("TRUNC(123.654,-1)", 120.),
            ("MODULUS(5,1.4)", 0.8),
            ("MODULUS(5,-1.4)", -0.6),
            ("MODULUS(-5,1.4)", 0.6),
            ("MODULUS(-5,-1.4)", -0.8),
            ("CEILING(3.7)", 4.),
            ("CEILING(-3.7)", -4.),
            ("CEILING(3.7,0.25)", 3.75),
            ("FLOOR(3.7)", 3.),
            ("FLOOR(-3.7)", -3.),
            ("FLOOR(3.7,0.5)", 3.5),
        ] {
            near(f, n, 1e-12)
        }
    }
    #[test]
    fn function_errors_limits_and_arity() {
        for f in ["SQRT(-1)", "ACOS(2)", "ASIN(-2)", "CEILING(1,-1)", "FLOOR(-1,1)", "COSH(1e300)"] {
            assert_eq!(result(f), Value::Error(Error::Num), "{f}");
        }
        assert_eq!(result("MODULUS(1,0)"), Value::Error(Error::Div0));
        for f in ["ABS()", "IF(1,2)", "PI(1)", "GUARD(1,2)", "ROUND(1)", "TRUNC(1)", "AND()", "MIN()"] {
            assert_eq!(result(f), Value::Error(Error::Value), "{f}");
        }
        near("POW(0,0)", 0., 0.);
        near("POW(0,-1)", 0., 0.);
        near("ATAN2(0,0)", 0., 0.);
        near("ROUND(1,1e300)", 1., 0.);
        near("TRUNC(1,-1e300)", 0., 0.);
    }
}
