//! Bounded Open Packaging Conventions container and XML.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
pub mod opc;
pub mod xml;
pub use opc::*;
#[derive(Debug, thiserror::Error)]
pub enum OpcError {
    #[error("invalid OPC package: {0}")]
    InvalidPackage(String),
    #[error("invalid XML: {0}")]
    Xml(String),
    #[error("package write failed: {0}")]
    Write(String),
}
