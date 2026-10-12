//! PDF path conversion and font embedding retargeted from deckcraft crates/pdf/src/lib.rs,
//! be14248577b92059eb2b8c0c9460cceffbc1f01d, MIT OR Apache-2.0.
//! Scene paths remain vectors and glyphs remain extractable text.
use crate::scene::{Op, Options, Paint, Scene};
use diagramcraft_geom::Affine;
use krilla::{
    geom::{PathBuilder, Point, Size, Transform},
    num::NormalizedF32,
    paint::{Fill, FillRule},
    surface::Surface,
    text::{Font, GlyphId, KrillaGlyph, Tag},
};
use std::collections::HashMap;
fn tf(a: Affine) -> Transform {
    let [a, b, c, d, e, f] = a.as_coeffs();
    Transform::from_row(a as f32, b as f32, c as f32, d as f32, e as f32, f as f32)
}
fn fill(c: diagramcraft_color::Rgba) -> Fill {
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
        let (w, h, pixel_view) = scene.canvas(options)?;
        let view = Affine::scale(72. / (96. * options.scale)) * pixel_view;
        let size = Size::from_wh(f32::from(w) * 0.75 / options.scale as f32, f32::from(h) * 0.75 / options.scale as f32).ok_or("PDF page size")?;
        let mut page = pdf.start_page_with(krilla::page::PageSettings::new(size));
        let mut surface = page.surface();
        for op in &scene.ops {
            match op {
                Op::Bounds(_) => {}
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
                                let (pixels, m) = crate::raster::pattern(paint, pixel_view, p)?;
                                surface.push_clip_path(&path, &FillRule::NonZero);
                                image(&mut surface, &pixels, Affine::scale(72. / (96. * options.scale)) * m)?;
                                surface.pop();
                            }
                        }
                    }
                }
                Op::Image { pixels, transform } => image(&mut surface, pixels, view * *transform)?,
                Op::Text { layout, transform } => {
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
                        let text: String = run.cells.iter().map(|(_, c)| *c).collect();
                        let starts: Vec<usize> = text.char_indices().map(|(b, _)| b).chain(std::iter::once(text.len())).collect();
                        let single = starts.len().saturating_sub(1) == run.glyphs.len();
                        let glyphs: Vec<_> = run
                            .glyphs
                            .iter()
                            .enumerate()
                            .map(|(i, g)| {
                                let next = run.glyphs.get(i + 1).map_or(g.1 + run.face.advance(g.0) * run.size / run.face.upem.max(1.), |n| n.1);
                                let range = if single {
                                    starts.get(i).copied().unwrap_or(0)..starts.get(i + 1).copied().unwrap_or(text.len())
                                } else if i == 0 {
                                    0..text.len()
                                } else {
                                    text.len()..text.len()
                                };
                                KrillaGlyph::new(
                                    GlyphId::new(g.0),
                                    ((next - g.1) / run.size) as f32,
                                    0.,
                                    ((g.2 - y0) / run.size) as f32,
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
                        surface.draw_glyphs(Point::from_xy(0., 0.), &glyphs, font, &text, run.size as f32, false);
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
