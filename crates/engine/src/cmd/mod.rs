//! The headless document command registry.

use diagramcraft_model::ShapeId;
use diagramcraft_model::Xfrm;
use serde::Serialize;
use serde_json::Value;

use crate::{EngineError, Result, Session};

pub type Run = fn(&mut Session, &Value) -> Result<Value>;
pub type Enabled = fn(&Session) -> std::result::Result<(), String>;

pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    /// Where it lives: `["Home", "Font"]` (ribbon tab and group) or `["Arrange"]` (menu).
    pub menu: &'static [&'static str],
    pub shortcut: Option<&'static str>,
    pub params: &'static str,
    pub enabled: Enabled,
    pub run: Run,
    pub journal: bool,
    pub undoable: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct CommandInfo {
    pub id: &'static str,
    pub label: &'static str,
    pub menu: Vec<&'static str>,
    pub shortcut: Option<&'static str>,
    pub params: &'static str,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
}

impl CommandSpec {
    pub fn info(&self, s: &Session) -> CommandInfo {
        let e = (self.enabled)(s);
        CommandInfo {
            id: self.id,
            label: self.label,
            menu: self.menu.to_vec(),
            shortcut: self.shortcut,
            params: self.params,
            enabled: e.is_ok(),
            disabled_reason: e.err(),
        }
    }
}

pub fn always(_: &Session) -> std::result::Result<(), String> {
    Ok(())
}
#[macro_export]
macro_rules! cmd {
    ($id:literal, $label:literal, [$($m:literal),*], $sc:expr, $params:literal, $en:expr, $run:expr) => {
        $crate::cmd::CommandSpec { id: $id, label: $label, menu: &[$($m),*], shortcut: $sc, params: $params, enabled: $en, run: $run, journal: true, undoable: true }
    };
    (query $id:literal, $label:literal, [$($m:literal),*], $sc:expr, $params:literal, $en:expr, $run:expr) => {
        $crate::cmd::CommandSpec { id: $id, label: $label, menu: &[$($m),*], shortcut: $sc, params: $params, enabled: $en, run: $run, journal: false, undoable: false }
    };
    (noundo $id:literal, $label:literal, [$($m:literal),*], $sc:expr, $params:literal, $en:expr, $run:expr) => {
        $crate::cmd::CommandSpec { id: $id, label: $label, menu: &[$($m),*], shortcut: $sc, params: $params, enabled: $en, run: $run, journal: true, undoable: false }
    };
}
pub use cmd;

pub fn command_specs() -> &'static [CommandSpec] {
    &[
        cmd!(noundo "file.new", "New drawing", ["File"], None,
            r#"{"type":"object","properties":{"metric":{"type":"boolean"}},"additionalProperties":false}"#, always, crate::files::new),
        cmd!(noundo "file.open", "Open drawing", ["File"], None,
            r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":32768}},"required":["path"],"additionalProperties":false}"#, always, crate::files::open),
        cmd!(noundo "file.save", "Save drawing", ["File"], None,
            r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":32768}},"required":["path"],"additionalProperties":false}"#, always, crate::files::save),
        cmd!(noundo "file.export", "Export drawing", ["File"], None,
            r#"{"type":"object","properties":{"path":{"type":"string","minLength":1,"maxLength":32768},"page":{"type":"integer","minimum":1,"description":"One-based page ordinal"},"scale":{"type":"number","exclusiveMinimum":0,"maximum":100}},"required":["path"],"additionalProperties":false}"#, always, crate::files::export),
        cmd!(noundo "edit.undo", "Undo", ["Edit"], None,
            r#"{"type":"object","additionalProperties":false}"#, always, Session::undo),
        cmd!(noundo "edit.redo", "Redo", ["Edit"], None,
            r#"{"type":"object","additionalProperties":false}"#, always, Session::redo),
    ]
}

pub fn find_command(id: &str) -> Option<&'static CommandSpec> {
    command_specs().iter().find(|c| c.id == id)
}

// ---------- param helpers ----------

pub fn bad(cmd: &str, msg: impl Into<String>) -> EngineError {
    EngineError::BadParams { cmd: cmd.into(), msg: msg.into() }
}
pub fn f64_param(p: &Value, key: &str) -> Option<f64> {
    p.get(key).and_then(Value::as_f64).filter(|v| v.is_finite())
}
pub fn f64_or(p: &Value, key: &str, default: f64) -> f64 {
    f64_param(p, key).unwrap_or(default)
}
pub fn bool_param(p: &Value, key: &str) -> Option<bool> {
    p.get(key).and_then(Value::as_bool)
}
pub fn bool_or(p: &Value, key: &str, default: bool) -> bool {
    bool_param(p, key).unwrap_or(default)
}
pub fn str_param<'a>(p: &'a Value, key: &str) -> Option<&'a str> {
    p.get(key).and_then(Value::as_str)
}
pub fn usize_param(p: &Value, key: &str) -> Option<usize> {
    p.get(key).and_then(Value::as_u64).and_then(|v| usize::try_from(v).ok())
}
pub fn id_param(p: &Value, key: &str) -> Option<ShapeId> {
    p.get(key).and_then(Value::as_u64).and_then(|v| u32::try_from(v).ok()).map(ShapeId)
}
pub fn ids_param(p: &Value, key: &str) -> Option<Vec<ShapeId>> {
    p.get(key).and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_u64).filter_map(|v| u32::try_from(v).ok()).map(ShapeId).collect())
}
/// `[x, y, w, h]` in inches.
pub fn rect_param(p: &Value, key: &str) -> Option<Xfrm> {
    let a = p.get(key)?.as_array()?;
    let f = |i: usize| a.get(i).and_then(Value::as_f64).filter(|v| v.is_finite());
    Some(Xfrm::new(f(0)?, f(1)?, f(2)?.abs(), f(3)?.abs()))
}
pub fn with_param(p: &Value, key: &str, v: Value) -> Value {
    let mut m = p.as_object().cloned().unwrap_or_default();
    m.insert(key.to_string(), v);
    Value::Object(m)
}
