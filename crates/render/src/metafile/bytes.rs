//! Retargeted from vectorcraft crates/metafile/src/bytes.rs,
//! 806b1a6972288319577d75d097c140f9bf530ce0, MIT OR Apache-2.0.
//! Little-endian values: the buffer both writers build records in, and a reader that never reads
//! past its data (every read is an `Option`).

/// Sequential little-endian reads from a slice.
#[derive(Clone, Copy)]
pub(crate) struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    pub fn bytes(&mut self, n: usize) -> Option<&'a [u8]> {
        let b = self.data.get(self.pos..self.pos.checked_add(n)?)?;
        self.pos += n;
        Some(b)
    }
    fn array<const N: usize>(&mut self) -> Option<[u8; N]> {
        self.bytes(N)?.try_into().ok()
    }
    pub fn u16(&mut self) -> Option<u16> {
        self.array().map(u16::from_le_bytes)
    }
    pub fn i16(&mut self) -> Option<i16> {
        self.array().map(i16::from_le_bytes)
    }
    pub fn u32(&mut self) -> Option<u32> {
        self.array().map(u32::from_le_bytes)
    }
    pub fn i32(&mut self) -> Option<i32> {
        self.array().map(i32::from_le_bytes)
    }
    pub fn f32(&mut self) -> Option<f32> {
        self.array().map(f32::from_le_bytes)
    }
    pub fn skip(&mut self, n: usize) -> Option<()> {
        self.bytes(n).map(drop)
    }
    /// The bytes not read yet.
    pub fn rest(&self) -> &'a [u8] {
        self.data.get(self.pos..).unwrap_or_default()
    }
}

/// `len` bytes of `data` from `offset` (record-relative offsets read from untrusted files).
pub(crate) fn slice(data: &[u8], offset: u32, len: u32) -> Option<&[u8]> {
    let start = usize::try_from(offset).ok()?;
    data.get(start..start.checked_add(usize::try_from(len).ok()?)?)
}
