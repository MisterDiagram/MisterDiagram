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
    if host.unaffected && force.is_none() {
        return Ok(0);
    }
    if let Some((index, shape)) = force {
        let page = doc.pages.get(index).ok_or("route page missing")?;
        for id in page.connects.iter().map(|c| c.from_sheet) {
            if shape.is_none_or(|s| s == id) {
                host.activate(page.id, id);
            }
        }
    }
    let dirty_pages = doc.pages.iter().filter(|p| host.page_dirty(p.id)).map(|p| p.id).collect::<std::collections::BTreeSet<_>>();
    let mut outcomes = host.outcomes.clone();
    let themes = misterdiagram_theme::Themes::from_document(doc).map_err(|e| e.to_string())?;
    let old_themes = misterdiagram_theme::Themes::from_document(before).map_err(|e| e.to_string())?;
    let text = misterdiagram_text::Host;
    let mut e = Evaluator::new(doc).map_err(|e| e.to_string())?.with_theme_host(&themes).with_text_host(&text).with_glue_host(host);
    let mut old = Evaluator::new(before).map_err(|e| e.to_string())?.with_theme_host(&old_themes).with_text_host(&text);
    let mut routes = vec![];
    let mut horizontal_runs = Vec::<(u32, misterdiagram_geom::Point, misterdiagram_geom::Point)>::new();
    let mut obstacles_memo = crate::orthogonal::ObstacleBounds::default();
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
            // Targeted reroutes must see the unchanged preceding connectors too.
            // Reuse their serialized centerlines, including across jump arcs.
            let transform = e.shape_to_page(scope).map_err(|v| v.to_string())?;
            let mut rows = std::collections::BTreeMap::new();
            for n in e.resolved_named_cells(scope) {
                if n.section.is_some_and(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0)
                    && let Some(row) = n.row.filter(|r| !r.deleted)
                {
                    rows.insert(row.index.unwrap_or(0), row.row_type.clone());
                    if rows.len() > 16384 {
                        return Err("route row count limit".into());
                    }
                }
            }
            let mut previous = None;
            for (ix, kind) in rows {
                let x = crate::orthogonal::number(&mut e, scope, &format!("Geometry1.X{ix}"), 0.);
                let y = crate::orthogonal::number(&mut e, scope, &format!("Geometry1.Y{ix}"), 0.);
                let point = transform * misterdiagram_geom::Point::new(x, y);
                if !point.x.is_finite() || !point.y.is_finite() || point.x.abs() > 1e6 || point.y.abs() > 1e6 {
                    return Err("route coordinate limit".into());
                }
                if kind.as_deref() != Some("MoveTo")
                    && let Some(a) = previous
                {
                    let a: misterdiagram_geom::Point = a;
                    if (a.y - point.y).abs() < 1e-9 {
                        if horizontal_runs.len() >= 100000 {
                            return Err("route segment limit".into());
                        }
                        horizontal_runs.push((page, a, point));
                    }
                }
                previous = Some(point);
            }
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
        let style = crate::orthogonal::style(&mut e, scope, page);
        let gathered = if style == 2. || style == 16. {
            Ok(Vec::new())
        } else {
            crate::orthogonal::obstacles(&mut e, page, &excluded)
                .map(|boxes| boxes.into_iter().map(|b| inverse.transform_rect_bbox(b)).collect::<Vec<_>>())
        };
        // Only newly routable over-cap pages use Visio's redundant-local elision.
        // Existing successful pages retain their exact serialized representation.
        // Witness: corpus/_vm/out/brief32-final-oracle/pairs-130/saved.vsdx.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
        let inherited_route = if gathered.as_ref().is_err_and(|error| error == "route obstacle count limit") {
            e.master_source(scope).map_or_else(Vec::new, |master| {
                ["Height", "Geometry1.Y2"].into_iter().filter_map(|name| e.cell(master, name).cloned().map(|cell| (name, cell))).collect()
            })
        } else {
            Vec::new()
        };
        let cx = crate::orthogonal::number(&mut e, Scope::Page(page), "LineToLineX", 0.125) * 1.5;
        let cy = crate::orthogonal::number(&mut e, Scope::Page(page), "LineToLineY", 0.125) * 1.5;
        let begin = misterdiagram_geom::Point::new(bx, by);
        let end = misterdiagram_geom::Point::new(ex, ey);
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
        let static_normal = |e: &mut Evaluator<'_>, point, side: &str| -> Result<Option<misterdiagram_geom::Vec2>, String> {
            let link = doc.pages.get(index).ok_or("route page")?.connects.iter().find(|c| c.from_sheet == id && c.from_cell.as_deref() == Some(side));
            let Some(link) = link.filter(|c| !matches!(c.to_cell.as_deref(), Some("PinX" | "PinY"))) else { return Ok(None) };
            let bounds = inverse.transform_rect_bbox(crate::orthogonal::bounds(e, Scope::PageShape { page, shape: link.to_sheet })?);
            Ok(crate::stub::normal(bounds, point))
        };
        let mut normals = [static_normal(&mut e, begin, "BeginX")?, static_normal(&mut e, end, "EndX")?];
        if normals.iter().any(Option::is_some) {
            for (slot, point, side) in [(0, begin, "BeginX"), (1, end, "EndX")] {
                if normals.get(slot).is_some_and(Option::is_none)
                    && let Some(link) =
                        doc.pages.get(index).ok_or("route page")?.connects.iter().find(|c| c.from_sheet == id && c.from_cell.as_deref() == Some(side))
                {
                    let bounds = inverse.transform_rect_bbox(crate::orthogonal::bounds(&mut e, Scope::PageShape { page, shape: link.to_sheet })?);
                    *normals.get_mut(slot).ok_or("route endpoint index")? = crate::stub::normal(bounds, point);
                }
            }
        }
        let runs = horizontal_runs.iter().filter(|(p, _, _)| *p == page).map(|(_, a, b)| (inverse * *a, inverse * *b)).collect::<Vec<_>>();
        // B25 office-style-17 observes a vertical-first midpoint jog. Preserve
        // the existing obstacle router when that direct tree path is blocked.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/shaperoutestyle-cell-shape-layout-section
        let attempt = (|| {
            if let Some(Some(error)) = outcomes.get(&(page, id)) {
                return Err(error.clone());
            }
            let solve = |obstacles: &[misterdiagram_geom::Rect], axes: Option<(&[misterdiagram_geom::Rect], misterdiagram_geom::Rect)>| {
                let obstacles = crate::orthogonal::clearance(obstacles, if style == 17. { (0., 0.) } else { (cx, cy) }, [begin, end]);
                let projected = axes.map(|(boxes, corridor)| {
                    (crate::orthogonal::clearance(boxes, if style == 17. { (0., 0.) } else { (cx, cy) }, [begin, end]), corridor)
                });
                let projected = projected.as_ref().map(|(boxes, corridor)| (boxes.as_slice(), *corridor));
                let points = if style == 2. || style == 16. {
                    vec![begin, end]
                } else if style == 17. {
                    let mid = (by + ey) / 2.;
                    let points = if bx == ex || by == ey {
                        vec![begin, end]
                    } else {
                        vec![begin, misterdiagram_geom::Point::new(bx, mid), misterdiagram_geom::Point::new(ex, mid), end]
                    };
                    if points.windows(2).any(|pair| {
                        obstacles.iter().any(|r| {
                            let Some(a) = pair.first() else { return true };
                            let Some(b) = pair.get(1) else { return true };
                            if a.x == b.x {
                                a.x > r.x0 && a.x < r.x1 && a.y.min(b.y) < r.y1 && a.y.max(b.y) > r.y0
                            } else {
                                a.y > r.y0 && a.y < r.y1 && a.x.min(b.x) < r.x1 && a.x.max(b.x) > r.x0
                            }
                        })
                    }) {
                        crate::orthogonal::path_with_axes(begin, end, &obstacles, dirs.0, dirs.1, projected)?
                    } else {
                        points
                    }
                } else {
                    crate::orthogonal::path_with_axes(begin, end, &obstacles, dirs.0, dirs.1, projected)?
                };
                if style == 1. && normals.iter().any(Option::is_some) {
                    crate::stub::path(begin, end, normals, &obstacles, &runs, points)
                } else {
                    Ok(points)
                }
            };
            match gathered {
                Ok(obstacles) => solve(&obstacles, None),
                Err(error) if error == "route obstacle count limit" => {
                    let mut boxes = [misterdiagram_geom::Rect::new(bx, by, bx, by), misterdiagram_geom::Rect::new(ex, ey, ex, ey)];
                    for (slot, target) in boxes.iter_mut().zip(&excluded) {
                        *slot =
                            slot.union(inverse.transform_rect_bbox(crate::orthogonal::bounds(&mut e, Scope::PageShape { page, shape: *target })?));
                    }
                    crate::orthogonal::with_obstacles(&mut e, page, &excluded, inverse, boxes, &mut obstacles_memo, solve)
                }
                Err(error) => Err(error),
            }
        })();
        let points = match attempt {
            Ok(points) => {
                outcomes.insert((page, id), None);
                points
            }
            Err(error) if crate::orthogonal::capacity(&error) => {
                outcomes.insert((page, id), Some(error));
                crate::orthogonal::fallback(begin, end, style)
            }
            Err(error) => return Err(error),
        };
        for pair in points.windows(2) {
            if let Some((a, b)) = pair.first().zip(pair.get(1)) {
                let a = parent * *a;
                let b = parent * *b;
                if (a.y - b.y).abs() < 1e-9 && horizontal_runs.len() < 100000 {
                    horizontal_runs.push((page, a, b));
                }
            }
        }
        routes.push((index, id, bx, by, ex, ey, style, points, crate::orthogonal::inherited_zero(&e, scope), inherited_route));
        if routes.len() > 100000 {
            return Err("route count limit".into());
        }
    }
    drop(e);
    host.outcomes = outcomes;
    let count = routes.len();
    let routed = routes.iter().map(|r| (r.0, r.1)).collect();
    crate::fixed::apply(doc, fixed)?;
    for (index, id, bx, by, ex, ey, _style, points, inherited_zero, inherited_route) in routes {
        let page = doc.pages.get_mut(index).ok_or("route page missing")?;
        let layer = connector_layer(page)?;
        let shape = shape_mut(&mut page.shapes, id, 0).ok_or("route shape missing")?;
        let dx = ex - bx;
        let dy = ey - by;
        // Preserve Visio's signed zero-axis frame. The asymmetric FlipX witness
        // has Width=-0.25DL even though its page-space x delta is below 1e-15.
        // https://learn.microsoft.com/en-us/office/client-developer/visio/width-cell-shape-transform-section
        let zero = |name: &str| {
            if shape
                .sheet
                .cell(name)
                .is_some_and(|c| c.value.as_deref() == Some("-0.25") && c.formula.as_deref().is_some_and(|f| f.starts_with("GUARD(-0.25")))
            {
                -0.25
            } else {
                0.25
            }
        };
        let width = if dx.abs() < 1e-12 { zero("Width") } else { dx };
        let height = if dy.abs() < 1e-12 { zero("Height") } else { dy };
        let width_formula = if dx.abs() < 1e-12 { if width < 0. { "GUARD(-0.25DL)" } else { "GUARD(0.25DL)" } } else { "GUARD(EndX-BeginX)" };
        let height_formula = if dy.abs() < 1e-12 { if height < 0. { "GUARD(-0.25DL)" } else { "GUARD(0.25DL)" } } else { "GUARD(EndY-BeginY)" };
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
                    let x = p.x - bx + if dx.abs() < 1e-12 { width / 2. } else { 0. };
                    let y = p.y - by + if dy.abs() < 1e-12 { height / 2. } else { 0. };
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
        for (name, inherited) in inherited_route {
            if name == "Height" {
                shape.sheet.cells.retain(|cell| cell != &inherited);
            } else if let Some(geometry) = shape.sheet.sections.iter_mut().find(|s| s.name == "Geometry" && s.index.unwrap_or(0) == 0)
                && let Some(row) = geometry.rows.iter_mut().find(|row| row.index == Some(2) && !row.deleted)
            {
                row.cells.retain(|cell| cell != &inherited);
            }
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
