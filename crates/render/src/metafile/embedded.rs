//! Embedded EMF comment streams, bounded independently of the fallback WMF player.
//! https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-wmf/cfc88064-d86d-4b52-9374-3ce27d456179

use super::{bytes::Reader, header};

pub(super) const INCOMPLETE: &str = "WMF comment EMF incomplete, playing the WMF fallback";

pub(super) enum Embedded {
    Absent,
    Incomplete,
    Complete(Vec<u8>),
}

struct Chunk<'a> {
    count: usize,
    remaining: usize,
    total: usize,
    data: &'a [u8],
}

fn chunk(body: &[u8]) -> Option<Chunk<'_>> {
    let mut r = Reader::new(body);
    if r.u16()? != 15 {
        return None;
    }
    let byte_count = usize::from(r.u16()?);
    if r.u32()? != 0x4346_4d57 || r.u32()? != 1 || r.u32()? != 0x0001_0000 {
        return None;
    }
    r.u16()?; // The native checksum covers the entire outer stream, not this chunk.
    if r.u32()? != 0 {
        return None;
    }
    let count = usize::try_from(r.u32()?).ok()?;
    let size = usize::try_from(r.u32()?).ok()?;
    let remaining = usize::try_from(r.u32()?).ok()?;
    let total = usize::try_from(r.u32()?).ok()?;
    if count == 0 || size == 0 || size > 8192 || total > header::BYTE_LIMIT || total == 0 {
        return None;
    }
    // Native imports accept both the actual record size and the reference's total-size form.
    if byte_count != size.checked_add(34)? && byte_count != total.checked_add(34)? {
        return None;
    }
    let data = r.bytes(size)?;
    if r.rest().len() != size % 2 {
        return None;
    }
    Some(Chunk { count, remaining, total, data })
}

/// Select a complete, consecutive comment sequence before playing any fallback record.
/// Native synthetic imports require the first record and a zero additive WORD checksum
/// over the complete WMF. See the recorded observation in docs/file-format-parity.md.
pub(super) fn read(bytes: &[u8], start: usize, budget: &mut super::SceneBudget) -> Result<Embedded, String> {
    let mut at = start;
    while let Some((function, body, next)) = header::record(bytes, at) {
        if function == 0 {
            break;
        }
        if function == 0x0626 && body.get(..2) == Some(&15u16.to_le_bytes()) {
            if at != start {
                return Ok(Embedded::Incomplete);
            }
            return assemble(bytes, start, budget);
        }
        at = next;
    }
    // A physically truncated first escape still needs a diagnostic.
    if bytes.get(start + 4..start + 8) == Some(&[0x26, 0x06, 0x0f, 0x00]) {
        return Ok(Embedded::Incomplete);
    }
    Ok(Embedded::Absent)
}

fn assemble(bytes: &[u8], start: usize, budget: &mut super::SceneBudget) -> Result<Embedded, String> {
    let Some((_, body, _)) = header::record(bytes, start) else { return Ok(Embedded::Incomplete) };
    let Some(first) = chunk(body) else { return Ok(Embedded::Incomplete) };
    let Some(stream) = bytes.get(start.saturating_sub(18)..) else { return Ok(Embedded::Incomplete) };
    if stream.len() % 2 != 0
        || stream.as_chunks::<2>().0.iter().fold(0u16, |sum, pair| sum.wrapping_add(u16::from_le_bytes([pair[0], pair[1]]))) != 0
        || first.total > stream.len()
        || first.count > stream.len() / 44
    {
        return Ok(Embedded::Incomplete);
    }
    budget.charge_bytes(first.total)?;
    let mut data = Vec::new();
    data.try_reserve_exact(first.total).map_err(|_| super::EMBEDDED_ALLOCATION_LIMIT)?;
    let mut at = start;
    for _ in 0..first.count {
        let Some((0x0626, body, next)) = header::record(bytes, at) else { return Ok(Embedded::Incomplete) };
        let Some(part) = chunk(body) else { return Ok(Embedded::Incomplete) };
        let Some(end) = data.len().checked_add(part.data.len()) else { return Ok(Embedded::Incomplete) };
        if part.count != first.count || part.total != first.total || first.total.checked_sub(end) != Some(part.remaining) {
            return Ok(Embedded::Incomplete);
        }
        data.extend_from_slice(part.data);
        at = next;
    }
    if data.len() != first.total {
        return Ok(Embedded::Incomplete);
    }
    if header::record(bytes, at).is_some_and(|(function, body, _)| function == 0x0626 && body.get(..2) == Some(&15u16.to_le_bytes())) {
        return Ok(Embedded::Incomplete);
    }
    if header::sniff(&data) != Some(header::Kind::Emf) {
        return Err("WMF comment contains an invalid EMF header".into());
    }
    Ok(Embedded::Complete(data))
}
