//! Numeric functions operate on internal units: inches, radians and days.
use crate::{Error, Value};
/// ABS uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/abs-function-visioshapesheet
fn abs(x: f64) -> f64 {
    x.abs()
}
/// ACOS uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/acos-function
fn acos(x: f64) -> f64 {
    x.acos()
}
/// ASIN uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/asin-function
fn asin(x: f64) -> f64 {
    x.asin()
}
/// ATAN uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/atan-function
fn atan(x: f64) -> f64 {
    x.atan()
}
/// COS uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cos-function-visioshapesheet
fn cos(x: f64) -> f64 {
    x.cos()
}
/// COSH uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cosh-function
fn cosh(x: f64) -> f64 {
    x.cosh()
}
/// SIN uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/sin-function-visioshapesheet
fn sin(x: f64) -> f64 {
    x.sin()
}
/// SINH uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/sinh-function
fn sinh(x: f64) -> f64 {
    x.sinh()
}
/// TAN uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/tan-function-visioshapesheet
fn tan(x: f64) -> f64 {
    x.tan()
}
/// TANH uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/tanh-function
fn tanh(x: f64) -> f64 {
    x.tanh()
}
/// SQRT uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/sqrt-function
fn sqrt(x: f64) -> f64 {
    x.sqrt()
}
/// INT uses the Microsoft ShapeSheet definition.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/int-function
fn int(x: f64) -> f64 {
    x.floor()
}

/// Angle measured counterclockwise from the x axis; argument order is y, x.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/atan2-function
fn atan2(y: f64, x: f64) -> f64 {
    if y == 0.0 && x == 0.0 { 0.0 } else { y.atan2(x) }
}
/// Constant pi.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pi-function
fn pi() -> f64 {
    std::f64::consts::PI
}
/// The documented POW zero/negative exceptional cases differ from ordinary exponentiation.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pow-function
fn pow(x: f64, y: f64) -> f64 {
    if x == 0.0 && y <= 0.0 || x < 0.0 && y.fract() != 0.0 {
        0.0
    } else if x < 0.0 && y < 0.0 {
        f64::NAN
    } else {
        x.powf(y)
    }
}
/// Boolean inversion of numeric truth.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/not-function
fn not(x: f64) -> Value {
    Value::Bool(x == 0.0)
}
/// Select the smallest value, toward negative infinity.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/min-function
fn min(values: &[f64]) -> f64 {
    values.iter().copied().fold(f64::INFINITY, f64::min)
}
/// Select the largest value, toward positive infinity.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/max-function
fn max(values: &[f64]) -> f64 {
    values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}
/// Decimal rounding, with negative precision counting left of the decimal.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/round-function-visioshapesheet
fn round(x: f64, digits: f64) -> f64 {
    decimal(x, digits, false)
}
/// Truncate at the requested decimal precision.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/trunc-function
fn trunc(x: f64, digits: f64) -> f64 {
    decimal(x, digits, true)
}
fn decimal(x: f64, digits: f64, truncate: bool) -> f64 {
    if digits >= 309.0 {
        return x;
    }
    if digits <= -309.0 {
        return 0.0;
    }
    let scale = 10.0_f64.powf(digits.trunc());
    let n = x * scale;
    if !n.is_finite() {
        return x;
    }
    (if truncate { n.trunc() } else { n.round() }) / scale
}
/// Remainder has the divisor's sign.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/modulus-function
fn modulus(x: f64, y: f64) -> Value {
    if y == 0.0 {
        return Value::Error(Error::Div0);
    }
    let r = x % y;
    Value::number(if r != 0.0 && r.signum() != y.signum() { r + y } else { r })
}
/// Legacy internal remainder uses the magnitude, unlike MODULUS's divisor-signed result.
/// Witness: ADS_M.vssx master7 shape6 Scratch.A1, angle -pi/4, divisor 2pi, result pi/4.
/// Legacy MOD accepts the observed internal spelling as an explicit compatibility alias.
/// Only _MOD is corpus-witnessed; the public reference names MOD without defining its sign rule.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/modulus-function
fn internal_mod(x: f64, y: f64) -> Value {
    if y == 0. { Value::Error(Error::Div0) } else { Value::number((x % y).abs()) }
}
/// CEILING rounds away from zero to a same-sign multiple.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/ceiling-function
fn ceiling(x: f64, multiple: Option<f64>) -> Value {
    multiple_round(x, multiple, true)
}
/// FLOOR rounds toward zero to a same-sign multiple.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/floor-function
fn floor(x: f64, multiple: Option<f64>) -> Value {
    multiple_round(x, multiple, false)
}
fn multiple_round(x: f64, m: Option<f64>, away: bool) -> Value {
    let m = m.unwrap_or(x.signum());
    if x == 0.0 || m == 0.0 {
        return Value::Number(0.0);
    }
    if x.signum() != m.signum() {
        return Value::Error(Error::Num);
    }
    let quotient = x / m;
    Value::number((if away { quotient.ceil() } else { quotient.floor() }) * m)
}
/// Keep a text block upright using a normalized, order-independent limit interval.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/gravity-function
fn gravity(values: &[f64]) -> Value {
    let tau = std::f64::consts::TAU;
    let angle = values.first().copied().unwrap_or(0.).rem_euclid(tau);
    let lo = values.get(1).copied().unwrap_or(std::f64::consts::FRAC_PI_2).rem_euclid(tau);
    let hi = values.get(2).copied().unwrap_or(3. * std::f64::consts::FRAC_PI_2).rem_euclid(tau);
    Value::Number(if angle > lo.min(hi) && angle < lo.max(hi) { std::f64::consts::PI } else { 0. })
}
/// Pattern selection is a read-time numeric sentinel; painting the named master is Phase 2.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/use-function
fn use_pattern(value: &Value) -> Value {
    value.error().map(Value::Error).unwrap_or(Value::Number(254.))
}
/// Add all supplied internal-unit numbers in argument order.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/sum-function
fn sum(values: &[f64]) -> Value {
    Value::number(values.iter().sum())
}
pub(super) fn call(name: &str, values: &[Value]) -> Value {
    let arity = values.len();
    if name == "USE" {
        return match values {
            [value] => use_pattern(value),
            _ => Value::Error(Error::Value),
        };
    }
    let valid = match name {
        "PI" => arity == 0,
        "GRAVITY" => (1..=3).contains(&arity),
        "MIN" | "MAX" => arity > 0,
        "SUM" => arity >= 2,
        "CEILING" | "FLOOR" => (1..=2).contains(&arity),
        "ATAN2" | "POW" | "ROUND" | "TRUNC" | "MODULUS" | "MOD" | "_MOD" => arity == 2,
        "ABS" | "ACOS" | "ASIN" | "ATAN" | "COS" | "COSH" | "SIN" | "SINH" | "TAN" | "TANH" | "SQRT" | "INT" | "NOT" => arity == 1,
        _ => return Value::Error(Error::Unsupported(name.into())),
    };
    if !valid {
        return Value::Error(Error::Value);
    }
    let values: Vec<f64> = match values.iter().map(Value::numeric).collect::<Result<_, _>>() {
        Ok(v) => v,
        Err(e) => return Value::Error(e),
    };
    let x = values.first().copied().unwrap_or(0.0);
    let y = values.get(1).copied().unwrap_or(0.0);
    match name {
        "ABS" => Value::number(abs(x)),
        "ACOS" => Value::number(acos(x)),
        "ASIN" => Value::number(asin(x)),
        "ATAN" => Value::number(atan(x)),
        "COS" => Value::number(cos(x)),
        "COSH" => Value::number(cosh(x)),
        "SIN" => Value::number(sin(x)),
        "SINH" => Value::number(sinh(x)),
        "TAN" => Value::number(tan(x)),
        "TANH" => Value::number(tanh(x)),
        "SQRT" => Value::number(sqrt(x)),
        "INT" => Value::number(int(x)),
        "PI" => Value::number(pi()),
        "GRAVITY" => gravity(&values),
        "ATAN2" => Value::number(atan2(x, y)),
        "POW" => Value::number(pow(x, y)),
        "NOT" => not(x),
        "SUM" => sum(&values),
        "MIN" => Value::number(min(&values)),
        "MAX" => Value::number(max(&values)),
        "ROUND" => Value::number(round(x, y)),
        "TRUNC" => Value::number(trunc(x, y)),
        "MODULUS" => modulus(x, y),
        "MOD" | "_MOD" => internal_mod(x, y),
        "CEILING" => ceiling(x, values.get(1).copied()),
        "FLOOR" => floor(x, values.get(1).copied()),
        _ => Value::Error(Error::Unsupported(name.into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn microsoft_sum_and_use_examples() {
        assert_eq!(call("SUM", &[Value::Number(5.), Value::Number(7.), Value::Number(12.)]), Value::Number(24.));
        assert_eq!(call("USE", &[Value::Text("Railroad Tracks".into())]), Value::Number(254.));
        assert_eq!(call("USE", &[]), Value::Error(Error::Value));
        assert_eq!(call("USE", &[Value::Error(Error::Ref)]), Value::Error(Error::Ref));
        assert_eq!(call("SUM", &[Value::Number(f64::MAX), Value::Number(f64::MAX)]), Value::Error(Error::Num));
    }
    #[test]
    fn observed_internal_mod_has_magnitude_sign() {
        assert_eq!(internal_mod(-std::f64::consts::FRAC_PI_4, std::f64::consts::TAU), Value::Number(std::f64::consts::FRAC_PI_4));
        assert_eq!(internal_mod(5., 2.), Value::Number(1.));
        assert_eq!(call("MOD", &[Value::Number(-5.), Value::Number(2.)]), Value::Number(1.));
        assert_eq!(internal_mod(5., 0.), Value::Error(Error::Div0));
    }
    #[test]
    fn microsoft_gravity_examples() {
        assert_eq!(gravity(&[2.]), Value::Number(std::f64::consts::PI));
        let r = f64::to_radians;
        assert_eq!(gravity(&[r(100.), r(110.), r(290.)]), Value::Number(0.));
        assert_eq!(gravity(&[r(100.), r(290.), r(110.)]), Value::Number(0.));
        assert_eq!(call("GRAVITY", &[]), Value::Error(Error::Value));
        assert_eq!(call("GRAVITY", &[Value::Number(f64::INFINITY)]), Value::Error(Error::Num));
    }
}
