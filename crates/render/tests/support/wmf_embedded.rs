//! Contributor-original byte recipes, MIT OR Apache-2.0.
//! https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/cfc88064-d86d-4b52-9374-3ce27d456179
#![allow(dead_code)]
use super::base_wmf as wmf;

pub fn checksum(bytes: &mut [u8]) {
    bytes[40..42].fill(0);
    let sum = bytes.as_chunks::<2>().0.iter().fold(0u16, |sum, w| sum.wrapping_add(u16::from_le_bytes([w[0], w[1]])));
    bytes[40..42].copy_from_slice(&0u16.wrapping_sub(sum).to_le_bytes());
}

pub fn wrapper(inner: &[u8], size: usize, truncated: bool) -> Vec<u8> {
    assert!(size > 0 && size <= 8192 && size.is_multiple_of(2));
    let chunks: Vec<_> = inner.chunks(size).collect();
    let mut records = Vec::new();
    let mut remaining = inner.len();
    for chunk in &chunks {
        remaining -= chunk.len();
        let mut body = 15u16.to_le_bytes().to_vec();
        body.extend(((chunk.len() + 34) as u16).to_le_bytes());
        for value in [0x4346_4d57u32, 1, 0x0001_0000] {
            body.extend(value.to_le_bytes());
        }
        body.extend(0u16.to_le_bytes());
        for value in [0, chunks.len() as u32, chunk.len() as u32, remaining as u32, inner.len() as u32] {
            body.extend(value.to_le_bytes());
        }
        body.extend(*chunk);
        if !body.len().is_multiple_of(2) {
            body.push(0);
        }
        records.push(wmf::packed(0x0626, &body));
    }
    let mut fallback = wmf::setup();
    fallback.push(wmf::record(0x041b, &[540, 1260, 180, 180]));
    records.extend(fallback);
    let mut bytes = wmf::file(&records, None);
    checksum(&mut bytes);
    if truncated {
        let at = 18 + records.iter().take(chunks.len() - 1).map(Vec::len).sum::<usize>();
        let length = records[chunks.len() - 1].len();
        bytes.drain(at + length - 2..at + length);
        bytes[at..at + 4].copy_from_slice(&((length / 2 - 1) as u32).to_le_bytes());
        let words = (bytes.len() / 2) as u32;
        bytes[6..10].copy_from_slice(&words.to_le_bytes());
    }
    bytes
}

/// A small solid triangle, deliberately different from the wrapper's rectangle.
/// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/
pub fn triangle() -> Vec<u8> {
    fn record(kind: u32, words: &[u32]) -> Vec<u8> {
        [kind, 8 + words.len() as u32 * 4].into_iter().chain(words.iter().copied()).flat_map(u32::to_le_bytes).collect()
    }
    let mut bytes = vec![0u8; 88];
    for (at, value) in [
        (0, 1u32),
        (4, 88),
        (16, 96),
        (20, 96),
        (32, 2540),
        (36, 2540),
        (40, 0x464d4520),
        (44, 0x10000),
        (56, 2),
        (72, 960),
        (76, 960),
        (80, 254),
        (84, 254),
    ] {
        bytes[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    bytes.extend(record(39, &[1, 0, 0x005a3cb4, 0]));
    bytes.extend(record(37, &[1]));
    bytes.extend(record(37, &[0x80000008]));
    bytes.extend(record(3, &[10, 10, 86, 86, 3, 10, 10, 86, 10, 48, 86]));
    bytes.extend(record(14, &[0, 0, 20]));
    let len = bytes.len() as u32;
    bytes[48..52].copy_from_slice(&len.to_le_bytes());
    bytes[52..56].copy_from_slice(&6u32.to_le_bytes());
    bytes
}

pub fn pens() -> Vec<u8> {
    let mut records = vec![wmf::record(0x0103, &[8]), wmf::record(0x020b, &[0, 0]), wmf::record(0x020c, &[1436, 1829])];
    for (handle, width) in [0, 1].into_iter().enumerate() {
        let handle = handle as i16;
        records.extend([
            wmf::record(0x02fa, &[0, width, 0, 0, 0]),
            wmf::record(0x012d, &[handle]),
            wmf::record(0x0214, &[300 + handle * 600, 200]),
            wmf::record(0x0213, &[300 + handle * 600, 1600]),
            wmf::record(0x012d, &[handle]),
            wmf::record(0x0214, &[200, 400 + handle * 800]),
            wmf::record(0x0213, &[1200, 400 + handle * 800]),
        ]);
    }
    wmf::file(&records, None)
}
