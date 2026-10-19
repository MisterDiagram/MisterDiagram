//! Atomic session batches retain every strict write/re-import boundary.
use crate::{Result, Session, operation, params_size};
use serde_json::{Value, json};
use std::collections::BTreeMap;
// References are command-surface state only, never document cells or metadata.
fn reference_name(item: &Value) -> Result<Option<&str>> {
    item.get("as")
        .map(|v| {
            v.as_str()
                .filter(|name| {
                    !name.is_empty() && name.len() <= 64 && name.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.'))
                })
                .ok_or_else(|| operation("as requires a name of 1 through 64 ASCII letters, digits, underscore, hyphen or dot"))
        })
        .transpose()
}
fn resolve(value: &mut Value, shape_id: bool, references: &BTreeMap<String, Value>, depth: usize) -> Result<()> {
    if depth > 64 {
        return Err(operation("batch reference nesting limit"));
    }
    match value {
        Value::String(text) => {
            if shape_id {
                if let Some(name) = text.strip_prefix('$') {
                    *value = references.get(name).cloned().ok_or_else(|| operation(format!("unknown batch reference ${name}")))?;
                }
            } else if let Some(literal) = text.strip_prefix("$$") {
                *text = format!("${literal}");
            }
        }
        Value::Array(items) => {
            for item in items {
                resolve(item, shape_id, references, depth + 1)?;
            }
        }
        Value::Object(items) => {
            for (key, item) in items {
                let is_id = matches!(key.as_str(), "id" | "source" | "target" | "root" | "hub" | "from" | "to" | "ids" | "spokes" | "edges");
                resolve(item, is_id, references, depth + 1)?;
            }
        }
        _ => {}
    }
    Ok(())
}
impl Session {
    /// Execute in order, roll back the entire session on failure, or retain one
    /// undo snapshot on success. File outputs already published by commands are
    /// external side effects and are not reverted by session rollback.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.application.beginundoscope
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.application.endundoscope
    pub fn execute_batch(&mut self, params: &Value) -> Result<Value> {
        params_size(params)?;
        if self.editing {
            return Err(operation("batch cannot run inside an edit"));
        }
        if params.as_object().is_none_or(|p| p.len() != 1) {
            return Err(operation("batch requires only commands"));
        }
        let commands = params
            .get("commands")
            .and_then(Value::as_array)
            .filter(|v| !v.is_empty() && v.len() <= 1024)
            .ok_or_else(|| operation("batch requires 1 through 1024 commands"))?;
        let mut candidate = self.clone();
        let reservations = self.retained_documents().into_iter().cloned().collect::<Vec<_>>();
        let mut results = Vec::new();
        let mut output_bytes = 0usize;
        let mut references = BTreeMap::new();
        for (index, item) in commands.iter().enumerate() {
            let result = (|| {
                params_size(item)?;
                if item.as_object().is_none_or(|p| p.keys().any(|k| k != "command" && k != "params" && k != "as")) {
                    return Err(operation("batch item requires command and optional params/as"));
                }
                let command = crate::edit::name(item, "command")?;
                if command == "engine.batch"
                    || (command == "engine.execute"
                        && item.get("params").and_then(|p| p.get("command")).and_then(Value::as_str) == Some("engine.batch"))
                {
                    return Err(operation("nested batches are not supported; combine commands in one batch"));
                }
                let alias = reference_name(item)?;
                if alias.is_some_and(|name| references.contains_key(name)) {
                    return Err(operation("duplicate batch reference name"));
                }
                let mut params = item.get("params").cloned().unwrap_or_else(|| json!({}));
                resolve(&mut params, false, &references, 0)?;
                candidate.read_reservations = reservations.clone();
                candidate.stencil_reservations = self.retained_stencils();
                let result = candidate.execute(command, &params)?;
                if let Some(name) = alias {
                    let id = result
                        .get("id")
                        .filter(|v| v.as_u64().is_some_and(|id| id <= u64::from(u32::MAX)))
                        .ok_or_else(|| operation("named batch command did not return an id"))?;
                    references.insert(name.to_owned(), id.clone());
                }
                output_bytes = output_bytes.saturating_add(serde_json::to_vec(&result).map_err(operation)?.len());
                if output_bytes > 1024 * 1024 {
                    return Err(operation("batch result byte limit; use fewer batch commands or cells/sections filters"));
                }
                Ok(result)
            })();
            match result {
                Ok(value) => results.push(value),
                Err(error) => {
                    self.observe_external_save(&candidate);
                    return Err(operation(format!("batch command index {index} (command {}): {error}", index + 1)));
                }
            }
        }
        self.publish_batch(candidate, json!({"results":results,"references":references}))
    }
    pub(crate) fn publish_batch(&mut self, mut candidate: Self, result: Value) -> Result<Value> {
        let mut before = self.snapshot();
        // Immutable open stencils are already shared by Arc. A document-only
        // batch must not charge their sources again as undo state (#58).
        if self.stencils.len() != candidate.stencils.len()
            || self.stencils.iter().zip(&candidate.stencils).any(|(a, b)| !std::sync::Arc::ptr_eq(&a.document, &b.document))
        {
            before.stencils = Some(self.stencils.clone());
        }
        candidate.undo = self.undo.clone();
        candidate.undo.push_back(before);
        candidate.redo.clear();
        candidate.read_reservations = self.read_reservations.clone();
        candidate.stencil_reservations = self.stencil_reservations.clone();
        let history = candidate.retention().history;
        let inventory_changed = candidate.undo.back().is_some_and(|s| s.stencils.is_some());
        candidate.trim_history();
        if candidate.undo.is_empty() {
            self.observe_external_save(&candidate);
            let error = crate::retention::retention_error(
                "history",
                256 * 1024 * 1024,
                history.saturating_sub(self.retention().history),
                self.retention().history,
            );
            let hint = if inventory_changed { "; opening stencils in their own call avoids inventory rollback retention" } else { "" };
            return Err(operation(format!("batch undo snapshot exceeds retained budget: {error}{hint}")));
        }
        if let Err(error) = crate::result_budget(&result) {
            self.observe_external_save(&candidate);
            return Err(error);
        }
        *self = candidate;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn third_failure_restores_document_selection_stencils_history_and_journal() {
        let mut s = Session::new();
        s.execute("shape.draw.rect", &json!({"x":1,"y":1,"width":2,"height":1})).unwrap();
        s.execute("shape.move", &json!({"id":1,"x":3,"y":3})).unwrap();
        s.execute("edit.undo", &json!({})).unwrap();
        let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/batch-stencils").join(std::process::id().to_string());
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("original.vssx");
        s.execute("file.save", &json!({"path":path})).unwrap();
        let before = s.clone();
        let commands = json!({"commands":[
            {"command":"stencil.open","params":{"path":path}},
            {"command":"page.add","params":{"name":"Transient"}},
            {"command":"shape.move","params":{"id":999,"x":0,"y":0}},
            {"command":"file.new"},{"command":"page.add","params":{"name":"Not run"}}
        ]});
        let error = s.execute_batch(&commands).unwrap_err().to_string();
        assert!(error.contains("index 2 (command 3)"), "{error}");
        assert_eq!(s.document(), before.document(), "failed batch must restore the entire document");
        assert_eq!(s.selection, before.selection);
        assert_eq!(s.path, before.path);
        assert_eq!(s.stencils.len(), before.stencils.len());
        assert_eq!(s.undo.len(), before.undo.len());
        assert_eq!(s.redo.len(), before.redo.len());
        assert_eq!(s.journal.len(), before.journal.len());
        s.execute("edit.redo", &json!({})).unwrap();
        assert_eq!(s.execute("sheet.get", &json!({"id":1,"name":"PinX"})).unwrap()["value"], "3");
    }
    #[test]
    fn successful_stencil_batch_undo_redo_restores_stencil_inventory() {
        let mut s = Session::new();
        let directory =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/batch-stencils-success").join(std::process::id().to_string());
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("original.vssx");
        s.execute("file.save", &json!({"path":path})).unwrap();
        s.execute_batch(&json!({"commands":[{"command":"stencil.open","params":{"path":path}}]})).unwrap();
        assert_eq!(s.stencils.len(), 1);
        s.execute("edit.undo", &json!({})).unwrap();
        assert!(s.stencils.is_empty());
        s.execute("edit.redo", &json!({})).unwrap();
        assert_eq!(s.stencils.len(), 1);
    }
    #[test]
    fn success_is_one_undo_and_redo_step_with_strict_cached_reads() {
        let mut s = Session::new();
        let result = s
            .execute_batch(&json!({"commands":[
                {"command":"shape.draw.rect","params":{"x":1,"y":1,"width":2,"height":1}},
                {"command":"shape.move","params":{"id":1,"x":4,"y":5}},
                {"command":"sheet.get","params":{"id":1,"name":"PinX"}}
            ]}))
            .unwrap();
        assert_eq!(result["results"][2]["cached"], "4");
        assert_eq!(s.undo.len(), 1);
        let after = s.document().clone();
        s.execute("edit.undo", &json!({})).unwrap();
        assert!(s.document().pages[0].shapes.is_empty());
        s.execute("edit.redo", &json!({})).unwrap();
        assert_eq!(s.document(), &after);
        for p in [
            Value::Null,
            json!([]),
            json!({}),
            json!({"commands":[]}),
            json!({"commands":[null]}),
            json!({"commands":[{"command":"file.new","params":[]}]}),
            json!({"commands":vec![json!({"command":"file.new"});1025]}),
        ] {
            assert!(s.execute_batch(&p).is_err());
            assert_eq!(s.document(), &after);
        }
    }
}
