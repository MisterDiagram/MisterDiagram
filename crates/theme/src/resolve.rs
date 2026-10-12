//! Dynamic theme selectors and DrawingML style tables.
use diagramcraft_color::{ColorTransform, Rgba, apply};
use diagramcraft_model::Document;
use diagramcraft_opc::xml::{self, El};
use diagramcraft_sheet::{
    Context, Error, Value,
    host::{ThemeHost, no_theme, theme_cell},
};

#[derive(Default)]
pub struct Themes {
    themes: Vec<xml::Doc>,
}
impl Themes {
    /// Read theme parts without importing any built-in visual theme into the program.
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/43361fc1-8acc-410f-a853-c36ff053682d
    pub fn from_document(document: &Document) -> Result<Self, Error> {
        let mut themes = Self::default();
        let mut bytes = 0usize;
        for (name, part) in &document.parts {
            if !name.starts_with("visio/theme/") || !name.ends_with(".xml") {
                continue;
            }
            bytes = bytes.checked_add(part.bytes.len()).ok_or(Error::Limit)?;
            if themes.themes.len() >= 256 || bytes > 32 * 1024 * 1024 {
                return Err(Error::Limit);
            }
            themes.themes.push(Self::parse(&part.bytes)?);
        }
        Ok(themes)
    }
    fn parse(bytes: &[u8]) -> Result<xml::Doc, Error> {
        if bytes.len() > 4 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        let doc = xml::parse(bytes).map_err(|e| Error::Parse(e.to_string()))?;
        if !doc.root.is("theme") {
            return Err(Error::Value);
        }
        Ok(doc)
    }
    /// Adjust low-contrast quick-style colors against the theme background.
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/68bb0221-d8a1-476e-a132-8c60a49cea63
    fn contrast_variant(&self, name: &str, args: &[Value], value: Value, cx: &mut dyn Context) -> Result<Value, Error> {
        if !matches!(name, "THEMEVAL" | "THEME") {
            return Ok(value);
        }
        let cell = theme_cell(&args.first().map(Value::text).unwrap_or_else(|| cx.host_cell()));
        let bit = match cell.as_str() {
            "char.color" => 2,
            "linecolor" => 4,
            "fillforegnd" => 8,
            _ => return Ok(value),
        };
        let flags = selector(cx, "QuickStyleVariation")?;
        if flags & bit == 0 {
            return Ok(value);
        }
        fn rgb(value: &Value, cx: &dyn Context) -> Option<Rgba> {
            match value {
                Value::Color { rgb: [r, g, b], .. } => Some(Rgba::rgb(*r, *g, *b)),
                Value::Number(n) if (0. ..=f64::from(u32::MAX)).contains(n) && n.fract() == 0. => {
                    cx.color(*n as u32).map(|[r, g, b]| Rgba::rgb(r, g, b))
                }
                _ => None,
            }
        }
        let Some(current) = rgb(&value, cx) else {
            return Ok(value);
        };
        let background = self.resolve("THEMEVAL", &[Value::Text("BackgroundColor".into())], cx)?;
        let Some(background) = rgb(&background, cx) else {
            return Ok(value);
        };
        let lum = current.to_hsl().2;
        let bg = background.to_hsl().2;
        if (lum - bg).abs() >= 0.1666 {
            return Ok(value);
        }
        if bg <= 0.7292 {
            return color_value(Rgba::WHITE, cx);
        }
        let mut best = value;
        let mut difference = 0.;
        let candidates = if bit == 2 { &["TextColor", "FillColor", "LineColor"][..] } else { &["FillColor", "LineColor"][..] };
        for candidate in candidates {
            let value = self.resolve("THEMEVAL", &[Value::Text((*candidate).into())], cx)?;
            if let Some(c) = rgb(&value, cx) {
                let distance = (lum - c.to_hsl().2).abs();
                if distance > difference {
                    best = value;
                    difference = distance;
                }
            }
        }
        Ok(best)
    }
    fn component(&self, kind: &str, id: u32) -> Option<(&El, &El)> {
        if id == 0 {
            return None;
        }
        self.themes.iter().find_map(|doc| {
            let component = doc.root.find(kind)?;
            let ids = match kind {
                "fmtScheme" => doc.root.find("fmtSchemeEx"),
                "fmtConnectorScheme" => doc.root.find("fmtConnectorSchemeEx"),
                _ => Some(component),
            }?;
            (ids.find("schemeID")?.u32("schemeEnum")? == id).then_some((&doc.root, component))
        })
    }
    /// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/bd72b8b3-fbf7-4463-ad23-cd8a5cc4acfd
    fn property(&self, args: &[Value], cx: &mut dyn Context) -> Result<Value, Error> {
        let key = args.first().ok_or(Error::Value)?.text().to_ascii_lowercase();
        if !matches!(key.as_str(), "embellishment" | "multiformat") {
            return Err(Error::Value);
        }
        for name in ["ThemeIndex", "ColorSchemeIndex", "EffectSchemeIndex", "ConnectorSchemeIndex", "FontSchemeIndex"] {
            if selector(cx, name)? == 0 {
                return Ok(Value::Number(0.));
            }
        }
        let colors = selector(cx, "ColorSchemeIndex")?;
        let effects = selector(cx, "EffectSchemeIndex")?;
        let (root, _) = self.component("fmtScheme", effects).ok_or(Error::Ref)?;
        let value = if key == "embellishment" {
            root.find("variationStyleSchemeLst")
                .and_then(|e| e.elements().nth(selector(cx, "VariationStyleIndex").ok()? as usize))
                .and_then(|e| e.u32("embellishment"))
        } else {
            self.component("clrScheme", colors)
                .and_then(|(_, c)| c.find("variationClrSchemeLst"))
                .and_then(|e| e.elements().nth(selector(cx, "VariationColorIndex").ok()? as usize))
                .map(|e| u32::from(e.bool("monotone").unwrap_or(false)))
        };
        value.map(|n| Value::Number(f64::from(n))).ok_or(Error::Ref)
    }
    fn resolve(&self, name: &str, args: &[Value], cx: &mut dyn Context) -> Result<Value, Error> {
        if name == "THEMEPROP" {
            return self.property(args, cx);
        }
        let requested = if name == "THEMECBV" { "FillColor".into() } else { args.first().map(Value::text).unwrap_or_else(|| cx.host_cell()) };
        if requested.len() > 1024 {
            return Err(Error::Limit);
        }
        let mut cell = theme_cell(&requested);
        if cell.starts_with("char.")
            && let Some((base, _)) = cell.split_once('[')
        {
            cell = base.to_owned();
        }
        let colors_id = selector(cx, "ColorSchemeIndex")?;
        let font = cell.starts_with("char.");
        let line = cell.starts_with("line")
            || matches!(cell.as_str(), "beginarrow" | "endarrow" | "beginarrowsize" | "endarrowsize" | "rounding" | "compoundtype");
        let fill = cell.starts_with("fill") || matches!(cell.as_str(), "rotategradientwithshape" | "usegroupgradient");
        let one_d = cx.resolve("OneD").numeric().is_ok_and(|v| v != 0.);
        let quick_type = selector(cx, "QuickStyleType")?;
        let connector = if quick_type == 1 {
            false
        } else if quick_type == 2 {
            true
        } else {
            one_d
        };
        let scheme_name = if font {
            "FontSchemeIndex"
        } else if connector {
            "ConnectorSchemeIndex"
        } else {
            "EffectSchemeIndex"
        };
        let scheme_id = selector(cx, scheme_name)?;
        if colors_id == 0 && scheme_id == 0 {
            return Ok(no_theme(name, args, cx));
        }
        let color_component = self.component("clrScheme", colors_id);
        let variant = selector(cx, "VariationColorIndex")?;
        let direct = match requested.to_ascii_lowercase().as_str() {
            "variantcolor1" => Some(100),
            "variantcolor2" => Some(101),
            "variantcolor3" => Some(102),
            "variantcolor4" => Some(103),
            "variantcolor5" => Some(104),
            "variantcolor6" => Some(105),
            "variantcolor7" => Some(106),
            "1" | "dark" => Some(0),
            "2" | "light" => Some(1),
            "3" | "accentcolor" => Some(2),
            "4" | "accentcolor2" => Some(3),
            "5" | "accentcolor3" => Some(4),
            "6" | "accentcolor4" => Some(5),
            "7" | "accentcolor5" => Some(6),
            "8" | "accentcolor6" => Some(7),
            "backgroundcolor" => Some(8),
            _ => None,
        };
        let direct = direct.or_else(|| requested.parse::<u32>().ok().filter(|n| (100..=106).contains(n)));
        if let Some(index) = direct {
            let (_, colors) = color_component.ok_or(Error::Ref)?;
            return color_value(quick_color(colors, index, variant)?, cx);
        }
        let color_selector = if font {
            "QuickStyleFontColor"
        } else if line {
            "QuickStyleLineColor"
        } else if fill {
            "QuickStyleFillColor"
        } else {
            "QuickStyleShadowColor"
        };
        let ph = match color_component {
            Some((_, colors)) => quick_color(colors, selector(cx, color_selector)?, variant)?,
            None => Rgba::BLACK,
        };
        let root_and_component = self.component(
            if font {
                "fontScheme"
            } else if connector {
                "fmtConnectorScheme"
            } else {
                "fmtScheme"
            },
            scheme_id,
        );
        let Some((root, component)) = root_and_component else {
            if scheme_id != 0 {
                return Err(Error::Ref);
            }
            return if matches!(cell.as_str(), "linecolor" | "fillforegnd" | "fillbkgnd" | "char.color") && colors_id != 0 {
                color_value(ph, cx)
            } else {
                Ok(no_theme(name, args, cx))
            };
        };
        let matrix_name = if font {
            "QuickStyleFontMatrix"
        } else if line {
            "QuickStyleLineMatrix"
        } else if fill {
            "QuickStyleFillMatrix"
        } else {
            "QuickStyleEffectsMatrix"
        };
        let matrix_attr = if font {
            "fontIdx"
        } else if line {
            "lineIdx"
        } else if fill {
            "fillIdx"
        } else {
            "effectIdx"
        };
        let matrix = matrix_index(root, selector(cx, matrix_name)?, selector(cx, "VariationStyleIndex")?, matrix_attr)?;
        if matrix == 0 {
            return Ok(no_theme(name, args, cx));
        }
        let index = usize::try_from(matrix - 1).map_err(|_| Error::Limit)?;
        let colors = color_component.map(|(_, c)| c);
        if font {
            let props = root.find(if connector { "connectorFontStyles" } else { "fontStyles" }).and_then(|e| e.elements().nth(index));
            return match cell.as_str() {
                "char.color" => color_value(props.and_then(|p| p.child("color")).map(|e| color(e, colors, ph, 0)).transpose()?.unwrap_or(ph), cx),
                "char.style" => Ok(Value::Number(f64::from(props.and_then(|p| p.u32("style")).unwrap_or(0)))),
                "char.font" | "char.asianfont" | "char.complexscriptfont" => {
                    let script = match cell.as_str() {
                        "char.asianfont" => "ea",
                        "char.complexscriptfont" => "cs",
                        _ => "latin",
                    };
                    let face = component.path(&["minorFont", script]).and_then(|p| p.attr("typeface")).filter(|s| !s.is_empty());
                    match face {
                        Some(face) => cx.font_index(face).map(|n| Value::Number(f64::from(n))).ok_or(Error::Ref),
                        None => Ok(cx.theme_default(&cell)),
                    }
                }
                _ => Err(Error::Unsupported(cell)),
            };
        }
        if line {
            let ln = component.child("lnStyleLst").and_then(|e| e.elements().nth(index)).ok_or(Error::Ref)?;
            if let Some(value) = gradient_value(ln.child("gradFill"), &cell, colors, ph, cx)? {
                return Ok(value);
            }
            let extra = root
                .find(if connector { "fmtConnectorSchemeLineStyles" } else { "fmtSchemeLineStyles" })
                .and_then(|e| e.elements().nth(index))
                .and_then(|e| e.child("lineEx"));
            let attr = match cell.as_str() {
                "beginarrow" => "start",
                "endarrow" => "end",
                "beginarrowsize" => "startSize",
                "endarrowsize" => "endSize",
                "rounding" => "rndg",
                "linepattern" => "pattern",
                _ => "",
            };
            if !attr.is_empty()
                && let Some(value) = extra.and_then(|e| e.f64(attr))
            {
                return Ok(Value::number(if attr == "rndg" { value / 914400. } else { value }));
            }
            return match cell.as_str() {
                "lineweight" => Ok(Value::number(ln.f64("w").unwrap_or(0.) / 914400.)),
                "linecap" => Ok(Value::Number(match ln.attr("cap") {
                    Some("rnd") => 0.,
                    Some("sq") => 1.,
                    _ => 2.,
                })),
                "compoundtype" => Ok(Value::Number(match ln.attr("cmpd") {
                    Some("dbl") => 1.,
                    Some("thickThin") => 2.,
                    Some("thinThick") => 3.,
                    Some("tri") => 4.,
                    _ => 0.,
                })),
                "linecolor" => color_value(color(ln, colors, ph, 0)?, cx),
                "linecolortrans" => Ok(Value::number(1. - f64::from(color(ln, colors, ph, 0)?.a) / 255.)),
                "linegradientenabled" => Ok(Value::Bool(ln.child("gradFill").is_some())),
                _ => Ok(cx.theme_default(&cell)),
            };
        }
        if fill {
            let fill = component.child("fillStyleLst").and_then(|e| e.elements().nth(index)).ok_or(Error::Ref)?;
            if let Some(value) = gradient_value(Some(fill).filter(|e| e.is("gradFill")), &cell, colors, ph, cx)? {
                return Ok(value);
            }
            if name == "THEMECBV" {
                let input = args.first().ok_or(Error::Value)?;
                if !fill.is("gradFill") {
                    return Ok(input.clone());
                }
                let stop = args.get(1).ok_or(Error::Value)?.numeric()?;
                if !(1. ..=10.).contains(&stop) || stop.fract() != 0. {
                    return Err(Error::Value);
                }
                let Some(el) = fill.child("gsLst").and_then(|e| e.elements().nth(stop as usize - 1)).and_then(|e| e.elements().next()) else {
                    return Ok(input.clone());
                };
                let rgb = match input {
                    Value::Color { rgb, .. } => *rgb,
                    _ => {
                        let n = input.numeric()?;
                        if !(0. ..=f64::from(u32::MAX)).contains(&n) || n.fract() != 0. {
                            return Err(Error::Value);
                        }
                        cx.color(n as u32).ok_or(Error::Ref)?
                    }
                };
                let mut c = Rgba::rgb(rgb[0], rgb[1], rgb[2]);
                for t in el.elements().take(64) {
                    let n = t.i32("val").ok_or(Error::Value)?;
                    match t.local() {
                        "tint" => c = apply(c, &[ColorTransform::Tint(n)]),
                        "shade" => c = apply(c, &[ColorTransform::Shade(n)]),
                        _ => {}
                    }
                }
                return color_value(c, cx);
            }
            return match cell.as_str() {
                "fillforegnd" | "fillbkgnd" => {
                    let selected =
                        if fill.is("pattFill") { fill.child(if cell == "fillbkgnd" { "bgClr" } else { "fgClr" }).ok_or(Error::Value)? } else { fill };
                    color_value(color(selected, colors, ph, 0)?, cx)
                }
                "fillforegndtrans" | "fillbkgndtrans" => Ok(Value::number(1. - f64::from(color(fill, colors, ph, 0)?.a) / 255.)),
                "fillpattern" => Ok(Value::Number(f64::from(
                    root.find("fillStyles").and_then(|e| e.elements().nth(index)).and_then(|e| e.u32("pattern")).unwrap_or(if fill.is("noFill") {
                        0
                    } else {
                        1
                    }),
                ))),
                "fillgradientenabled" => Ok(Value::Bool(fill.is("gradFill"))),
                "fillgradientangle" => {
                    Ok(Value::number(fill.child("lin").and_then(|e| e.f64("ang")).unwrap_or(0.) / 60000. * std::f64::consts::PI / 180.))
                }
                "rotategradientwithshape" => Ok(Value::Bool(fill.bool("rotWithShape").unwrap_or(true))),
                _ => Ok(cx.theme_default(&cell)),
            };
        }
        let effect = component.child("effectStyleLst").and_then(|e| e.elements().nth(index)).and_then(|e| e.child("effectLst"));
        let shadow = effect.and_then(|e| e.child("outerShdw").or_else(|| e.child("innerShdw")));
        let number = |element: Option<&El>, attr: &str, default: f64| element.and_then(|e| e.f64(attr)).unwrap_or(default);
        let distance = number(shadow, "dist", 0.) / 914400.;
        let direction = number(shadow, "dir", 0.) / 60000. * std::f64::consts::PI / 180.;
        let value = match cell.as_str() {
            "shdwforegnd" => match shadow {
                Some(shadow) => return color_value(color(shadow, colors, ph, 0)?, cx),
                None => return Ok(cx.theme_default(&cell)),
            },
            "shdwforegndtrans" => match shadow {
                Some(shadow) => 1. - f64::from(color(shadow, colors, ph, 0)?.a) / 255.,
                None => 0.,
            },
            "shdwpattern" => {
                if shadow.is_some() {
                    1.
                } else {
                    0.
                }
            }
            "shapeshdwoffsetx" => distance * direction.cos(),
            "shapeshdwoffsety" => -distance * direction.sin(),
            "shapeshdwblur" => number(shadow, "blurRad", 0.) / 914400.,
            "shapeshdwscalefactor" => number(shadow, "sx", 100000.) / 100000.,
            "shapeshdwobliqueangle" => number(shadow, "kx", 0.) / 60000. * std::f64::consts::PI / 180.,
            "softedgessize" => number(effect.and_then(|e| e.child("softEdge")), "rad", 0.) / 914400.,
            "glowsize" => number(effect.and_then(|e| e.child("glow")), "rad", 0.) / 914400.,
            "glowcolor" => match effect.and_then(|e| e.child("glow")) {
                Some(glow) => return color_value(color(glow, colors, ph, 0)?, cx),
                None => return Ok(cx.theme_default(&cell)),
            },
            _ => return Ok(cx.theme_default(&cell)),
        };
        Ok(Value::number(value))
    }
}
impl ThemeHost for Themes {
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function
    fn theme(&self, name: &str, args: &[Value], context: &mut dyn Context) -> Value {
        self.resolve(name, args, context).and_then(|value| self.contrast_variant(name, args, value, context)).unwrap_or_else(Value::Error)
    }
}
/// A theme supplies at most ten stop slots, with omitted slots using blank defaults.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/themeval-function
fn gradient_value(fill: Option<&El>, cell: &str, colors: Option<&El>, ph: Rgba, cx: &mut dyn Context) -> Result<Option<Value>, Error> {
    let Some(column) = cell.strip_prefix("fillgradientstops.gradientstop").or_else(|| cell.strip_prefix("linegradientstops.gradientstop")) else {
        return Ok(None);
    };
    let (column, index) = match column.split_once('[') {
        Some((column, index)) => (column, index.strip_suffix(']').and_then(|s| s.parse::<usize>().ok()).ok_or(Error::Value)?),
        None => (column, 1),
    };
    if !(1..=10).contains(&index) {
        return Err(Error::Value);
    }
    let Some(stop) = fill.and_then(|e| e.child("gsLst")).and_then(|e| e.elements().nth(index - 1)) else {
        return Ok(Some(cx.theme_default(cell)));
    };
    Ok(Some(match column {
        "color" => color_value(color(stop, colors, ph, 0)?, cx)?,
        "colortrans" => Value::number(1. - f64::from(color(stop, colors, ph, 0)?.a) / 255.),
        "position" => Value::number(stop.f64("pos").ok_or(Error::Value)? / 100000.),
        _ => return Err(Error::Value),
    }))
}
fn selector(cx: &mut dyn Context, name: &str) -> Result<u32, Error> {
    let n = cx.theme_selector(name).numeric()?;
    if n < 0. || n > f64::from(u32::MAX) || n.fract() != 0. {
        return Err(Error::Num);
    }
    Ok(n as u32)
}
/// Resolve variant quick-style indexes to one-based style-table positions.
/// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/25689058-b1e7-4d3c-a833-0a4c7180f5f2
fn matrix_index(root: &El, index: u32, variant: u32, attr: &str) -> Result<u32, Error> {
    if index < 100 {
        return Ok(index);
    }
    if index > 103 || variant > 3 {
        return Err(Error::Value);
    }
    root.find("variationStyleSchemeLst")
        .and_then(|e| e.elements().nth(variant as usize))
        .and_then(|e| e.elements().nth((index - 100) as usize))
        .and_then(|e| e.u32(attr))
        .ok_or(Error::Ref)
}
/// Resolve quick colors and the seven slots of a color variant.
/// https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/723529ce-4ac3-416d-b7be-362aa28be341
fn quick_color(colors: &El, index: u32, variant: u32) -> Result<Rgba, Error> {
    let names = ["dk1", "lt1", "accent1", "accent2", "accent3", "accent4", "accent5", "accent6", "bkgnd"];
    let element = if let Some(name) = names.get(index as usize) {
        colors.child(name).or_else(|| colors.find(name))
    } else if (100..=106).contains(&index) && variant < 4 {
        colors
            .find("variationClrSchemeLst")
            .and_then(|e| e.elements().nth(variant as usize))
            .and_then(|e| e.child(&format!("varColor{}", index - 99)))
    } else {
        None
    }
    .ok_or(Error::Ref)?;
    color(element, Some(colors), Rgba::BLACK, 0)
}
fn color_value(c: Rgba, cx: &mut dyn Context) -> Result<Value, Error> {
    let rgb = [c.r, c.g, c.b];
    let index = cx.color_index(rgb).ok_or(Error::Limit)?;
    Ok(Value::Color { index, rgb, hsl: None })
}
fn color(el: &El, scheme: Option<&El>, ph: Rgba, depth: usize) -> Result<Rgba, Error> {
    if depth > 32 {
        return Err(Error::Limit);
    }
    let c = match el.local() {
        "srgbClr" => Rgba::from_hex(el.attr("val").ok_or(Error::Value)?).ok_or(Error::Value)?,
        "sysClr" => Rgba::from_hex(el.attr("lastClr").ok_or(Error::Value)?).ok_or(Error::Value)?,
        "schemeClr" => {
            let name = el.attr("val").ok_or(Error::Value)?;
            if name == "phClr" {
                ph
            } else {
                let scheme = scheme.ok_or(Error::Ref)?;
                color(scheme.child(name).ok_or(Error::Ref)?, Some(scheme), ph, depth + 1)?
            }
        }
        "prstClr" => diagramcraft_color::preset(el.attr("val").ok_or(Error::Value)?).ok_or(Error::Value)?,
        "noFill" => Rgba::TRANSPARENT,
        _ => {
            let child = el
                .elements()
                .find(|c| {
                    matches!(
                        c.local(),
                        "solidFill"
                            | "gradFill"
                            | "pattFill"
                            | "fgClr"
                            | "bgClr"
                            | "srgbClr"
                            | "sysClr"
                            | "schemeClr"
                            | "prstClr"
                            | "gsLst"
                            | "gs"
                            | "noFill"
                    )
                })
                .ok_or(Error::Value)?;
            return color(child, scheme, ph, depth + 1);
        }
    };
    let mut transforms = Vec::new();
    for t in el.elements() {
        if transforms.len() >= 64 {
            return Err(Error::Limit);
        }
        let value = t.i32("val").unwrap_or(0);
        let transform = match t.local() {
            "tint" => ColorTransform::Tint(value),
            "shade" => ColorTransform::Shade(value),
            "lumMod" => ColorTransform::LumMod(value),
            "lumOff" => ColorTransform::LumOff(value),
            "satMod" => ColorTransform::SatMod(value),
            "satOff" => ColorTransform::SatOff(value),
            "hueMod" => ColorTransform::HueMod(value),
            "hueOff" => ColorTransform::HueOff(value),
            "alpha" => ColorTransform::Alpha(value),
            "alphaMod" => ColorTransform::AlphaMod(value),
            "alphaOff" => ColorTransform::AlphaOff(value),
            "comp" => ColorTransform::Comp,
            "inv" => ColorTransform::Inv,
            "gray" => ColorTransform::Gray,
            _ => return Err(Error::Unsupported(t.local().into())),
        };
        transforms.push(transform);
    }
    Ok(apply(c, &transforms))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    struct TestContext {
        cells: BTreeMap<String, f64>,
    }
    impl Context for TestContext {
        fn resolve(&mut self, name: &str) -> Value {
            Value::Number(self.cells.get(name).copied().unwrap_or(0.))
        }
        fn host_cell(&self) -> String {
            "FillForegnd".into()
        }
        fn theme_default(&mut self, _: &str) -> Value {
            Value::Number(1.)
        }
        fn color_index(&mut self, _: [u8; 3]) -> Option<u32> {
            Some(24)
        }
    }
    // Contributor-original numerical fixture, not a shipped visual theme.
    fn theme() -> Themes {
        Themes { themes: vec![Themes::parse(br#"<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:v="http://schemas.microsoft.com/office/visio/2012/theme"><a:themeElements><a:clrScheme><a:dk1><a:srgbClr val="010305"/></a:dk1><a:accent3><a:srgbClr val="214365"/></a:accent3><a:extLst><a:ext><v:schemeID schemeEnum="123"/><v:variationClrSchemeLst><v:variationClrScheme><v:varColor1><a:srgbClr val="426384"/></v:varColor1></v:variationClrScheme></v:variationClrSchemeLst></a:ext></a:extLst></a:clrScheme><a:fmtScheme><a:fillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:fillStyleLst></a:fmtScheme><a:extLst><a:ext><v:fmtSchemeEx><v:schemeID schemeEnum="123"/></v:fmtSchemeEx><v:variationStyleSchemeLst><v:variationStyleScheme><v:varStyle fillIdx="1"/></v:variationStyleScheme></v:variationStyleSchemeLst></a:ext></a:extLst></a:themeElements></a:theme>"#).unwrap()] }
    }
    #[test]
    fn microsoft_themeval_examples_and_variant_selectors() {
        let theme = theme();
        let mut cx = TestContext {
            cells: BTreeMap::from([
                ("ColorSchemeIndex".into(), 123.),
                ("EffectSchemeIndex".into(), 123.),
                ("QuickStyleFillColor".into(), 100.),
                ("QuickStyleFillMatrix".into(), 100.),
            ]),
        };
        assert_eq!(theme.theme("THEMEVAL", &[Value::Text("5".into())], &mut cx), Value::Color { index: 24, rgb: [0x21, 0x43, 0x65], hsl: None });
        assert_eq!(theme.theme("THEMEVAL", &[], &mut cx), Value::Color { index: 24, rgb: [0x42, 0x63, 0x84], hsl: None });
        cx.cells.clear();
        assert_eq!(
            theme.theme("THEMEVAL", &[Value::Text("LineWeight".into()), Value::Text("0.7 pt.".into())], &mut cx),
            Value::Text("0.7 pt.".into())
        );
    }
    #[test]
    fn hostile_theme_xml_and_color_cycles_are_bounded() {
        for input in [b"<theme><a>".as_slice(), b"<!DOCTYPE theme><theme/>", b"<theme/><theme/>", b"<theme a='1' a='2'/>"] {
            assert!(Themes::parse(input).is_err());
        }
        assert!(Themes::parse("<theme>".repeat(257).as_bytes()).is_err());
        assert!(Themes::parse(&vec![b' '; 4 * 1024 * 1024 + 1]).is_err());
        let e = xml::parse(b"<clrScheme><dk1><schemeClr val='dk1'/></dk1></clrScheme>").unwrap();
        assert_eq!(quick_color(&e.root, 0, 0), Err(Error::Limit));
        assert_eq!(quick_color(&e.root, u32::MAX, 0), Err(Error::Ref));
    }
}
