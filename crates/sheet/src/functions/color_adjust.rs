//! Luminosity adjustments in the legacy 240-step and Office percentage colour spaces.
use super::color;
use crate::{Context, Error, Value, eval::eval, parser::Expr};
pub(super) fn call(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Option<Value> {
    if !matches!(name, "TINT" | "SHADE" | "MSOTINT" | "MSOSHADE" | "LUMDIFF") {
        return None;
    }
    Some(calculate(name, args, context, budget, depth).unwrap_or_else(Value::Error))
}
fn calculate(name: &str, args: &[Expr], context: &mut dyn Context, budget: &mut usize, depth: usize) -> Result<Value, Error> {
    let [first, second] = args else { return Err(Error::Value) };
    let a = eval(first, context, budget, depth + 1);
    let b = eval(second, context, budget, depth + 1);
    if let Some(e) = a.error().or_else(|| b.error()) {
        return Err(e);
    }
    if name == "LUMDIFF" {
        return lumdiff(&a, &b, context);
    }
    let amount = b.numeric()?;
    if amount.fract() != 0.0 {
        return Err(Error::Num);
    }
    match name {
        "TINT" => tint(&a, amount, context),
        "SHADE" => shade(&a, amount, context),
        "MSOTINT" => msotint(&a, amount, context),
        _ => msoshade(&a, amount, context),
    }
}
/// Add a signed luminosity amount, clamping the result to the 0 through 240 range.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/tint-function
fn tint(value: &Value, amount: f64, context: &mut dyn Context) -> Result<Value, Error> {
    legacy(value, amount, context)
}
/// Subtract a signed luminosity amount, clamping the result to the 0 through 240 range.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shade-function
fn shade(value: &Value, amount: f64, context: &mut dyn Context) -> Result<Value, Error> {
    legacy(value, -amount, context)
}
fn legacy(value: &Value, amount: f64, context: &mut dyn Context) -> Result<Value, Error> {
    if amount == 0.0 {
        return Ok(color::intern(color::rgb_components(value, context).ok_or(Error::Value)?, None, context));
    }
    let [h, s, l] = color::components(value, context).ok_or(Error::Value)?;
    let values = [Value::Number(h.round().rem_euclid(240.0)), Value::Number(s.round()), Value::Number((l.round() + amount).clamp(0.0, 240.0))];
    color::call("HSL", &values, context).ok_or(Error::Value)
}
/// Positive percentages move luminosity toward white; negative percentages toward black.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/msotint-function
fn msotint(value: &Value, amount: f64, context: &mut dyn Context) -> Result<Value, Error> {
    office(value, amount, context)
}
/// Positive percentages move luminosity toward black; negative percentages toward white.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/msoshade-function
fn msoshade(value: &Value, amount: f64, context: &mut dyn Context) -> Result<Value, Error> {
    office(value, -amount, context)
}
fn office(value: &Value, amount: f64, context: &mut dyn Context) -> Result<Value, Error> {
    let rgb = color::rgb_components(value, context).ok_or(Error::Value)?;
    if amount == 0.0 {
        return Ok(color::intern(rgb, None, context));
    }
    if amount >= 100.0 {
        return Ok(color::intern([255; 3], None, context));
    }
    if amount <= -100.0 {
        return Ok(color::intern([0; 3], None, context));
    }
    // Office works from RGB at full precision, not the rounded 240-step legacy grid.
    let [h, s, l] = color::components(&Value::Color { index: 0, rgb, hsl: None }, context).ok_or(Error::Value)?;
    let h = h / 240.0;
    let s = s / 240.0;
    let l = l / 240.0;
    let amount = amount.clamp(-100.0, 100.0) / 100.0;
    let l = if amount >= 0.0 { l + (1.0 - l) * amount } else { l * (1.0 + amount) };
    let upper = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let lower = 2.0 * l - upper;
    let channel = |h: f64| {
        let h = h.rem_euclid(1.0);
        let c = if h < 1.0 / 6.0 {
            lower + (upper - lower) * h * 6.0
        } else if h < 0.5 {
            upper
        } else if h < 2.0 / 3.0 {
            lower + (upper - lower) * (2.0 / 3.0 - h) * 6.0
        } else {
            lower
        };
        (c * 255.0).clamp(0.0, 255.0) as u8
    };
    let rgb = if s == 0.0 { [(l * 255.0).clamp(0.0, 255.0) as u8; 3] } else { [channel(h + 1.0 / 3.0), channel(h), channel(h - 1.0 / 3.0)] };
    Ok(color::intern(rgb, None, context))
}
/// Signed first-minus-second luminosity difference in the 240-step colour space.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/lumdiff-function
fn lumdiff(a: &Value, b: &Value, context: &dyn Context) -> Result<Value, Error> {
    let a = color::components(a, context).ok_or(Error::Value)?;
    let b = color::components(b, context).ok_or(Error::Value)?;
    Ok(Value::number(a[2].round() - b[2].round()))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Palette;
    impl Context for Palette {
        fn resolve(&mut self, _: &str) -> Value {
            Value::Error(Error::Ref)
        }
        fn color_index(&mut self, _: [u8; 3]) -> Option<u32> {
            Some(24)
        }
    }
    fn rgb(formula: &str) -> Option<[u8; 3]> {
        color::rgb_components(&crate::formula(formula, &mut Palette), &Palette)
    }
    #[test]
    fn additive_luminosity_definition_and_corpus_witnesses() {
        // SOCIAL_DISTANCING_M.vstx master56 Char.Color; NEW_CALNDR_U.vstx document gradient.
        assert_eq!(rgb("TINT(RGB(0,0,0),84)"), Some([89, 89, 89]));
        assert_eq!(rgb("TINT(RGB(255,255,255),-36)"), Some([217, 217, 217]));
        // AZUREOTHER_M.vssx master13 User shading from a white base colour.
        assert_eq!(rgb("SHADE(RGB(255,255,255),40)"), Some([213, 213, 213]));
        assert_eq!(rgb("TINT(2,1e300)"), Some([255, 255, 255]));
        assert_eq!(rgb("SHADE(2,1e300)"), Some([0, 0, 0]));
        assert_eq!(crate::formula("LUMDIFF(1,0)", &mut Palette), Value::Number(240.0));
        assert_eq!(crate::formula("LUMDIFF(0,1)", &mut Palette), Value::Number(-240.0));
    }
    #[test]
    fn observed_office_percentage_adjustments() {
        // HIERARCHICALORGCHART_U.vstx page1 and step_infographic_shapes_u.vssx masters1/21/22.
        for (formula, expected) in [
            ("MSOTINT(RGB(0,0,0),35)", [89, 89, 89]),
            ("MSOTINT(RGB(255,255,255),-25)", [191, 191, 191]),
            ("MSOTINT(RGB(255,255,255),-5)", [242, 242, 242]),
            ("MSOTINT(RGB(255,192,0),40)", [255, 217, 101]),
            ("MSOSHADE(RGB(245,208,223),20)", [229, 133, 172]),
            ("MSOSHADE(RGB(249,196,153),30)", [242, 130, 39]),
            ("MSOSHADE(RGB(255,255,255),20)", [204, 204, 204]),
        ] {
            assert_eq!(rgb(formula), Some(expected), "{formula}");
        }
    }
    #[test]
    fn percentage_zero_and_extremes_preserve_definition() {
        for formula in ["MSOTINT(RGB(255,192,0),0)", "MSOSHADE(RGB(255,192,0),0)", "TINT(RGB(255,192,0),0)", "SHADE(RGB(255,192,0),0)"] {
            assert_eq!(rgb(formula), Some([255, 192, 0]));
        }
        assert_eq!(rgb("MSOTINT(RGB(19,111,227),100)"), Some([255; 3]));
        assert_eq!(rgb("MSOTINT(RGB(19,111,227),-100)"), Some([0; 3]));
    }
    #[test]
    fn hostile_adjustments_are_values() {
        for formula in ["TINT()", "MSOTINT(1)", "SHADE(1,2,3)", "LUMDIFF(999,1)"] {
            assert_eq!(crate::formula(formula, &mut Palette), Value::Error(Error::Value), "{formula}");
        }
        assert_eq!(crate::formula("TINT(1,0.5)", &mut Palette), Value::Error(Error::Num));
        assert_eq!(crate::formula("MSOSHADE(1,1/0)", &mut Palette), Value::Error(Error::Div0));
        assert_eq!(rgb("MSOTINT(2,1e300)"), Some([255, 255, 255]));
        let bad = Value::Color { index: 24, rgb: [0; 3], hsl: Some([0.0, 0.0, f64::NAN]) };
        assert_eq!(lumdiff(&bad, &Value::Number(0.0), &Palette), Ok(Value::Error(Error::Num)));
    }
}
