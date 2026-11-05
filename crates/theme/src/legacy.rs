//! Measured legacy THEME scalar properties, independent of modern quick styles.
use misterdiagram_color::Rgba;
use misterdiagram_sheet::{Context, Error, Value};

pub(super) fn handles(args: &[Value]) -> bool {
    args.first().is_some_and(|value| {
        matches!(
            value.text().to_ascii_lowercase().as_str(),
            "fillcolor"
                | "fillcolor2"
                | "linecolor"
                | "textcolor"
                | "fillpattern"
                | "fillgradientenabled"
                | "filltransparency"
                | "fillgradstop1position"
                | "fillgradstop2position"
                | "fillgradstop3position"
        )
    })
}

/// Legacy selectors are User cells, not QuickStyle or DrawingML scheme indexes.
/// Synthetic controls: theme/legacy-fill and theme/no-theme-fill.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/theme-function
pub(super) fn resolve(args: &[Value], cx: &mut dyn Context) -> Result<Value, Error> {
    let property = args.first().ok_or(Error::Value)?.text().to_ascii_lowercase();
    if args.len() != 1 || property.len() > 1024 {
        return Err(Error::Value);
    }
    if let Some(value) = misterdiagram_sheet::host::legacy_line_theme("THEME", args) {
        return Ok(value);
    }
    let color_property = matches!(property.as_str(), "fillcolor" | "fillcolor2" | "linecolor" | "textcolor");
    let selector = cx.theme_selector(if color_property { "User.msvThemeColors" } else { "User.msvThemeEffects" });
    let selector = match selector {
        Value::Error(Error::Ref) => 0.,
        value => value.numeric()?,
    };
    if !selector.is_finite() || selector < 0. || selector.fract() != 0. {
        return Err(Error::Value);
    }
    let rgb = match (selector, property.as_str()) {
        (0., "fillcolor") => Some([150, 175, 207]),
        (0., "fillcolor2") => Some([191, 206, 225]),
        (0., "linecolor") => Some([31, 71, 125]),
        (0. | 36., "textcolor") => Some([0, 0, 0]),
        (36., "fillcolor") => Some([255, 255, 255]),
        (36., "fillcolor2") => Some([240, 240, 240]),
        (36., "linecolor") => Some([64, 64, 64]),
        _ => None,
    };
    if let Some([r, g, b]) = rgb {
        return super::resolve::color_value(Rgba::rgb(r, g, b), cx);
    }
    match (selector, property.as_str()) {
        (0., "fillpattern") => Ok(Value::Number(1.)),
        (16., "fillpattern") => Ok(Value::Number(30.)),
        (0., "fillgradientenabled") => Ok(Value::Bool(false)),
        (16., "fillgradientenabled") => Ok(Value::Bool(true)),
        (0. | 16., "filltransparency" | "fillgradstop1position" | "fillgradstop3position") => Ok(Value::Number(0.)),
        (16., "fillgradstop2position") => Ok(Value::Number(1.)),
        _ => Err(Error::Unsupported(format!("legacy THEME property {property}"))),
    }
}
