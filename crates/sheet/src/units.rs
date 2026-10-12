//! Universal unit suffixes. Numeric results use inches, radians and days internally.
use crate::{Error, Value};

/// Universal units, implicit flags and compound display units.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-units-of-measure-visio-shapesheet-reference
/// https://learn.microsoft.com/en-us/office/client-developer/visio/about-date-time-and-duration-values
/// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/6f23bcc4-af93-4023-a380-3e78a228e166
pub fn factor(unit: &str) -> Result<f64, Error> {
    if unit.len() > 128 {
        return Err(Error::Limit);
    }
    let clean = unit.trim().trim_matches(['[', ']']).to_ascii_uppercase();
    let unit = clean.split(',').next().unwrap_or("").trim().trim_end_matches('.');
    if let Some((base, power)) = unit.split_once('^') {
        let power: i32 = power.trim().parse().map_err(|_| Error::Value)?;
        if !(-3..=3).contains(&power) || base.contains('^') {
            return Err(Error::Value);
        }
        return Ok(factor(base)?.powi(power));
    }
    if let Some(base) = unit.strip_prefix("SQ.").or_else(|| unit.strip_prefix("SQ ")) {
        return Ok(factor(base.trim())?.powi(2));
    }
    Ok(match unit {
        "" | "IN" | "IN_F" | "DL" | "DP" | "DT" | "DA" | "DE" | "RAD" | "NUM" | "BOOL" | "COLOR" | "DATE" | "ED" => 1.0,
        "FT" | "F_I" | "FEET/INCH" => 12.0,
        "YD" => 36.0,
        "MI" | "MI_F" => 63360.0,
        "NM" => 1852000.0 / 25.4,
        "MM" => 1.0 / 25.4,
        "CM" => 1.0 / 2.54,
        "M" => 1000.0 / 25.4,
        "KM" => 1000000.0 / 25.4,
        "PT" => 1.0 / 72.0,
        "P" | "P_PT" | "PICAPOINTS" => 1.0 / 6.0,
        "D" => 0.3759715104 / 25.4,
        "C" | "C_D" | "CICERO/DIDOT" => 12.0 * 0.3759715104 / 25.4,
        "DEG" | "AD" | "°" => std::f64::consts::PI / 180.0,
        "'" => std::f64::consts::PI / 10800.0,
        "\"" => std::f64::consts::PI / 648000.0,
        "PER" | "%" => 0.01,
        "EW" => 7.0,
        "EH" => 1.0 / 24.0,
        "EM" => 1.0 / 1440.0,
        "ES" => 1.0 / 86400.0,
        "AC" | "ACRE" | "ACRES" => 43560.0 * 144.0,
        "HA" | "HECTARE" | "HECTARES" => 10000.0 * (1000.0_f64 / 25.4).powi(2),
        "CY" | "STR" | "PNT" | "GUID" | "NURBS" | "POLYLINE" | "MULTIDIM" => 1.0,
        _ => return Err(Error::Unsupported(format!("unit {unit}"))),
    })
}
pub fn number(value: f64, unit: Option<&str>) -> Value {
    match factor(unit.unwrap_or("")) {
        Ok(f) => Value::number(value * f),
        Err(e) => Value::Error(e),
    }
}
/// Type tags on a cached V value do not rescale it: V is already in internal units.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
pub fn cached(value: Option<&str>, unit: Option<&str>, error: Option<&str>) -> Value {
    if let Some(e) = error {
        return Value::Error(match e {
            "#DIV/0!" => Error::Div0,
            "#REF!" => Error::Ref,
            _ => Error::Value,
        });
    }
    let s = value.unwrap_or("");
    if s.len() > 65536 {
        return Value::Error(Error::Limit);
    }
    if matches!(unit, Some("STR" | "GUID" | "NURBS" | "POLYLINE")) {
        return Value::Text(s.into());
    }
    if unit == Some("PNT") {
        return cached_point(s).unwrap_or_else(Value::Error);
    }
    if s == "Themed" {
        return Value::Error(Error::Unsupported("uncached theme value".into()));
    }
    if s.is_empty() {
        return Value::Number(0.0);
    }
    match s.parse::<f64>() {
        Ok(n) => Value::number(n),
        Err(_) => Value::Text(s.into()),
    }
}
/// A stored point is a PNT literal, never executable formula input.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pnt-function
fn cached_point(s: &str) -> Result<Value, Error> {
    use crate::parser::{Expr, UnaryOp};
    fn coordinate(mut e: &Expr) -> Result<f64, Error> {
        let mut sign = 1.;
        for _ in 0..128 {
            match e {
                Expr::Number(n, u) => return number(sign * n, u.as_deref()).numeric(),
                Expr::Unary(UnaryOp::Minus, inner) => {
                    sign = -sign;
                    e = inner;
                }
                Expr::Unary(UnaryOp::Plus, inner) => {
                    e = inner;
                }
                _ => return Err(Error::Value),
            }
        }
        Err(Error::Limit)
    }
    let expr = crate::parse(s).map_err(|e| Error::Parse(e.to_string()))?;
    if let Expr::Call { name, args } = expr
        && name == "PNT"
        && args.len() == 2
    {
        return Ok(Value::Point(coordinate(args.first().ok_or(Error::Value)?)?, coordinate(args.get(1).ok_or(Error::Value)?)?));
    }
    Err(Error::Value)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stored_points_are_bounded_literals() {
        assert_eq!(cached(Some("PNT(-0.375IN,-0.16IN)"), Some("PNT"), None), Value::Point(-0.375, -0.16));
        for s in ["PNT(1)", "PNT(SETF(GETREF(Width),9),0)", "PNT(Width,0)", "PNT(1e999,0)"] {
            assert!(matches!(cached(Some(s), Some("PNT"), None), Value::Error(_)));
        }
    }
    #[test]
    fn reference_implicit_units_and_duration() {
        assert_eq!(number(2.0, Some("DL")), Value::Number(2.0));
        assert_eq!(number(2.0, Some("[ft,d]")), Value::Number(24.0));
        assert_eq!(number(168.0, Some("EH")), Value::Number(7.0));
        assert_eq!(number(37.0, Some("%")), Value::Number(0.37));
        assert!((factor("CM").unwrap() * 2.54 - 1.0).abs() < 1e-12);
    }
    #[test]
    fn every_contract_code_is_known() {
        for line in include_str!("../tests/units.txt").lines() {
            let code = line.split('#').next().unwrap().trim();
            if !code.is_empty() {
                assert!(factor(code).is_ok(), "{code}");
            }
        }
    }
    #[test]
    fn string_themed_is_literal_not_theme_sentinel() {
        assert_eq!(cached(Some("Themed"), Some("STR"), None), Value::Text("Themed".into()));
        assert!(matches!(cached(Some("Themed"), Some("COLOR"), None), Value::Error(Error::Unsupported(_))));
    }
    #[test]
    fn hostile_units_and_values() {
        for s in ["unrecognized", "IN^9999", "IN^2^3"] {
            assert!(factor(s).is_err())
        }
        assert_eq!(factor(&"a".repeat(129)), Err(Error::Limit));
        assert_eq!(number(f64::INFINITY, None), Value::Error(Error::Num));
        assert_eq!(cached(Some("NaN"), None, None), Value::Error(Error::Num));
    }
}
