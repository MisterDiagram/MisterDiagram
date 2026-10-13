//! Literal local caches for the acceptance reporter, with no formula evaluation.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
use super::{display_name, escape, io_error};
use diagramcraft_model::{Cell, Document, Shape, Sheet};
use std::io::Write;

/// Missing V/F/U/E are distinguished from empty attributes by a presence bit mask.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
pub(super) fn dump(document: &Document, out: &mut impl Write) -> Result<(), String> {
    crate::write::dump_bounds(document).map_err(|e| e.to_string())?;
    writeln!(out, "scope\towner\tshape\tname\tV\tF\tU\tE\tpresent\tactive").map_err(io_error)?;
    sheet("document", 0, 0, &document.document_sheet, true, out)?;
    for style in &document.styles {
        sheet("style", style.id, style.id, &style.sheet, true, out)?;
    }
    for page in &document.pages {
        sheet("page", page.id, 0, &page.sheet, true, out)?;
        shapes("page-shape", page.id, &page.shapes, out)?;
    }
    for master in &document.masters {
        sheet("master", master.id, master.id, &master.sheet, true, out)?;
        shapes("master-shape", master.id, &master.shapes, out)?;
    }
    Ok(())
}
fn shapes(kind: &str, owner: u32, shapes: &[Shape], out: &mut impl Write) -> Result<(), String> {
    let mut stack: Vec<_> = shapes.iter().rev().map(|s| (s, true)).collect();
    while let Some((s, active)) = stack.pop() {
        let active = active && s.sheet.attributes.get("Del").is_none_or(|v| v != "1" && v != "true");
        sheet(kind, owner, s.id, &s.sheet, active, out)?;
        stack.extend(s.shapes.iter().rev().map(|s| (s, active)));
    }
    Ok(())
}
fn sheet(kind: &str, owner: u32, shape: u32, sheet: &Sheet, active: bool, out: &mut impl Write) -> Result<(), String> {
    let mut emit = |name: &str, c: &Cell, active: bool| -> Result<(), String> {
        let values = [&c.value, &c.formula, &c.units, &c.error];
        let present = values.iter().enumerate().fold(0u8, |bits, (i, v)| bits | (u8::from(v.is_some()) << i));
        writeln!(
            out,
            "{kind}\t{owner}\t{shape}\t{}\t{}\t{}\t{}\t{}\t{present}\t{}",
            escape(&display_name(name)),
            escape(c.value.as_deref().unwrap_or_default()),
            escape(c.formula.as_deref().unwrap_or_default()),
            escape(c.units.as_deref().unwrap_or_default()),
            escape(c.error.as_deref().unwrap_or_default()),
            u8::from(active)
        )
        .map_err(io_error)
    };
    for c in &sheet.cells {
        emit(&c.name, c, active)?;
    }
    for section in &sheet.sections {
        let active = active && !section.deleted;
        let prefix = crate::evaluate::section_prefix(section).map_err(|e| e.to_string())?;
        for c in &section.cells {
            emit(&format!("{prefix}.{}", c.name), c, active)?;
        }
        for (i, row) in section.rows.iter().enumerate() {
            for c in &row.cells {
                emit(&crate::evaluate::cell_name(section, row, i, c).map_err(|e| e.to_string())?, c, active && !row.deleted)?;
            }
        }
    }
    Ok(())
}
