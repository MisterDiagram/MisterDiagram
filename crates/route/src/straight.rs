//! Straight connector geometry in the connector's own coordinate frame.
use crate::{
    GlueHost,
    static_glue::{cell, set, shape_mut},
};
use diagramcraft_model::{Document, Page, Row, Section};
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use std::collections::BTreeMap;
fn number(e: &mut Evaluator<'_>, scope: Scope, name: &str) -> Result<f64, String> {
    let value = e.evaluate(scope, name).numeric().map_err(|e| e.to_string())?;
    if value.is_finite() && value.abs() <= 1e6 { Ok(value) } else { Err("route coordinate limit".into()) }
}
fn connector_layer(page: &mut Page) -> Result<u32, String> {
    let section = page.sheet.sections.iter().find(|s| s.name == "Layer" && !s.deleted);
    if let Some(row) = section.and_then(|s| {
        s.rows
            .iter()
            .find(|r| !r.deleted && r.cells.iter().any(|c| matches!(c.name.as_str(), "Name" | "NameUniv") && c.value.as_deref() == Some("Connector")))
    }) {
        return row.index.ok_or("connector layer index missing".into());
    }
    let index = section
        .map_or(Some(0), |s| s.rows.iter().filter_map(|r| r.index).max().map_or(Some(0), |i| i.checked_add(1)))
        .ok_or("layer index overflow")?;
    if index >= 16384 || page.sheet.sections.len() >= 16384 {
        return Err("layer count limit".into());
    }
    let mut row = diagramcraft_model::skeleton::layer(index, "Connector", true)?;
    for c in &mut row.cells {
        c.formula = None;
        match c.name.as_str() {
            "Color" => c.value = Some("255".into()),
            "Snap" | "Glue" => c.value = Some("1".into()),
            "NameUniv" => c.value = Some("Connector".into()),
            _ => {}
        }
    }
    if let Some(section) = page.sheet.sections.iter_mut().find(|s| s.name == "Layer" && !s.deleted) {
        if section.rows.len() >= 16384 {
            return Err("layer row limit".into());
        }
        section.rows.push(row);
    } else {
        page.sheet.sections.push(Section { name: "Layer".into(), rows: vec![row], ..Default::default() });
    }
    Ok(index)
}
fn row(index: u32, kind: &str, cells: Vec<diagramcraft_model::Cell>, deleted: bool) -> Row {
    Row {
        index: Some(index),
        row_type: Some(kind.into()),
        deleted,
        attributes: BTreeMap::from([("IX".into(), index.to_string()), ("T".into(), kind.into())]),
        cells,
        ..Default::default()
    }
}
/// Recompute a straight route when its endpoint or glue target changes, or when explicitly requested.
/// Four horizontal witnesses retain signed Width=EndX-BeginX, Height=0.25DL, and the deleted third row.
/// Other slopes use a rotated length-based frame so the rendered endpoint remains glued.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/width-cell-shape-transform-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/angle-cell-shape-transform-section
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.reroute
pub fn reroute(before: &Document, doc: &mut Document, host: &mut GlueHost, force: Option<(usize, Option<u32>)>) -> Result<usize, String> {
    let mut e = Evaluator::new(doc).map_err(|e| e.to_string())?.with_glue_host(host);
    let mut old = Evaluator::new(before).map_err(|e| e.to_string())?;
    let mut routes = vec![];
    for scope in e.scopes().collect::<Vec<_>>() {
        let Scope::PageShape { page, shape: id } = scope else { continue };
        if e.evaluate(scope, "ObjType").numeric() != Ok(2.) || e.cell(scope, "BeginX").is_none() {
            continue;
        }
        let index = doc.pages.iter().position(|p| p.id == page).ok_or("route page missing")?;
        let forced = force.is_some_and(|(p, id2)| p == index && id2.is_none_or(|v| v == id));
        let current = e.shape(scope).ok_or("route shape missing")?;
        let previous = old.shape(scope);
        let changed = previous.is_none_or(|old| {
            ["BeginX", "BeginY", "EndX", "EndY", "BegTrigger", "EndTrigger"].iter().any(|name| old.sheet.cell(name) != current.sheet.cell(name))
        });
        let changed = changed || ["BeginX", "BeginY", "EndX", "EndY"].iter().any(|name| e.evaluate(scope, name) != old.evaluate(scope, name));
        if !forced && !changed {
            continue;
        }
        let [bx, by, ex, ey] = ["BeginX", "BeginY", "EndX", "EndY"]
            .map(|name| number(&mut e, scope, name))
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| "endpoint count")?;
        let dx = ex - bx;
        let dy = ey - by;
        let horizontal = dy.abs() < 1e-12;
        let width = if horizontal { dx } else { dx.hypot(dy) };
        if !width.is_finite() || width.abs() > 1e6 {
            return Err("route length limit".into());
        }
        routes.push((index, id, bx, by, ex, ey, width, horizontal));
        if routes.len() > 100000 {
            return Err("route count limit".into());
        }
    }
    drop(e);
    let count = routes.len();
    for (index, id, bx, by, ex, ey, width, horizontal) in routes {
        let page = doc.pages.get_mut(index).ok_or("route page missing")?;
        let layer = connector_layer(page)?;
        let shape = shape_mut(&mut page.shapes, id, 0).ok_or("route shape missing")?;
        let height = 0.25;
        let width_formula = if horizontal { "GUARD(EndX-BeginX)" } else { "GUARD(SQRT((EndX-BeginX)^2+(EndY-BeginY)^2))" };
        for (name, value, formula) in [
            ("PinX", (bx + ex) * 0.5, "Inh"),
            ("PinY", (by + ey) * 0.5, "Inh"),
            ("Width", width, width_formula),
            ("Height", height, "GUARD(0.25DL)"),
            ("LocPinX", width * 0.5, "Inh"),
            ("LocPinY", height * 0.5, "Inh"),
            ("TxtPinX", width * 0.5, "Inh"),
            ("TxtPinY", height * 0.5, "Inh"),
        ] {
            set(&mut shape.sheet, cell(name, value, Some(formula.into())))?;
        }
        if !horizontal {
            set(&mut shape.sheet, cell("Angle", (ey - by).atan2(ex - bx), Some("GUARD(ATAN2(EndY-BeginY,EndX-BeginX))".into())))?;
        } else if shape.sheet.cell("Angle").is_some_and(|c| c.formula.as_deref() == Some("GUARD(ATAN2(EndY-BeginY,EndX-BeginX))")) {
            shape.sheet.cells.retain(|c| c.name != "Angle");
        }
        set(&mut shape.sheet, cell("LayerMember", layer, None))?;
        let geometry = Section {
            name: "Geometry".into(),
            index: Some(0),
            rows: vec![
                row(1, "MoveTo", vec![cell("Y", height / 2., None)], false),
                row(2, "LineTo", vec![cell("X", width, None), cell("Y", height / 2., None)], false),
                row(3, "LineTo", vec![], true),
            ],
            ..Default::default()
        };
        if let Some(old) = shape.sheet.sections.iter_mut().find(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0) {
            *old = geometry;
        } else {
            shape.sheet.sections.push(geometry);
        }
        let control = Row {
            name: Some("TextPosition".into()),
            cells: vec![
                cell("X", width / 2., None),
                cell("Y", height / 2., None),
                cell("XDyn", width / 2., Some("Inh".into())),
                cell("YDyn", height / 2., Some("Inh".into())),
            ],
            ..Default::default()
        };
        if let Some(section) = shape.sheet.sections.iter_mut().find(|s| s.name == "Control" && !s.deleted) {
            if let Some(old) = section.rows.iter_mut().find(|r| r.name.as_deref() == Some("TextPosition")) {
                *old = control;
            } else {
                section.rows.push(control);
            }
        } else {
            shape.sheet.sections.push(Section { name: "Control".into(), rows: vec![control], ..Default::default() });
        }
    }
    Ok(count)
}
