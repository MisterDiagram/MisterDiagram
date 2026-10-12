//! Last-resort guard (craftrules `standards/never-crash.md`): a panic that escapes a command ,
//! any edit, import or export, from the UI, the CLI, MCP or the control channel; becomes an
//! error, and the document is kept as it was before the command (and saved for crash recovery
//! when a recovery directory is set). It is a safety net: commands still must not panic.

use std::panic::{AssertUnwindSafe, catch_unwind};

use serde_json::Value;

use crate::{EngineError, Result, Session};

/// Text of a panic payload (`panic!` with a literal or a formatted message).
pub fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unknown panic".into())
}

/// Log every panic (message and location) before it unwinds, then run the previous hook.
/// Installed once per process, when the first [`Session`] is made.
pub fn install_panic_hook() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let at = info.location().map(|l| format!(" at {}:{}", l.file(), l.line())).unwrap_or_default();
            log::error!("panic{at}: {}", panic_message(info.payload()));
            previous(info);
        }));
    });
}

impl Session {
    /// Run command `id` (`f`) inside the guard.
    pub(crate) fn guarded(&mut self, id: &str, f: impl FnOnce(&mut Session) -> Result<Value>) -> Result<Value> {
        let before = self.clone();
        match catch_unwind(AssertUnwindSafe(|| f(self))) {
            Ok(r) => r,
            Err(payload) => {
                *self = before;
                let msg = panic_message(payload.as_ref());
                log::error!("command `{id}` panicked: {msg}");
                Err(EngineError::Internal(id.to_string(), msg))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::Session;
    #[test]
    fn panic_restores_placeholder_state() {
        let mut s = Session::new();
        let err = s
            .guarded("test.panic", |s| {
                s.state = serde_json::json!(42);
                panic!("boom")
            })
            .unwrap_err();
        assert!(err.to_string().contains("boom"));
        assert!(s.state.is_null());
        assert!(s.execute("absent", &serde_json::Value::Null).is_err());
    }
}
