//! ShapeSheet syntax, units and bounded evaluation independent of the document model.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
mod eval;
pub mod functions;
pub mod host;
pub mod language;
pub mod parser;
pub mod units;
mod value;
pub use eval::{CachedHooks, Context, HostHooks, evaluate, formula, is_hook};
pub use parser::{Expr, parse};
pub use value::{Error, Value};

/// Frozen P1 text-host family, including implemented functions formerly in hooks.txt.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/textwidth-function
pub const TEXT_METRIC_FUNCTIONS: &[&str] = &["TEXTWIDTH", "TEXTHEIGHT", "SHAPETEXT"];
pub fn is_text_metric(name: &str) -> bool {
    TEXT_METRIC_FUNCTIONS.iter().any(|n| n.eq_ignore_ascii_case(name))
}
