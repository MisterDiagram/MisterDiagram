//! Retargeted from vectorcraft crates/metafile/src/dib.rs,
//! 806b1a6972288319577d75d097c140f9bf530ce0, MIT OR Apache-2.0.
//! Device-independent bitmaps (DIBs), the pixels inside image records: written bottom-up as 24-bit
//! (opaque, composited over white) or 32-bit premultiplied BGRA (AlphaBlend's source); read from 1,
//! 4, 8, 16, 24 and 32-bit uncompressed and bit-field bitmaps and from embedded JPEG and PNG.

use super::bytes::Reader;

/// Most pixels along a side, and in all, of a bitmap the reader decodes.
const MAX_READ_SIDE: u32 = 1 << 15;
const MAX_READ_PIXELS: u64 = 1 << 25;

const BI_RGB: u32 = 0;
const BI_BITFIELDS: u32 = 3;
const BI_JPEG: u32 = 4;
const BI_PNG: u32 = 5;

/// Pixels with straight (not premultiplied) alpha, row by row from the top.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Rgba {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

fn stride(width: u32, bits: u32) -> Option<usize> {
    let row_bits = u64::from(width).checked_mul(u64::from(bits))?;
    usize::try_from(row_bits.div_ceil(32) * 4).ok()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Alpha {
    /// Unused (StretchDIBits and BitBlt ignore it): every pixel opaque.
    Ignore,
    /// Premultiplied alpha (AlphaBlend with per-pixel alpha).
    Premultiplied,
}

/// A colour channel of `v` under `mask`, scaled to 0–255.
fn channel(v: u32, mask: u32) -> u8 {
    if mask == 0 {
        return 0;
    }
    let shift = mask.trailing_zeros();
    let max = u64::from(mask >> shift);
    let x = u64::from((v & mask) >> shift);
    ((x * 255 + max / 2) / max.max(1)).min(255) as u8
}

/// Read a DIB from its BITMAPINFO (`bmi`: the header and colour table) and pixel `bits`.
pub(crate) fn decode(bmi: &[u8], bits: &[u8], alpha: Alpha) -> Result<Rgba, String> {
    let bad = || "a bitmap is damaged".to_string();
    let mut r = Reader::new(bmi);
    let size = r.u32().ok_or_else(bad)?;
    let (width, height, bpp, compression, colors, palette_entry, masks_at) = if size == 12 {
        // BITMAPCOREHEADER: 16-bit sizes, RGB triples.
        let w = i32::from(r.u16().ok_or_else(bad)?);
        let h = i32::from(r.u16().ok_or_else(bad)?);
        r.skip(2).ok_or_else(bad)?;
        let bpp = r.u16().ok_or_else(bad)?;
        (w, h, bpp, BI_RGB, 0, 3, 0)
    } else if size >= 40 {
        let w = r.i32().ok_or_else(bad)?;
        let h = r.i32().ok_or_else(bad)?;
        r.skip(2).ok_or_else(bad)?;
        let bpp = r.u16().ok_or_else(bad)?;
        let c = r.u32().ok_or_else(bad)?;
        r.skip(12).ok_or_else(bad)?;
        let used = r.u32().ok_or_else(bad)?;
        (w, h, bpp, c, used, 4, 40)
    } else {
        return Err(bad());
    };
    if compression == BI_JPEG || compression == BI_PNG {
        let img = crate::raster::decode(bits)?;
        let (w, h) = img.dimensions();
        if w > MAX_READ_SIDE || h > MAX_READ_SIDE || u64::from(w) * u64::from(h) > MAX_READ_PIXELS {
            return Err("a bitmap is too large to read".into());
        }
        return Ok(Rgba { width: w, height: h, pixels: img.into_raw() });
    }
    if compression != BI_RGB && compression != BI_BITFIELDS {
        return Err("compressed (RLE) bitmaps can't be read yet".into());
    }
    let top_down = height < 0;
    let (w, h) = (width.unsigned_abs(), height.unsigned_abs());
    if w == 0 || h == 0 {
        return Err("a bitmap is empty".into());
    }
    if w > MAX_READ_SIDE || h > MAX_READ_SIDE || u64::from(w) * u64::from(h) > MAX_READ_PIXELS {
        return Err("a bitmap is too large to read".into());
    }
    let header_len = size as usize;
    // Bit-field masks follow a plain 40-byte header, or sit inside the larger ones.
    let fields = compression == BI_BITFIELDS;
    let masks: Option<[u32; 4]> = fields.then(|| {
        let mut m = Reader::new(bmi.get(masks_at..).unwrap_or_default());
        let rgb = [m.u32().unwrap_or(0), m.u32().unwrap_or(0), m.u32().unwrap_or(0)];
        // An alpha mask comes with BITMAPV3INFOHEADER and later.
        let a = if header_len >= 56 { m.u32().unwrap_or(0) } else { 0 };
        [rgb[0], rgb[1], rgb[2], a]
    });
    let palette_at = header_len + if fields && header_len == 40 { 12 } else { 0 };
    let bpp_u = u32::from(bpp);
    let palette: Vec<[u8; 3]> = if bpp_u <= 8 {
        let n = if colors == 0 { 1usize << bpp_u } else { (colors as usize).min(256) };
        bmi.get(palette_at..)
            .unwrap_or_default()
            .chunks_exact(palette_entry)
            .take(n)
            .map(|e| [e.get(2).copied().unwrap_or(0), e.get(1).copied().unwrap_or(0), e.first().copied().unwrap_or(0)])
            .collect()
    } else {
        vec![]
    };
    if !matches!(bpp, 1 | 4 | 8 | 16 | 24 | 32) {
        return Err(format!("{bpp}-bit bitmaps can't be read"));
    }
    let row = stride(w, bpp_u).ok_or_else(bad)?;
    if bits.len() < row.checked_mul(h as usize).ok_or_else(bad)? {
        return Err(bad());
    }
    let (mr, mg, mb, ma) = match (masks, bpp) {
        (Some([r, g, b, a]), _) => (r, g, b, a),
        (None, 16) => (0x7c00, 0x03e0, 0x001f, 0),
        _ => (0x00ff_0000, 0x0000_ff00, 0x0000_00ff, 0xff00_0000),
    };
    let mut pixels = Vec::with_capacity(w as usize * h as usize * 4);
    for y in 0..h as usize {
        let src_row = if top_down { y } else { h as usize - 1 - y };
        let line = bits.get(src_row * row..src_row * row + row).ok_or_else(bad)?;
        for x in 0..w as usize {
            let px: [u8; 4] = match bpp {
                1 | 4 | 8 => {
                    let bit = x * bpp_u as usize;
                    let byte = line.get(bit / 8).copied().unwrap_or(0);
                    let idx = (byte >> (8 - bpp_u as usize - bit % 8)) & ((1u16 << bpp_u) - 1) as u8;
                    let [r, g, b] = palette.get(idx as usize).copied().unwrap_or([0, 0, 0]);
                    [r, g, b, 255]
                }
                16 => {
                    let v = u32::from(u16::from_le_bytes([line.get(x * 2).copied().unwrap_or(0), line.get(x * 2 + 1).copied().unwrap_or(0)]));
                    [channel(v, mr), channel(v, mg), channel(v, mb), 255]
                }
                24 => {
                    let p = line.get(x * 3..x * 3 + 3).unwrap_or(&[0, 0, 0]);
                    [p[2], p[1], p[0], 255]
                }
                _ => {
                    let p = line.get(x * 4..x * 4 + 4).unwrap_or(&[0, 0, 0, 0]);
                    let v = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
                    let a = if alpha == Alpha::Premultiplied && ma != 0 { channel(v, ma) } else { 255 };
                    let un =
                        |c: u8| if alpha == Alpha::Premultiplied && a < 255 { (u32::from(c) * 255 / u32::from(a.max(1))).min(255) as u8 } else { c };
                    [un(channel(v, mr)), un(channel(v, mg)), un(channel(v, mb)), if alpha == Alpha::Premultiplied { a } else { 255 }]
                }
            };
            pixels.extend_from_slice(&px);
        }
    }
    Ok(Rgba { width: w, height: h, pixels })
}

#[cfg(test)]
mod size_tests {
    use super::*;

    #[test]
    fn embedded_png_and_jpeg_branches_check_header_before_decode() {
        let mut encoded = std::io::Cursor::new(Vec::new());
        image::GrayImage::new(1, 1).write_to(&mut encoded, image::ImageFormat::Png).unwrap();
        let mut png = encoded.into_inner();
        png[16..20].copy_from_slice(&10_000u32.to_be_bytes());
        png[20..24].copy_from_slice(&10_000u32.to_be_bytes());
        let mut crc = !0u32;
        for byte in &png[12..29] {
            crc ^= u32::from(*byte);
            for _ in 0..8 {
                crc = (crc >> 1) ^ (0xedb88320 & 0u32.wrapping_sub(crc & 1));
            }
        }
        png[29..33].copy_from_slice(&(!crc).to_be_bytes());
        // Both compressed DIB tags share the same signature-sniffed image reader.
        // The payload is deliberately inconsistent after its header: a pixel
        // decode would fail differently if the pre-allocation check were removed.
        for compression in [BI_PNG, BI_JPEG] {
            let mut bmi = vec![0u8; 40];
            bmi[..4].copy_from_slice(&40u32.to_le_bytes());
            bmi[16..20].copy_from_slice(&compression.to_le_bytes());
            assert_eq!(decode(&bmi, &png, Alpha::Ignore).unwrap_err(), "image pixel limit (header)");
        }
    }
}
