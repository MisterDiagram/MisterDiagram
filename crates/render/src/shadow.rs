//! Bounded outer-shadow masks from the input drawing's own colors and geometry.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/shapeshdwblur-cell-fill-format-section
use crate::scene::{MAX_PIXELS, Op};
use diagramcraft_color::Rgba;
use diagramcraft_geom::{Affine, BezPath};
use kurbo::Shape as _;
use std::sync::Arc;
use vello_common::peniko;
use vello_cpu::{PixmapMut, RenderContext};

/// Rasterize a silhouette with a Gaussian edge and retain its physical extent.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/shapeshdwoffsetx-cell-fill-format-section
pub(crate) fn outer(path: &BezPath, offset: (f64, f64), blur: f64, color: Rgba, dpi: f64) -> Result<Op, String> {
    if [offset.0, offset.1, blur, dpi].iter().any(|n| !n.is_finite()) || blur < 0. || dpi <= 0. || blur * dpi > 512. {
        return Err("shadow coordinate limit".into());
    }
    let b = path.bounding_box().inflate(blur, blur);
    if b.width() <= 0. || b.height() <= 0. {
        return Err("empty shadow extent".into());
    }
    let w = (b.width() * dpi).ceil().max(1.);
    let h = (b.height() * dpi).ceil().max(1.);
    if !w.is_finite() || !h.is_finite() || w > 16384. || h > 16384. || w * h > MAX_PIXELS as f64 {
        return Err("shadow pixel limit".into());
    }
    if w * h * (blur * dpi * 4. + 1.) > 256_000_000. {
        return Err("shadow filter work limit".into());
    }
    let (w, h) = (w as u16, h as u16);
    let view = Affine::scale_non_uniform(f64::from(w) / b.width(), -f64::from(h) / b.height()) * Affine::translate((-b.x0, -b.y1));
    let mut ctx = RenderContext::new_with(w, h, vello_cpu::RenderSettings { num_threads: 0, ..Default::default() });
    ctx.set_transform(view);
    ctx.set_paint(peniko::Color::WHITE);
    ctx.fill_path(path);
    ctx.flush();
    let len = usize::from(w).checked_mul(usize::from(h)).and_then(|n| n.checked_mul(4)).ok_or("shadow allocation overflow")?;
    let mut pixels = vec![0; len];
    let pm = PixmapMut::new(w, h, &mut pixels).ok_or("shadow pixmap")?;
    ctx.render(pm, &mut vello_cpu::Resources::new());
    let mask = image::RgbaImage::from_raw(u32::from(w), u32::from(h), pixels).ok_or("shadow dimensions")?;
    let mut mask = if blur > 0. { image::imageops::blur(&mask, (blur * dpi * 2. / 3.) as f32) } else { mask };
    for pixel in mask.pixels_mut() {
        *pixel = image::Rgba([color.r, color.g, color.b, ((u16::from(pixel.0[3]) * u16::from(color.a) + 127) / 255) as u8]);
    }
    Ok(Op::Image {
        pixels: Arc::new(mask),
        transform: Affine::translate((b.x0 + offset.0, b.y1 + offset.1))
            * Affine::scale_non_uniform(b.width() / f64::from(w), -b.height() / f64::from(h)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use diagramcraft_geom::Rect;
    #[test]
    fn shadow_retains_transparency_and_rejects_hostile_sizes() {
        let path = Rect::new(0., 0., 1., 1.).to_path(0.01);
        let Op::Image { pixels, .. } = outer(&path, (0.1, -0.1), 0.1, Rgba::rgb(20, 40, 60), 96.).unwrap() else { panic!("shadow image") };
        assert!(pixels.pixels().any(|p| p.0[3] > 0 && p.0[3] < 255));
        assert!(outer(&path, (f64::NAN, 0.), 0.1, Rgba::BLACK, 96.).is_err());
        assert!(outer(&path, (0., 0.), 100., Rgba::BLACK, 96.).is_err());
        assert!(outer(&Rect::new(0., 0., 40., 40.).to_path(0.01), (0., 0.), 1., Rgba::BLACK, 96.).is_err());
        assert!(outer(&Rect::new(0., 0., 1e6, 1e6).to_path(0.01), (0., 0.), 0.1, Rgba::BLACK, 96.).is_err());
    }
}
