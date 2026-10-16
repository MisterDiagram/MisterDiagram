//! Connection rows and fixed point glue, using the observed PAR(PNT(...)) formula.
use diagramcraft_model::{Cell, Connect, Document, Row, Section, Shape, Sheet};
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use std::collections::BTreeMap;

/// An endpoint of a one-dimensional shape.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
#[derive(Clone, Copy, Debug)]
pub enum End {
    Begin,
    End,
}
impl End {
    pub fn prefix(self) -> &'static str {
        match self {
            Self::Begin => "Begin",
            Self::End => "End",
        }
    }
    pub fn trigger(self) -> &'static str {
        match self {
            Self::Begin => "BegTrigger",
            Self::End => "EndTrigger",
        }
    }
    pub fn part(self) -> i32 {
        match self {
            Self::Begin => 9,
            Self::End => 12,
        }
    }
}
/// Authored local connection coordinates, direction, type and optional universal row name.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
#[derive(Clone, Debug)]
pub struct ConnectionPoint {
    pub name: Option<String>,
    pub x: Cell,
    pub y: Cell,
    pub direction: [f64; 2],
    pub kind: u8,
    pub auto_gen: bool,
    pub prompt: String,
}
pub(crate) fn cell(name: &str, value: impl ToString, formula: Option<String>) -> Cell {
    Cell { name: name.into(), value: Some(value.to_string()), formula, ..Default::default() }
}
pub(crate) fn set(sheet: &mut Sheet, c: Cell) -> Result<(), String> {
    if let Some(old) = sheet.cells.iter_mut().find(|old| old.name.eq_ignore_ascii_case(&c.name)) {
        *old = c;
    } else if sheet.cells.len() < 16384 {
        sheet.cells.push(c);
    } else {
        return Err("shape cell limit".into());
    }
    Ok(())
}
pub(crate) fn shape_mut(shapes: &mut [Shape], id: u32, depth: usize) -> Option<&mut Shape> {
    if depth > 128 {
        return None;
    }
    for shape in shapes {
        if shape.id == id {
            return Some(shape);
        }
        if let Some(found) = shape_mut(&mut shape.shapes, id, depth + 1) {
            return Some(found);
        }
    }
    None
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/dirxa-cell-connection-points-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/typec-cell-connection-points-section
/// Append a connection row, retaining the seven cells seen in static-glue.vsdx.
/// Coordinates are local to the target shape. Names select a named row; otherwise X1/Y1 syntax applies.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/x-cell-connection-points-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
pub fn add_connection(doc: &mut Document, page_index: usize, id: u32, mut point: ConnectionPoint) -> Result<String, String> {
    if point.kind > 2 || point.direction.iter().any(|v| !v.is_finite() || v.abs() > 1e6) || point.prompt.len() > 65536 {
        return Err("connection point value limit".into());
    }
    if point.name.as_ref().is_some_and(|n| n.is_empty() || n.len() > 128 || !n.chars().all(|c| c.is_alphanumeric() || c == '_')) {
        return Err("invalid universal connection row name".into());
    }
    for c in [&mut point.x, &mut point.y] {
        if let Some(f) = &c.formula {
            diagramcraft_sheet::parse(f).map_err(|e| e.to_string())?;
        }
        if !c.value.as_deref().and_then(|v| v.parse::<f64>().ok()).is_some_and(|v| v.is_finite() && v.abs() <= 1e6) {
            return Err("invalid connection coordinate".into());
        }
    }
    let page = doc.pages.get(page_index).ok_or("page missing")?;
    let e = Evaluator::new(doc).map_err(|e| e.to_string())?;
    let scope = Scope::PageShape { page: page.id, shape: id };
    let rows = e.resolved_named_cells(scope);
    let mut next = 0u32;
    for named in rows.iter().filter(|n| n.section.is_some_and(|s| s.name == "Connection")) {
        if let Some(row) = named.row {
            if point.name.is_some() && row.name.as_ref().zip(point.name.as_ref()).is_some_and(|(a, b)| a.eq_ignore_ascii_case(b)) {
                return Err("connection row exists".into());
            }
            next = next.max(row.index.unwrap_or(0).checked_add(1).ok_or("connection index overflow")?);
        }
    }
    if next >= 16384 {
        return Err("connection row limit".into());
    }
    let name = point.name.as_ref().map_or_else(|| format!("Connections.X{}", next + 1), |n| format!("Connections.{n}.X"));
    let page = doc.pages.get_mut(page_index).ok_or("page missing")?;
    let shape = shape_mut(&mut page.shapes, id, 0).ok_or("shape missing")?;
    if !shape.sheet.sections.iter().any(|s| s.name == "Connection" && !s.deleted) {
        if shape.sheet.sections.len() >= 16384 {
            return Err("section limit".into());
        }
        shape.sheet.sections.push(Section { name: "Connection".into(), ..Default::default() });
    }
    let section = shape.sheet.sections.iter_mut().find(|s| s.name == "Connection" && !s.deleted).ok_or("connection section missing")?;
    if section.rows.len() >= 16384 {
        return Err("connection row limit".into());
    }
    point.x.name = "X".into();
    point.y.name = "Y".into();
    let mut attributes = BTreeMap::from([("IX".into(), next.to_string()), ("T".into(), "Connection".into())]);
    if let Some(name) = &point.name {
        attributes.insert("N".into(), name.clone());
    }
    section.rows.push(Row {
        name: point.name,
        index: Some(next),
        row_type: Some("Connection".into()),
        attributes,
        cells: vec![
            point.x,
            point.y,
            cell("DirX", point.direction[0], None),
            cell("DirY", point.direction[1], None),
            cell("Type", point.kind, None),
            cell("AutoGen", u8::from(point.auto_gen), None),
            cell("Prompt", point.prompt, Some("No Formula".into())),
        ],
        ..Default::default()
    });
    Ok(name)
}
/// Glue an endpoint to a resolved connection X cell and add its Connect record.
/// The X and Y cells share the point-valued PAR(PNT(...)) formula.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
/// https://learn.microsoft.com/en-us/office/client-developer/visio/par-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pnt-function
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connect-element-connects_type-complextypevisio-xml
pub fn connect_static(doc: &mut Document, page_index: usize, id: u32, end: End, target: u32, point: &str) -> Result<(), String> {
    if id == target {
        return Err("cannot glue an endpoint to itself".into());
    }
    let page = doc.pages.get(page_index).ok_or("page missing")?;
    let mut e = Evaluator::new(doc).map_err(|e| e.to_string())?;
    let scope = Scope::PageShape { page: page.id, shape: id };
    if !["BeginX", "BeginY", "EndX", "EndY"].iter().all(|name| e.resolved_named_cells(scope).iter().any(|c| c.name == *name)) {
        return Err("glue source is not one-dimensional".into());
    }
    let scope = Scope::PageShape { page: page.id, shape: target };
    let named = e
        .resolved_named_cells(scope)
        .into_iter()
        .find(|n| n.name.eq_ignore_ascii_case(point) && n.cell.name == "X" && n.section.is_some_and(|s| s.name == "Connection"))
        .ok_or("connection X cell missing")?;
    let index = named.row.and_then(|r| r.index).unwrap_or(0);
    let point = named.name;
    let promote_target = e.evaluate(scope, "ObjType").numeric() == Ok(0.);
    let y = if let Some(prefix) = point.strip_suffix(".X") { format!("{prefix}.Y") } else { point.replacen("Connections.X", "Connections.Y", 1) };
    let formula = format!("PAR(PNT(Sheet.{target}!{point},Sheet.{target}!{y}))");
    let page = doc.pages.get_mut(page_index).ok_or("page missing")?;
    if page.connects.len() >= 100000 {
        return Err("connect count limit".into());
    }
    let shape = shape_mut(&mut page.shapes, id, 0).ok_or("connector missing")?;
    for axis in ["X", "Y"] {
        let name = format!("{}{axis}", end.prefix());
        let cached = shape.sheet.cell(&name).and_then(|c| c.value.clone()).unwrap_or_else(|| "0".into());
        set(&mut shape.sheet, cell(&name, cached, Some(formula.clone())))?;
    }
    set(&mut shape.sheet, cell(end.trigger(), 2, Some(format!("_XFTRIGGER(Sheet.{target}!EventXFMod)"))))?;
    let from_cell = format!("{}X", end.prefix());
    page.connects.retain(|c| !(c.from_sheet == id && c.from_cell.as_deref() == Some(&from_cell)));
    page.connects.push(Connect {
        from_sheet: id,
        from_cell: Some(from_cell),
        from_part: Some(end.part()),
        to_sheet: target,
        to_cell: Some(point),
        to_part: Some(i32::try_from(index).ok().and_then(|n| n.checked_add(100)).ok_or("connection part overflow")?),
        ..Default::default()
    });
    let target = shape_mut(&mut page.shapes, target, 0).ok_or("target missing")?;
    // Explicit ObjType flags, including the overriding &H4 bit, belong to the user.
    // https://learn.microsoft.com/en-us/office/client-developer/visio/objtype-cell-miscellaneous-section
    if promote_target {
        set(&mut target.sheet, cell("ObjType", 1, None))?;
    }
    Ok(())
}

/// Dynamic glue records the target pin and the paired trigger formulas used by Visio.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
/// https://learn.microsoft.com/en-us/office/client-developer/visio/walkpreference-cell-glue-info-section
pub fn connect_dynamic(doc: &mut Document, page_index: usize, id: u32, end: End, target: u32, vertical: bool) -> Result<(), String> {
    if id == target {
        return Err("cannot glue an endpoint to itself".into());
    }
    let page = doc.pages.get(page_index).ok_or("page missing")?;
    let mut e = Evaluator::new(doc).map_err(|e| e.to_string())?;
    let from = Scope::PageShape { page: page.id, shape: id };
    let to = Scope::PageShape { page: page.id, shape: target };
    let promote_target = e.evaluate(to, "ObjType").numeric() == Ok(0.);
    let default_policy = e.evaluate(from, "ConFixedCode").numeric() == Ok(0.)
        && e.shape(from).is_some_and(|s| s.sheet.cell("ConFixedCode").is_none_or(|c| c.formula.as_deref() == Some("Inh")));
    let initial_policy = if crate::orthogonal::style(&mut e, from, page.id) == 2. { 0 } else { 6 };
    let from_cells = e.resolved_named_cells(from);
    let to_cells = e.resolved_named_cells(to);
    if !["BeginX", "BeginY", "EndX", "EndY"].iter().all(|name| from_cells.iter().any(|n| n.name == *name)) {
        return Err("dynamic glue source is not one-dimensional".into());
    }
    if e.shape(to).is_none() || to_cells.iter().any(|n| n.name == "BeginX") {
        return Err("dynamic glue target must be two-dimensional".into());
    }
    let page = doc.pages.get_mut(page_index).ok_or("page missing")?;
    if page.connects.len() >= 100000 {
        return Err("connect count limit".into());
    }
    let shape = shape_mut(&mut page.shapes, id, 0).ok_or("connector missing")?;
    let formula = match end {
        End::Begin => "_WALKGLUE(BegTrigger,EndTrigger,WalkPreference)",
        End::End => "_WALKGLUE(EndTrigger,BegTrigger,WalkPreference)",
    };
    for axis in ["X", "Y"] {
        let name = format!("{}{axis}", end.prefix());
        let cached = shape.sheet.cell(&name).and_then(|c| c.value.clone()).unwrap_or_else(|| "0".into());
        set(&mut shape.sheet, cell(&name, cached, Some(formula.into())))?;
    }
    set(&mut shape.sheet, cell(end.trigger(), 2, Some(format!("_XFTRIGGER(Sheet.{target}!EventXFMod)"))))?;
    if default_policy {
        set(&mut shape.sheet, cell("ConFixedCode", initial_policy, None))?;
    }
    let from_cell = format!("{}X", end.prefix());
    page.connects.retain(|c| !(c.from_sheet == id && c.from_cell.as_deref() == Some(&from_cell)));
    page.connects.push(Connect {
        from_sheet: id,
        from_cell: Some(from_cell),
        from_part: Some(end.part()),
        to_sheet: target,
        to_cell: Some(if vertical { "PinY" } else { "PinX" }.into()),
        to_part: Some(3),
        ..Default::default()
    });
    let target = shape_mut(&mut page.shapes, target, 0).ok_or("target missing")?;
    // Explicit ObjType flags, including the overriding &H4 bit, belong to the user.
    // https://learn.microsoft.com/en-us/office/client-developer/visio/objtype-cell-miscellaneous-section
    if promote_target {
        set(&mut target.sheet, cell("ObjType", 1, None))?;
    }
    Ok(())
}
