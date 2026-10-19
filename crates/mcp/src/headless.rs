//! In-process engine control backend, shared with future thin clients.
use crate::backend::Backend;
use misterdiagram_engine::Session;
use serde_json::Value;
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
    fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        self.session.execute(method, &params).map_err(|e| e.to_string())
    }
    fn has_ui(&self) -> bool {
        false
    }
    fn describe(&self) -> String {
        "headless strict engine".into()
    }
}
