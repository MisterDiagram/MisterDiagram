//! Command-surface conveniences use the exact strict fine-grained edits.
use crate::{Result, Session, edit::*, operation};
use serde_json::{Value, json};

pub(crate) fn applies(command: &str, p: &Value) -> bool {
    command == "connector.add"
        || ((command.starts_with("shape.draw.") || command == "master.drop")
            && ["text", "fill", "line", "font"].iter().any(|key| p.get(key).is_some()))
}
fn style(s: &mut Session, p: &Value, id: &Value, page: usize) -> Result<()> {
    for (key, command) in [("text", "shape.text"), ("fill", "format.fill"), ("line", "format.line"), ("font", "format.text")] {
        let Some(value) = p.get(key) else {
            continue;
        };
        let mut args = if key == "text" {
            json!({"text":value})
        } else {
            let object = value.as_object().ok_or_else(|| operation(format!("{key} must be a formatting object")))?;
            if object.contains_key("id") || object.contains_key("page") {
                return Err(operation("style objects cannot override id or page"));
            }
            value.clone()
        };
        args["id"] = id.clone();
        args["page"] = json!(page + 1);
        s.execute(command, &args)?;
    }
    Ok(())
}
/// One transaction and undo step, retaining every fine-grained publication boundary.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.application.beginundoscope
/// https://learn.microsoft.com/en-us/office/vba/api/visio.application.endundoscope
pub(crate) fn execute(s: &mut Session, command: &str, p: &Value) -> Result<Value> {
    if s.editing {
        return Err(operation("compound command cannot run inside an edit"));
    }
    let page = page_index(s, p)?;
    let mut candidate = s.clone();
    candidate.read_reservations = s.retained_documents().into_iter().cloned().collect();
    candidate.stencil_reservations = s.retained_stencils();
    let result = if command == "connector.add" {
        connector(&mut candidate, p, page)?
    } else {
        let mut base = p.clone();
        if let Some(object) = base.as_object_mut() {
            for key in ["text", "fill", "line", "font"] {
                object.remove(key);
            }
        }
        candidate.execute(command, &base)?
    };
    let id = result.get("id").ok_or_else(|| operation("creation command did not return id"))?;
    style(&mut candidate, p, id, page)?;
    // Internal edits stay visible in the saved cells, but the public journal is
    // one convenience command and the history boundary is one user edit.
    candidate.journal = s.journal.clone();
    candidate.journal_bytes = s.journal_bytes;
    candidate.record(command, p, crate::params_size(p)?);
    s.publish_batch(candidate, result)
}
/// Drop the document's Dynamic connector, glue both ends, then apply line style.
/// Automatic parallel links use the ordinary facing-side and fan commands.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.page.drop
/// https://learn.microsoft.com/en-us/office/vba/api/visio.cell.glueto
fn connector(s: &mut Session, p: &Value, page: usize) -> Result<Value> {
    let from = uint(p, "from")?;
    let to = uint(p, "to")?;
    if from == to {
        return Err(operation("connector endpoints must be distinct shapes"));
    }
    let glue = p.get("glue").and_then(Value::as_str).unwrap_or("static");
    let explicit = p.get("from_point").is_some() || p.get("to_point").is_some();
    if explicit && (glue != "static" || p.get("from_point").is_none() || p.get("to_point").is_none()) {
        return Err(operation("explicit connection points require glue static and both from_point and to_point"));
    }
    let result = s.execute("master.drop", &json!({"master":"Dynamic connector","page":page+1,"x":0,"y":0}))?;
    let id = result.get("id").ok_or_else(|| operation("connector drop did not return id"))?;
    if explicit {
        s.execute(
            "connect.static",
            &json!({"page":page+1,"id":id,"end":"begin","target":from,"point":p.get("from_point").ok_or_else(|| operation("from_point missing"))?}),
        )?;
        s.execute(
            "connect.static",
            &json!({"page":page+1,"id":id,"end":"end","target":to,"point":p.get("to_point").ok_or_else(|| operation("to_point missing"))?}),
        )?;
    } else {
        let page_data = s.document.pages.get(page).ok_or_else(|| operation("page missing"))?;
        let parallel = page_data
            .connects
            .iter()
            .filter(|c| c.to_sheet == from)
            .filter(|c| {
                page_data.connects.iter().any(|other| other.from_sheet == c.from_sheet && other.to_sheet == to && other.from_cell != c.from_cell)
            })
            .map(|c| c.from_sheet)
            .collect::<std::collections::BTreeSet<_>>();
        if !parallel.is_empty() {
            // Establish the witnessed straight style on every parallel route
            // before changing either endpoint, avoiding transient orthogonal
            // routes which could fail before the final ports are installed.
            for link in parallel.into_iter().map(|v| json!(v)).chain(std::iter::once(id.clone())) {
                s.execute("sheet.set", &json!({"page":page+1,"id":link,"name":"ShapeRouteStyle","value":2}))?;
            }
        }
        if glue == "dynamic" {
            for (end, target) in [("begin", from), ("end", to)] {
                s.execute("connect.dynamic", &json!({"page":page+1,"id":id,"end":end,"target":target}))?;
            }
        } else {
            s.execute("connect.sides", &json!({"page":page+1,"id":id,"source":from,"target":to}))?;
        }
        let newest = id.as_u64().and_then(|v| u32::try_from(v).ok()).ok_or_else(|| operation("connector id missing"))?;
        for edit in crate::layout::parallel_plan(s, page, from, to, newest)? {
            if let Some(id) = edit.get("style") {
                s.execute("sheet.set", &json!({"page":page+1,"id":id,"name":"ShapeRouteStyle","value":2}))?;
            } else {
                let point = s.execute("shape.connection.add", &json!({"page":page+1,"id":edit.get("target"),"x":edit.get("x"),"y":edit.get("y")}))?;
                s.execute(
                    "connect.static",
                    &json!({"page":page+1,"id":edit.get("id"),"end":edit.get("end"),"target":edit.get("target"),"point":point.get("cell")}),
                )?;
            }
        }
    }
    Ok(json!({"id":id}))
}

// Session dispatches this command outside an active edit so it can retain the
// fine-grained publication boundaries under one enclosing rollback snapshot.
pub(crate) fn connector_command(s: &mut Session, p: &Value) -> Result<Value> {
    execute(s, "connector.add", p)
}
