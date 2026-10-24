//! Bounded, adjacent mask/colour pairs in cached foreign-object presentations.
use super::{bitmap, bottom_up, source};
use crate::metafile::{
    bytes::Reader,
    dib::{Alpha, Rgba},
};
use kurbo::Rect;

const SRCAND: u32 = 0x0088_00c6;
const SRCINVERT: u32 = 0x0066_0046;
const SRCPAINT: u32 = 0x00ee_0086;

struct Blit {
    pixels: Rgba,
    dest: Rect,
}

/// Identical adjacent destinations with a binary AND mask followed by XOR/OR
/// reduce exactly to alpha copy when transparent colour pixels are black.
/// https://learn.microsoft.com/en-us/windows/win32/gdi/ternary-raster-operations
pub(super) fn pair(first: &[u8], remaining: &[u8]) -> Option<(Rgba, Rect, usize)> {
    let mut head = Reader::new(remaining);
    head.u32()?;
    let size = usize::try_from(head.u32()?).ok()?;
    let second = remaining.get(..size).filter(|_| size >= 8)?;
    let mask = blit(first, &[SRCAND])?;
    let mut colour = blit(second, &[SRCINVERT, SRCPAINT])?;
    if mask.dest != colour.dest || mask.pixels.width != colour.pixels.width || mask.pixels.height != colour.pixels.height {
        return None;
    }
    let expected = usize::try_from(colour.pixels.width).ok()?.checked_mul(usize::try_from(colour.pixels.height).ok()?)?.checked_mul(4)?;
    if expected == 0 || mask.pixels.pixels.len() != expected || colour.pixels.pixels.len() != expected {
        return None;
    }
    for (mask, colour) in mask.pixels.pixels.as_chunks::<4>().0.iter().zip(colour.pixels.pixels.as_chunks_mut::<4>().0.iter_mut()) {
        let alpha = match mask.get(..3)? {
            [0, 0, 0] => 255,
            [255, 255, 255] if colour.get(..3)? == [0, 0, 0] => 0,
            _ => return None,
        };
        *colour.get_mut(3)? = alpha;
    }
    Some((colour.pixels, colour.dest, size))
}

/// Decode the documented bitmap-transfer fields without admitting transformed,
/// mirrored or oversized source rectangles into the mask-pair optimization.
/// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-stretchblt
/// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-stretchdibits
fn blit(rec: &[u8], allowed: &[u32]) -> Option<Blit> {
    let mut r = Reader::new(rec);
    let kind = r.u32()?;
    if usize::try_from(r.u32()?).ok()? != rec.len() {
        return None;
    }
    r.skip(16)?;
    let (xd, yd) = (r.i32()?, r.i32()?);
    let (wd, hd, xs, ys, ws, hs, off_bmi, cb_bmi, off_bits, cb_bits, from_bottom) = match kind {
        76 | 77 => {
            let (wd, hd) = (r.i32()?, r.i32()?);
            if !allowed.contains(&r.u32()?) {
                return None;
            }
            let (xs, ys) = (r.i32()?, r.i32()?);
            for expected in [1., 0., 0., 1., 0., 0.] {
                if r.f32()? != expected {
                    return None;
                }
            }
            r.skip(4)?;
            if r.u32()? != 0 {
                return None;
            }
            let (off_bmi, cb_bmi, off_bits, cb_bits) = (r.u32()?, r.u32()?, r.u32()?, r.u32()?);
            let (ws, hs) = if kind == 77 { (r.i32()?, r.i32()?) } else { (wd, hd) };
            (wd, hd, xs, ys, ws, hs, off_bmi, cb_bmi, off_bits, cb_bits, false)
        }
        81 => {
            let (xs, ys, ws, hs) = (r.i32()?, r.i32()?, r.i32()?, r.i32()?);
            let (off_bmi, cb_bmi, off_bits, cb_bits) = (r.u32()?, r.u32()?, r.u32()?, r.u32()?);
            if r.u32()? != 0 || !allowed.contains(&r.u32()?) {
                return None;
            }
            let (wd, hd) = (r.i32()?, r.i32()?);
            (wd, hd, xs, ys, ws, hs, off_bmi, cb_bmi, off_bits, cb_bits, bottom_up(rec, off_bmi))
        }
        _ => return None,
    };
    if xs < 0 || ys < 0 || ws <= 0 || hs <= 0 || wd <= 0 || hd <= 0 {
        return None;
    }
    // The restricted path uses the signed-height BITMAPINFOHEADER origin rule.
    // Core headers have unsigned, bottom-up heights and remain unsupported here.
    let bmi = crate::metafile::bytes::slice(rec, off_bmi, cb_bmi)?;
    let header_size = usize::try_from(Reader::new(bmi).u32()?).ok()?;
    if header_size < 40 || header_size > bmi.len() {
        return None;
    }
    let pixels = bitmap(rec, off_bmi, cb_bmi, off_bits, cb_bits, Alpha::Ignore);
    let src = source(xs, ys, ws, hs, &pixels, from_bottom);
    let pixels = pixels.ok()?;
    if src.x0 < 0. || src.y0 < 0. || src.x1 > f64::from(pixels.width) || src.y1 > f64::from(pixels.height) {
        return None;
    }
    let pixels = crate::metafile::crop(pixels, src);
    let dest = Rect::new(f64::from(xd), f64::from(yd), f64::from(xd) + f64::from(wd), f64::from(yd) + f64::from(hd));
    Some(Blit { pixels, dest })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn put(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    fn transfer(kind: u32, rop: u32, pixels: &[[u8; 3]]) -> Vec<u8> {
        let fixed = if kind == 77 { 108 } else { 80 };
        let mut bytes = vec![0; fixed + 40];
        put(&mut bytes, 0, kind);
        if kind == 77 {
            for (offset, value) in
                [(32, 2), (36, 1), (40, rop), (52, 1f32.to_bits()), (64, 1f32.to_bits()), (84, 108), (88, 40), (92, 148), (96, 8), (100, 2), (104, 1)]
            {
                put(&mut bytes, offset, value);
            }
        } else {
            for (offset, value) in [(40, 2), (44, 1), (48, 80), (52, 40), (56, 120), (60, 8), (68, rop), (72, 2), (76, 1)] {
                put(&mut bytes, offset, value);
            }
        }
        for (offset, value) in [(0, 40), (4, 2), (8, u32::MAX), (12, 0x0020_0001), (20, 8)] {
            put(&mut bytes, fixed + offset, value);
        }
        for [r, g, b] in pixels {
            bytes.extend_from_slice(&[*b, *g, *r, 255]);
        }
        let len = bytes.len() as u32;
        put(&mut bytes, 4, len);
        bytes
    }
    fn metafile(mask: &[u8], colour: &[u8]) -> Vec<u8> {
        let mut emf = vec![0; 88];
        for (offset, value) in [(0, 1), (4, 88), (16, 2), (20, 1), (32, 2540), (36, 1270), (40, 0x464d4520), (72, 96), (76, 96), (80, 25), (84, 25)] {
            put(&mut emf, offset, value);
        }
        emf.extend_from_slice(mask);
        emf.extend_from_slice(colour);
        emf.extend_from_slice(&14u32.to_le_bytes());
        emf.extend_from_slice(&8u32.to_le_bytes());
        emf
    }
    #[test]
    fn original_binary_masks_preserve_destination_or_copy_colour_exactly() {
        let mask = transfer(77, SRCAND, &[[0, 0, 0], [255, 255, 255]]);
        for rop in [SRCINVERT, SRCPAINT] {
            let colour = transfer(81, rop, &[[20, 80, 140], [0, 0, 0]]);
            let (pixels, dest, consumed) = pair(&mask, &colour).unwrap();
            assert_eq!(pixels.pixels, vec![20, 80, 140, 255, 0, 0, 0, 0]);
            assert_eq!(dest, Rect::new(0., 0., 2., 1.));
            assert_eq!(consumed, colour.len());
            let emf = metafile(&mask, &colour);
            let strict = crate::metafile::import(&emf).unwrap();
            assert!(!strict.ops.iter().any(|op| matches!(op, crate::scene::Op::Image { .. })));
            let recovered = crate::metafile::import_cached_picture(&emf).unwrap();
            assert_eq!(recovered.ops.iter().filter(|op| matches!(op, crate::scene::Op::Image { .. })).count(), 1);
            assert!(!recovered.warnings.iter().any(|w| w.contains("raster operations")));
        }
    }
    #[test]
    fn truncated_offsets_mismatched_destinations_and_nonbinary_masks_are_not_admitted() {
        let mask = transfer(77, SRCAND, &[[0, 0, 0], [255, 255, 255]]);
        let colour = transfer(81, SRCINVERT, &[[20, 80, 140], [0, 0, 0]]);
        for end in 0..mask.len() {
            assert!(pair(&mask[..end], &colour).is_none());
        }
        for end in 0..colour.len() {
            assert!(pair(&mask, &colour[..end]).is_none());
        }
        for (offset, value) in [(84, u32::MAX), (32, 3), (52, f32::NAN.to_bits()), (100, i32::MAX as u32)] {
            let mut bad = mask.clone();
            put(&mut bad, offset, value);
            assert!(pair(&bad, &colour).is_none());
        }
        let nonbinary = transfer(77, SRCAND, &[[127, 127, 127], [255, 255, 255]]);
        assert!(pair(&nonbinary, &colour).is_none());
        let not_transparent = transfer(81, SRCINVERT, &[[20, 80, 140], [1, 0, 0]]);
        assert!(pair(&mask, &not_transparent).is_none());
    }

    #[test]
    fn cropped_core_header_dib_is_not_mistaken_for_a_top_down_picture() {
        let mut mask = transfer(77, SRCAND, &[[0, 0, 0], [255, 255, 255]]);
        put(&mut mask, 32, 1);
        put(&mut mask, 100, 1);
        // Original 1x2 core DIB: bottom blue pixel, top orange pixel. ys=0,
        // hs=1 selects the bottom row in StretchDIBits, never the top row.
        let mut colour = vec![0u8; 100];
        for (offset, value) in
            [(0, 81), (4, 100), (40, 1), (44, 1), (48, 80), (52, 12), (56, 92), (60, 8), (68, SRCINVERT), (72, 1), (76, 1), (80, 12)]
        {
            put(&mut colour, offset, value);
        }
        for (offset, value) in [(84, 1u16), (86, 2), (88, 1), (90, 24)] {
            colour[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
        }
        colour[92..].copy_from_slice(&[90, 60, 30, 0, 20, 80, 140, 0]);
        assert!(pair(&mask, &colour).is_none());
        let error = crate::metafile::import_cached_picture(&metafile(&mask, &colour)).err().unwrap();
        assert!(error.contains("unsupported or damaged bitmap"), "{error}");
    }
}
