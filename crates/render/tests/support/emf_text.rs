//! Contributor-original synthetic record writer, MIT OR Apache-2.0.
//! https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/dd585d0a-5d7c-4034-963a-1141af836972
#![allow(dead_code)]
fn put(out: &mut [u8], at: usize, value: u32) {
    out[at..at + 4].copy_from_slice(&value.to_le_bytes());
}
pub fn record(kind: u32, body: &[u8]) -> Vec<u8> {
    let length = (8 + body.len() + 3) & !3;
    let mut out = vec![0; length];
    put(&mut out, 0, kind);
    put(&mut out, 4, length as u32);
    out[8..8 + body.len()].copy_from_slice(body);
    out
}
fn word_record(kind: u32, value: u32) -> Vec<u8> {
    record(kind, &value.to_le_bytes())
}
pub fn font(handle: u32, weight: u32, charset: u8) -> Vec<u8> {
    let mut body = vec![0; 324];
    put(&mut body, 0, handle);
    put(&mut body, 4, (-16i32) as u32);
    put(&mut body, 20, weight);
    body[27] = charset;
    for (i, c) in "Arial".encode_utf16().enumerate() {
        body[32 + i * 2..34 + i * 2].copy_from_slice(&c.to_le_bytes());
    }
    record(82, &body)
}
pub fn text(at: (i32, i32), units: &[u16], options: u32) -> Vec<u8> {
    let mut body = vec![0; 68];
    put(&mut body, 16, 1); // GM_COMPATIBLE
    put(&mut body, 20, 1f32.to_bits());
    put(&mut body, 24, 1f32.to_bits());
    put(&mut body, 28, at.0 as u32);
    put(&mut body, 32, at.1 as u32);
    put(&mut body, 36, units.len() as u32);
    put(&mut body, 40, 76);
    put(&mut body, 44, options);
    for (i, v) in [20, 40, 360, 80].into_iter().enumerate() {
        put(&mut body, 48 + i * 4, v);
    }
    body.extend(units.iter().flat_map(|c| c.to_le_bytes()));
    record(84, &body)
}
pub fn file(records: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = vec![0; 88];
    for (at, value) in [
        (0, 1),
        (4, 88),
        (16, 384),
        (20, 192),
        (32, 10160),
        (36, 5080),
        (40, 0x464d4520),
        (44, 0x10000),
        (56, 4),
        (72, 960),
        (76, 960),
        (80, 254),
        (84, 254),
    ] {
        put(&mut bytes, at, value);
    }
    for r in records {
        bytes.extend(r);
    }
    bytes.extend(record(14, &[0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0]));
    let length = bytes.len() as u32;
    put(&mut bytes, 48, length);
    put(&mut bytes, 52, records.len() as u32 + 2);
    bytes
}
pub fn fixture() -> Vec<u8> {
    file(&[
        font(1, 400, 0),
        word_record(37, 1),
        word_record(22, 0),
        text((20, 20), &"Searchable 144F".encode_utf16().collect::<Vec<_>>(), 0),
        font(2, 700, 0),
        word_record(37, 2),
        word_record(22, 6),
        text((192, 50), &"Centred".encode_utf16().collect::<Vec<_>>(), 4),
        font(3, 400, 2),
        word_record(37, 3),
        word_record(22, 0),
        text((20, 95), &"ab".encode_utf16().collect::<Vec<_>>(), 0),
        word_record(37, 1),
        text((20, 140), &[65, 66], 0x10),
    ])
}
