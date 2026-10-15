//! Diagramcraft route.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod static_glue;
pub use static_glue::{ConnectionPoint, End, add_connection, connect_dynamic, connect_static};

mod glue_host;
pub use glue_host::GlueHost;

mod jumps;
mod orthogonal;
mod straight;
pub use straight::reroute;
