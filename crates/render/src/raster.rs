//! CPU path, fill and image rendering.
//! Retargeted from deckcraft crates/render/src/{lib,paint,images}.rs,
//! be14248577b92059eb2b8c0c9460cceffbc1f01d, MIT OR Apache-2.0.
use crate::scene::{MAX_PIXELS, Op, Options, Paint, Scene, text_paths};
use diagramcraft_geom::{Affine, BezPath, Rect};
use std::sync::Arc;
use vello_common::{paint::Image, peniko};
use vello_cpu::{Pixmap, PixmapMut, RenderContext};

pub fn decode(bytes: &[u8]) -> Result<image::RgbaImage, String> {
    if bytes.len() > 128 * 1024 * 1024 {
        return Err("image byte limit".into());
    }
    let header = image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().map_err(|e| e.to_string())?;
    let (width, height) = header.into_dimensions().map_err(|e| e.to_string())?;
    if width == 0 || height == 0 || width > 16384 || height > 16384 || u64::from(width) * u64::from(height) > MAX_PIXELS as u64 {
        return Err("image pixel limit (header)".into());
    }
    let mut reader = image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().map_err(|e| e.to_string())?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(16384);
    limits.max_image_height = Some(16384);
    limits.max_alloc = Some((MAX_PIXELS * 8) as u64);
    reader.limits(limits);
    let image = reader.decode().map_err(|e| e.to_string())?.into_rgba8();
    if u64::from(image.width()) * u64::from(image.height()) > MAX_PIXELS as u64 {
        return Err("image pixel limit".into());
    }
    Ok(image)
}
fn pixmap(img: &image::RgbaImage) -> Result<Arc<Pixmap>, String> {
    let (w, h) = img.dimensions();
    let w = u16::try_from(w).map_err(|_| "image width limit")?;
    let h = u16::try_from(h).map_err(|_| "image height limit")?;
    let data = img
        .pixels()
        .map(|p| {
            let [r, g, b, a] = p.0;
            let m = |c: u8| ((u16::from(c) * u16::from(a) + 127) / 255) as u8;
            vello_cpu::color::PremulRgba8 { r: m(r), g: m(g), b: m(b), a }
        })
        .collect();
    Ok(Arc::new(Pixmap::from_parts(data, w, h)))
}
/// Transparent samples outside the source image participate in the reconstruction filter.
pub(crate) fn transparent_border(pixels: &image::RgbaImage) -> Result<image::RgbaImage, String> {
    let w = pixels.width().checked_add(4).ok_or("image border width overflow")?;
    let h = pixels.height().checked_add(4).ok_or("image border height overflow")?;
    if pixels.width() == 0 || pixels.height() == 0 || u64::from(w) * u64::from(h) > MAX_PIXELS as u64 {
        return Err("image border pixel limit".into());
    }
    let mut output = image::RgbaImage::new(w, h);
    image::imageops::overlay(&mut output, pixels, 2, 2);
    Ok(output)
}
fn draw_image(ctx: &mut RenderContext, pixels: &image::RgbaImage, m: Affine, nearest: bool) -> Result<(), String> {
    let bordered;
    let (source, matrix, bounds) = if nearest {
        (pixels, m, Rect::new(0., 0., f64::from(pixels.width()), f64::from(pixels.height())))
    } else {
        bordered = transparent_border(pixels)?;
        (&bordered, m * Affine::translate((-2., -2.)), Rect::new(2., 2., f64::from(pixels.width()) + 2., f64::from(pixels.height()) + 2.))
    };
    let pm = pixmap(source)?;
    let image = Image {
        image: vello_cpu::ImageSource::Pixmap(pm),
        sampler: peniko::ImageSampler { quality: if nearest { peniko::ImageQuality::Low } else { peniko::ImageQuality::High }, ..Default::default() },
    };
    ctx.set_transform(matrix);
    ctx.set_paint(image);
    ctx.fill_rect(&bounds);
    Ok(())
}
/// Sample the procedural paint in its shape coordinate system into a bounded image.
/// The same image and clip are used in SVG and PDF output.
pub(crate) fn pattern(paint: &Paint, view: Affine, path: &BezPath) -> Result<(image::RgbaImage, Affine), String> {
    use kurbo::Shape as _;
    let Paint::Pattern { fill, gradient, bounds, transform } = paint else {
        return Err("expected pattern".into());
    };
    let b = (view * path).bounding_box();
    let x = b.x0.floor();
    let y = b.y0.floor();
    let w = (b.x1.ceil() - x).max(1.);
    let h = (b.y1.ceil() - y).max(1.);
    if !w.is_finite() || !h.is_finite() || w > 16384. || h > 16384. || w * h > MAX_PIXELS as f64 {
        return Err("paint pixel limit".into());
    }
    let local = (*transform).inverse() * view.inverse();
    if local.as_coeffs().iter().any(|n| !n.is_finite()) {
        return Err("singular paint transform".into());
    }
    let image = image::RgbaImage::from_fn(w as u32, h as u32, |px, py| {
        let p = local * kurbo::Point::new(x + f64::from(px) + 0.5, y + f64::from(py) + 0.5);
        let nx = (p.x - bounds.x0) / bounds.width().max(1e-12);
        let ny = 1. - (p.y - bounds.y0) / bounds.height().max(1e-12);
        let c = if let Some(gradient) = gradient {
            if gradient.radial {
                gradient.at(((1. - nx) * bounds.width()).hypot((1. - ny) * bounds.height()) / bounds.width().hypot(bounds.height()).max(1e-12))
            } else {
                gradient.sample(nx, ny)
            }
        } else {
            fill.sample(nx, ny, (x + f64::from(px)) as i32, (y + f64::from(py)) as i32)
        };
        image::Rgba([c.r, c.g, c.b, c.a])
    });
    Ok((image, Affine::translate((x, y))))
}
pub fn render(scene: &Scene, options: Options) -> Result<image::RgbaImage, String> {
    let (w, h, view) = scene.canvas(options)?;
    let mut ctx = RenderContext::new_with(w, h, vello_cpu::RenderSettings { num_threads: 0, ..Default::default() });
    ctx.set_paint(peniko::Color::WHITE);
    ctx.fill_rect(&Rect::new(0., 0., f64::from(w), f64::from(h)));
    for op in &scene.ops {
        match op {
            Op::Bounds(_) => {}
            Op::Clip { path, even_odd } => {
                ctx.set_transform(view);
                ctx.set_fill_rule(if *even_odd { peniko::Fill::EvenOdd } else { peniko::Fill::NonZero });
                ctx.push_clip_layer(path);
                ctx.set_fill_rule(peniko::Fill::NonZero);
            }
            Op::Pop => ctx.pop_layer(),
            Op::Path { path, paint } => match paint {
                Paint::Solid(c) => {
                    ctx.set_transform(Affine::IDENTITY);
                    ctx.set_paint(peniko::Color::from_rgba8(c.r, c.g, c.b, c.a));
                    ctx.fill_path(&device_path(path, view));
                }
                _ => {
                    ctx.set_transform(view);
                    ctx.push_clip_layer(path);
                    let (pixels, m) = pattern(paint, view, path)?;
                    draw_image(&mut ctx, &pixels, m, true)?;
                    ctx.pop_layer();
                }
            },
            Op::Stroke { path, stroke, color } => {
                let outline = crate::scene::stroke_path(path, stroke, view, 96. * options.scale);
                ctx.set_transform(Affine::IDENTITY);
                ctx.set_paint(peniko::Color::from_rgba8(color.r, color.g, color.b, color.a));
                ctx.fill_path(&device_path(&outline, Affine::IDENTITY));
            }
            Op::Image { pixels, transform } => draw_image(&mut ctx, pixels, view * *transform, false)?,
            Op::Text { layout, transform } => {
                for (path, c) in text_paths(layout, *transform) {
                    ctx.set_transform(view);
                    ctx.set_paint(peniko::Color::from_rgba8(c.r, c.g, c.b, c.a));
                    ctx.fill_path(&path);
                }
            }
        }
    }
    ctx.flush();
    let len = usize::from(w).checked_mul(usize::from(h)).and_then(|n| n.checked_mul(4)).ok_or("pixel allocation overflow")?;
    let mut pixels = vec![0; len];
    let pm = PixmapMut::new(w, h, &mut pixels).ok_or("invalid pixmap")?;
    ctx.render(pm, &mut vello_cpu::Resources::new());
    image::RgbaImage::from_raw(u32::from(w), u32::from(h), pixels).ok_or_else(|| "pixel dimensions disagree".into())
}
/// Eighth-pixel coverage lattice observed in the generated 96 dpi edge profiles.
fn device_path(path: &BezPath, view: Affine) -> BezPath {
    let q = |p: kurbo::Point| {
        let p = view * p;
        kurbo::Point::new((p.x * 8. - 1e-8).ceil() / 8., (p.y * 8. - 1e-8).ceil() / 8.)
    };
    path.elements()
        .iter()
        .map(|e| match *e {
            kurbo::PathEl::MoveTo(p) => kurbo::PathEl::MoveTo(q(p)),
            kurbo::PathEl::LineTo(p) => kurbo::PathEl::LineTo(q(p)),
            kurbo::PathEl::QuadTo(a, b) => kurbo::PathEl::QuadTo(q(a), q(b)),
            kurbo::PathEl::CurveTo(a, b, c) => kurbo::PathEl::CurveTo(q(a), q(b), q(c)),
            kurbo::PathEl::ClosePath => kurbo::PathEl::ClosePath,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::Shape as _;
    #[test]
    fn image_filter_uses_premultiplied_alpha_and_transparent_border() {
        let mut source = image::RgbaImage::new(2, 1);
        source.put_pixel(0, 0, image::Rgba([255, 0, 0, 255]));
        source.put_pixel(1, 0, image::Rgba([0, 0, 255, 0]));
        let scene = Scene {
            warnings: vec![],
            ops: vec![Op::Image { pixels: Arc::new(source), transform: Affine::scale(0.1) }],
            bounds: Rect::new(0., 0., 0.2, 0.1),
            page_size: (0.2, 0.1),
        };
        let rendered = render(&scene, Options::default()).unwrap();
        assert!(rendered.pixels().all(|p| p.0[0] == 255 && p.0[1] == p.0[2]), "transparent blue must not bleed into opaque red");
        assert!(rendered.pixels().any(|p| p.0[1] > 0 && p.0[1] < 255), "filter must retain the alpha fringe");
        assert!(transparent_border(&image::RgbaImage::new(0, 0)).is_err());
    }
    #[test]
    fn hostile_images_and_canvas_limits() {
        for bytes in [&[][..], b"\x89PNG\r\n\x1a\n", b"\xff\xd8\xff"] {
            assert!(decode(bytes).is_err());
        }
        let scene = Scene { warnings: vec![], ops: vec![], bounds: Rect::new(0., 0., 1e6, 1e6), page_size: (1e6, 1e6) };
        assert!(render(&scene, Options::default()).is_err());
        assert!(scene.canvas(Options { scale: f64::NAN, ..Default::default() }).is_err());
        for stroke in [kurbo::Stroke::new(f64::NAN), kurbo::Stroke::new(1.).with_dashes(0., [0., 0.])] {
            let scene = Scene {
                warnings: vec![],
                ops: vec![Op::Stroke { path: Rect::new(0., 0., 1., 1.).to_path(0.01), stroke, color: diagramcraft_color::Rgba::BLACK }],
                bounds: Rect::new(0., 0., 1., 1.),
                page_size: (1., 1.),
            };
            assert!(render(&scene, Options::default()).is_err());
        }
    }
}
