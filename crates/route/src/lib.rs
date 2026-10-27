//! MisterDiagram route.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod static_glue;
pub use static_glue::{ConnectionPoint, End, add_connection, connect_dynamic, connect_static};

mod changes;
mod foreign_glue;
mod glue_host;
mod mixed_glue;
pub use glue_host::GlueHost;

mod fixed;
mod jumps;
mod lanes;
mod orthogonal;
mod scope;
mod straight;
mod stub;
pub use straight::reroute;
