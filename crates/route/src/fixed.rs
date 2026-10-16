//! A never-reroute connector follows its glued ends without replacing its path.
//! Witness: generated/review-a/w4-never-reroute-before and -after.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/confixedcode-cell-shape-layout-section
use crate::static_glue::shape_mut;
use diagramcraft_geom::Point;
use diagramcraft_model::{Cell, Document, Row, Section};
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use std::collections::BTreeMap;

pub(crate) struct EndUpdate {
    page: usize,
    shape: u32,
    index: u32,
    kind: Option<String>,
    cells: Vec<Cell>,
}
/// Keep all interior rows and formulas; only literal terminal coordinates follow glue.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/moveto-row-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/lineto-row-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/relmoveto-row-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/rellineto-row-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/relcubbezto-row-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/relquadbezto-row-geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/relellipticalarcto-row-geometry-section
pub(crate) fn end_updates(e: &mut Evaluator<'_>, page: usize, scope: Scope, ends: [f64; 4]) -> Result<Vec<EndUpdate>, String> {
    let Scope::PageShape { shape, .. } = scope else { return Err("fixed connector scope".into()) };
    let mut rows = BTreeMap::new();
    for n in e.resolved_named_cells(scope) {
        if n.section.is_some_and(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0)
            && let Some(row) = n.row.filter(|r| !r.deleted)
        {
            if rows.len() >= 16384 && !rows.contains_key(&row.index.unwrap_or(0)) {
                return Err("fixed connector row limit".into());
            }
            rows.insert(row.index.unwrap_or(0), row.row_type.clone());
        }
    }
    let (Some(first), Some(last)) = (rows.first_key_value(), rows.last_key_value()) else { return Ok(vec![]) };
    if first.0 == last.0 {
        return Ok(vec![]);
    }
    let frame = e.shape_to_page(scope).map_err(|v| v.to_string())?;
    if !frame.determinant().is_finite() || frame.determinant().abs() < 1e-15 {
        return Err("fixed connector transform limit".into());
    }
    let local = frame.inverse() * e.parent_to_page(scope).map_err(|v| v.to_string())?;
    let [bx, by, ex, ey] = ends;
    let mut updates = vec![];
    for ((ix, kind), point) in [(first, Point::new(bx, by)), (last, Point::new(ex, ey))] {
        let point = local * point;
        let mut cells = vec![];
        for (axis, coordinate) in [("X", point.x), ("Y", point.y)] {
            let value = if matches!(kind.as_deref(), Some("RelMoveTo" | "RelLineTo" | "RelCubBezTo" | "RelQuadBezTo" | "RelEllipticalArcTo")) {
                let extent = e.evaluate(scope, if axis == "X" { "Width" } else { "Height" }).numeric().map_err(|v| v.to_string())?;
                if extent == 0. && coordinate.abs() < 1e-12 { 0. } else { coordinate / extent }
            } else {
                coordinate
            };
            if !value.is_finite() || value.abs() > 1e6 {
                return Err("fixed connector coordinate limit".into());
            }
            let name = format!("Geometry1.{axis}{ix}");
            let mut cell = e.cell(scope, &name).cloned().unwrap_or_else(|| Cell { name: axis.into(), ..Default::default() });
            // Formula coordinates already follow the changing shape frame. Preserve them.
            if cell.formula.as_deref().is_some_and(|f| !f.is_empty() && !f.eq_ignore_ascii_case("No Formula")) {
                continue;
            }
            let value = diagramcraft_model::skeleton::number_text(value).ok_or("fixed connector coordinate")?;
            if cell.value.as_deref() == Some(&value) {
                continue;
            }
            cell.value = Some(value);
            cells.push(cell);
        }
        if !cells.is_empty() {
            updates.push(EndUpdate { page, shape, index: *ix, kind: kind.clone(), cells });
        }
    }
    Ok(updates)
}
pub(crate) fn apply(doc: &mut Document, updates: Vec<EndUpdate>) -> Result<(), String> {
    for update in updates {
        let page = doc.pages.get_mut(update.page).ok_or("fixed connector page missing")?;
        let sheet = &mut shape_mut(&mut page.shapes, update.shape, 0).ok_or("fixed connector missing")?.sheet;
        if !sheet.sections.iter().any(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0) {
            if sheet.sections.len() >= 16384 {
                return Err("fixed connector section limit".into());
            }
            sheet.sections.push(Section { name: "Geometry".into(), index: Some(0), ..Default::default() });
        }
        let section =
            sheet.sections.iter_mut().find(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0).ok_or("fixed connector geometry missing")?;
        if !section.rows.iter().any(|r| r.index == Some(update.index)) {
            if section.rows.len() >= 16384 {
                return Err("fixed connector row limit".into());
            }
            section.rows.push(Row { index: Some(update.index), row_type: update.kind, ..Default::default() });
        }
        let row = section.rows.iter_mut().find(|r| r.index == Some(update.index)).ok_or("fixed connector row missing")?;
        for cell in update.cells {
            if let Some(old) = row.cells.iter_mut().find(|c| c.name == cell.name) {
                *old = cell;
            } else {
                if row.cells.len() >= 16384 {
                    return Err("fixed connector cell limit".into());
                }
                row.cells.push(cell);
            }
        }
    }
    Ok(())
}
