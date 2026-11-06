//! A small SVG writer for the shared render scene; no external resources.
use crate::scene::{Op, Options, Paint, Scene, text_paths};
use base64::Engine as _;
use misterdiagram_geom::Affine;
use std::fmt::Write as _;
mod metatext;
mod text;
use text::text;
fn matrix(m: Affine) -> String {
    let [a, b, c, d, e, f] = m.as_coeffs();
    format!("matrix({a} {b} {c} {d} {e} {f})")
}
fn path(out: &mut String, p: &kurbo::BezPath, c: misterdiagram_color::Rgba) {
    fill_path(out, p, c, false);
}
fn fill_path(out: &mut String, p: &kurbo::BezPath, c: misterdiagram_color::Rgba, even_odd: bool) {
    let rule = if even_odd { r#" fill-rule="evenodd""# } else { "" };
    let _ = write!(out, "<path{rule} d=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{}\"/>", p.to_svg(), c.r, c.g, c.b, f64::from(c.a) / 255.);
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
    let (w, h, view) = if options.fit {
        let (w, h, view) = scene.fit_canvas(options)?;
        let factor = 72. / (96. * options.scale);
        (w * factor, h * factor, Affine::scale(factor) * view)
    } else if options.page_canvas && !options.fit {
        let (w, h, view) = scene.page_canvas(options)?;
        let factor = 72. / (96. * options.scale);
        (w * factor, h * factor, Affine::scale(factor) * view)
    } else {
        let (w, h, view) = scene.svg_canvas(options)?;
        (f64::from(w), f64::from(h), view)
    };
    let dpi = if options.page_canvas || options.fit { 72. } else { 96. * options.scale };
    let pattern_view = if options.fit {
        scene.fit_canvas(options)?.2
    } else if options.page_canvas {
        scene.svg_canvas(options)?.2
    } else {
        view
    };
    // No background element: Visio's SVG export leaves the page transparent
    // (private corpus field/vahegan-2026-10-26 x2.svg, MisterDiagram#101).
    let mut out = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">");
    if options.fit {
        out = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}in\" height=\"{}in\" viewBox=\"0 0 {w} {h}\">", w / 72., h / 72.);
    } else if options.page_canvas {
        // Native SVG retains off-page operations and exposes them when embedded.
        out = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}in\" height=\"{}in\" viewBox=\"0 0 {w} {h}\" overflow=\"visible\">",
            scene.page_size.0, scene.page_size.1
        );
    }
    let mut legacy_index = 0usize;
    let mut skip = 0usize;
    let mut record_index = 0usize;
    for op in &scene.ops {
        if let Op::MetaText(record) = op {
            if !options.text_outlines && !record.outlines_only {
                metatext::write(&mut out, record, view, record_index, options.font_names)?;
                skip = record.outline_ops;
            }
            record_index = record_index.saturating_add(1);
            continue;
        }
        let i = legacy_index;
        legacy_index = legacy_index.saturating_add(1);
        if skip > 0 {
            skip -= 1;
            continue;
        }
        match op {
            Op::Bounds(_) | Op::MetaText(_) => {}
            Op::TextBackground { rect, transform, color } => {
                write!(out, "<rect data-text-background=\"true\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" transform=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{}\"/>",
                    rect.x0, rect.y0, rect.width(), rect.height(), matrix(view * *transform),
                    color.r, color.g, color.b, f64::from(color.a) / 255.).map_err(|e| e.to_string())?;
            }
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
            Op::Path { even_odd, path: p, paint } => match paint {
                Paint::Solid(c) => fill_path(&mut out, &(view * p), *c, *even_odd),
                _ => {
                    let (pixels, m) = crate::raster::pattern(paint, pattern_view, p)?;
                    write!(
                        out,
                        "<defs><clipPath id=\"p{i}\"><path clip-rule=\"{}\" d=\"{}\"/></clipPath></defs><g clip-path=\"url(#p{i})\">",
                        if *even_odd { "evenodd" } else { "nonzero" },
                        (view * p).to_svg()
                    )
                    .map_err(|e| e.to_string())?;
                    let m = if options.page_canvas || options.fit { view * pattern_view.inverse() * m } else { m };
                    image(&mut out, &pixels, m, true)?;
                    out.push_str("</g>");
                }
            },
            Op::HairlineMarker { svg_path, color, .. } => path(&mut out, &(view * svg_path), *color),
            Op::Stroke { path: p, stroke, color } | Op::HairlineStroke { svg_path: p, stroke, color, .. } => {
                let mut stroke = stroke.clone();
                if let Op::HairlineStroke { svg_width, .. } = op {
                    stroke = crate::scene::vector_stroke(&stroke, *svg_width)?;
                }
                // Native SVG strokes avoid overlapping offset contours on narrow closed paths.
                // Keep distinct marker-side caps through the shared outline fallback.
                // https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
                if stroke.start_cap != stroke.end_cap {
                    path(&mut out, &crate::scene::stroke_path(p, &stroke, view, dpi), *color);
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
                    let dash_scale = view.determinant().abs().sqrt();
                    let dashes = stroke.dash_pattern.iter().map(|v| (v * dash_scale).to_string()).collect::<Vec<_>>().join(" ");
                    write!(out,"<path d=\"{}\" fill=\"none\" stroke=\"#{:02x}{:02x}{:02x}\" stroke-opacity=\"{}\" stroke-width=\"{}\" stroke-linecap=\"{cap}\" stroke-linejoin=\"{join}\" stroke-miterlimit=\"{}\" stroke-dasharray=\"{}\" stroke-dashoffset=\"{}\"/>",(view*p).to_svg(),color.r,color.g,color.b,f64::from(color.a)/255.,stroke.width*dpi,stroke.miter_limit,if dashes.is_empty(){"none"}else{&dashes},stroke.dash_offset*dash_scale).map_err(|e|e.to_string())?;
                }
            }
            Op::Image { pixels, transform } | Op::RasterImage { pixels, transform } => image(&mut out, pixels, view * *transform, false)?,
            Op::Text { layout, transform, .. } => {
                if options.text_outlines {
                    for (p, c) in text_paths(layout, view * *transform) {
                        path(&mut out, &p, c)
                    }
                } else {
                    for deco in layout.decos.iter().filter(|d| d.behind) {
                        path(&mut out, &(view * *transform * kurbo::Shape::to_path(&deco.rect, 1e-5)), deco.color);
                    }
                    text(&mut out, layout, view * *transform, options.font_names)?;
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

/// Format-specific diagnostics do not enter the shared scene warning census.
pub fn warnings(scene: &Scene, options: Options) -> Vec<String> {
    if options.text_outlines {
        return Vec::new();
    }
    let mut counts = std::collections::BTreeMap::<(u32, u32), usize>::new();
    for op in &scene.ops {
        if let Op::MetaText(record) = op
            && record.outlines_only
            && let Some(shape) = record.shape
        {
            if let Some(count) = counts.get_mut(&shape) {
                *count = count.saturating_add(1);
            } else if counts.len() < 1000 {
                counts.insert(shape, 1);
            }
        }
    }
    counts
        .into_iter()
        .map(|((_, shape), count)| format!("shape {shape}: {count} metafile text records kept as outlines (symbol font or glyph indexes)"))
        .collect()
}
