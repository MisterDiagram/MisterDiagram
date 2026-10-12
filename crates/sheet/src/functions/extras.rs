//! Error handling, bit operations and numeric helpers from the ShapeSheet reference.
use crate::{Context, Error, Value, eval::eval, parser::Expr};

pub(super) const NAMES: &[&str] =
    &["ISERROR", "ISERRVALUE", "ISERR", "ISERRNA", "IFERROR", "NA", "MAGNITUDE", "BITXOR", "BITAND", "BITOR", "BITNOT", "DEG", "RAD", "SIGN"];
pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if !NAMES.contains(&name) {
        return None;
    }
    Some(dispatch(name, args, context, budget, depth).unwrap_or_else(Value::Error))
}
fn dispatch(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    let valid = match name {
        "NA" => args.is_empty(),
        "MAGNITUDE" => args.len() == 4,
        "BITXOR" | "BITAND" | "BITOR" | "IFERROR" => args.len() == 2,
        "SIGN" => (1..=2).contains(&args.len()),
        _ => args.len() == 1,
    };
    if !valid {
        return Err(Error::Value);
    }
    if name == "NA" {
        return Ok(na());
    }
    let first = eval(args.first().ok_or(Error::Value)?, context, budget, depth + 1);
    match name {
        "ISERROR" => return iserror(first),
        "ISERRVALUE" => return iserrvalue(first),
        "ISERR" => return iserr(first),
        "ISERRNA" => return iserrna(first),
        "IFERROR" => return iferror(first, args.get(1).ok_or(Error::Value)?, context, budget, depth),
        _ => {}
    }
    let mut numbers = Vec::with_capacity(args.len());
    numbers.push(first.numeric()?);
    for arg in args.iter().skip(1) {
        numbers.push(eval(arg, context, budget, depth + 1).numeric()?);
    }
    let x = *numbers.first().ok_or(Error::Value)?;
    let y = numbers.get(1).copied();
    Ok(match name {
        "MAGNITUDE" => magnitude(x, y.ok_or(Error::Value)?, *numbers.get(2).ok_or(Error::Value)?, *numbers.get(3).ok_or(Error::Value)?),
        "BITXOR" => bitxor(x, y.ok_or(Error::Value)?)?,
        "BITAND" => bitand(x, y.ok_or(Error::Value)?)?,
        "BITOR" => bitor(x, y.ok_or(Error::Value)?)?,
        "BITNOT" => bitnot(x)?,
        "DEG" => deg(x),
        "RAD" => rad(x),
        "SIGN" => sign(x, y.unwrap_or(1e-9))?,
        _ => return Err(Error::Unsupported(name.into())),
    })
}
// Implementation failures must not masquerade as errors produced by Visio itself.
fn engine_failure(value: &Value) -> Result<(), Error> {
    match value {
        Value::Error(error @ (Error::Unsupported(_) | Error::Limit | Error::Parse(_))) => Err(error.clone()),
        _ => Ok(()),
    }
}
/// Recognize any computed ShapeSheet error value.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/iserror-function-visioshapesheet
fn iserror(value: Value) -> Result<Value, Error> {
    engine_failure(&value)?;
    Ok(Value::Bool(matches!(value, Value::Error(_))))
}
/// Recognize only an argument or cell type error.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/iserrvalue-function
fn iserrvalue(value: Value) -> Result<Value, Error> {
    engine_failure(&value)?;
    Ok(Value::Bool(matches!(value, Value::Error(Error::Value))))
}
/// Recognize every computed error except the not-available value.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/iserr-function
fn iserr(value: Value) -> Result<Value, Error> {
    engine_failure(&value)?;
    Ok(Value::Bool(matches!(value, Value::Error(e) if e != Error::Na)))
}
/// Recognize only the not-available value.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/iserrna-function
fn iserrna(value: Value) -> Result<Value, Error> {
    engine_failure(&value)?;
    Ok(Value::Bool(matches!(value, Value::Error(Error::Na))))
}
/// Evaluate the alternate expression only when the primary result is a ShapeSheet error.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/iferror-function
fn iferror(value: Value, alternate: &Expr, context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    engine_failure(&value)?;
    Ok(if matches!(value, Value::Error(_)) { eval(alternate, context, budget, depth + 1) } else { value })
}
/// Supply an explicit not-available error instead of a cell default.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/na-function
fn na() -> Value {
    Value::Error(Error::Na)
}
/// Euclidean magnitude of a vector with independently scaled components.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/magnitude-function
fn magnitude(a: f64, x: f64, b: f64, y: f64) -> Value {
    Value::number((a * x).hypot(b * y))
}
// The reference defines 16-bit binary inputs, but does not specify coercion of fractions
// or out-of-range values. Reject those domains rather than inventing truncation/wrapping.
fn bits(x: f64) -> Result<u16, Error> {
    if x.is_finite() && (0.0..=f64::from(u16::MAX)).contains(&x) && x.fract() == 0.0 { Ok(x as u16) } else { Err(Error::Num) }
}
/// Exclusive OR of two unsigned 16-bit words.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/bitxor-function
fn bitxor(x: f64, y: f64) -> Result<Value, Error> {
    Ok(Value::Number(f64::from(bits(x)? ^ bits(y)?)))
}
/// AND of two unsigned 16-bit words.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/bitand-function
fn bitand(x: f64, y: f64) -> Result<Value, Error> {
    Ok(Value::Number(f64::from(bits(x)? & bits(y)?)))
}
/// Inclusive OR of two unsigned 16-bit words.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/bitor-function
fn bitor(x: f64, y: f64) -> Result<Value, Error> {
    Ok(Value::Number(f64::from(bits(x)? | bits(y)?)))
}
/// Complement all sixteen bits, retaining an unsigned result.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/bitnot-function
fn bitnot(x: f64) -> Result<Value, Error> {
    Ok(Value::Number(f64::from(!bits(x)?)))
}
/// Convert a radian number to its degree number.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/deg-function
fn deg(x: f64) -> Value {
    Value::number(x.to_degrees())
}
/// Convert a degree number to its radian number.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/rad-function
fn rad(x: f64) -> Value {
    Value::number(x.to_radians())
}
/// Classify a number's sign with an optional near-zero tolerance, defaulting to 1e-9.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/sign-function
fn sign(x: f64, fuzz: f64) -> Result<Value, Error> {
    if fuzz < 0.0 {
        return Err(Error::Num);
    }
    Ok(Value::Number(if x.abs() <= fuzz { 0.0 } else { x.signum() }))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Example;
    impl Context for Example {
        fn resolve(&mut self, name: &str) -> Value {
            match name {
                "Scratch.A1" => Value::Error(Error::Na),
                "Scratch.X1" => Value::Error(Error::Value),
                "Scratch.B1" => Value::Text("5 + 7".into()),
                _ => Value::Error(Error::Ref),
            }
        }
    }
    fn result(formula: &str) -> Value {
        crate::formula(formula, &mut Example)
    }
    #[test]
    fn microsoft_error_examples() {
        for f in ["ISERROR(Scratch.A1)", "ISERROR(Scratch.X1)", "ISERR(Scratch.X1)", "ISERRNA(Scratch.A1)"] {
            assert_eq!(result(f), Value::Bool(true), "{f}");
        }
        for f in ["ISERR(Scratch.A1)", "ISERRNA(Scratch.B1)"] {
            assert_eq!(result(f), Value::Bool(false), "{f}");
        }
        assert_eq!(result("IF(ISERRVALUE(Scratch.X1),2,Scratch.X1)"), Value::Number(2.0));
        // The ISERRVALUE page's table preserves this string; its prose incorrectly says 12.
        assert_eq!(result("IF(ISERRVALUE(Scratch.B1),2,Scratch.B1)"), Value::Text("5 + 7".into()));
        assert_eq!(result("NA()"), Value::Error(Error::Na));
        assert_eq!(result("NA()").text(), "#N/A!");
    }
    #[test]
    fn microsoft_numeric_examples() {
        for (formula, expected) in [
            ("MAGNITUDE(1,3,1,4)", 5.0),
            ("BITXOR(12,6)", 10.0),
            ("BITAND(12,6)", 4.0),
            ("BITOR(12,6)", 14.0),
            ("BITNOT(6)", 65529.0),
            ("DEG(PI()/4)", 45.0),
            ("RAD(45)", std::f64::consts::FRAC_PI_4),
            ("SIGN(-5)", -1.0),
            ("SIGN(0)", 0.0),
            ("SIGN(0.00000000001)", 0.0),
            ("SIGN(0.00000000001,0)", 1.0),
        ] {
            assert_eq!(result(formula), Value::Number(expected), "{formula}");
        }
    }
    #[test]
    fn lazy_error_fallback_and_honest_implementation_failures() {
        assert_eq!(result("IFERROR(1/0,7)"), Value::Number(7.0));
        assert_eq!(result("IFERROR(3,1/0)"), Value::Number(3.0));
        assert_eq!(result("IFERROR(NA(),\"missing\")"), Value::Text("missing".into()));
        for f in ["IFERROR(UNKNOWN(),0)", "ISERROR(UNKNOWN())", "ISERRVALUE(UNKNOWN())", "ISERR(UNKNOWN())", "ISERRNA(UNKNOWN())"] {
            assert!(matches!(result(f), Value::Error(Error::Unsupported(_))), "{f}");
        }
    }
    #[test]
    fn hostile_arity_and_numeric_domains() {
        for f in ["ISERROR()", "IFERROR(1)", "NA(1)", "MAGNITUDE(1,2)", "BITNOT()", "BITAND(1)", "DEG()", "RAD(1,2)", "SIGN()"] {
            assert_eq!(result(f), Value::Error(Error::Value), "{f}");
        }
        for f in ["BITNOT(-1)", "BITAND(65536,1)", "BITXOR(1.5,2)", "BITOR(1e300,2)", "SIGN(1,-1)", "MAGNITUDE(1e300,1e300,1,1)"] {
            assert_eq!(result(f), Value::Error(Error::Num), "{f}");
        }
        assert_eq!(result("BITNOT(65535)"), Value::Number(0.0));
    }
}
