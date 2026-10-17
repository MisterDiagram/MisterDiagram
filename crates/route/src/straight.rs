//! Straight connector geometry in the connector's own coordinate frame.
use crate::{
    GlueHost,
    static_glue::{cell, set, shape_mut},
};
use misterdiagram_model::{Document, Page, Row, Section};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
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
    let mut row = misterdiagram_model::skeleton::layer(index, "Connector", true)?;
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
fn row(index: u32, kind: &str, cells: Vec<misterdiagram_model::Cell>, deleted: bool) -> Row {
    Row {
        index: Some(index),
        row_type: Some(kind.into()),
        deleted,
        attributes: misterdiagram_model::Attributes::from([("IX".into(), index.to_string()), ("T".into(), kind.into())]),
        cells,
        ..Default::default()
    }
}
/// Recompute connector Geometry after endpoint, obstacle or page-layout edits.
/// Four horizontal witnesses retain signed Width=EndX-BeginX, Height=0.25DL, and the deleted third row.
/// Offset and vertical witnesses use an unrotated signed frame, with a 0.25DL zero axis.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/geometry-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/width-cell-shape-transform-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/angle-cell-shape-transform-section
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.reroute
pub fn reroute(before: &Document, doc: &mut Document, host: &mut GlueHost, force: Option<(usize, Option<u32>)>) -> Result<usize, String> {
    if let Some((index, shape)) = force {
        let page = doc.pages.get(index).ok_or("route page missing")?;
        for id in page.connects.iter().map(|c| c.from_sheet) {
            if shape.is_none_or(|s| s == id) {
                host.activate(page.id, id);
            }
        }
    }
    let dirty_pages = doc.pages.iter().filter(|p| host.page_dirty(p.id)).map(|p| p.id).collect::<std::collections::BTreeSet<_>>();
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(|e| e.to_string())?;
    let old_themes = misterdiagram_theme::Themes::from_document(before).map_err(|e| e.to_string())?;
    let text = misterdiagram_text::Host;
    let mut e = Evaluator::new(doc).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&text).with_glue_host(host);
    let mut old = Evaluator::new(before).map_err(|e| e.to_string())?.with_theme_host(&old_themes).with_text_host(&text);
    let mut routes = vec![];
    let mut fixed = vec![];
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
        if !forced && !changed && !dirty_pages.contains(&page) {
            continue;
        }
        let [bx, by, ex, ey] = ["BeginX", "BeginY", "EndX", "EndY"]
            .map(|name| number(&mut e, scope, name))
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| "endpoint count")?;
        if number(&mut e, scope, "ConFixedCode")? == 2. {
            if changed {
                fixed.extend(crate::fixed::end_updates(&mut e, index, scope, [bx, by, ex, ey])?);
                if fixed.len() > 200000 {
                    return Err("fixed connector update limit".into());
                }
            }
            continue;
        }
        let excluded =
            doc.pages.get(index).ok_or("route page missing")?.connects.iter().filter(|c| c.from_sheet == id).map(|c| c.to_sheet).collect::<Vec<_>>();
        let parent = e.parent_to_page(scope).map_err(|v| v.to_string())?;
        let inverse = parent.inverse();
        let obstacles =
            crate::orthogonal::obstacles(&mut e, page, &excluded)?.into_iter().map(|b| inverse.transform_rect_bbox(b)).collect::<Vec<_>>();
        let style = crate::orthogonal::style(&mut e, scope, page);
        let cx = crate::orthogonal::number(&mut e, Scope::Page(page), "LineToLineX", 0.125) * 1.5;
        let cy = crate::orthogonal::number(&mut e, Scope::Page(page), "LineToLineY", 0.125) * 1.5;
        let begin = misterdiagram_geom::Point::new(bx, by);
        let end = misterdiagram_geom::Point::new(ex, ey);
        let obstacles = crate::orthogonal::clearance(&obstacles, (cx, cy), [begin, end]);
        let direction = |e: &mut Evaluator<'_>, point: misterdiagram_geom::Point, side: &str| -> Result<Option<usize>, String> {
            let link = doc.pages.get(index).ok_or("route page")?.connects.iter().find(|c| c.from_sheet == id && c.from_cell.as_deref() == Some(side));
            if let Some(link) = link {
                if !matches!(link.to_cell.as_deref(), Some("PinX" | "PinY")) {
                    return Ok(None);
                }
                let b = inverse.transform_rect_bbox(crate::orthogonal::bounds(e, Scope::PageShape { page, shape: link.to_sheet })?);
                let dx = (point.x - b.x0).abs().min((point.x - b.x1).abs());
                let dy = (point.y - b.y0).abs().min((point.y - b.y1).abs());
                Ok(Some(if dx < dy { 0 } else { 1 }))
            } else {
                Ok(None)
            }
        };
        let dirs = (direction(&mut e, begin, "BeginX")?, direction(&mut e, end, "EndX")?);
        let points = if style == 2. || style == 16. { vec![begin, end] } else { crate::orthogonal::path(begin, end, &obstacles, dirs.0, dirs.1)? };
        routes.push((index, id, bx, by, ex, ey, style, points, crate::orthogonal::inherited_zero(&e, scope)));
        if routes.len() > 100000 {
            return Err("route count limit".into());
        }
    }
    drop(e);
    let count = routes.len();
    let routed = routes.iter().map(|r| (r.0, r.1)).collect();
    crate::fixed::apply(doc, fixed)?;
    for (index, id, bx, by, ex, ey, _style, points, inherited_zero) in routes {
        let page = doc.pages.get_mut(index).ok_or("route page missing")?;
        let layer = connector_layer(page)?;
        let shape = shape_mut(&mut page.shapes, id, 0).ok_or("route shape missing")?;
        let dx = ex - bx;
        let dy = ey - by;
        let width = if dx.abs() < 1e-12 { 0.25 } else { dx };
        let height = if dy.abs() < 1e-12 { 0.25 } else { dy };
        let width_formula = if dx.abs() < 1e-12 { "GUARD(0.25DL)" } else { "GUARD(EndX-BeginX)" };
        let height_formula = if dy.abs() < 1e-12 { "GUARD(0.25DL)" } else { "GUARD(EndY-BeginY)" };
        for (name, value, formula) in [
            ("PinX", (bx + ex) * 0.5, "Inh"),
            ("PinY", (by + ey) * 0.5, "Inh"),
            ("Width", width, width_formula),
            ("Height", height, height_formula),
            ("LocPinX", width * 0.5, "Inh"),
            ("LocPinY", height * 0.5, "Inh"),
            ("TxtPinX", width * 0.5, "Inh"),
            ("TxtPinY", height * 0.5, "Inh"),
        ] {
            set(&mut shape.sheet, cell(name, value, Some(formula.into())))?;
        }
        if shape.sheet.cell("Angle").is_some_and(|c| c.formula.as_deref() == Some("GUARD(ATAN2(EndY-BeginY,EndX-BeginX))")) {
            shape.sheet.cells.retain(|c| c.name != "Angle");
        }
        for (name, value) in [("BeginX", bx), ("BeginY", by), ("EndX", ex), ("EndY", ey)] {
            // A local literal is an override too. Inh would restore the master's
            // original endpoint when the transaction is written and read back.
            // https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
            let formula = shape.sheet.cell(name).map_or_else(|| Some("Inh".into()), |c| c.formula.clone());
            set(&mut shape.sheet, cell(name, value, formula))?;
        }
        // https://learn.microsoft.com/en-us/office/client-developer/visio/layermember-cell-layer-membership-section
        let mut memberships = shape
            .sheet
            .cell("LayerMember")
            .and_then(|c| c.value.as_deref())
            .unwrap_or("")
            .split(';')
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if !memberships.iter().any(|s| s == &layer.to_string()) {
            memberships.push(layer.to_string());
        }
        set(&mut shape.sheet, cell("LayerMember", memberships.join(";"), None))?;
        let geometry = Section {
            name: "Geometry".into(),
            index: Some(0),
            rows: {
                let mut rows = Vec::new();
                for (i, p) in points.iter().enumerate() {
                    let x = p.x - bx + if dx.abs() < 1e-12 { 0.125 } else { 0. };
                    let y = p.y - by + if dy.abs() < 1e-12 { 0.125 } else { 0. };
                    let mut cells = vec![cell("Y", y, None)];
                    if i != 0 || x.abs() > 1e-12 {
                        cells.insert(0, cell("X", x, None));
                    }
                    rows.push(row(u32::try_from(i + 1).map_err(|_| "route row count")?, if i == 0 { "MoveTo" } else { "LineTo" }, cells, false));
                }
                if rows.len() == 2 {
                    rows.push(row(3, "LineTo", vec![], true));
                }
                crate::orthogonal::trim_inherited_start(&mut rows, inherited_zero);
                rows
            },
            ..Default::default()
        };
        if let Some(old) = shape.sheet.sections.iter_mut().find(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0) {
            old.rows = geometry.rows;
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
    crate::jumps::apply(doc, host, &routed)?;
    Ok(count)
}
