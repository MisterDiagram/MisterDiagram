//! Allocation preflight for ShapeSheet XML, before either DOM is constructed.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/section-element-sheet_type-complextypevisio-xml
use crate::{ReadError, bad};
use quick_xml::{Reader, events::Event};

pub(crate) const MAX_SECTION_CELLS: usize = 16_384;
pub(crate) const MAX_SECTION_ROWS: usize = 16_384;
pub(crate) const MAX_SHEET_CELLS: usize = 65_536;
pub(crate) const MAX_SHEET_BYTES: usize = 16 * 1024 * 1024;

#[derive(Default)]
struct Frame {
    start: u64,
    sheet: bool,
    section: bool,
    cells: usize,
    rows: usize,
}

/// Streaming events borrow the input. The only retained state is a bounded depth stack.
pub(crate) fn preflight(bytes: &[u8]) -> Result<(), ReadError> {
    if bytes.len() > misterdiagram_opc::MAX_XML_PART {
        return Err(bad("XML part byte limit"));
    }
    let mut reader = Reader::from_reader(bytes);
    let mut stack: Vec<Frame> = Vec::new();
    loop {
        let start = reader.buffer_position();
        let event = reader.read_event().map_err(|e| bad(format!("XML preflight: {e}")))?;
        let close = matches!(&event, Event::End(_) | Event::Empty(_));
        match &event {
            Event::Start(e) | Event::Empty(e) => {
                if stack.len() >= misterdiagram_opc::xml::MAX_DEPTH {
                    return Err(bad("sheet depth limit"));
                }
                let local = e.local_name();
                let name = local.as_ref();
                for frame in stack.iter_mut().filter(|f| f.section) {
                    frame.cells += usize::from(name == b"Cell");
                    frame.rows += usize::from(name == b"Row");
                    if frame.cells > MAX_SECTION_CELLS || frame.rows > MAX_SECTION_ROWS {
                        return Err(bad("section cell or row limit"));
                    }
                }
                if name == b"Cell"
                    && let Some(frame) = stack.iter_mut().rev().find(|f| f.sheet)
                {
                    frame.cells += 1;
                    if frame.cells > MAX_SHEET_CELLS {
                        return Err(bad("sheet cell limit"));
                    }
                }
                stack.push(Frame {
                    start,
                    sheet: matches!(name, b"Shape" | b"PageSheet" | b"DocumentSheet" | b"StyleSheet"),
                    section: name == b"Section",
                    ..Default::default()
                });
            }
            Event::Eof => break,
            _ => {}
        }
        let end = reader.buffer_position();
        if stack.iter().any(|f| f.sheet && end.saturating_sub(f.start) > MAX_SHEET_BYTES as u64) {
            return Err(bad("sheet byte limit"));
        }
        if close {
            stack.pop();
        }
    }
    Ok(())
}
