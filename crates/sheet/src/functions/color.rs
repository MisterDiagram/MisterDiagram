//! ShapeSheet colour expressions retain component precision alongside their palette index.
use crate::{Context, Error, Value};
pub const NAMES: &[&str] = &["RGB", "HSL", "HUE", "SAT", "LUM", "FONT", "FONTTOID", "RED", "GREEN", "BLUE"];
/// The specification fixes indices 0 through 23; custom colours start at 24.
/// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/1fac45bf-ef10-4b29-ada1-4acf47fed340
pub fn default_color(index: u32) -> Option<[u8; 3]> {
    const COLORS: [[u8; 3]; 24] = [
        [0, 0, 0],
        [255, 255, 255],
        [255, 0, 0],
        [0, 255, 0],
        [0, 0, 255],
        [255, 255, 0],
        [255, 0, 255],
        [0, 255, 255],
        [128, 0, 0],
        [0, 128, 0],
        [0, 0, 128],
        [128, 128, 0],
        [128, 0, 128],
        [0, 128, 128],
        [192, 192, 192],
        [230, 230, 230],
        [205, 205, 205],
        [179, 179, 179],
        [154, 154, 154],
        [128, 128, 128],
        [102, 102, 102],
        [77, 77, 77],
        [51, 51, 51],
        [26, 26, 26],
    ];
    COLORS.get(usize::try_from(index).ok()?).copied()
}
/// RGB returns a custom document palette index, creating an entry if necessary.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/rgb-function-visioshapesheet
fn rgb(values: &[Value], context: &mut dyn Context) -> Value {
    let mut rgb = [0; 3];
    for (slot, value) in rgb.iter_mut().zip(values) {
        let n = match value.numeric() {
            Ok(n) => n,
            Err(e) => return Value::Error(e),
        };
        if !(0.0..=255.0).contains(&n) {
            return Value::Error(Error::Num);
        }
        *slot = n.round() as u8;
    }
    intern(rgb, None, context)
}
fn intern(rgb: [u8; 3], hsl: Option<[f64; 3]>, context: &mut dyn Context) -> Value {
    match context.color_index(rgb) {
        Some(index) => Value::Color { index, rgb, hsl },
        None => Value::Error(Error::Limit),
    }
}
/// HSL uses hue in [0,240) and saturation/luminosity in [0,240].
/// https://learn.microsoft.com/en-us/office/client-developer/visio/hsl-function
fn hsl(values: &[Value], context: &mut dyn Context) -> Value {
    let mut hsl = [0.; 3];
    for (slot, value) in hsl.iter_mut().zip(values) {
        *slot = match value.numeric() {
            Ok(v) => v,
            Err(e) => return Value::Error(e),
        }
    }
    let [h, s, l] = hsl;
    if !(0.0..240.0).contains(&h) || !(0.0..=240.0).contains(&s) || !(0.0..=240.0).contains(&l) {
        return Value::Error(Error::Num);
    }
    let h = h / 40.;
    let s = s / 240.;
    let l = l / 240.;
    let c = (1. - (2. * l - 1.).abs()) * s;
    let x = c * (1. - (h % 2. - 1.).abs());
    let m = l - c / 2.;
    let rgb = if h < 1. {
        [c, x, 0.]
    } else if h < 2. {
        [x, c, 0.]
    } else if h < 3. {
        [0., c, x]
    } else if h < 4. {
        [0., x, c]
    } else if h < 5. {
        [x, 0., c]
    } else {
        [c, 0., x]
    };
    intern(rgb.map(|v| ((v + m) * 255.).round().clamp(0., 255.) as u8), Some(hsl), context)
}
fn rgb_components(value: &Value, context: &dyn Context) -> Option<[u8; 3]> {
    if let Value::Color { rgb, .. } = value {
        return Some(*rgb);
    }
    let i = value.numeric().ok()?;
    if !(0.0..=f64::from(u32::MAX)).contains(&i) || i.fract() != 0.0 {
        return None;
    }
    context.color(i as u32).or_else(|| default_color(i as u32))
}
/// Extract a red component, or zero for an invalid colour index.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/red-function
fn red(v: &Value, c: &dyn Context) -> Value {
    Value::Number(f64::from(rgb_components(v, c).map_or(0, |v| v[0])))
}
/// Extract a green component, or zero for an invalid colour index.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/green-function
fn green(v: &Value, c: &dyn Context) -> Value {
    Value::Number(f64::from(rgb_components(v, c).map_or(0, |v| v[1])))
}
/// Extract a blue component, or zero for an invalid colour index.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/blue-function
fn blue(v: &Value, c: &dyn Context) -> Value {
    Value::Number(f64::from(rgb_components(v, c).map_or(0, |v| v[2])))
}
fn components(value: &Value, context: &dyn Context) -> Option<[f64; 3]> {
    let rgb = match value {
        Value::Color { hsl: Some(hsl), .. } => return Some(*hsl),
        Value::Color { rgb, .. } => *rgb,
        _ => {
            let i = value.numeric().ok()?;
            if !(0.0..=f64::from(u32::MAX)).contains(&i) || i.fract() != 0. {
                return None;
            }
            context.color(i as u32).or_else(|| default_color(i as u32))?
        }
    };
    let [r, g, b] = rgb.map(|v| f64::from(v) / 255.);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let c = max - min;
    let l = (max + min) / 2.;
    if c == 0. {
        return Some([0., 0., l * 240.]);
    }
    let h = if max == r {
        ((g - b) / c).rem_euclid(6.)
    } else if max == g {
        (b - r) / c + 2.
    } else {
        (r - g) / c + 4.
    };
    Some([h * 40., c / (1. - (2. * l - 1.).abs()) * 240., l * 240.])
}
/// HUE extracts hue, returning zero for an invalid colour index.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/hue-function
fn hue(v: &Value, c: &dyn Context) -> Value {
    Value::number(components(v, c).map_or(0., |v| v[0]).round().rem_euclid(240.0))
}
/// SAT extracts saturation, returning zero for an invalid colour index.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/sat-function
fn sat(v: &Value, c: &dyn Context) -> Value {
    Value::number(components(v, c).map_or(0., |v| v[1]).round())
}
/// LUM extracts luminosity, returning zero for an invalid colour index.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/lum-function
fn lum(v: &Value, c: &dyn Context) -> Value {
    Value::number(components(v, c).map_or(0., |v| v[2]).round())
}
/// FONT resolves a face name using the document table, independently of installed fonts.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/font-function
fn font(v: &Value, c: &dyn Context) -> Value {
    c.font_index(&v.text()).map(|v| Value::Number(f64::from(v))).unwrap_or(Value::Error(Error::Value))
}
/// FONTTOID is the older spelling of FONT.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/fonttoid-function
fn fonttoid(v: &Value, c: &dyn Context) -> Value {
    font(v, c)
}
pub(super) fn call(name: &str, values: &[Value], context: &mut dyn Context) -> Option<Value> {
    if !NAMES.contains(&name) {
        return None;
    }
    let arity = if matches!(name, "RGB" | "HSL") { 3 } else { 1 };
    if values.len() != arity {
        return Some(Value::Error(Error::Value));
    }
    if let Some(e) = values.iter().find_map(Value::error) {
        return Some(Value::Error(e));
    }
    let Some(first) = values.first() else { return Some(Value::Error(Error::Value)) };
    Some(match name {
        "RED" => red(first, context),
        "GREEN" => green(first, context),
        "BLUE" => blue(first, context),
        "RGB" => rgb(values, context),
        "HSL" => hsl(values, context),
        "HUE" => hue(first, context),
        "SAT" => sat(first, context),
        "LUM" => lum(first, context),
        "FONT" => font(first, context),
        "FONTTOID" => fonttoid(first, context),
        _ => Value::Error(Error::Value),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Palette {
        colors: BTreeMap<u32, [u8; 3]>,
    }
    impl Context for Palette {
        fn resolve(&mut self, _: &str) -> Value {
            Value::Error(Error::Ref)
        }
        fn color(&self, i: u32) -> Option<[u8; 3]> {
            self.colors.get(&i).copied().or_else(|| default_color(i))
        }
        fn color_index(&mut self, rgb: [u8; 3]) -> Option<u32> {
            if let Some(i) = self.colors.iter().find_map(|(i, c)| (*c == rgb).then_some(*i)) {
                return Some(i);
            }
            let i = 24 + self.colors.len() as u32;
            self.colors.insert(i, rgb);
            Some(i)
        }
        fn font_index(&self, name: &str) -> Option<u32> {
            (name == "Calibri").then_some(4)
        }
    }
    #[test]
    fn microsoft_color_and_font_examples() {
        let mut c = Palette::default();
        let rgb = crate::formula("RGB(0,0,255)", &mut c);
        assert_eq!(rgb.numeric(), Ok(24.));
        assert_eq!(c.color(24), Some([0, 0, 255]));
        assert_eq!(crate::formula("HSL(160,240,120)", &mut c).numeric(), Ok(24.));
        for (f, n) in [
            ("RED(22)", 51.),
            ("RED(RGB(10,20,30))", 10.),
            ("GREEN(11)", 128.),
            ("GREEN(RGB(10,20,30))", 20.),
            ("BLUE(13)", 128.),
            ("BLUE(RGB(10,20,30))", 30.),
            ("HUE(7)", 120.),
            ("SAT(8)", 240.),
            ("LUM(6)", 120.),
            ("HUE(HSL(10,20,30))", 10.),
            ("SAT(HSL(10,20,30))", 20.),
            ("LUM(HSL(10,20,30))", 30.),
            ("FONT(\"Calibri\")", 4.),
            ("FONTTOID(\"Calibri\")", 4.),
        ] {
            assert_eq!(crate::formula(f, &mut c).numeric(), Ok(n), "{f}");
        }
        assert_eq!(default_color(23), Some([26, 26, 26]));
        assert_eq!(default_color(24), None);
    }
    #[test]
    fn color_invalid_inputs_and_palette_reuse() {
        let mut c = Palette::default();
        for f in ["RGB(-1,0,0)", "RGB(256,0,0)", "HSL(240,0,0)", "HSL(0,-1,0)"] {
            assert_eq!(crate::formula(f, &mut c), Value::Error(Error::Num), "{f}");
        }
        for f in ["HUE(-1)", "SAT(999)", "LUM(999)"] {
            assert_eq!(crate::formula(f, &mut c), Value::Number(0.), "{f}");
        }
        assert_eq!(crate::formula("FONT(\"absent\")", &mut c), Value::Error(Error::Value));
        assert_eq!(crate::formula("RGB(1,2)", &mut c), Value::Error(Error::Value));
        assert_eq!(crate::formula("RGB(1,2,3)=RGB(1,2,3)", &mut c), Value::Bool(true));
        assert_eq!(c.colors.len(), 1);
    }
}
