//! Transactional headless document session.
//! Session, snapshot history and journal pattern adapted from deckcraft
//! be14248577b92059eb2b8c0c9460cceffbc1f01d, MIT OR Apache-2.0.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
mod arrange;
pub mod cmd;
mod content;
mod edit;
mod files;
mod glue;
pub mod guard;
mod masters;
mod pages;
mod shapes;
mod sheet;
mod stencils;
pub use cmd::{command_specs, find_command};
use diagramcraft_model::{Document, ShapeId};
use serde_json::Value;
use std::{collections::VecDeque, path::PathBuf, sync::Arc};

pub type Result<T> = std::result::Result<T, EngineError>;
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("{cmd}: {msg}")]
    BadParams { cmd: String, msg: String },
    #[error("command {0} failed internally: {1}")]
    Internal(String, String),
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("{0}")]
    Operation(String),
}
pub(crate) fn operation(e: impl std::fmt::Display) -> EngineError {
    EngineError::Operation(e.to_string())
}

/// Page index and IDs in that page. IDs are never vector indices.
/// https://learn.microsoft.com/en-us/office/vba/api/visio.selection
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Selection {
    pub page: usize,
    pub shapes: Vec<ShapeId>,
}
#[derive(Clone)]
struct Snapshot {
    document: Arc<Document>,
    selection: Selection,
    path: Option<PathBuf>,
}
#[derive(Clone)]
pub struct JournalEntry {
    pub command: String,
    pub params: Arc<Value>,
    bytes: usize,
}
#[derive(Clone)]
pub struct Session {
    stencils: Vec<stencils::Stencil>,
    document: Arc<Document>,
    selection: Selection,
    path: Option<PathBuf>,
    undo: VecDeque<Snapshot>,
    redo: VecDeque<Snapshot>,
    journal: VecDeque<JournalEntry>,
    journal_bytes: usize,
    dirty: bool,
    editing: bool,
}
impl Default for Session {
    fn default() -> Self {
        Self {
            document: Arc::new(files::blank_document(false)),
            stencils: Vec::new(),
            selection: Selection::default(),
            path: None,
            undo: VecDeque::new(),
            redo: VecDeque::new(),
            journal: VecDeque::new(),
            journal_bytes: 0,
            dirty: false,
            editing: false,
        }
    }
}
impl Session {
    pub fn new() -> Self {
        guard::install_panic_hook();
        Self::default()
    }
    pub fn document(&self) -> &Document {
        &self.document
    }
    pub fn selection(&self) -> &Selection {
        &self.selection
    }
    pub fn journal(&self) -> &VecDeque<JournalEntry> {
        &self.journal
    }
    pub fn commands(&self) -> Vec<cmd::CommandInfo> {
        command_specs().iter().map(|c| c.info(self)).collect()
    }
    fn snapshot(&self) -> Snapshot {
        Snapshot { document: self.document.clone(), selection: self.selection.clone(), path: self.path.clone() }
    }
    fn restore(&mut self, snapshot: Snapshot) {
        self.document = snapshot.document;
        self.selection = snapshot.selection;
        self.path = snapshot.path;
    }
    /// Execute one command as one rollback, undo and journal boundary.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.application.beginundoscope
    pub fn execute(&mut self, id: &str, params: &Value) -> Result<Value> {
        let bytes = params_size(params)?;
        let spec = find_command(id).ok_or_else(|| EngineError::UnknownCommand(id.chars().take(256).collect()))?;
        validate_schema(spec.params, params)?;
        self.edit(|s| {
            (spec.enabled)(s).map_err(operation)?;
            let before = s.snapshot();
            let result = (spec.run)(s, params)?;
            if spec.undoable && !Arc::ptr_eq(&before.document, &s.document) {
                s.undo.push_back(before);
                s.redo.clear();
                s.trim_history();
            }
            if spec.journal {
                s.record(id, params, bytes);
            }
            Ok(result)
        })
    }
    fn record(&mut self, id: &str, params: &Value, bytes: usize) {
        self.journal.push_back(JournalEntry { command: id.into(), params: Arc::new(params.clone()), bytes });
        self.journal_bytes = self.journal_bytes.saturating_add(bytes);
        while self.journal.len() > 1000 || self.journal_bytes > 8 * 1024 * 1024 {
            if let Some(entry) = self.journal.pop_front() {
                self.journal_bytes = self.journal_bytes.saturating_sub(entry.bytes);
            }
        }
    }
    /// Snapshot before running an edit, recalculate affected caches before publication,
    /// and restore all session state on either a returned error or a panic.
    /// The writer's dependency-aware cache policy also preserves unchanged imported V/F/U.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.application.endundoscope
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
    pub fn edit<T>(&mut self, f: impl FnOnce(&mut Self) -> Result<T>) -> Result<T> {
        if self.editing {
            return Err(operation("nested session edits are not supported"));
        }
        let before = self.clone();
        self.editing = true;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let result = f(self)?;
            if self.dirty {
                let identities = |d: &Document| {
                    (
                        d.pages.iter().map(|p| (p.id, p.name.clone())).collect::<Vec<_>>(),
                        d.masters.iter().map(|m| (m.id, m.name.clone())).collect::<Vec<_>>(),
                    )
                };
                if identities(&before.document) != identities(&self.document)
                    && let Some(mut part) = self.document.parts.get("docProps/app.xml").cloned()
                    && let Some(app) = &mut part.xml
                {
                    diagramcraft_model::skeleton::update_summary(&self.document, app).map_err(operation)?;
                    part.bytes.clear();
                    Arc::make_mut(&mut self.document).parts.insert("docProps/app.xml".into(), part);
                }
                let mut glue = diagramcraft_route::GlueHost::prepare(&before.document, Arc::make_mut(&mut self.document)).map_err(operation)?;
                diagramcraft_route::reroute(&before.document, Arc::make_mut(&mut self.document), &mut glue, None).map_err(operation)?;
                let theme = diagramcraft_theme::Themes::from_document(&self.document).map_err(operation)?;
                let written = diagramcraft_vsdx::write_with_glue_host(&self.document, Some(&theme), Some(&diagramcraft_text::Host), Some(&mut glue))
                    .map_err(operation)?;
                self.document = Arc::new(diagramcraft_vsdx::import(&written.bytes).map_err(operation)?);
                files::update_palette(Arc::make_mut(&mut self.document))?;
                self.dirty = false;
            }
            if self.selection.page >= self.document.pages.len() {
                self.selection.page = 0;
                self.selection.shapes.clear();
            }
            self.editing = false;
            Ok(result)
        }));
        match result {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => {
                *self = before;
                Err(error)
            }
            Err(payload) => {
                *self = before;
                Err(EngineError::Internal("session.edit".into(), guard::panic_message(payload.as_ref())))
            }
        }
    }
    /// Mutate only within a transaction. The edited document is validated and
    /// recalculated by the existing writer before the transaction is published.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.application.beginundoscope
    pub fn edit_document<T>(&mut self, f: impl FnOnce(&mut Document, &mut Selection) -> Result<T>) -> Result<T> {
        if !self.editing {
            return self.edit(|s| {
                let before = s.snapshot();
                let result = s.edit_document(f)?;
                s.undo.push_back(before);
                s.redo.clear();
                s.trim_history();
                Ok(result)
            });
        }
        self.dirty = true;
        f(Arc::make_mut(&mut self.document), &mut self.selection)
    }
    fn trim_history(&mut self) {
        // Keep snapshots bounded both by count and retained source size. Parsed
        // trees are charged conservatively in addition to their source XML bytes.
        let cost = |s: &Snapshot| s.document.parts.values().fold(1024 * 1024usize, |n, p| n.saturating_add(p.bytes.len().saturating_mul(16)));
        let mut bytes = self.undo.iter().chain(&self.redo).fold(0usize, |n, s| n.saturating_add(cost(s)));
        while self.undo.len() + self.redo.len() > 32 || bytes > 256 * 1024 * 1024 {
            let removed = self.undo.pop_front().or_else(|| self.redo.pop_front());
            if let Some(s) = removed {
                bytes = bytes.saturating_sub(cost(&s));
            } else {
                break;
            }
        }
    }
    /// Reverse one successful document edit, retaining a redo snapshot.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.application.undo
    pub(crate) fn undo(&mut self, _: &Value) -> Result<Value> {
        let Some(old) = self.undo.pop_back() else {
            return Ok(serde_json::json!({"changed":false}));
        };
        self.redo.push_back(self.snapshot());
        self.restore(old);
        self.trim_history();
        Ok(serde_json::json!({"changed":true}))
    }
    /// Reapply one undone edit. A new edit clears the redo stack.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.application.redo
    pub(crate) fn redo(&mut self, _: &Value) -> Result<Value> {
        let Some(next) = self.redo.pop_back() else {
            return Ok(serde_json::json!({"changed":false}));
        };
        self.undo.push_back(self.snapshot());
        self.restore(next);
        self.trim_history();
        Ok(serde_json::json!({"changed":true}))
    }
}
fn params_size(params: &Value) -> Result<usize> {
    if !params.is_object() {
        return Err(operation("command parameters must be an object"));
    }
    let mut pending = vec![(params, 0usize)];
    let (mut nodes, mut bytes) = (0usize, 0usize);
    while let Some((value, depth)) = pending.pop() {
        nodes += 1;
        if nodes > 65536 || depth > 64 {
            return Err(operation("command parameter nesting or node limit"));
        }
        bytes = bytes.saturating_add(32);
        match value {
            Value::String(s) => bytes = bytes.saturating_add(s.len()),
            Value::Array(items) => {
                if items.len().saturating_add(pending.len()).saturating_add(nodes) > 65536 {
                    return Err(operation("parameter collection limit"));
                }
                pending.extend(items.iter().map(|v| (v, depth + 1)));
            }
            Value::Object(items) => {
                if items.len().saturating_add(pending.len()).saturating_add(nodes) > 65536 {
                    return Err(operation("parameter collection limit"));
                }
                for (k, v) in items {
                    bytes = bytes.saturating_add(k.len());
                    pending.push((v, depth + 1));
                }
            }
            _ => {}
        }
        if bytes > 1024 * 1024 {
            return Err(operation("command parameter byte limit"));
        }
    }
    Ok(bytes)
}

fn validate_schema(schema: &str, params: &Value) -> Result<()> {
    let schema: Value = serde_json::from_str(schema).map_err(operation)?;
    let object = params.as_object().ok_or_else(|| operation("parameters must be an object"))?;
    if let Some(required) = schema.get("required").and_then(Value::as_array) {
        for name in required.iter().filter_map(Value::as_str) {
            if !object.contains_key(name) {
                return Err(operation(format!("missing parameter {name}")));
            }
        }
    }
    for (name, value) in object {
        let rule = schema.get("properties").and_then(|p| p.get(name));
        if rule.is_none() && schema.get("additionalProperties") == Some(&Value::Bool(false)) {
            return Err(operation(format!("unknown parameter {name}")));
        }
        let Some(rule) = rule else { continue };
        let valid = match rule.get("type").and_then(Value::as_str) {
            Some("string") => value.is_string(),
            Some("boolean") => value.is_boolean(),
            Some("number") => value.as_f64().is_some_and(f64::is_finite),
            Some("integer") => value.as_i64().is_some() || value.as_u64().is_some(),
            Some("array") => value.is_array(),
            Some("object") => value.is_object(),
            _ => true,
        };
        if !valid {
            return Err(operation(format!("invalid type for {name}")));
        }
        if let Some(number) = value.as_f64() {
            for (key, rejected) in [
                ("minimum", rule.get("minimum").and_then(Value::as_f64).is_some_and(|n| number < n)),
                ("maximum", rule.get("maximum").and_then(Value::as_f64).is_some_and(|n| number > n)),
                ("exclusiveMinimum", rule.get("exclusiveMinimum").and_then(Value::as_f64).is_some_and(|n| number <= n)),
            ] {
                if rejected {
                    return Err(operation(format!("{name} violates {key}")));
                }
            }
        }
        if let Some(text) = value.as_str() {
            let length = text.chars().count() as u64;
            if rule.get("minLength").and_then(Value::as_u64).is_some_and(|n| length < n)
                || rule.get("maxLength").and_then(Value::as_u64).is_some_and(|n| length > n)
            {
                return Err(operation(format!("{name} length is out of bounds")));
            }
        }
        if let Some(allowed) = rule.get("enum").and_then(Value::as_array)
            && !allowed.contains(value)
        {
            return Err(operation(format!("invalid {name}")));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parameter_limits_cover_depth_nodes_and_bytes_before_clone() {
        let mut value = Value::Null;
        for _ in 0..62 {
            value = json!([value]);
        }
        assert!(params_size(&json!({"x":value.clone()})).is_ok());
        value = json!([value]);
        assert!(params_size(&json!({"x":value.clone()})).is_ok());
        value = json!([value]);
        assert!(params_size(&json!({"x":value})).unwrap_err().to_string().contains("nesting"));
        assert!(params_size(&json!({"x":vec![Value::Null;65536]})).unwrap_err().to_string().contains("collection"));
        let text = "x".repeat(1024 * 1024 - 65);
        assert_eq!(params_size(&json!({"x":text})).unwrap(), 1024 * 1024);
        assert!(params_size(&json!({"x":"x".repeat(1024 * 1024 - 64)})).unwrap_err().to_string().contains("byte limit"));
    }

    #[test]
    fn history_and_journal_evict_oldest_entries() {
        let mut s = Session::new();
        for i in 0..36 {
            s.edit_document(|doc, _| {
                doc.pages[0].name = format!("Page {i}");
                Ok(())
            })
            .unwrap();
        }
        assert_eq!(s.undo.len(), 32);
        for _ in 0..32 {
            assert_eq!(s.execute("edit.undo", &json!({})).unwrap()["changed"], true);
        }
        assert_eq!(s.document.pages[0].name, "Page 3");
        assert_eq!(s.execute("edit.undo", &json!({})).unwrap()["changed"], false);
        for _ in 0..1001 {
            s.execute("edit.undo", &json!({})).unwrap();
        }
        assert_eq!(s.journal.len(), 1000);
        assert_eq!(s.journal_bytes, s.journal.iter().map(|e| e.bytes).sum::<usize>());
        // Large retained source parts cause eviction before the count limit.
        let mut doc = s.document.as_ref().clone();
        let part = doc.parts.values_mut().next().unwrap();
        part.bytes.resize(17 * 1024 * 1024, 0);
        s.undo.push_back(Snapshot { document: Arc::new(doc), selection: Selection::default(), path: None });
        s.trim_history();
        assert!(s.undo.is_empty());
    }
    #[test]
    fn journal_byte_budget_evicts_before_entry_count_limit() {
        let mut s = Session::new();
        for i in 0..9 {
            let params = json!({"index":i,"text":"x".repeat(1024 * 1024 - 200)});
            let bytes = params_size(&params).unwrap();
            s.record("test", &params, bytes);
        }
        assert_eq!(s.journal.len(), 8);
        assert_eq!(s.journal.front().unwrap().params["index"], 1);
        assert!(s.journal_bytes <= 8 * 1024 * 1024);
        assert_eq!(s.journal_bytes, s.journal.iter().map(|e| e.bytes).sum::<usize>());
    }
}
