//! Bounded command helpers for model edits.
use crate::{Result, Session, operation};
use diagramcraft_model::{Cell, Document, Page, Shape, Sheet};
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use serde_json::Value;

pub(crate) fn number(p: &Value, key: &str) -> Result<f64> {
    p.get(key)
        .and_then(Value::as_f64)
        .filter(|v| v.is_finite() && v.abs() <= 1e6)
        .ok_or_else(|| operation(format!("{key} must be a finite coordinate within one million inches")))
}
pub(crate) fn uint(p: &Value, key: &str) -> Result<u32> {
    p.get(key)
        .and_then(Value::as_u64)
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| operation(format!("{key} must be a 32-bit nonnegative integer")))
}
pub(crate) fn name<'a>(p: &'a Value, key: &str) -> Result<&'a str> {
    p.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty() && s.len() <= 65536 && !s.contains('\0'))
        .ok_or_else(|| operation(format!("missing or invalid {key}")))
}
pub(crate) fn page_index(s: &Session, p: &Value) -> Result<usize> {
    let index = p
        .get("page")
        .map(|_| {
            uint(p, "page")
                .and_then(|v| usize::try_from(v).map_err(operation))
                .and_then(|v| v.checked_sub(1).ok_or_else(|| operation("page is one-based")))
        })
        .transpose()?
        .unwrap_or(s.selection.page);
    if index >= s.document.pages.len() {
        return Err(operation("page index does not exist"));
    }
    Ok(index)
}
pub(crate) fn page_mut(doc: &mut Document, index: usize) -> Result<&mut Page> {
    doc.pages.get_mut(index).ok_or_else(|| operation("page index does not exist"))
}
pub(crate) fn shape_mut(shapes: &mut [Shape], id: u32) -> Result<&mut Shape> {
    fn find(shapes: &mut [Shape], id: u32, depth: usize) -> Option<&mut Shape> {
        if depth > 128 {
            return None;
        }
        for shape in shapes {
            if shape.id == id {
                return Some(shape);
            }
            if let Some(found) = find(&mut shape.shapes, id, depth + 1) {
                return Some(found);
            }
        }
        None
    }
    find(shapes, id, 0).ok_or_else(|| operation(format!("shape {id} does not exist")))
}
pub(crate) fn next_id(page: &Page) -> Result<u32> {
    let mut pending: Vec<_> = page.shapes.iter().map(|s| (s, 0usize)).collect();
    let mut maximum = 0u32;
    let mut count = 0usize;
    while let Some((shape, depth)) = pending.pop() {
        count += 1;
        if count >= 100_000 || depth > 128 || pending.len().saturating_add(shape.shapes.len()).saturating_add(count) >= 100_000 {
            return Err(operation("shape count or nesting limit"));
        }
        if shape.id != u32::MAX {
            maximum = maximum.max(shape.id);
        }
        pending.extend(shape.shapes.iter().map(|s| (s, depth + 1)));
    }
    maximum.checked_add(1).filter(|v| *v != u32::MAX).ok_or_else(|| operation("shape ID limit"))
}
pub(crate) fn cell(name: &str, value: &str, formula: Option<&str>) -> Cell {
    Cell { name: name.into(), value: Some(value.into()), formula: formula.map(str::to_owned), ..Default::default() }
}
pub(crate) fn set(sheet: &mut Sheet, name: &str, value: &str, formula: Option<&str>) -> Result<()> {
    if let Some(c) = sheet.cells.iter_mut().find(|c| c.name.eq_ignore_ascii_case(name)) {
        c.value = Some(value.into());
        c.formula = formula.map(str::to_owned);
        c.error = None;
    } else {
        if sheet.cells.len() >= 16384 {
            return Err(operation("sheet cell limit"));
        }
        sheet.cells.push(cell(name, value, formula));
    }
    Ok(())
}
pub(crate) fn set_number(sheet: &mut Sheet, name: &str, value: f64) -> Result<()> {
    if !value.is_finite() || value.abs() > 1e6 {
        return Err(operation("computed coordinate limit"));
    }
    let value = diagramcraft_model::skeleton::number_text(value).ok_or_else(|| operation("nonfinite computed coordinate"))?;
    set(sheet, name, &value, None)
}
pub(crate) fn evaluated(doc: &Document, scope: Scope, name: &str) -> Result<f64> {
    let themes = diagramcraft_theme::Themes::from_document(doc).map_err(operation)?;
    let mut evaluator = Evaluator::new(doc).map_err(operation)?.with_theme_host(&themes).with_text_host(&diagramcraft_text::Host);
    let value = evaluator.evaluate(scope, name);
    match value.numeric() {
        Ok(v) if v.is_finite() && v.abs() <= 1e6 => Ok(v),
        _ => Err(operation(format!("cannot evaluate {name}: {}", value.text()))),
    }
}
pub(crate) fn shape_scope(s: &Session, p: &Value) -> Result<(usize, u32, Scope)> {
    let index = page_index(s, p)?;
    let id = uint(p, "id")?;
    let page = s.document.pages.get(index).ok_or_else(|| operation("page not found"))?;
    let scope = Scope::PageShape { page: page.id, shape: id };
    let e = Evaluator::new(&s.document).map_err(operation)?;
    if e.shape(scope).is_none() {
        return Err(operation("shape not found"));
    }
    Ok((index, id, scope))
}
