//! Transactional headless document session.
//! Session, snapshot history and journal pattern adapted from deckcraft
//! be14248577b92059eb2b8c0c9460cceffbc1f01d, MIT OR Apache-2.0.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
mod arrange;
mod batch;
pub mod cmd;
mod composition;
mod content;
mod edit;
mod files;
mod glue;
pub mod guard;
mod inspect;
mod layout;
mod master_scale;
#[cfg(test)]
mod master_scale_tests;
mod masters;
mod network;
mod pages;
#[cfg(test)]
mod panic_tests;
mod reference_cleanup;
mod retention;
mod shapes;
mod sheet;
mod stencil_path;
#[cfg(test)]
mod stencil_path_tests;
mod stencils;
pub use cmd::{command_specs, find_command};
use misterdiagram_model::{Document, ShapeId};
pub use retention::MemoryLimits;
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
    route_warnings: std::collections::BTreeMap<(u32, u32), String>,
    stencils: Option<Vec<stencils::Stencil>>,
    document: Arc<Document>,
    selection: Selection,
    path: Option<PathBuf>,
    revision: u64,
}
#[derive(Clone)]
pub struct JournalEntry {
    pub command: String,
    pub params: Arc<Value>,
    bytes: usize,
}
#[derive(Clone)]
pub struct Session {
    route_warnings: std::collections::BTreeMap<(u32, u32), String>,
    memory: MemoryLimits,
    stencils: Vec<stencils::Stencil>,
    stencil_path: Vec<PathBuf>,
    document: Arc<Document>,
    selection: Selection,
    path: Option<PathBuf>,
    undo: VecDeque<Snapshot>,
    redo: VecDeque<Snapshot>,
    journal: VecDeque<JournalEntry>,
    journal_bytes: usize,
    dirty: bool,
    routing_validated: std::sync::Weak<Document>,
    editing: bool,
    read_reservations: Vec<Arc<Document>>,
    stencil_reservations: Vec<Arc<Document>>,
    revision: u64,
    saved_revision: Option<u64>,
    save_generation: u64,
    next_revision: u64,
}
impl Default for Session {
    fn default() -> Self {
        Self {
            route_warnings: Default::default(),
            memory: MemoryLimits::default(),
            document: Arc::new(files::blank_document(false)),
            stencils: Vec::new(),
            stencil_path: Vec::new(),
            selection: Selection::default(),
            path: None,
            undo: VecDeque::new(),
            redo: VecDeque::new(),
            journal: VecDeque::new(),
            journal_bytes: 0,
            dirty: false,
            routing_validated: std::sync::Weak::new(),
            revision: 0,
            saved_revision: Some(0),
            save_generation: 0,
            next_revision: 1,
            editing: false,
            read_reservations: Vec::new(),
            stencil_reservations: Vec::new(),
        }
    }
}
impl Session {
    pub fn new() -> Self {
        guard::install_panic_hook();
        Self::default()
    }
    pub fn with_memory_limits(memory: MemoryLimits) -> Self {
        Self { memory, ..Self::new() }
    }
    fn retained_documents(&self) -> Vec<&Arc<Document>> {
        let mut documents = vec![&self.document];
        documents.extend(self.stencils.iter().map(|s| &s.document));
        documents.extend(&self.read_reservations);
        for snapshot in self.undo.iter().chain(&self.redo) {
            documents.push(&snapshot.document);
            if let Some(stencils) = &snapshot.stencils {
                documents.extend(stencils.iter().map(|s| &s.document));
            }
        }
        documents
    }
    pub(crate) fn retained_xml_nodes(&self) -> usize {
        self.retention().nodes
    }
    fn observe_external_save(&mut self, candidate: &Self) {
        if self.save_generation != candidate.save_generation {
            self.saved_revision = None;
            self.save_generation = candidate.save_generation;
            self.next_revision = self.next_revision.max(candidate.next_revision);
        }
    }
    fn advance_revision(&mut self) -> Result<()> {
        let next = self.next_revision.checked_add(1).ok_or_else(|| operation("document revision limit"))?;
        self.revision = self.next_revision;
        self.next_revision = next;
        Ok(())
    }
    pub fn has_unsaved_changes(&self) -> bool {
        Some(self.revision) != self.saved_revision
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
        Snapshot {
            route_warnings: self.route_warnings.clone(),
            stencils: None,
            document: self.document.clone(),
            selection: self.selection.clone(),
            path: self.path.clone(),
            revision: self.revision,
        }
    }
    fn restore(&mut self, snapshot: Snapshot) {
        self.route_warnings = snapshot.route_warnings;
        if let Some(stencils) = snapshot.stencils {
            self.stencils = stencils;
        }
        self.document = snapshot.document;
        self.selection = snapshot.selection;
        self.path = snapshot.path;
        self.revision = snapshot.revision;
    }
    /// Execute one command as one rollback, undo and journal boundary.
    /// https://learn.microsoft.com/en-us/office/vba/api/visio.application.beginundoscope
    pub fn execute(&mut self, id: &str, params: &Value) -> Result<Value> {
        // Edits restore their own snapshot. Orchestrators publish a candidate only
        // on success, so unwinding discards it without another rollback snapshot.
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.dispatch(id, params))) {
            Ok(result) => result,
            Err(payload) => Err(EngineError::Internal(id.into(), guard::panic_message(payload.as_ref()))),
        }
    }
    fn dispatch(&mut self, id: &str, params: &Value) -> Result<Value> {
        let bytes = params_size(params)?;
        let spec = find_command(id).ok_or_else(|| EngineError::UnknownCommand(id.chars().take(256).collect()))?;
        validate_schema(spec.params, params)?;
        if composition::applies(id, params) {
            return composition::execute(self, id, params);
        }
        if id == "engine.batch" {
            return self.execute_batch(params);
        }
        if id == "engine.execute" {
            return inspect::execute(self, params);
        }
        self.edit_finalized(
            |s| {
                (spec.enabled)(s).map_err(operation)?;
                let before = s.snapshot();
                let result = (spec.run)(s, params)?;
                result_budget(&result)?;
                if spec.undoable && !Arc::ptr_eq(&before.document, &s.document) {
                    s.undo.push_back(before);
                    s.redo.clear();
                    s.trim_history();
                }
                if spec.journal {
                    s.record(id, params, bytes);
                }
                Ok(result)
            },
            |s, result| {
                s.add_route_warnings(result);
                result_budget(result)
            },
        )
    }
    fn apply_route_outcomes(&mut self, host: &misterdiagram_route::GlueHost) {
        for (key, outcome) in host.outcomes() {
            if let Some(limit) = outcome {
                self.route_warnings.insert(*key, limit.clone());
            } else {
                self.route_warnings.remove(key);
            }
        }
        fn contains(shapes: &[misterdiagram_model::Shape], id: u32, depth: usize) -> bool {
            depth <= 128 && shapes.iter().any(|s| s.id == id || contains(&s.shapes, id, depth + 1))
        }
        self.route_warnings.retain(|(page, id), _| self.document.pages.iter().any(|p| p.id == *page && contains(&p.shapes, *id, 0)));
    }
    fn add_route_warnings(&self, result: &mut Value) {
        if let Some(object) = result.as_object_mut() {
            object.remove("route_warnings");
            if !self.route_warnings.is_empty() {
                object.insert(
                    "route_warnings".into(),
                    serde_json::json!(
                        self.route_warnings
                            .iter()
                            .map(|((page, id), limit)| serde_json::json!({"page":page,"connector":id,"limit":limit}))
                            .collect::<Vec<_>>()
                    ),
                );
            }
        }
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
        self.edit_finalized(f, |_, _| Ok(()))
    }
    fn edit_finalized<T>(&mut self, f: impl FnOnce(&mut Self) -> Result<T>, finish: impl FnOnce(&Self, &mut T) -> Result<()>) -> Result<T> {
        if self.editing {
            return Err(operation("nested session edits are not supported"));
        }
        let mut before = self.clone();
        // Rollback documents can leave the active census after copy-on-write or
        // history eviction, but the transaction snapshot keeps them alive.
        self.read_reservations.extend(before.retained_documents().into_iter().cloned());
        self.stencil_reservations.extend(before.retained_stencils());
        self.editing = true;
        let mut published = false;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut result = f(self)?;
            self.check_retention(before.retention(), self.retention())?;
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
                    misterdiagram_model::skeleton::update_summary(&self.document, app).map_err(operation)?;
                    part.bytes.clear();
                    Arc::make_mut(&mut self.document).parts.insert("docProps/app.xml".into(), part);
                }
                let validated = self.routing_validated.upgrade().is_some_and(|d| Arc::ptr_eq(&d, &before.document));
                let mut glue = misterdiagram_route::GlueHost::prepare_scoped(&before.document, Arc::make_mut(&mut self.document), validated)
                    .map_err(operation)?;
                misterdiagram_route::reroute(&before.document, Arc::make_mut(&mut self.document), &mut glue, None).map_err(operation)?;
                self.apply_route_outcomes(&glue);
                let theme = misterdiagram_theme::Themes::from_document(&self.document).map_err(operation)?;
                let written = misterdiagram_vsdx::write::write_parts_with_glue_host(
                    &self.document,
                    Some(&theme),
                    Some(&misterdiagram_text::Host),
                    Some(&mut glue),
                )
                .map_err(operation)?;
                let held_nodes = self.retained_xml_nodes();
                let document =
                    misterdiagram_vsdx::import_written_parts(written, held_nodes).map_err(|error| retention::admission_error(error, held_nodes))?;
                self.reserve(Some((&document, false)))?;
                self.document = Arc::new(document);
                files::update_palette(Arc::make_mut(&mut self.document))?;
                self.routing_validated = Arc::downgrade(&self.document);
                self.advance_revision()?;
                published = true;
                self.dirty = false;
            }
            if self.selection.page >= self.document.pages.len() {
                self.selection.page = 0;
                self.selection.shapes.clear();
            }
            self.read_reservations = before.read_reservations.clone();
            self.stencil_reservations = before.stencil_reservations.clone();
            self.editing = false;
            // Keep post-publication history maintenance inside the edit guard.
            if published || self.stencils.len() > before.stencils.len() {
                self.trim_history();
            }
            finish(self, &mut result)?;
            Ok(result)
        }));
        match result {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => {
                before.observe_external_save(self);
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
        if Arc::strong_count(&self.document) > 1 {
            self.reserve(Some((&self.document, false)))?;
        }
        self.dirty = true;
        f(Arc::make_mut(&mut self.document), &mut self.selection)
    }
    fn trim_history(&mut self) {
        // Keep snapshots bounded by count, source size and retained XML nodes.
        // After a successful publication, reserve two current-document-sized copies: copy-on-write
        // and the next reader admission. Active documents and stencils never yield.
        // During an edit, rollback reservations pin old history, so node eviction
        // happens after commit, ahead of the next transaction snapshot.
        let admission = self.document.retained_xml_nodes.saturating_mul(2);
        while self.undo.len() + self.redo.len() > 32
            || self.retention().history > self.memory.history()
            || (!self.editing && self.retained_xml_nodes().saturating_add(admission) > misterdiagram_vsdx::MAX_RETAINED_XML_NODES)
        {
            let removed = self.undo.pop_front().or_else(|| self.redo.pop_front());
            if removed.is_none() {
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
        let mut next = self.snapshot();
        if old.stencils.is_some() {
            next.stencils = Some(self.stencils.clone());
        }
        self.redo.push_back(next);
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
        let mut old = self.snapshot();
        if next.stencils.is_some() {
            old.stencils = Some(self.stencils.clone());
        }
        self.undo.push_back(old);
        self.restore(next);
        self.trim_history();
        Ok(serde_json::json!({"changed":true}))
    }
}
/// Bound a serialized result including the escaped JSON text envelope used by MCP.
/// Counting while serializing avoids building an unbounded intermediate string.
pub(crate) fn result_budget(value: &Value) -> Result<()> {
    struct Budget(usize);
    impl std::io::Write for Budget {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            let escaped = bytes.iter().filter(|b| matches!(b, b'"' | b'\\')).count();
            self.0 = self.0.checked_sub(bytes.len().saturating_add(escaped)).ok_or_else(|| {
                std::io::Error::other(
                    "result exceeds 1048576 byte budget; use cells/sections/page filters, query/limit for stencil search, or fewer batch commands",
                )
            })?;
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    // Reserve 64 bytes for the complete text-content envelope (including quotes).
    serde_json::to_writer(Budget(1024 * 1024 - 64), value).map_err(operation)
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
        s.undo.push_back(Snapshot {
            route_warnings: Default::default(),
            stencils: None,
            document: Arc::new(doc),
            selection: Selection::default(),
            path: None,
            revision: s.revision,
        });
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
    #[test]
    fn reader_reservation_includes_rollback_only_document() {
        let mut s = Session::new();
        Arc::make_mut(&mut s.document).retained_xml_nodes = 10;
        for nodes in [40, 50] {
            let mut d = s.document.as_ref().clone();
            d.retained_xml_nodes = nodes;
            let snapshot = Snapshot {
                route_warnings: Default::default(),
                stencils: None,
                document: Arc::new(d),
                selection: Selection::default(),
                path: None,
                revision: s.revision,
            };
            if nodes == 40 {
                s.undo.push_back(snapshot);
            } else {
                s.redo.push_back(snapshot);
            }
        }
        let error = s
            .edit::<()>(|s| {
                s.undo.clear();
                s.redo.clear();
                s.edit_document(|d, _| {
                    d.retained_xml_nodes = 20;
                    Ok(())
                })?;
                assert_eq!(s.retained_xml_nodes(), 120);
                Err(operation("abort test transaction"))
            })
            .unwrap_err();
        assert!(error.to_string().contains("abort test transaction"));
        assert_eq!(s.retained_xml_nodes(), 100);
        assert_eq!(s.undo.len(), 1);
        assert_eq!(s.redo.len(), 1);
        assert!(s.read_reservations.is_empty());
    }
    #[test]
    fn history_yields_before_the_next_reader_admission() {
        let mut s = Session::new();
        s.execute("sheet.set", &json!({"scope":"page","name":"PageWidth","value":20})).unwrap();
        let nodes = s.document.retained_xml_nodes;
        assert!(nodes > 0);
        // Stand in for a large already admitted stencil, as in the stencil-budget
        // tests. Real small documents, commands, snapshots and reader admissions
        // exercise the remaining space without allocating millions of test nodes.
        let mut stencil = files::blank_document(false);
        stencil.retained_xml_nodes = misterdiagram_vsdx::MAX_RETAINED_XML_NODES - 6 * nodes;
        s.stencils.push(stencils::Stencil { path: "invented-history-stencil".into(), document: Arc::new(stencil) });
        for i in 0..40 {
            let result = s.execute("sheet.set", &json!({"scope":"page","name":"PageWidth","value":21+i}));
            assert!(result.is_ok(), "history must yield before edit {} is refused: {result:?}", i + 1);
            assert!(s.retained_xml_nodes() <= misterdiagram_vsdx::MAX_RETAINED_XML_NODES);
        }
        assert!(!s.undo.is_empty(), "recent edits should remain undoable");
        assert!(s.undo.len() < 32, "nodes must evict history before the count limit");
        assert_eq!(s.execute("edit.undo", &json!({})).unwrap()["changed"], true);
        assert_eq!(s.execute("edit.redo", &json!({})).unwrap()["changed"], true);
    }
    #[test]
    fn capacity_pressure_does_not_trim_reads_failures_or_undo() {
        let mut s = Session::new();
        for value in [20, 21, 22] {
            s.execute("sheet.set", &json!({"scope":"page","name":"PageWidth","value":value})).unwrap();
        }
        let mut stencil = files::blank_document(false);
        stencil.retained_xml_nodes = misterdiagram_vsdx::MAX_RETAINED_XML_NODES - s.retained_xml_nodes();
        s.stencils.push(stencils::Stencil { path: "invented-full-stencil".into(), document: Arc::new(stencil) });
        let history = s.undo.len();
        s.execute("sheet.get", &json!({"scope":"page","name":"PageWidth"})).unwrap();
        assert_eq!(s.undo.len(), history, "a read must not discard undo history");
        let before = s.document.clone();
        let failed = s.edit_document::<()>(|doc, _| {
            doc.pages[0].name = "not committed".into();
            Err(operation("deliberate failure"))
        });
        assert!(failed.is_err());
        assert!(Arc::ptr_eq(&s.document, &before));
        assert_eq!(s.undo.len(), history, "a failed command must restore history");
        assert_eq!(s.execute("edit.undo", &json!({})).unwrap()["changed"], true);
        assert_eq!(s.undo.len(), history - 1, "undo must move one snapshot, not trim others");
        assert_eq!(s.execute("edit.redo", &json!({})).unwrap()["changed"], true);
        assert_eq!(s.undo.len(), history);
    }
}
