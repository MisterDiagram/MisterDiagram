//! Glue commands compose the route host with transactional model edits.
use crate::{Result, Session, edit::*, operation};
use serde_json::{Value, json};
/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.addrow
/// https://learn.microsoft.com/en-us/office/client-developer/visio/connection-points-section
pub(crate) fn connection(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let coordinate = |key: &str| -> Result<misterdiagram_model::Cell> {
        if let Some(f) = p.get(key).and_then(Value::as_str) {
            misterdiagram_sheet::parse(f).map_err(operation)?;
            // Visio drops literal-zero formulas from connection coordinates.
            // Witness: corpus/_state/brief27/first-acceptance/b27-san/saved.vsdx.
            Ok(cell(key, "0", (f.trim() != "0").then_some(f)))
        } else {
            Ok(cell(key, &number(p, key)?.to_string(), None))
        }
    };
    let point = misterdiagram_route::ConnectionPoint {
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
    s.edit_document(|doc, _| misterdiagram_route::add_connection(doc, index, id, point).map(|name| json!({"cell":name})).map_err(operation))
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
pub(crate) fn static_glue(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let target = uint(p, "target")?;
    let point = name(p, "point")?;
    let end = match name(p, "end")? {
        "begin" => misterdiagram_route::End::Begin,
        "end" => misterdiagram_route::End::End,
        _ => return Err(operation("endpoint must be begin or end")),
    };
    s.edit_document(|doc, _| {
        misterdiagram_route::connect_static(doc, index, id, end, target, point).map_err(operation)?;
        Ok(json!({"id":id}))
    })
}
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
pub(crate) fn dynamic_glue(s: &mut Session, p: &Value) -> Result<Value> {
    let (index, id, _) = shape_scope(s, p)?;
    let target = uint(p, "target")?;
    let end = match name(p, "end")? {
        "begin" => misterdiagram_route::End::Begin,
        "end" => misterdiagram_route::End::End,
        _ => return Err(operation("endpoint must be begin or end")),
    };
    let vertical = p.get("preference").and_then(Value::as_str) == Some("vertical");
    s.edit_document(|doc, _| {
        misterdiagram_route::connect_dynamic(doc, index, id, end, target, vertical).map_err(operation)?;
        Ok(json!({"id":id}))
    })
}

/// https://learn.microsoft.com/en-us/office/vba/api/visio.shape.reroute
pub(crate) fn reroute(s: &mut Session, p: &Value) -> Result<Value> {
    let index = page_index(s, p)?;
    let id = p.get("id").map(|_| uint(p, "id")).transpose()?;
    let (result, host) = s.edit_document(|doc, _| {
        let before = doc.clone();
        let mut host = misterdiagram_route::GlueHost::default();
        let count = misterdiagram_route::reroute(&before, doc, &mut host, Some((index, id))).map_err(operation)?;
        Ok((json!({"routed":count}), host))
    })?;
    s.apply_route_outcomes(&host);
    Ok(result)
}
