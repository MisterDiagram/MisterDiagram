//! PDF path conversion and font embedding retargeted from deckcraft crates/pdf/src/lib.rs,
//! be14248577b92059eb2b8c0c9460cceffbc1f01d, MIT OR Apache-2.0.
//! Cluster text ranges and their test from deckcraft crates/pdf/src/{lib,tests}.rs,
//! dd404af8b53029e80d1ff9a7ede02619fdc7ab8a, MarekSpodBiedry, MIT OR Apache-2.0.
//! Scene paths remain vectors and glyphs remain extractable text.
use crate::scene::{Op, Options, Paint, Scene};
use krilla::{
    geom::{PathBuilder, Point, Size, Transform},
    num::NormalizedF32,
    paint::{Fill, FillRule},
    surface::Surface,
    text::{Font, GlyphId, KrillaGlyph, Tag},
};
use misterdiagram_geom::Affine;
use std::collections::HashMap;
fn tf(a: Affine) -> Transform {
    let [a, b, c, d, e, f] = a.as_coeffs();
    Transform::from_row(a as f32, b as f32, c as f32, d as f32, e as f32, f as f32)
}
fn fill(c: misterdiagram_color::Rgba) -> Fill {
    Fill {
        paint: krilla::color::rgb::Color::new(c.r, c.g, c.b).into(),
        opacity: NormalizedF32::new(f32::from(c.a) / 255.).unwrap_or(NormalizedF32::ONE),
        ..Default::default()
    }
}
fn path(p: &kurbo::BezPath) -> Option<krilla::geom::Path> {
    let mut b = PathBuilder::new();
    for el in p.elements() {
        match *el {
            kurbo::PathEl::MoveTo(p) => b.move_to(p.x as f32, p.y as f32),
            kurbo::PathEl::LineTo(p) => b.line_to(p.x as f32, p.y as f32),
            kurbo::PathEl::QuadTo(a, p) => b.quad_to(a.x as f32, a.y as f32, p.x as f32, p.y as f32),
            kurbo::PathEl::CurveTo(a, c, p) => b.cubic_to(a.x as f32, a.y as f32, c.x as f32, c.y as f32, p.x as f32, p.y as f32),
            kurbo::PathEl::ClosePath => b.close(),
        }
    }
    b.finish()
}
fn image(surface: &mut Surface, pixels: &image::RgbaImage, m: Affine) -> Result<(), String> {
    let size = Size::from_wh(pixels.width() as f32, pixels.height() as f32).ok_or("invalid PDF image dimensions")?;
    surface.push_transform(&tf(m));
    surface.draw_image(krilla::image::Image::from_rgba8(pixels.as_raw().clone(), pixels.width(), pixels.height()), size);
    surface.pop();
    Ok(())
}
pub fn render(scenes: &[Scene], options: Options) -> Result<Vec<u8>, String> {
    if scenes.is_empty() || scenes.len() > 4096 {
        return Err("PDF page count limit".into());
    }
    let mut pdf = krilla::Document::new();
    let mut fonts: HashMap<u32, Font> = HashMap::new();
    for scene in scenes {
        let (width, height, pixel_view) = if options.page_canvas {
            let (_, _, view) = scene.page_canvas(options)?;
            // The native 2021 fixed-format exporter truncates the page to its
            // 192-device-unit grid. Raster resolution settings do not alter it.
            // field/export-canvas-2026-10-31/native-fractional and native-pdf-resolution.
            // https://learn.microsoft.com/en-us/office/vba/api/visio.document.exportasfixedformat
            let extent = |inches: f64| (inches * 192. + 1e-9).floor() * (72. / 192.);
            (extent(scene.page_size.0), extent(scene.page_size.1), view)
        } else {
            let (w, h, view) = scene.canvas(options)?;
            (f64::from(w) * 0.75 / options.scale, f64::from(h) * 0.75 / options.scale, view)
        };
        let pattern_view = if options.page_canvas { scene.canvas(options)?.2 } else { pixel_view };
        let view = Affine::scale(72. / (96. * options.scale)) * pixel_view;
        let size = Size::from_wh(width as f32, height as f32).ok_or("PDF page size")?;
        let mut page = pdf.start_page_with(krilla::page::PageSettings::new(size));
        let mut surface = page.surface();
        for (index, op) in scene.ops.iter().enumerate() {
            if let Some(pen) = scene.pdf_strokes.get(&index) {
                let color = match op {
                    Op::Stroke { color, .. }
                    | Op::HairlineStroke { color, .. }
                    | Op::HairlineMarker { color, .. }
                    | Op::Path { paint: Paint::Solid(color), .. } => *color,
                    _ => return Err("invalid PDF stroke operation".into()),
                };
                let mut outline = crate::scene::stroke_path(&pen.path, &pen.stroke, view, 72.);
                if pen.compound {
                    let mut inner = pen.stroke.clone();
                    inner.width /= 3.;
                    outline.extend(crate::scene::stroke_path(&pen.path, &inner, view, 72.).reverse_subpaths().elements().iter().copied());
                }
                if let Some(p) = path(&outline) {
                    surface.set_fill(Some(fill(color)));
                    surface.draw_path(&p);
                }
                continue;
            }
            match op {
                Op::Bounds(_) => {}
                Op::TextBackground { rect, transform, color } => {
                    if let Some(p) = path(&(view * *transform * kurbo::Shape::to_path(rect, 1e-5))) {
                        surface.set_fill(Some(fill(*color)));
                        surface.draw_path(&p);
                    }
                }
                Op::Clip { path: p, even_odd } => {
                    let p = path(&(view * p)).ok_or("empty PDF clip")?;
                    surface.push_clip_path(&p, &if *even_odd { FillRule::EvenOdd } else { FillRule::NonZero });
                }
                Op::Pop => surface.pop(),
                Op::Path { path: p, paint } => {
                    if let Some(path) = path(&(view * p)) {
                        match paint {
                            Paint::Solid(c) => {
                                surface.set_fill(Some(fill(*c)));
                                surface.draw_path(&path);
                            }
                            _ => {
                                let (pixels, m) = crate::raster::pattern(paint, pattern_view, p)?;
                                surface.push_clip_path(&path, &FillRule::NonZero);
                                let m = if options.page_canvas {
                                    view * pattern_view.inverse() * m
                                } else {
                                    Affine::scale(72. / (96. * options.scale)) * m
                                };
                                image(&mut surface, &pixels, m)?;
                                surface.pop();
                            }
                        }
                    }
                }
                Op::HairlineMarker { path: p, color, .. } => {
                    if let Some(p) = path(&(view * p)) {
                        surface.set_fill(Some(fill(*color)));
                        surface.draw_path(&p);
                    }
                }
                Op::Stroke { path: p, stroke, color } | Op::HairlineStroke { path: p, stroke, color, .. } => {
                    if let Some(p) = path(&crate::scene::stroke_path(p, stroke, view, 72.)) {
                        surface.set_fill(Some(fill(*color)));
                        surface.draw_path(&p);
                    }
                }
                Op::Image { pixels, transform } => image(&mut surface, pixels, view * *transform)?,
                Op::Text { layout, transform, .. } => {
                    for deco in &layout.decos {
                        if let Some(path) = path(&(view * *transform * kurbo::Shape::to_path(&deco.rect, 1e-5))) {
                            surface.set_fill(Some(fill(deco.color)));
                            surface.draw_path(&path);
                        }
                    }
                    for run in &layout.runs {
                        let Some(&(_, x0, y0)) = run.glyphs.first() else {
                            continue;
                        };
                        if run.size <= 0. {
                            continue;
                        }
                        let id = run.face.id();
                        if let std::collections::hash_map::Entry::Vacant(entry) = fonts.entry(id) {
                            let coords = [(Tag::new(b"wght"), run.face.weight)];
                            let font = Font::new_variable(
                                run.face.data().to_vec().into(),
                                run.face.index(),
                                if run.face.is_variable() { &coords } else { &[] },
                            )
                            .ok_or("PDF font rejected")?;
                            entry.insert(font);
                        }
                        let font = fonts.get(&id).ok_or("missing PDF font")?.clone();
                        let text = &run.text;
                        let ranges = glyph_text_ranges(text, run.chars, &run.cells, run.glyphs.len());
                        let glyphs: Vec<_> = run
                            .glyphs
                            .iter()
                            .zip(ranges)
                            .enumerate()
                            .map(|(i, (g, range))| {
                                let next = run.glyphs.get(i + 1).map_or(g.1 + run.face.advance(g.0) * run.size / run.face.upem.max(1.), |n| n.1);
                                KrillaGlyph::new(
                                    GlyphId::new(g.0),
                                    ((next - g.1) / run.size) as f32,
                                    0.,
                                    // krilla subtracts this offset from its downward-positive pen.
                                    ((y0 - g.2) / run.size) as f32,
                                    0.,
                                    range,
                                    None,
                                )
                            })
                            .collect();
                        surface.push_transform(&tf(view * *transform * Affine::translate((x0, y0))));
                        let mut c = run.color;
                        c.a = (f64::from(c.a) * run.alpha.clamp(0., 1.)).round() as u8;
                        surface.set_fill(Some(fill(c)));
                        surface.draw_glyphs(Point::from_xy(0., 0.), &glyphs, font, text, run.size as f32, false);
                        surface.pop();
                    }
                }
            }
        }
        surface.finish();
        page.finish();
    }
    pdf.finish().map_err(|e| format!("PDF finish: {e:?}"))
}

/// Byte range in `text` (the run's characters `chars`) of each of `n` glyphs, for the ToUnicode map.
/// `cells` holds each glyph's shaping-cluster head (paragraph character index); a cluster runs to the
/// next head in the run, so a ligature maps to all its characters and every glyph of a cluster shares
/// its range (krilla then writes the text once).
fn glyph_text_ranges(text: &str, chars: (usize, usize), cells: &[(usize, char)], n: usize) -> Vec<std::ops::Range<usize>> {
    let starts: Vec<usize> = text.char_indices().map(|(b, _)| b).collect();
    let byte = |ci: usize| ci.checked_sub(chars.0).and_then(|i| starts.get(i).copied()).unwrap_or(text.len());
    let mut heads: Vec<usize> = cells.iter().map(|c| c.0).collect();
    heads.sort_unstable();
    heads.dedup();
    let end = |ci: usize| heads.get(heads.partition_point(|&h| h <= ci)).copied().unwrap_or(chars.1);
    (0..n).map(|gi| cells.get(gi).map_or(text.len()..text.len(), |&(ci, _)| byte(ci)..byte(end(ci)))).collect()
}

#[cfg(test)]
mod tests {
    use super::glyph_text_ranges;

    #[test]
    fn glyph_ranges_follow_shaping_clusters() {
        // "fl" ligature: its glyph maps to both characters, the rest stay one to one (#164).
        assert_eq!(glyph_text_ranges("flash", (0, 5), &[(0, 'f'), (2, 'a'), (3, 's'), (4, 'h')], 4), vec![0..2, 2..3, 3..4, 4..5]);
        // Two glyphs in one cluster (base and mark) share its range.
        assert_eq!(glyph_text_ranges("ab", (0, 2), &[(0, 'a'), (0, 'a'), (1, 'b')], 3), vec![0..1, 0..1, 1..2]);
        // A run starting mid-paragraph, multi-byte characters in visual (right-to-left) order.
        assert_eq!(glyph_text_ranges("αβγ", (3, 6), &[(5, 'γ'), (4, 'β'), (3, 'α')], 3), vec![4..6, 2..4, 0..2]);
    }
}
