//! Bytes-only metafile identification shared with foreign glue, without a sideways crate dependency.
//! WMF header identification retargeted from vectorcraft crates/metafile/src/wmf.rs,
//! 806b1a6972288319577d75d097c140f9bf530ce0, MIT OR Apache-2.0.
//! https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/d169108a-e3fe-436a-bb44-bea61a46ce56

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Emf,
    Wmf,
}
pub const PLACEABLE_KEY: u32 = 0x9AC6_CDD7;
pub const BYTE_LIMIT: usize = 128 * 1024 * 1024;
fn word(bytes: &[u8], at: usize) -> Option<u16> {
    bytes.get(at..at.checked_add(2)?).and_then(|b| b.try_into().ok()).map(u16::from_le_bytes)
}
fn dword(bytes: &[u8], at: usize) -> Option<u32> {
    bytes.get(at..at.checked_add(4)?).and_then(|b| b.try_into().ok()).map(u32::from_le_bytes)
}
fn meta_header(bytes: &[u8]) -> bool {
    matches!(word(bytes, 0), Some(1 | 2)) && word(bytes, 2) == Some(9) && matches!(word(bytes, 4), Some(0x0100 | 0x0300))
}
/// Identify the bytes, independently of part names, MIME types and ForeignType labels.
/// A recognized but truncated header is left to the bounded parser to reject.
pub fn sniff(bytes: &[u8]) -> Option<Kind> {
    if bytes.get(40..44) == Some(b" EMF") {
        Some(Kind::Emf)
    } else if dword(bytes, 0) == Some(PLACEABLE_KEY) || meta_header(bytes) {
        Some(Kind::Wmf)
    } else {
        None
    }
}
/// A whole, advancing record. Its parameters never include bytes from the next record.
/// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/28fbdde6-22ee-4e49-b162-abf826f5bc00
pub fn record(bytes: &[u8], at: usize) -> Option<(u16, &[u8], usize)> {
    let size = usize::try_from(dword(bytes, at)?).ok()?.checked_mul(2)?;
    if size < 6 {
        return None;
    }
    let next = at.checked_add(size)?;
    let data = bytes.get(at..next)?;
    Some((word(data, 4)?, data.get(6..)?, next))
}
/// Header data in logical units. Signed extents retain axis reversals.
#[derive(Debug)]
pub struct WmfFrame {
    pub start: usize,
    pub rect: Option<[f64; 4]>,
    pub inch: f64,
}
fn signed(bytes: &[u8], at: usize) -> Option<f64> {
    bytes.get(at..at.checked_add(2)?).and_then(|b| b.try_into().ok()).map(i16::from_le_bytes).map(f64::from)
}
/// Placeable frame or first bounded SETWINDOWORG/SETWINDOWEXT pair, at 1440 units/inch.
/// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/ba5458c6-e885-41e6-b5d7-d54ef9e1065f
pub fn wmf_frame(bytes: &[u8]) -> Result<WmfFrame, String> {
    if bytes.len() > BYTE_LIMIT {
        return Err("metafile byte limit".into());
    }
    let placeable = dword(bytes, 0) == Some(PLACEABLE_KEY);
    let base = if placeable { 22 } else { 0 };
    bytes.get(base..base + 18).filter(|h| meta_header(h)).ok_or("the WMF header is damaged")?;
    let start = base + 18;
    if placeable {
        let fields = [6, 8, 10, 12].map(|at| signed(bytes, at));
        if let [Some(l), Some(t), Some(r), Some(b)] = fields
            && l != r
            && t != b
        {
            let inch = word(bytes, 14).filter(|n| *n != 0).map_or(1440., f64::from);
            return Ok(WmfFrame { start, rect: Some([l, t, r, b]), inch });
        }
    }
    let (mut origin, mut extent) = (None, None);
    let mut at = start;
    while let Some((function, body, next)) = record(bytes, at) {
        if function == 0 {
            break;
        }
        let point = || Some((signed(body, 2)?, signed(body, 0)?));
        match function {
            0x020b if origin.is_none() => origin = point(),
            0x020c if extent.is_none() => extent = point(),
            _ => {}
        }
        if origin.is_some() && extent.is_some() {
            break;
        }
        at = next;
    }
    let (x, y) = origin.unwrap_or((0., 0.));
    let rect = extent.filter(|(w, h)| *w != 0. && *h != 0.).map(|(w, h)| [x, y, x + w, y + h]);
    Ok(WmfFrame { start, rect, inch: 1440. })
}
