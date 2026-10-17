//! A small SVG writer for the shared render scene; no external resources.
use crate::scene::{Op, Options, Paint, Scene, text_paths};
use base64::Engine as _;
use misterdiagram_geom::Affine;
use std::fmt::Write as _;
fn matrix(m: Affine) -> String {
    let [a, b, c, d, e, f] = m.as_coeffs();
    format!("matrix({a} {b} {c} {d} {e} {f})")
}
fn path(out: &mut String, p: &kurbo::BezPath, c: misterdiagram_color::Rgba) {
    let _ = write!(out, "<path d=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{}\"/>", p.to_svg(), c.r, c.g, c.b, f64::from(c.a) / 255.);
}
fn image(out: &mut String, pixels: &image::RgbaImage, m: Affine, nearest: bool) -> Result<(), String> {
    let bordered;
    let source = if nearest {
        pixels
    } else {
        bordered = crate::raster::transparent_border(pixels)?;
        &bordered
    };
    let mut bytes = std::io::Cursor::new(Vec::new());
    source.write_to(&mut bytes, image::ImageFormat::Png).map_err(|e| e.to_string())?;
    if !nearest {
        write!(
            out,
            "<g transform=\"{}\"><svg width=\"{}\" height=\"{}\" viewBox=\"2 2 {} {}\" overflow=\"hidden\">",
            matrix(m),
            pixels.width(),
            pixels.height(),
            pixels.width(),
            pixels.height()
        )
        .map_err(|e| e.to_string())?;
    }
    let data = base64::engine::general_purpose::STANDARD.encode(bytes.get_ref());
    write!(
        out,
        "<image image-rendering=\"{}\" width=\"{}\" height=\"{}\" transform=\"{}\" href=\"data:image/png;base64,{data}\"/>",
        if nearest { "optimizeSpeed" } else { "optimizeQuality" },
        source.width(),
        source.height(),
        matrix(if nearest { m } else { Affine::IDENTITY })
    )
    .map_err(|e| e.to_string())?;
    if !nearest {
        out.push_str("</svg></g>");
    }
    Ok(())
}
pub fn render(scene: &Scene, options: Options) -> Result<String, String> {
    let (w, h, view) = scene.canvas(options)?;
    let mut out = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\"><rect width=\"100%\" height=\"100%\" fill=\"white\"/>"
    );
    for (i, op) in scene.ops.iter().enumerate() {
        match op {
            Op::Bounds(_) => {}
            Op::Clip { path: p, even_odd } => {
                write!(
                    out,
                    "<defs><clipPath id=\"c{i}\"><path clip-rule=\"{}\" d=\"{}\"/></clipPath></defs><g clip-path=\"url(#c{i})\">",
                    if *even_odd { "evenodd" } else { "nonzero" },
                    (view * p).to_svg()
                )
                .map_err(|e| e.to_string())?;
            }
            Op::Pop => out.push_str("</g>"),
            Op::Path { path: p, paint } => match paint {
                Paint::Solid(c) => path(&mut out, &(view * p), *c),
                _ => {
                    let (pixels, m) = crate::raster::pattern(paint, view, p)?;
                    write!(out, "<defs><clipPath id=\"p{i}\"><path d=\"{}\"/></clipPath></defs><g clip-path=\"url(#p{i})\">", (view * p).to_svg())
                        .map_err(|e| e.to_string())?;
                    image(&mut out, &pixels, m, true)?;
                    out.push_str("</g>");
                }
            },
            Op::Stroke { path: p, stroke, color } => {
                // Native SVG strokes avoid overlapping offset contours on narrow closed paths.
                // Keep distinct marker-side caps through the shared outline fallback.
                // https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
                if stroke.start_cap != stroke.end_cap {
                    path(&mut out, &crate::scene::stroke_path(p, stroke, view, 96. * options.scale), *color);
                } else {
                    let cap = match stroke.start_cap {
                        kurbo::Cap::Butt => "butt",
                        kurbo::Cap::Round => "round",
                        kurbo::Cap::Square => "square",
                    };
                    let join = match stroke.join {
                        kurbo::Join::Miter => "miter",
                        kurbo::Join::Round => "round",
                        kurbo::Join::Bevel => "bevel",
                    };
                    let dpi = 96. * options.scale;
                    let dashes = stroke.dash_pattern.iter().map(|v| (v * dpi).to_string()).collect::<Vec<_>>().join(" ");
                    write!(out,"<path d=\"{}\" fill=\"none\" stroke=\"#{:02x}{:02x}{:02x}\" stroke-opacity=\"{}\" stroke-width=\"{}\" stroke-linecap=\"{cap}\" stroke-linejoin=\"{join}\" stroke-miterlimit=\"{}\" stroke-dasharray=\"{}\" stroke-dashoffset=\"{}\"/>",(view*p).to_svg(),color.r,color.g,color.b,f64::from(color.a)/255.,stroke.width*dpi,stroke.miter_limit,if dashes.is_empty(){"none"}else{&dashes},stroke.dash_offset*dpi).map_err(|e|e.to_string())?;
                }
            }
            Op::Image { pixels, transform } => image(&mut out, pixels, view * *transform, false)?,
            Op::Text { layout, transform } => {
                for (p, c) in text_paths(layout, view * *transform) {
                    path(&mut out, &p, c)
                }
            }
        }
        if out.len() > 256 * 1024 * 1024 {
            return Err("SVG output limit".into());
        }
    }
    out.push_str("</svg>");
    Ok(out)
}
