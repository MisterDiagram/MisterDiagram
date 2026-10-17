//! Atomic session batches retain every strict write/re-import boundary.
use crate::{Result, Session, operation, params_size};
use serde_json::{Value, json};
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
        for (index, item) in commands.iter().enumerate() {
            let result = (|| {
                params_size(item)?;
                if item.as_object().is_none_or(|p| p.keys().any(|k| k != "command" && k != "params")) {
                    return Err(operation("batch item requires command and optional params"));
                }
                let command = crate::edit::name(item, "command")?;
                candidate.read_reservations = reservations.clone();
                let result = candidate.execute(command, item.get("params").unwrap_or(&json!({})))?;
                output_bytes = output_bytes.saturating_add(serde_json::to_vec(&result).map_err(operation)?.len());
                if output_bytes > 1024 * 1024 {
                    return Err(operation("batch result byte limit"));
                }
                Ok(result)
            })();
            match result {
                Ok(value) => results.push(value),
                Err(error) => return Err(operation(format!("batch command index {index} (command {}): {error}", index + 1))),
            }
        }
        let mut before = self.snapshot();
        before.stencils = Some(self.stencils.clone());
        candidate.undo = self.undo.clone();
        candidate.undo.push_back(before);
        candidate.redo.clear();
        candidate.read_reservations.clear();
        candidate.trim_history();
        if candidate.undo.is_empty() {
            return Err(operation("batch undo snapshot exceeds retained budget"));
        }
        *self = candidate;
        Ok(json!({"results":results}))
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
