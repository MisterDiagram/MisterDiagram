//! Contributor-original WMF byte recipes, MIT OR Apache-2.0.
//! https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/
#![allow(dead_code)]
pub fn record(function: u16, words: &[i16]) -> Vec<u8> {
    packed(function, &words.iter().flat_map(|v| v.to_le_bytes()).collect::<Vec<_>>())
}
pub fn packed(function: u16, body: &[u8]) -> Vec<u8> {
    assert_eq!(body.len() % 2, 0);
    let mut out = ((body.len() / 2 + 3) as u32).to_le_bytes().to_vec();
    out.extend(function.to_le_bytes());
    out.extend(body);
    out
}
pub fn file(records: &[Vec<u8>], placeable: Option<([i16; 4], u16)>) -> Vec<u8> {
    let mut out = Vec::new();
    if let Some((bounds, inch)) = placeable {
        out.extend(0x9ac6cdd7u32.to_le_bytes());
        out.extend(0u16.to_le_bytes());
        for v in bounds {
            out.extend(v.to_le_bytes());
        }
        out.extend(inch.to_le_bytes());
        out.extend(0u32.to_le_bytes());
        let checksum = out.as_chunks::<2>().0.iter().fold(0u16, |a, b| a ^ u16::from_le_bytes([b[0], b[1]]));
        out.extend(checksum.to_le_bytes());
    }
    let bytes = records.iter().map(Vec::len).sum::<usize>() + 6;
    out.extend(1u16.to_le_bytes());
    out.extend(9u16.to_le_bytes());
    out.extend(0x300u16.to_le_bytes());
    out.extend((9 + bytes as u32 / 2).to_le_bytes());
    out.extend(16u16.to_le_bytes());
    out.extend((records.iter().map(Vec::len).max().unwrap_or(6) as u32 / 2).to_le_bytes());
    out.extend(0u16.to_le_bytes());
    for record in records {
        out.extend(record);
    }
    out.extend(record(0, &[]));
    out
}
pub fn setup() -> Vec<Vec<u8>> {
    vec![
        record(0x103, &[8]),
        record(0x20b, &[0, 0]),
        record(0x20c, &[720, 1440]),
        record(0x2fa, &[5, 0, 0, 0, 0]),
        record(0x12d, &[0]),
        record(0x2fc, &[0, 0x5a1e, 150, 0]),
        record(0x12d, &[1]),
    ]
}
pub fn rectangle() -> Vec<u8> {
    let mut r = setup();
    r.push(record(0x41b, &[540, 1260, 180, 180]));
    file(&r, None)
}
pub fn dib() -> Vec<u8> {
    let mut b = vec![0u8; 40];
    b[..4].copy_from_slice(&40u32.to_le_bytes());
    b[4..8].copy_from_slice(&2i32.to_le_bytes());
    b[8..12].copy_from_slice(&1i32.to_le_bytes());
    b[12..14].copy_from_slice(&1u16.to_le_bytes());
    b[14..16].copy_from_slice(&24u16.to_le_bytes());
    b.extend([0, 0, 255, 0, 255, 0, 0, 0]);
    b
}
pub fn bitmap(function: u16) -> Vec<u8> {
    let mut body = 0x00cc0020u32.to_le_bytes().to_vec();
    let words: &[i16] = match function {
        0xf43 => &[0, 1, 2, 0, 0, 360, 720, 180, 180],
        0xb41 => &[1, 2, 0, 0, 360, 720, 180, 180],
        _ => &[0, 0, 1, 2, 180, 180],
    };
    for w in words {
        body.extend(w.to_le_bytes());
    }
    body.extend(dib());
    packed(function, &body)
}
