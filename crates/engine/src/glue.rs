//! Glue commands compose the route host with transactional model edits.
use crate::{Result, Session, edit::*, operation};
use serde_json::{Value, json};
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.addrow
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
pub(crate) fn connection(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let coordinate = |key: &str| -> Result<diagramcraft_model::Cell> {
        if let Some(f) = p.get(key).and_then(Value::as_str) {
            diagramcraft_sheet::parse(f).map_err(operation)?;
            Ok(cell(key, "0", Some(f)))
        } else {
            Ok(cell(key, &number(p, key)?.to_string(), None))
        }
    };
    let point = diagramcraft_route::ConnectionPoint {
        name: p.get("name").map(|_| name(p, "name").map(str::to_owned)).transpose()?,
        x: coordinate("x")?,
        y: coordinate("y")?,
        direction: [
            p.get("dir_x").map(|_| number(p, "dir_x")).transpose()?.unwrap_or(0.),
            p.get("dir_y").map(|_| number(p, "dir_y")).transpose()?.unwrap_or(0.),
        ],
        kind: u8::try_from(p.get("type").map(|_| uint(p, "type")).transpose()?.unwrap_or(0)).map_err(operation)?,
        auto_gen: p.get("auto_gen").and_then(Value::as_bool).unwrap_or(false),
        prompt: p.get("prompt").and_then(Value::as_str).unwrap_or("").into(),
    };
    s.edit_document(|doc, _| diagramcraft_route::add_connection(doc, index, id, point).map(|name| json!({"cell":name})).map_err(operation))
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
pub(crate) fn static_glue(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let target = uint(p, "target")?;
    let point = name(p, "point")?;
    let end = match name(p, "end")? {
        "begin" => diagramcraft_route::End::Begin,
        "end" => diagramcraft_route::End::End,
        _ => return Err(operation("endpoint must be begin or end")),
    };
    s.edit_document(|doc, _| {
        diagramcraft_route::connect_static(doc, index, id, end, target, point).map_err(operation)?;
        Ok(json!({"id":id}))
    })
}
