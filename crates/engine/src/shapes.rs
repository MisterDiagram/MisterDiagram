//! Shape construction and transforms in internal inches and radians.
use crate::{Result, Session, edit::*, operation};
use diagramcraft_model::{Row, Section, Shape, ShapeId, Sheet};
use serde_json::{Value, json};

/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.drawrectangle
pub(crate) fn rect(s: &mut Session, p: &Value) -> Result<Value> {
    draw(s, p, false)
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.drawoval
pub(crate) fn ellipse(s: &mut Session, p: &Value) -> Result<Value> {
    draw(s, p, true)
}
fn draw(s: &mut Session, p: &Value, ellipse: bool) -> Result<Value> {
    let index = page_index(s, p)?;
    let x = number(p, "x")?;
    let y = number(p, "y")?;
    let w = number(p, "width")?;
    let h = number(p, "height")?;
    s.edit_document(|doc, selection| {
        let page = page_mut(doc, index)?;
        let id = next_id(page)?;
        let mut shape = diagramcraft_model::skeleton::rectangle(id, x, y, w, h, "").map_err(operation)?;
        if ellipse {
            let geometry = shape.sheet.sections.first_mut().ok_or_else(|| operation("missing geometry"))?;
            geometry.rows = vec![Row {
                index: Some(1),
                row_type: Some("Ellipse".into()),
                cells: vec![
                    cell("X", &(w / 2.).to_string(), Some("Width*0.5")),
                    cell("Y", &(h / 2.).to_string(), Some("Height*0.5")),
                    cell("A", &w.to_string(), Some("Width*1")),
                    cell("B", &(h / 2.).to_string(), Some("Height*0.5")),
                    cell("C", &(w / 2.).to_string(), Some("Width*0.5")),
                    cell("D", &h.to_string(), Some("Height*1")),
                ],
                ..Default::default()
            }];
            if let Some(row) = geometry.rows.first_mut() {
                for c in row.cells.iter_mut().skip(2) {
                    c.units = Some("DL".into());
                }
            }
        }
        page.shapes.push(shape);
        selection.page = index;
        selection.shapes = vec![ShapeId(id)];
        Ok(json!({"id":id}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.drawline
pub(crate) fn line(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let bx = number(p, "x1")?;
    let by = number(p, "y1")?;
    let ex = number(p, "x2")?;
    let ey = number(p, "y2")?;
    let width = (ex - bx).hypot(ey - by);
    if width > 1e6 {
        return Err(operation("line extent limit"));
    }
    s.edit_document(|doc, selection| {
        let page = page_mut(doc, index)?;
        let id = next_id(page)?;
        let mut cells = vec![];
        for (n, v, f) in [
            ("PinX", (bx + ex) / 2., Some("(BeginX+EndX)/2")),
            ("PinY", (by + ey) / 2., Some("(BeginY+EndY)/2")),
            ("Width", width, Some("SQRT((EndX-BeginX)^2+(EndY-BeginY)^2)")),
            ("Height", 0., None),
            ("LocPinX", width / 2., Some("Width*0.5")),
            ("LocPinY", 0., Some("Height*0.5")),
            ("Angle", (ey - by).atan2(ex - bx), Some("ATAN2(EndY-BeginY,EndX-BeginX)")),
            ("FlipX", 0., None),
            ("FlipY", 0., None),
            ("ResizeMode", 0., None),
            ("BeginX", bx, None),
            ("BeginY", by, None),
            ("EndX", ex, None),
            ("EndY", ey, None),
        ] {
            cells.push(cell(n, &v.to_string(), f));
        }
        cells.push(cell("TextBkgnd", "#ffffff", Some("THEMEGUARD(THEMEVAL(\"BackgroundColor\")+1)")));
        for n in ["QuickStyleLineMatrix", "QuickStyleFillMatrix", "QuickStyleEffectsMatrix", "QuickStyleFontMatrix"] {
            cells.push(cell(n, "1", None));
        }
        let section = Section {
            name: "Geometry".into(),
            index: Some(0),
            cells: ["NoFill", "NoLine", "NoShow", "NoSnap", "NoQuickDrag"]
                .into_iter()
                .map(|n| cell(n, if n == "NoFill" { "1" } else { "0" }, None))
                .collect(),
            rows: vec![
                Row {
                    index: Some(1),
                    row_type: Some("MoveTo".into()),
                    cells: vec![cell("X", "0", Some("Width*0")), cell("Y", "0", None)],
                    ..Default::default()
                },
                Row {
                    index: Some(2),
                    row_type: Some("LineTo".into()),
                    cells: vec![cell("X", &width.to_string(), Some("Width*1")), cell("Y", "0", None)],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        page.shapes.push(Shape {
            id,
            shape_type: Some("Shape".into()),
            line_style: Some(3),
            fill_style: Some(3),
            text_style: Some(3),
            sheet: Sheet { cells, sections: vec![section], ..Default::default() },
            ..Default::default()
        });
        selection.page = index;
        selection.shapes = vec![ShapeId(id)];
        Ok(json!({"id":id}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/pinx-cell-shape-transform-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/piny-cell-shape-transform-section
pub(crate) fn move_shape(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, scope) = shape_scope(s, p)?;
    let x = number(p, "x")?;
    let y = number(p, "y")?;
    let old_x = evaluated(&s.document, scope, "PinX")?;
    let old_y = evaluated(&s.document, scope, "PinY")?;
    let e = diagramcraft_vsdx::evaluate::Evaluator::new(&s.document).map_err(operation)?;
    let one_d = e.cell(scope, "BeginX").is_some();
    let ends = if one_d {
        Some([
            evaluated(&s.document, scope, "BeginX")?,
            evaluated(&s.document, scope, "BeginY")?,
            evaluated(&s.document, scope, "EndX")?,
            evaluated(&s.document, scope, "EndY")?,
        ])
    } else {
        None
    };
    s.edit_document(|doc, _| {
        let sheet = &mut shape_mut(&mut page_mut(doc, index)?.shapes, id)?.sheet;
        if let Some([bx, by, ex, ey]) = ends {
            for (name, v) in [("BeginX", bx + x - old_x), ("BeginY", by + y - old_y), ("EndX", ex + x - old_x), ("EndY", ey + y - old_y)] {
                set_number(sheet, name, v)?;
            }
        } else {
            set_number(sheet, "PinX", x)?;
            set_number(sheet, "PinY", y)?;
        }
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/width-cell-shape-transform-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/height-cell-shape-transform-section
pub(crate) fn resize(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, scope) = shape_scope(s, p)?;
    let w = number(p, "width")?;
    let h = number(p, "height")?;
    let e = diagramcraft_vsdx::evaluate::Evaluator::new(&s.document).map_err(operation)?;
    let line = e.cell(scope, "BeginX").is_some() && e.cell(scope, "Width").and_then(|c| c.formula.as_deref()).is_some_and(|f| f.starts_with("SQRT("));
    if w <= 0. || h < 0. || (!line && h == 0.) {
        return Err(operation("shape dimensions must be positive (a line may have zero height)"));
    }
    let line_frame = if line {
        Some((evaluated(&s.document, scope, "PinX")?, evaluated(&s.document, scope, "PinY")?, evaluated(&s.document, scope, "Angle")?))
    } else {
        None
    };
    s.edit_document(|doc, _| {
        let sheet = &mut shape_mut(&mut page_mut(doc, index)?.shapes, id)?.sheet;
        if let Some((x, y, angle)) = line_frame {
            let dx = w * angle.cos() / 2.;
            let dy = w * angle.sin() / 2.;
            for (n, v) in [("BeginX", x - dx), ("BeginY", y - dy), ("EndX", x + dx), ("EndY", y + dy)] {
                set_number(sheet, n, v)?;
            }
            set_number(sheet, "Height", h)?;
        } else {
            set_number(sheet, "Width", w)?;
            set_number(sheet, "Height", h)?;
        }
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/angle-cell-shape-transform-section
pub(crate) fn rotate(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, scope) = shape_scope(s, p)?;
    let angle = number(p, "angle")?;
    let e = diagramcraft_vsdx::evaluate::Evaluator::new(&s.document).map_err(operation)?;
    let line = e.cell(scope, "BeginX").is_some() && e.cell(scope, "Width").and_then(|c| c.formula.as_deref()).is_some_and(|f| f.starts_with("SQRT("));
    let ends = if line {
        let x = evaluated(&s.document, scope, "PinX")?;
        let y = evaluated(&s.document, scope, "PinY")?;
        let width = evaluated(&s.document, scope, "Width")?;
        let dx = width * angle.cos() / 2.;
        let dy = width * angle.sin() / 2.;
        Some([x - dx, y - dy, x + dx, y + dy])
    } else {
        None
    };
    s.edit_document(|doc, _| {
        let sheet = &mut shape_mut(&mut page_mut(doc, index)?.shapes, id)?.sheet;
        if let Some([bx, by, ex, ey]) = ends {
            for (n, v) in [("BeginX", bx), ("BeginY", by), ("EndX", ex), ("EndY", ey)] {
                set_number(sheet, n, v)?;
            }
        } else {
            set_number(sheet, "Angle", angle)?;
        }
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/client-developer/visio/flipx-cell-shape-transform-section
/// https://learn.microsoft.com/en-us/office/client-developer/visio/flipy-cell-shape-transform-section
pub(crate) fn flip(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, scope) = shape_scope(s, p)?;
    let axis = name(p, "axis")?;
    let field = match axis {
        "x" => "FlipX",
        "y" => "FlipY",
        _ => return Err(operation("axis must be x or y")),
    };
    let old = evaluated(&s.document, scope, field)?;
    s.edit_document(|doc, _| {
        set_number(&mut shape_mut(&mut page_mut(doc, index)?.shapes, id)?.sheet, field, if old == 0. { 1. } else { 0. })?;
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.delete
pub(crate) fn delete(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    fn remove(shapes: &mut Vec<Shape>, id: u32, removed: &mut Vec<u32>, depth: usize) -> Result<()> {
        if depth > 128 {
            return Err(operation("shape nesting limit"));
        }
        if let Some(i) = shapes.iter().position(|v| v.id == id) {
            let shape = shapes.remove(i);
            let mut pending = vec![&shape];
            while let Some(s) = pending.pop() {
                removed.push(s.id);
                pending.extend(&s.shapes);
            }
        } else {
            for shape in shapes {
                remove(&mut shape.shapes, id, removed, depth + 1)?;
            }
        }
        Ok(())
    }
    s.edit_document(|doc, selection| {
        let page = page_mut(doc, index)?;
        let mut removed = vec![];
        remove(&mut page.shapes, id, &mut removed, 0)?;
        page.connects.retain(|c| !removed.contains(&c.from_sheet) && !removed.contains(&c.to_sheet));
        selection.shapes.retain(|v| !removed.contains(&v.0));
        Ok(json!({}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.bringtofront
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.sendtoback
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.bringforward
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.sendbackward
pub(crate) fn order(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let direction = name(p, "order")?.to_owned();
    if !matches!(direction.as_str(), "front" | "back" | "forward" | "backward") {
        return Err(operation("invalid order"));
    }
    fn reorder(shapes: &mut Vec<Shape>, id: u32, order: &str, depth: usize) -> Result<bool> {
        if depth > 128 {
            return Err(operation("shape nesting limit"));
        }
        if let Some(from) = shapes.iter().position(|v| v.id == id) {
            let to = match order {
                "front" => shapes.len() - 1,
                "back" => 0,
                "forward" => (from + 1).min(shapes.len() - 1),
                _ => from.saturating_sub(1),
            };
            let shape = shapes.remove(from);
            shapes.insert(to, shape);
            return Ok(true);
        }
        for shape in shapes {
            if reorder(&mut shape.shapes, id, order, depth + 1)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
    s.edit_document(|doc, _| {
        reorder(&mut page_mut(doc, index)?.shapes, id, &direction, 0)?;
        Ok(json!({}))
    })
}
