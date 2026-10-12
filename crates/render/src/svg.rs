//! A small SVG writer for the shared render scene; no external resources.
use crate::scene::{Op, Options, Paint, Scene, text_paths};
use base64::Engine as _;
use diagramcraft_geom::Affine;
use std::fmt::Write as _;
fn matrix(m: Affine) -> String {
    let [a, b, c, d, e, f] = m.as_coeffs();
    format!("matrix({a} {b} {c} {d} {e} {f})")
}
fn path(out: &mut String, p: &kurbo::BezPath, c: diagramcraft_color::Rgba) {
    let _ = write!(out, "<path d=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{}\"/>", p.to_svg(), c.r, c.g, c.b, f64::from(c.a) / 255.);
}
fn image(out: &mut String, pixels: &image::RgbaImage, m: Affine, nearest: bool) -> Result<(), String> {
    let mut bytes = std::io::Cursor::new(Vec::new());
    pixels.write_to(&mut bytes, image::ImageFormat::Png).map_err(|e| e.to_string())?;
    let data = base64::engine::general_purpose::STANDARD.encode(bytes.get_ref());
    write!(
        out,
        "<image image-rendering=\"{}\" width=\"{}\" height=\"{}\" transform=\"{}\" href=\"data:image/png;base64,{data}\"/>",
        if nearest { "optimizeSpeed" } else { "optimizeQuality" },
        pixels.width(),
        pixels.height(),
        matrix(m)
    )
    .map_err(|e| e.to_string())
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
            Op::Stroke { path: p, stroke, color } => path(&mut out, &crate::scene::stroke_path(p, stroke, view, 96. * options.scale), *color),
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
