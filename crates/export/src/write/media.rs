//! Bounded PNG parts for the foreign pixels in the export model.
use image::ImageEncoder as _;
use misterdiagram_opc::{PackageOut, RelsOut};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct Media {
    parts: BTreeMap<[u8; 32], String>,
}
impl Media {
    pub fn add(&mut self, pixels: &image::RgbaImage, pkg: &mut PackageOut, rels: &mut RelsOut, remaining: &mut usize) -> Result<String, String> {
        let mut hash = Sha256::new();
        hash.update(pixels.width().to_le_bytes());
        hash.update(pixels.height().to_le_bytes());
        hash.update(pixels.as_raw());
        let digest: [u8; 32] = hash.finalize().into();
        let part = if let Some(part) = self.parts.get(&digest) {
            part.clone()
        } else {
            let mut buffer = Limited { bytes: Vec::new(), limit: *remaining / 4 };
            image::codecs::png::PngEncoder::new(&mut buffer)
                .write_image(pixels.as_raw(), pixels.width(), pixels.height(), image::ExtendedColorType::Rgba8)
                .map_err(|e| format!("PPTX PNG: {e}"))?;
            *remaining = remaining.checked_sub(buffer.bytes.len().checked_mul(4).ok_or("PPTX media limit")?).ok_or("PPTX media limit")?;
            let part = format!("image{}.png", self.parts.len() + 1);
            pkg.binary(&format!("ppt/media/{part}"), "image/png", buffer.bytes);
            self.parts.insert(digest, part.clone());
            part
        };
        Ok(rels.add("image", &format!("../media/{part}"), false))
    }
}
struct Limited {
    bytes: Vec<u8>,
    limit: usize,
}
impl std::io::Write for Limited {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.bytes.len().checked_add(bytes.len()).is_none_or(|n| n > self.limit) {
            return Err(std::io::Error::other("PPTX media limit"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn media_respects_shared_budget_and_reuses_identical_content() {
        let pixels = image::RgbaImage::from_pixel(4, 4, image::Rgba([1, 2, 3, 255]));
        let mut media = Media::default();
        let mut package = PackageOut::default();
        let mut rels = RelsOut::default();
        assert!(media.add(&pixels, &mut package, &mut rels, &mut 1).is_err());
        let mut remaining = 8192;
        media.add(&pixels, &mut package, &mut rels, &mut remaining).unwrap();
        let after = remaining;
        media.add(&pixels, &mut package, &mut rels, &mut remaining).unwrap();
        assert_eq!(remaining, after);
        let different = image::RgbaImage::from_pixel(4, 4, image::Rgba([3, 2, 1, 255]));
        assert!(media.add(&different, &mut package, &mut rels, &mut 1).is_err());
    }
}
