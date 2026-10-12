//! Headless command registry and panic boundary scaffold.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
pub mod cmd;
pub mod guard;
pub use cmd::{command_specs, find_command};
use serde_json::Value;
pub type Result<T> = std::result::Result<T, EngineError>;
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("{cmd}: {msg}")]
    BadParams { cmd: String, msg: String },
    #[error("command {0} failed internally: {1}")]
    Internal(String, String),
    #[error("unknown command: {0}")]
    UnknownCommand(String),
}
#[derive(Clone, Default)]
pub struct Session {
    pub state: Value,
}
impl Session {
    pub fn new() -> Self {
        guard::install_panic_hook();
        Self::default()
    }
    pub fn commands(&self) -> Vec<cmd::CommandInfo> {
        command_specs().iter().map(|c| c.info(self)).collect()
    }
    pub fn execute(&mut self, id: &str, params: &Value) -> Result<Value> {
        self.guarded(id, |s| {
            let c = find_command(id).ok_or_else(|| EngineError::UnknownCommand(id.into()))?;
            (c.run)(s, params)
        })
    }
}
