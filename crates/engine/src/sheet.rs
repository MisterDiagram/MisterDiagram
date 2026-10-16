//! Formula-level ShapeSheet edits use universal cell names and bounded parsing.
use crate::{Result, Session, edit::*, operation};
use diagramcraft_model::{Cell, Document, Row, Section, Sheet};
use diagramcraft_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};

pub(crate) fn target(s: &Session, p: &Value) -> Result<Scope> {
    let scope = p.get("scope").and_then(Value::as_str);
    if scope == Some("shape") && p.get("id").is_none() {
        return Err(operation("shape scope requires id"));
    }
    if matches!(scope, Some("document" | "page")) && p.get("id").is_some() {
        return Err(operation("id is only valid in shape scope"));
    }
    if scope == Some("document") {
        return Ok(Scope::Document);
    }
    let i = page_index(s, p)?;
    let page = s.document.pages.get(i).ok_or_else(|| operation("page not found"))?;
    if p.get("id").is_some() { Ok(Scope::PageShape { page: page.id, shape: uint(p, "id")? }) } else { Ok(Scope::Page(page.id)) }
}
pub(crate) fn sheet_mut(doc: &mut Document, scope: Scope) -> Result<&mut Sheet> {
    match scope {
        Scope::Document => Ok(&mut doc.document_sheet),
        Scope::Page(id) => doc.pages.iter_mut().find(|p| p.id == id).map(|p| &mut p.sheet).ok_or_else(|| operation("page not found")),
        Scope::PageShape { page, shape } => {
            let page = doc.pages.iter_mut().find(|p| p.id == page).ok_or_else(|| operation("page not found"))?;
            Ok(&mut shape_mut(&mut page.shapes, shape)?.sheet)
        }
        _ => Err(operation("unsupported sheet scope")),
    }
}
#[derive(Clone)]
pub(crate) struct Address {
    section: Option<Section>,
    row: Option<Row>,
    pub(crate) cell: Cell,
}
fn prefix(value: &str) -> &str {
    match value {
        "Char" => "Character",
        "Para" => "Paragraph",
        "Fields" => "Field",
        "Prop" => "Property",
        "Controls" => "Control",
        "Connections" => "Connection",
        "Actions" => "Action",
        "Layers" => "Layer",
        "FillGradientStops" => "FillGradient",
        "LineGradientStops" => "LineGradient",
        v => v,
    }
}
pub(crate) fn address(doc: &Document, scope: Scope, name: &str) -> Result<Address> {
    let e = Evaluator::new(doc).map_err(operation)?;
    let named = e
        .resolved_named_cells(scope)
        .into_iter()
        .find(|c| c.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| operation("cell does not exist; add its section and row first"))?;
    let mut cell = named.cell.clone();
    cell.attributes.retain(|k, _| !k.starts_with("Com"));
    cell.ref_by.clear();
    let section = named.section.map(|s| Section { name: s.name.clone(), index: s.index, ..Default::default() });
    let row = named.row.map(|r| Row { name: r.name.clone(), index: r.index, row_type: r.row_type.clone(), ..Default::default() });
    if section.is_some() || !named.name.contains('.') {
        return Ok(Address { section, row, cell });
    }
    // Scoped baseline formatting slots carry COM metadata rather than XML rows.
    let (head, tail) = named.name.split_once('.').ok_or_else(|| operation("invalid cell name"))?;
    let mut section = Section { name: prefix(head).into(), ..Default::default() };
    let row_index = named.cell.attributes.get("ComRow").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
    if let Some(index) = head.strip_prefix("Geometry") {
        section.name = "Geometry".into();
        section.index = Some(index.parse::<u32>().map_err(operation)?.checked_sub(1).ok_or_else(|| operation("invalid geometry index"))?);
    }
    cell.name = tail.split('[').next().ok_or_else(|| operation("invalid cell name"))?.into();
    Ok(Address { section: Some(section), row: Some(Row { index: Some(row_index), ..Default::default() }), cell })
}
pub(crate) fn install(sheet: &mut Sheet, address: Address) -> Result<()> {
    let Address { section, row, cell } = address;
    let cells = if let Some(header) = section {
        let at = if let Some(i) = sheet.sections.iter().position(|s| s.name == header.name && s.index.unwrap_or(0) == header.index.unwrap_or(0)) {
            i
        } else {
            if sheet.sections.len() >= 16384 {
                return Err(operation("section limit"));
            }
            sheet.sections.push(header);
            sheet.sections.len() - 1
        };
        let section = sheet.sections.get_mut(at).ok_or_else(|| operation("section missing"))?;
        section.deleted = false;
        if let Some(header) = row {
            let at = if let Some(i) = section.rows.iter().position(|r| match &header.name {
                Some(n) => r.name.as_ref().is_some_and(|v| v.eq_ignore_ascii_case(n)),
                None => r.name.is_none() && r.index.unwrap_or(0) == header.index.unwrap_or(0),
            }) {
                i
            } else {
                if section.rows.len() >= 65536 {
                    return Err(operation("row limit"));
                }
                section.rows.push(header);
                section.rows.len() - 1
            };
            let row = section.rows.get_mut(at).ok_or_else(|| operation("row missing"))?;
            row.deleted = false;
            &mut row.cells
        } else {
            &mut section.cells
        }
    } else {
        &mut sheet.cells
    };
    if let Some(old) = cells.iter_mut().find(|c| c.name.eq_ignore_ascii_case(&cell.name)) {
        *old = cell;
    } else {
        if cells.len() >= 16384 {
            return Err(operation("cell limit"));
        }
        cells.push(cell);
    }
    Ok(())
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.cellsu
pub(crate) fn get(s: &mut Session, p: &Value) -> Result<Value> {
    let scope = target(s, p)?;
    let name = name(p, "name")?;
    let address = address(&s.document, scope, name)?;
    let themes = diagramcraft_theme::Themes::from_document(&s.document).map_err(operation)?;
    let mut e = Evaluator::new(&s.document).map_err(operation)?.with_theme_host(&themes).with_text_host(&diagramcraft_text::Host);
    Ok(
        json!({"name":name,"value":e.evaluate(scope,name).text(),"cached":address.cell.value,"formula":address.cell.formula,"units":address.cell.units}),
    )
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.formulau
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.formulaforceu
pub(crate) fn set_cell(s: &mut Session, p: &Value) -> Result<Value> {
    let scope = target(s, p)?;
    let name = name(p, "name")?;
    let mut address = address(&s.document, scope, name)?;
    let force = p.get("force").and_then(Value::as_bool).unwrap_or(false);
    if !force && address.cell.formula.as_ref().is_some_and(|f| f.trim_start_matches('=').trim_start().to_ascii_uppercase().starts_with("GUARD(")) {
        return Err(operation("cell is guarded; force is required"));
    }
    let formula = p.get("formula").map(|_| crate::edit::name(p, "formula")).transpose()?;
    if let Some(formula) = formula {
        if !matches!(formula, "Inh" | "No Formula") {
            diagramcraft_sheet::parse(formula).map_err(operation)?;
        }
        address.cell.formula = Some(formula.into());
    } else {
        address.cell.formula = None;
    }
    match p.get("value") {
        Some(Value::String(value)) if value.len() <= 65536 => address.cell.value = Some(value.clone()),
        Some(Value::Number(value)) => address.cell.value = Some(value.to_string()),
        None if formula.is_some() => {
            // An explicit formula replaces an inherited theme sentinel.
            if address.cell.value.as_deref() == Some("Themed") {
                address.cell.value = None;
            }
        }
        _ => return Err(operation("value must be a bounded string or number, or supply a formula")),
    }
    if let Some(units) = p.get("units") {
        address.cell.units = Some(units.as_str().filter(|s| s.len() <= 32).ok_or_else(|| operation("invalid unit code"))?.into());
    }
    address.cell.error = None;
    s.edit_document(|doc, _| {
        install(sheet_mut(doc, scope)?, address.clone())?;
        // Setting an override back to the master's own formula/value removes it.
        // Witness: probe connector5 zero Geometry/Control cells after Visio SaveAs.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
        let mut evaluator = Evaluator::new(doc).map_err(operation)?;
        let redundant = evaluator
            .master_source(scope)
            .and_then(|master| {
                let provider = evaluator.cell(master, name)?.clone();
                if address.cell.units != provider.units || address.cell.formula != provider.formula {
                    return None;
                }
                let local = evaluator.evaluate(scope, name);
                let inherited = evaluator.evaluate(master, name);
                Some(match (local.numeric(), inherited.numeric()) {
                    (Ok(a), Ok(b)) => a.is_finite() && b.is_finite() && (a - b).abs() < 1e-12,
                    _ => local == inherited && !matches!(local, diagramcraft_sheet::Value::Error(_)),
                })
            })
            .unwrap_or(false);
        drop(evaluator);
        // Keep an explicit routing policy even when it equals the inherited default.
        // Glue initialization may otherwise replace the lost user choice.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/confixedcode-cell-shape-layout-section
        if redundant && !name.eq_ignore_ascii_case("ConFixedCode") {
            remove_override(sheet_mut(doc, scope)?, &address);
        }
        Ok(json!({}))
    })
}
fn remove_override(sheet: &mut Sheet, address: &Address) {
    if let Some(header) = &address.section {
        if let Some(section) = sheet.sections.iter_mut().find(|s| s.name == header.name && s.index == header.index) {
            if let Some(row_header) = &address.row {
                if let Some(row) = section.rows.iter_mut().find(|r| r.name == row_header.name && r.index == row_header.index) {
                    row.cells.retain(|c| c.name != address.cell.name);
                }
                section.rows.retain(|r| r.deleted || !r.cells.is_empty());
            } else {
                section.cells.retain(|c| c.name != address.cell.name);
            }
        }
        sheet.sections.retain(|s| s.deleted || !s.cells.is_empty() || !s.rows.is_empty());
    } else {
        sheet.cells.retain(|c| c.name != address.cell.name);
    }
}
fn section_name(p: &Value) -> Result<(&str, Option<u32>)> {
    let name = name(p, "section")?;
    if !matches!(
        name,
        "Geometry"
            | "Character"
            | "Paragraph"
            | "Tabs"
            | "Control"
            | "Connection"
            | "User"
            | "Action"
            | "Scratch"
            | "Property"
            | "Hyperlink"
            | "Field"
            | "Layer"
            | "FillGradient"
            | "LineGradient"
    ) {
        return Err(operation("unsupported section name"));
    }
    let index = if name == "Geometry" { Some(p.get("section_index").map(|_| uint(p, "section_index")).transpose()?.unwrap_or(0)) } else { None };
    if index.is_some_and(|v| v >= 65536) {
        return Err(operation("geometry section limit"));
    }
    Ok((name, index))
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.addsection
pub(crate) fn add_section(s: &mut Session, p: &Value) -> Result<Value> {
    let scope = target(s, p)?;
    let (section, index) = section_name(p)?;
    s.edit_document(|doc, _| {
        let sheet = sheet_mut(doc, scope)?;
        if sheet.sections.len() >= 16384 {
            return Err(operation("section limit"));
        }
        if sheet.sections.iter().any(|s| s.name == section && s.index == index && !s.deleted) {
            return Err(operation("section already exists"));
        }
        let cells = if section == "Geometry" {
            ["NoFill", "NoLine", "NoShow", "NoSnap", "NoQuickDrag"].into_iter().map(|n| cell(n, "0", None)).collect()
        } else {
            vec![]
        };
        sheet.sections.push(Section { name: section.into(), index, cells, ..Default::default() });
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.addrow
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.addnamedrow
pub(crate) fn add_row(s: &mut Session, p: &Value) -> Result<Value> {
    let scope = target(s, p)?;
    let (section, index) = section_name(p)?;
    let row_name = p.get("row_name").map(|_| name(p, "row_name")).transpose()?;
    if row_name.is_some_and(|n| !n.chars().all(|c| c.is_alphanumeric() || c == '_')) {
        return Err(operation("invalid universal row name"));
    }
    let row_type = p.get("type").map(|_| name(p, "type")).transpose()?;
    if section == "Geometry" {
        row_type.ok_or_else(|| operation("geometry row requires type"))?.parse::<diagramcraft_geom::RowKind>().map_err(operation)?;
    }
    s.edit_document(|doc, _| {
        let sheet = sheet_mut(doc, scope)?;
        let section = sheet
            .sections
            .iter_mut()
            .find(|s| s.name == section && s.index == index && !s.deleted)
            .ok_or_else(|| operation("section does not exist"))?;
        if section.rows.len() >= 65536 {
            return Err(operation("row limit"));
        }
        let next = section
            .rows
            .iter()
            .enumerate()
            .map(|(i, r)| r.index.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX)))
            .max()
            .map_or(Some(if section.name == "Geometry" { 1 } else { 0 }), |v| v.checked_add(1))
            .ok_or_else(|| operation("row index overflow"))?;
        let row_index = p.get("index").map(|_| uint(p, "index")).transpose()?.unwrap_or(next);
        if row_index >= 65536
            || section.rows.iter().any(|r| {
                !r.deleted && (r.index == Some(row_index) || row_name.is_some_and(|n| r.name.as_ref().is_some_and(|v| v.eq_ignore_ascii_case(n))))
            })
        {
            return Err(operation("row index limit or duplicate row"));
        }
        let columns: &[&str] = match section.name.as_str() {
            "Geometry" => match row_type.unwrap_or("") {
                "MoveTo" | "LineTo" | "RelMoveTo" | "RelLineTo" => &["X", "Y"],
                "ArcTo" | "SplineKnot" => &["X", "Y", "A"],
                "InfiniteLine" | "RelQuadBezTo" => &["X", "Y", "A", "B"],
                "PolylineTo" => &["X", "Y", "A"],
                "NURBSTo" => &["X", "Y", "A", "B", "C", "D", "E"],
                _ => &["X", "Y", "A", "B", "C", "D"],
            },
            "Scratch" => &["X", "Y", "A", "B", "C", "D"],
            "User" => &["Value", "Prompt"],
            "Control" => &["X", "Y", "XDyn", "YDyn", "XCon", "YCon", "CanGlue", "Tip"],
            "Connection" => &["X", "Y", "DirX", "DirY", "Type", "AutoGen", "Prompt"],
            "Property" => &["Value", "Label", "Type", "Format", "Invisible", "SortKey", "Prompt"],
            "Hyperlink" => &["Description", "Address", "SubAddress", "ExtraInfo", "Frame", "NewWindow", "Default", "Invisible"],
            "Character" => &["Font", "Size", "Color", "Style"],
            "Paragraph" => &["HorzAlign", "IndFirst", "IndLeft", "IndRight", "SpLine", "SpBefore", "SpAfter"],
            "Action" => &["Action", "Menu", "Prompt", "Checked", "Disabled", "Invisible"],
            "Field" => &["Value", "Format"],
            "Tabs" => &["Position", "Alignment"],
            "Layer" => &["Name", "Color", "Status", "Visible", "Print", "Active", "Lock", "Snap", "Glue"],
            "FillGradient" | "LineGradient" => &["GradientStopColor", "GradientStopColorTrans", "GradientStopPosition"],
            _ => return Err(operation("unsupported row")),
        };
        section.rows.retain(|r| !(r.deleted && r.index == Some(row_index)));
        section.rows.push(Row {
            name: row_name.map(str::to_owned),
            index: Some(row_index),
            row_type: row_type.map(str::to_owned),
            cells: columns.iter().map(|n| cell(n, "0", None)).collect(),
            ..Default::default()
        });
        Ok(json!({"index":row_index}))
    })
}
