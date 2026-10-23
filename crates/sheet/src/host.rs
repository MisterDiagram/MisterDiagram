//! Interfaces for document services implemented above the formula layer.
use crate::{Context, Error, Value};

/// Theme services consume evaluated selectors through the formula context.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function
pub trait ThemeHost {
    fn theme(&self, name: &str, args: &[Value], context: &mut dyn Context) -> Value;
    /// Number of real theme fill stops, excluding the unused ShapeSheet slots.
    /// None means this host does not provide stop provenance.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/fillgradient-section
    fn fill_gradient_stop_count(&self, _context: &mut dyn Context) -> Result<Option<usize>, Error> {
        Ok(None)
    }
}

/// Translate public THEMEVAL names to universal cell names.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function
pub fn theme_cell(name: &str) -> String {
    let name = name.to_ascii_lowercase();
    for prefix in ["fillstop", "linestop"] {
        if let Some(tail) = name.strip_prefix(prefix) {
            let end = tail.find(|c: char| !c.is_ascii_digit()).unwrap_or(tail.len());
            if let Some(index) = tail.get(..end).and_then(|s| s.parse::<u32>().ok()).filter(|n| (1..=10).contains(n)) {
                let column = match tail.get(end..).unwrap_or("") {
                    "color" => "color",
                    "transparency" => "colortrans",
                    "position" => "position",
                    _ => "",
                };
                if !column.is_empty() {
                    return format!(
                        "{}gradientstops.gradientstop{column}{}",
                        prefix.strip_suffix("stop").unwrap_or(""),
                        if index == 1 { String::new() } else { format!("[{index}]") }
                    );
                }
            }
        }
    }
    let name = name.strip_prefix("connector").map(|s| format!("line{s}")).unwrap_or(name);
    match name.as_str() {
        "fillcolor" => "fillforegnd",
        "fillcolor2" => "fillbkgnd",
        "filltransparency" => "fillforegndtrans",
        "filltransparencybkgnd" => "fillbkgndtrans",
        "linebegin" => "beginarrow",
        "lineend" => "endarrow",
        "linebeginsize" => "beginarrowsize",
        "lineendsize" => "endarrowsize",
        "linecompoundtype" => "compoundtype",
        "linetransparency" => "linecolortrans",
        "linerounding" => "rounding",
        "latinfont" => "char.font",
        "asianfont" => "char.asianfont",
        "complexfont" => "char.complexscriptfont",
        "textcolor" => "char.color",
        "textstyle" => "char.style",
        "shadowcolor" => "shdwforegnd",
        "shadowtransparency" => "shdwforegndtrans",
        "shadowpattern" => "shdwpattern",
        "shadowtype" | "shadowstyle" => "shapeshdwtype",
        "shadowmagnification" => "shapeshdwscalefactor",
        "shadowblur" => "shapeshdwblur",
        "shadowxoffset" => "shapeshdwoffsetx",
        "shadowyoffset" => "shapeshdwoffsety",
        "shadowdirection" => "shapeshdwobliqueangle",
        "glowtransparency" => "glowcolortrans",
        _ => &name,
    }
    .into()
}

/// No-theme values come from the host's blank-document baseline, never the queried cell's cache.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function
pub fn no_theme(name: &str, args: &[Value], context: &mut dyn Context) -> Value {
    if name == "THEMEPROP" {
        return if args.first().is_some_and(|v| matches!(v.text().to_ascii_lowercase().as_str(), "embellishment" | "multiformat")) {
            Value::Number(0.)
        } else {
            Value::Error(Error::Value)
        };
    }
    if name == "THEMECBV" {
        return args.first().cloned().unwrap_or(Value::Error(Error::Value));
    }
    if let Some(default) = args.get(1) {
        return default.clone();
    }
    let name = args.first().map(Value::text).unwrap_or_else(|| context.host_cell());
    match name.to_ascii_lowercase().as_str() {
        "backgroundcolor" | "light" | "2" => {
            let rgb = [255, 255, 255];
            return context.color_index(rgb).map(|index| Value::Color { index, rgb, hsl: None }).unwrap_or(Value::Error(Error::Limit));
        }
        "dark" | "1" => {
            let rgb = [0, 0, 0];
            return context.color_index(rgb).map(|index| Value::Color { index, rgb, hsl: None }).unwrap_or(Value::Error(Error::Limit));
        }
        "accentcolor" | "3" => return theme_rgb([0xc0, 0x50, 0x46], context),
        "accentcolor2" | "4" => return theme_rgb([0x9d, 0xbb, 0x61], context),
        "accentcolor3" | "5" => return theme_rgb([0xab, 0x9a, 0xc0], context),
        "accentcolor4" | "6" => return theme_rgb([0x4b, 0xac, 0xc6], context),
        "accentcolor5" | "7" => return theme_rgb([0xf5, 0x9d, 0x56], context),
        "accentcolor6" | "8" => return theme_rgb([0xff, 0xc0, 0x00], context),
        _ => {}
    }
    context.theme_default(&theme_cell(&name))
}

/// Scalar no-theme function constants, not a bundled theme asset.
/// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/4f684ea2-7ae3-40d7-8956-b0a7a5cf2a26
fn theme_rgb(rgb: [u8; 3], context: &mut dyn Context) -> Value {
    context.color_index(rgb).map(|index| Value::Color { index, rgb, hsl: None }).unwrap_or(Value::Error(Error::Limit))
}
