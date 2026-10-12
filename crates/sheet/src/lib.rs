//! ShapeSheet syntax, units and bounded evaluation independent of the document model.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
mod eval;
pub mod functions;
pub mod language;
pub mod parser;
pub mod units;
mod value;
pub use eval::{CachedHooks, Context, HostHooks, evaluate, formula, is_hook};
pub use parser::{Expr, parse};
pub use value::{Error, Value};
