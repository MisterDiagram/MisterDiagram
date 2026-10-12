//! An in-process engine session that answers the engine-level control-channel methods itself.

use diagramcraft_engine::Session;
use serde_json::{Value, json};

use crate::backend::Backend;

/// Headless backend: a [`Session`] rendering slides on the CPU.
pub struct Headless {
    pub session: Session,
}

impl Default for Headless {
    fn default() -> Self {
        Self::new()
    }
}

impl Headless {
    pub fn new() -> Self {
        Self { session: Session::new() }
    }
}
impl Backend for Headless {
    fn call(&mut self, method: &str, p: Value) -> Result<Value, String> {
        match method {
            "engine.execute" => {
                let id = p.get("command").and_then(Value::as_str).ok_or("missing `command`")?;
                self.session.execute(id, &p.get("params").cloned().unwrap_or(json!({}))).map_err(|e| e.to_string())
            }
            "engine.commands" => serde_json::to_value(self.session.commands()).map_err(|e| e.to_string()),
            _ => Err(format!("not implemented in Phase 0: {method}")),
        }
    }
    fn has_ui(&self) -> bool {
        false
    }
    fn describe(&self) -> String {
        "headless Phase 0 scaffold".into()
    }
}
