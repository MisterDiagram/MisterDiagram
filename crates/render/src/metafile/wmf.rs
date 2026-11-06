//! Retargeted from vectorcraft crates/metafile/src/import/wmf.rs,
//! 806b1a6972288319577d75d097c140f9bf530ce0, MIT OR Apache-2.0.
//! WMF records (MS-WMF) played on a [`Player`]. A placeable header gives the picture's box in
//! logical units and the units an inch; without one, the first window origin and extent are the
//! box, at 1440 units an inch.

use kurbo::{Affine, Point, Rect, Shape as _};

use super::bytes::Reader;
use super::dib::{self, Alpha};
use super::header;
use super::{ArcKind, BrushObj, FontObj, Obj, PenObj, Player};
use super::{Imported, Kind};

/// Records that change nothing VectorCraft keeps (background, raster and mapping details, palettes,
/// escapes).
const IGNORED: &[u16] = &[
    0x0102, 0x0103, 0x0104, 0x0105, 0x0107, 0x0108, 0x0201, 0x020A, 0x020B, 0x020C, 0x020D, 0x020E, 0x020F, 0x0211, 0x0410, 0x0412, 0x0231, 0x0234,
    0x0035, 0x0037, 0x0139, 0x0436, 0x0626,
];

fn point(r: &mut Reader) -> Option<Point> {
    Some(Point::new(f64::from(r.i16()?), f64::from(r.i16()?)))
}

/// A point stored y first.
fn point_yx(r: &mut Reader) -> Option<Point> {
    let y = f64::from(r.i16()?);
    Some(Point::new(f64::from(r.i16()?), y))
}

/// A rectangle stored bottom, right, top, left.
fn rect_brtl(r: &mut Reader) -> Option<Rect> {
    let (b, rt, t, l) = (r.i16()?, r.i16()?, r.i16()?, r.i16()?);
    Some(Rect::new(f64::from(l), f64::from(t), f64::from(rt), f64::from(b)))
}

fn color(r: &mut Reader) -> Option<[u8; 3]> {
    let [red, g, b, _] = r.u32()?.to_le_bytes();
    Some([red, g, b])
}

fn points(r: &mut Reader, n: usize) -> Option<Vec<Point>> {
    if r.rest().len() < n * 4 {
        return None;
    }
    (0..n).map(|_| point(r)).collect()
}

/// ANSI text (read as Latin-1).
fn ansi(b: &[u8]) -> String {
    b.iter().map(|c| char::from(*c)).collect()
}

/// Play bounded records, preserving the same operation/memory failures as EMF.
/// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/
pub(super) fn play(bytes: &[u8]) -> Result<Imported, String> {
    play_budgeted(bytes, super::SceneBudget::default(), super::MAX_NODES)
}
fn play_budgeted(bytes: &[u8], budget: super::SceneBudget, operations: usize) -> Result<Imported, String> {
    play_options(bytes, budget, operations, None)
}
pub(super) fn play_placed(bytes: &[u8], pixel: f64, placement: Affine) -> Result<Imported, String> {
    play_options(bytes, super::SceneBudget::default(), super::MAX_NODES, Some((pixel, placement)))
}
fn play_options(bytes: &[u8], mut budget: super::SceneBudget, operations: usize, pixel: Option<(f64, Affine)>) -> Result<Imported, String> {
    let frame = header::wmf_frame(bytes)?;
    let embedded = super::embedded::read(bytes, frame.start, &mut budget)?;
    if let super::embedded::Embedded::Complete(data) = embedded {
        return super::emf::play_embedded(&data, budget, operations);
    }
    let k = 72. / frame.inch;
    let logical = frame.rect.map(|[l, t, r, b]| Rect::new(l, t, r, b));
    let artboard = logical.map_or(Rect::new(0., 0., 1., 1.), |r| Rect::new(0., 0., r.width().abs() * k, r.height().abs() * k));
    let mut p = Player::new(Kind::Wmf, Affine::scale(k), 25.4 / frame.inch, artboard);
    p.budget = budget;
    p.operation_limit = operations;
    if let Some((width, placement)) = pixel {
        p.wmf_cosmetic_width = Some(width);
        p.wmf_placement = Some(placement);
    }
    if matches!(embedded, super::embedded::Embedded::Incomplete) {
        p.warn(super::embedded::INCOMPLETE);
    }
    p.set_map_mode(8);
    if let Some(r) = logical {
        p.set_window_org(r.x0, r.y0);
        p.set_window_ext(r.width(), r.height());
        p.set_viewport_ext(r.width().abs(), r.height().abs());
    }
    let mut at = frame.start;
    let mut ended = false;
    while let Some((function, body, next)) = header::record(bytes, at) {
        if function == 0 {
            ended = true;
            break;
        }
        if !matches!(function, 0x0213 | 0x0214) {
            p.flush();
        }
        if record(&mut p, function, body).is_none() {
            p.skip();
        }
        if let Some(error) = p.failure.take() {
            return Err(error);
        }
        at = next;
    }
    if !ended {
        p.warn("the file ends early: what it held was imported");
    }
    p.flush();
    if let Some(error) = p.failure.take() {
        return Err(error);
    }
    let mut out = p.finish();
    if logical.is_none() {
        let mut bounds: Option<Rect> = None;
        for op in &out.ops {
            let rect = match op {
                super::Op::Path { path, .. } => path.bounding_box(),
                super::Op::Image { pixels, transform } => {
                    transform.transform_rect_bbox(Rect::new(0., 0., f64::from(pixels.width()), f64::from(pixels.height())))
                }
                _ => continue,
            };
            bounds = Some(bounds.map_or(rect, |b| b.union(rect)));
        }
        if let Some(b) = bounds {
            out.frame = Rect::from_origin_size(b.origin(), (b.width().max(1.), b.height().max(1.)));
        }
    }
    Ok(out)
}

/// An image record's source rectangle (from the bottom for a bottom-up DIB) and destination.
fn blit(p: &mut Player, rop: u32, src: (i16, i16, i16, i16), dest: (i16, i16, i16, i16), packed: Option<&[u8]>) {
    let (xd, yd, wd, hd) = (f64::from(dest.0), f64::from(dest.1), f64::from(dest.2), f64::from(dest.3));
    let d = Rect::new(xd, yd, xd + wd, yd + hd);
    let Some(packed) = packed else { return p.pattern_blit(d, rop) };
    let parts = dib::split(packed);
    let img = parts.ok_or_else(|| "damaged".to_string()).and_then(|(bmi, bits)| dib::decode(bmi, bits, Alpha::Ignore));
    let bottom_up = parts.and_then(|(bmi, _)| bmi.get(8..12)).is_some_and(|h| i32::from_le_bytes([h[0], h[1], h[2], h[3]]) > 0);
    let ih = img.as_ref().map_or(0.0, |i| f64::from(i.height));
    let (xs, ys, ws, hs) = (f64::from(src.0), f64::from(src.1), f64::from(src.2), f64::from(src.3));
    let top = if bottom_up { ih - ys - hs } else { ys };
    p.image(img, Rect::new(xs, top, xs + ws, top + hs), d, rop, 1.0);
}

/// Play one record (`r`: its parameters). `None`: unknown or damaged.
fn record(p: &mut Player, function: u16, body: &[u8]) -> Option<()> {
    let mut r = Reader::new(body);
    match function {
        0x0103 => p.set_map_mode(u32::from(r.u16()?)),
        0x020b | 0x020c | 0x020d | 0x020e | 0x020f | 0x0211 => {
            let pt = point_yx(&mut r)?;
            match function {
                0x020b => p.set_window_org(pt.x, pt.y),
                0x020c => p.set_window_ext(pt.x, pt.y),
                0x020d => p.set_viewport_org(pt.x, pt.y),
                0x020e => p.set_viewport_ext(pt.x, pt.y),
                0x020f => p.set_window_org(p.dc.win_org.x + pt.x, p.dc.win_org.y + pt.y),
                _ => p.set_viewport_org(p.dc.vp_org.x + pt.x, p.dc.vp_org.y + pt.y),
            }
        }
        0x0410 | 0x0412 => {
            let (yd, yn, xd, xn) = (r.i16()?, r.i16()?, r.i16()?, r.i16()?);
            p.scale_ext(function == 0x0410, (f64::from(xn), f64::from(xd)), (f64::from(yn), f64::from(yd)));
        }
        0x001E => p.save(),
        0x0127 => p.restore(i32::from(r.i16()?)),
        0x0106 => p.set_fill_mode(r.u16()? == 1),
        0x0209 => p.set_text_color(color(&mut r)?),
        0x012E => p.set_text_align(u32::from(r.u16()?)),
        0x0214 => p.move_to(point_yx(&mut r)?),
        0x0213 => {
            let pt = point_yx(&mut r)?;
            p.poly_to(&[pt], false);
        }
        0x0324 | 0x0325 => {
            let n = usize::try_from(r.i16()?).ok()?;
            let pts = points(&mut r, n)?;
            p.poly(&pts, function == 0x0324, false);
        }
        0x0538 => {
            let n = usize::from(r.u16()?);
            if r.rest().len() < n * 2 {
                return None;
            }
            let counts: Vec<usize> = (0..n).map(|_| r.u16().map(usize::from)).collect::<Option<_>>()?;
            // Charge aggregate points and contour closures before allocating point vectors
            // or a combined path, rather than after expansion in Player::draw.
            let total = counts.iter().try_fold(n, |total, count| total.checked_add(*count))?;
            if !p.check_expansion(total) {
                return Some(());
            }
            let mut polys = Vec::with_capacity(n);
            for c in counts {
                polys.push(points(&mut r, c)?);
            }
            p.poly_poly(&polys, true);
        }
        0x041B => p.figure(rect_brtl(&mut r)?.abs().to_path(0.01), true),
        0x0418 => p.figure(kurbo::Ellipse::from_rect(rect_brtl(&mut r)?.abs()).to_path(0.01), true),
        0x061C => {
            let (h, w) = (f64::from(r.i16()?), f64::from(r.i16()?));
            p.figure(super::round_rect(rect_brtl(&mut r)?, w, h), true);
        }
        0x0817 | 0x081A | 0x0830 => {
            let (end, start) = (point_yx(&mut r)?, point_yx(&mut r)?);
            let b = rect_brtl(&mut r)?;
            let kind = match function {
                0x0817 => ArcKind::Open,
                0x081A => ArcKind::Pie,
                _ => ArcKind::Chord,
            };
            p.arc(b, start, end, kind);
        }
        0x0416 => p.clip_rect(rect_brtl(&mut r)?),
        0x0415 => p.exclude_clip(),
        0x012D => p.select(u32::from(r.u16()?)),
        0x01F0 => p.delete(u32::from(r.u16()?)),
        0x02FA => {
            let style = u32::from(r.u16()?);
            let width = f64::from(r.i16()?);
            r.skip(2)?;
            p.create(Obj::Pen(PenObj { style, width, color: color(&mut r)?, entries: vec![] }));
        }
        0x02FC => {
            let style = u32::from(r.u16()?);
            p.create(Obj::Brush(BrushObj::from_log(style, color(&mut r)?)));
        }
        0x02FB => {
            let height = f64::from(r.i16()?);
            r.skip(2)?;
            let escapement = f64::from(r.i16()?);
            r.skip(2)?;
            let weight = i32::from(r.i16()?);
            let italic = r.bytes(1)?.first().is_some_and(|b| *b != 0);
            r.skip(2)?;
            let charset = *r.bytes(1)?.first()?;
            r.skip(4)?;
            let face: Vec<u8> = r.rest().iter().take(32).take_while(|b| **b != 0).copied().collect();
            p.create(Obj::Font(FontObj { height, escapement, weight, italic, charset, face: ansi(&face) }));
        }
        0x00F7 | 0x06FF => p.create(Obj::Other),
        0x01F9 | 0x0142 => p.create(Obj::Brush(BrushObj::Pattern)),
        0x0521 => {
            let n = usize::try_from(r.i16()?).ok()?;
            if !p.check_expansion(n) {
                return Some(());
            }
            let s = ansi(r.bytes(n)?);
            r.skip(n % 2)?;
            let at = point_yx(&mut r)?;
            p.text(at, &s, 0, None);
        }
        0x0A32 => {
            let at = point_yx(&mut r)?;
            let n = usize::try_from(r.i16()?).ok()?;
            let opts = r.u16()?;
            // ETO_OPAQUE or ETO_CLIPPED: a rectangle comes first.
            // https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/7d07c44a-a828-4b82-9af0-e0a81cced5a8
            let clip = if opts & 0x6 != 0 {
                let rectangle = Rect::new(f64::from(r.i16()?), f64::from(r.i16()?), f64::from(r.i16()?), f64::from(r.i16()?));
                (opts & 4 != 0).then_some(rectangle)
            } else {
                None
            };
            if !p.check_expansion(n) {
                return Some(());
            }
            let s = ansi(r.bytes(n)?);
            p.text(at, &s, u32::from(opts), clip);
        }
        0x0F43 => {
            let rop = r.u32()?;
            r.skip(2)?;
            let (hs, ws, ys, xs) = (r.i16()?, r.i16()?, r.i16()?, r.i16()?);
            let (hd, wd, yd, xd) = (r.i16()?, r.i16()?, r.i16()?, r.i16()?);
            blit(p, rop, (xs, ys, ws, hs), (xd, yd, wd, hd), Some(r.rest()));
        }
        0x0B41 => {
            let rop = r.u32()?;
            let (hs, ws, ys, xs) = (r.i16()?, r.i16()?, r.i16()?, r.i16()?);
            // Without a bitmap, a reserved word comes before the destination.
            let with_bitmap = body.len() > 22;
            if !with_bitmap {
                r.skip(2)?;
            }
            let (hd, wd, yd, xd) = (r.i16()?, r.i16()?, r.i16()?, r.i16()?);
            blit(p, rop, (xs, ys, ws, hs), (xd, yd, wd, hd), with_bitmap.then(|| r.rest()));
        }
        0x0940 => {
            let rop = r.u32()?;
            let (ys, xs) = (r.i16()?, r.i16()?);
            let with_bitmap = body.len() > 20;
            if !with_bitmap {
                r.skip(2)?;
            }
            let (h, w, yd, xd) = (r.i16()?, r.i16()?, r.i16()?, r.i16()?);
            blit(p, rop, (xs, ys, w, h), (xd, yd, w, h), with_bitmap.then(|| r.rest()));
        }
        0x061D => {
            let rop = r.u32()?;
            let (h, w, y, x) = (f64::from(r.i16()?), f64::from(r.i16()?), f64::from(r.i16()?), f64::from(r.i16()?));
            p.pattern_blit(Rect::new(x, y, x + w, y + h), rop);
        }
        f if IGNORED.contains(&f) => {}
        _ => return None,
    }
    Some(())
}

#[cfg(test)]
#[path = "../../tests/support/wmf_embedded.rs"]
mod embedded_recipe;
#[cfg(test)]
use recipe as base_wmf;
#[cfg(test)]
#[path = "../../tests/support/wmf.rs"]
mod recipe;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cosmetic_dash_expansion_is_bounded_after_anisotropic_placement() {
        let mut records = recipe::setup();
        records.extend([
            recipe::record(0x02fa, &[1, 0, 0, 0, 0]),
            recipe::record(0x012d, &[2]),
            recipe::record(0x0214, &[0, 0]),
            recipe::record(0x0213, &[0, 30]),
        ]);
        let bytes = recipe::file(&records, None);
        let result = play_options(&bytes, super::super::SceneBudget::default(), 16, Some((0.75, Affine::scale_non_uniform(100., 0.01))));
        assert!(result.err().is_some_and(|e| e.starts_with("metafile operation limit:") && e.contains("dash pieces")));
    }
    #[test]
    fn complete_embedded_stream_preserves_resource_refusals() {
        let bytes = embedded_recipe::wrapper(&embedded_recipe::triangle(), 80, false);
        assert!(play_budgeted(&bytes, super::super::SceneBudget::new(1), 100_000).err().unwrap().starts_with("export scene memory limit"));
        assert_eq!(play_budgeted(&bytes, super::super::SceneBudget::default(), 0).err().unwrap(), "metafile operation limit");
    }
    #[test]
    fn resource_failures_propagate_during_records_and_final_line_flush() {
        let bytes = recipe::rectangle();
        assert!(play_budgeted(&bytes, super::super::SceneBudget::new(1), 100_000).err().unwrap().starts_with("export scene memory limit"));
        let mut r = recipe::setup();
        r.extend([
            recipe::record(0x2fa, &[0, 20, 0, 0, 0]),
            recipe::record(0x12d, &[2]),
            recipe::record(0x214, &[0, 0]),
            recipe::record(0x213, &[720, 1440]),
        ]);
        let bytes = recipe::file(&r, None);
        assert_eq!(play_budgeted(&bytes, super::super::SceneBudget::default(), 0).err().unwrap(), "metafile operation limit");
        assert!(play(&bytes).is_ok());
    }
    #[test]
    fn final_dashed_path_and_repeated_drawing_hit_controlled_limits() {
        let mut r = recipe::setup();
        r.extend([
            recipe::record(0x2fa, &[1, 0, 0, 0, 0]),
            recipe::record(0x12d, &[2]),
            recipe::record(0x214, &[0, 0]),
            recipe::record(0x213, &[0, 32767]),
        ]);
        let error = play_budgeted(&recipe::file(&r, None), super::super::SceneBudget::default(), 16).err().unwrap();
        assert!(error.starts_with("metafile operation limit:") && error.contains("dash pieces"), "{error}");
        let mut r = recipe::setup();
        r.extend((0..100).map(|_| recipe::record(0x41b, &[540, 1260, 180, 180])));
        let error = play_budgeted(&recipe::file(&r, None), super::super::SceneBudget::default(), 32).err().unwrap();
        assert!(error.starts_with("metafile operation limit"), "{error}");
    }
}
