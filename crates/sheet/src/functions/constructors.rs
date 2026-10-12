//! Bounded evaluated geometry-value constructors, without curve evaluation or rendering.
use crate::{Context, Error, Value, eval::eval, parser::Expr};
const MAX_ARGUMENTS: usize = 8192;
const MAX_OUTPUT: usize = 65536;
pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if !matches!(name, "NURBS" | "POLYLINE" | "FIELDPICTURE") {
        return None;
    }
    Some(construct(name, args, context, budget, depth).unwrap_or_else(Value::Error))
}
fn construct(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    if args.len() > MAX_ARGUMENTS {
        return Err(Error::Limit);
    }
    let valid = match name {
        "NURBS" => args.len() >= 8 && args.len().is_multiple_of(4),
        "POLYLINE" => args.len() >= 4 && args.len().is_multiple_of(2),
        _ => args.len() == 1,
    };
    if !valid {
        return Err(Error::Value);
    }
    let values = args.iter().map(|e| eval(e, context, budget, depth + 1).numeric()).collect::<Result<Vec<_>, _>>()?;
    match name {
        "NURBS" => nurbs(&values),
        "POLYLINE" => polyline(&values),
        _ => fieldpicture(*values.first().ok_or(Error::Value)?),
    }
}
/// Construct a spline value from its final knot, degree, coordinate modes and control tuples.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/nurbs-function
fn nurbs(values: &[f64]) -> Result<Value, Error> {
    let degree = *values.get(1).ok_or(Error::Value)?;
    if degree < 0.0 || degree.fract() != 0.0 {
        return Err(Error::Num);
    }
    modes(values.get(2..4).ok_or(Error::Value)?)?;
    serialize("NURBS", values, 4, 4)
}
/// Construct a polyline value from coordinate modes and at least one coordinate pair.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/polyline-function
fn polyline(values: &[f64]) -> Result<Value, Error> {
    modes(values.get(..2).ok_or(Error::Value)?)?;
    serialize("POLYLINE", values, 2, 2)
}
fn modes(values: &[f64]) -> Result<(), Error> {
    if values.iter().all(|&n| n == 0.0 || n == 1.0) { Ok(()) } else { Err(Error::Value) }
}
// Visio's observed cached geometry values use fourteen significant decimal digits.
// BLDCOR_M.vssx master10 and master23 witness tuple spacing and exponent spelling.
fn serialize(name: &str, values: &[f64], header: usize, tuple: usize) -> Result<Value, Error> {
    let mut out = format!("{name}(");
    for (i, &number) in values.iter().enumerate() {
        let separator = if i == 0 {
            ""
        } else if i <= header || (i - header).is_multiple_of(tuple) {
            ", "
        } else {
            ","
        };
        let number = general14(number)?;
        if out.len().checked_add(separator.len()).and_then(|n| n.checked_add(number.len())).is_none_or(|n| n >= MAX_OUTPUT) {
            return Err(Error::Limit);
        }
        out.push_str(separator);
        out.push_str(&number);
    }
    out.push(')');
    Ok(Value::Text(out))
}
fn general14(number: f64) -> Result<String, Error> {
    if !number.is_finite() {
        return Err(Error::Num);
    }
    if number == 0.0 {
        return Ok("0".into());
    }
    let scientific = format!("{:.13e}", number.abs());
    let (mantissa, power) = scientific.split_once('e').ok_or(Error::Value)?;
    let exponent = power.parse::<i32>().map_err(|_| Error::Value)?;
    let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
    let sign = if number < 0.0 { "-" } else { "" };
    if !(-4..14).contains(&exponent) {
        return Ok(format!("{sign}{mantissa}E{exponent}"));
    }
    let digits = mantissa.replace('.', "");
    let point = exponent + 1;
    if point <= 0 {
        let zeros = usize::try_from(-point).map_err(|_| Error::Value)?;
        return Ok(format!("{sign}0.{}{digits}", "0".repeat(zeros)));
    }
    let point = usize::try_from(point).map_err(|_| Error::Value)?;
    if point >= digits.len() {
        return Ok(format!("{sign}{digits}{}", "0".repeat(point - digits.len())));
    }
    Ok(format!("{sign}{}.{}", digits.get(..point).ok_or(Error::Value)?, digits.get(point..).ok_or(Error::Value)?))
}
/// Resolve a field format code to its format-picture representation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fieldpicture-function
/// https://learn.microsoft.com/en-us/office/vba/api/visio.visfieldformats
fn fieldpicture(code: f64) -> Result<Value, Error> {
    if !code.is_finite() || !(0.0..=65535.0).contains(&code) || code.fract() != 0.0 {
        return Err(Error::Num);
    }
    let code = code as u16;
    let picture = match code {
        0..=18 => format!("esc({code})"),
        20 => "ddddd".into(),
        30 => "T".into(),
        37 => "@".into(),
        39 => "@+".into(),
        200 | 202 | 209 => format!("{{<{code}>}}"),
        _ => return Err(Error::Unsupported(format!("FIELDPICTURE({code})"))),
    };
    Ok(Value::Text(picture))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Example;
    impl Context for Example {
        fn resolve(&mut self, name: &str) -> Value {
            match name {
                "Width" => Value::Number(240.0),
                "Height" => Value::Number(40.0),
                _ => Value::Error(Error::Ref),
            }
        }
    }
    fn result(formula: &str) -> Value {
        crate::formula(formula, &mut Example)
    }
    #[test]
    fn microsoft_polyline_and_fieldpicture_examples() {
        assert_eq!(result("POLYLINE(0,0,0,0,0,1,1,1,1,0,0,0)"), Value::Text("POLYLINE(0, 0, 0,0, 0,1, 1,1, 1,0, 0,0)".into()));
        assert_eq!(result("FIELDPICTURE(0)"), Value::Text("esc(0)".into()));
    }
    #[test]
    fn observed_geometry_serialization_uses_evaluated_arguments() {
        // BLDCOR_M.vssx master10 and master23 provide these cached constructor witnesses.
        assert_eq!(
            result("POLYLINE(1,1,Width*0,Height*1,Width*1,Height*1,Width*1,Height*0)"),
            Value::Text("POLYLINE(1, 1, 0,40, 240,40, 240,0)".into())
        );
        assert_eq!(
            result("NURBS(1,3,0,0,0.0011598525330351,-1.9381257339979E-16,0,1,0,0.064882164678499,0,1)"),
            Value::Text("NURBS(1, 3, 0, 0, 0.0011598525330351,-1.9381257339979E-16,0,1, 0,0.064882164678499,0,1)".into())
        );
        assert_eq!(result("NURBS(1,3,1,1,Width/2,Height/2,0,1)"), Value::Text("NURBS(1, 3, 1, 1, 120,20,0,1)".into()));
        for (number, expected) in [(0.0001, "0.0001"), (0.00001, "1E-5"), (1e14, "1E14"), (-0.0, "0"), (1.234567890123456, "1.2345678901235")] {
            assert_eq!(general14(number).unwrap(), expected);
        }
    }
    #[test]
    fn observed_picture_special_cases() {
        for (code, picture) in [(3, "esc(3)"), (20, "ddddd"), (30, "T"), (37, "@"), (39, "@+"), (200, "{<200>}"), (202, "{<202>}"), (209, "{<209>}")]
        {
            assert_eq!(result(&format!("FIELDPICTURE({code})")), Value::Text(picture.into()));
        }
        assert!(matches!(result("FIELDPICTURE(999)"), Value::Error(Error::Unsupported(_))));
    }
    #[test]
    fn hostile_constructor_arguments() {
        for f in ["NURBS()", "NURBS(1,3,0,0)", "NURBS(1,3,0,0,1,2,3)", "POLYLINE(0,0)", "POLYLINE(0,0,1)", "POLYLINE(2,0,1,2)", "FIELDPICTURE()"] {
            assert_eq!(result(f), Value::Error(Error::Value), "{f}");
        }
        assert_eq!(result("NURBS(1,-3,0,0,1,2,0,1)"), Value::Error(Error::Num));
        assert_eq!(result("POLYLINE(0,0,1/0,1)"), Value::Error(Error::Div0));
        assert_eq!(result("FIELDPICTURE(1e300)"), Value::Error(Error::Num));
        assert_eq!(general14(f64::NAN), Err(Error::Num));
        assert_eq!(serialize("POLYLINE", &vec![1.234567890123456; MAX_ARGUMENTS], 2, 2), Err(Error::Limit));
    }
}
