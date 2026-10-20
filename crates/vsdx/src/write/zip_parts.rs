//! Preserve compressed-file admission without copying immutable media for publication.
use super::{WriteError, error};
use misterdiagram_model::Shared;
use std::{
    collections::BTreeMap,
    io::{Cursor, Seek, SeekFrom, Write},
};

type Files = BTreeMap<String, Shared<Vec<u8>>>;
fn compress<W: Write + Seek>(files: &Files, out: W) -> Result<W, WriteError> {
    let mut zip = zip::ZipWriter::new(out);
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in files {
        zip.start_file(name, options).map_err(|e| error(e.to_string()))?;
        zip.write_all(bytes).map_err(|e| error(e.to_string()))?;
    }
    zip.finish().map_err(|e| error(e.to_string()))
}
pub(super) fn bytes(files: &Files) -> Result<Vec<u8>, WriteError> {
    let bytes = compress(files, Cursor::new(Vec::new()))?.into_inner();
    if bytes.len() as u64 > crate::MAX_FILE {
        return Err(error("written compressed file limit"));
    }
    Ok(bytes)
}
pub(super) fn admit(files: &Files) -> Result<(), WriteError> {
    // This deliberately loose envelope covers DEFLATE blocks, ZIP headers, extra
    // fields and duplicated UTF-8 names. Inconclusive envelopes use the actual ZIP.
    let envelope = files.iter().try_fold(65536u64, |n, (name, bytes)| {
        n.checked_add((bytes.len() as u64).checked_mul(2)?)?.checked_add((name.len() as u64).checked_mul(4)?)?.checked_add(65536)
    });
    if envelope.is_some_and(|n| n <= crate::MAX_FILE) {
        return Ok(());
    }
    let count = compress(files, Count::default())?;
    if count.end > crate::MAX_FILE {
        return Err(error("written compressed file limit"));
    }
    Ok(())
}
#[derive(Default)]
struct Count {
    position: u64,
    end: u64,
}
impl Write for Count {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.position = self.position.checked_add(bytes.len() as u64).ok_or_else(|| std::io::Error::other("ZIP size overflow"))?;
        self.end = self.end.max(self.position);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl Seek for Count {
    fn seek(&mut self, seek: SeekFrom) -> std::io::Result<u64> {
        let next = match seek {
            SeekFrom::Start(n) => Some(n),
            SeekFrom::Current(n) => self.position.checked_add_signed(n),
            SeekFrom::End(n) => self.end.checked_add_signed(n),
        }
        .ok_or_else(|| std::io::Error::other("ZIP seek overflow"))?;
        self.position = next;
        Ok(next)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counting_zip_matches_real_compression_and_rejects_invalid_seek() {
        let files = BTreeMap::from([
            ("original.xml".into(), b"<original>invented content</original>".to_vec().into()),
            ("empty.bin".into(), Vec::new().into()),
            ("mixed.bin".into(), (0u8..=255).cycle().take(100_003).collect::<Vec<_>>().into()),
        ]);
        assert_eq!(compress(&files, Count::default()).unwrap().end, bytes(&files).unwrap().len() as u64);
        let mut sink = Count::default();
        assert!(sink.seek(SeekFrom::Current(-1)).is_err());
        sink.seek(SeekFrom::Start(u64::MAX)).unwrap();
        assert!(sink.write(&[1]).is_err());
    }
}
