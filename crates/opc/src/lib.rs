//! Bounded Open Packaging Conventions container and XML.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
pub mod opc;
pub mod xml;
pub use opc::*;
#[derive(Debug, thiserror::Error)]
pub enum OpcError {
    #[error("invalid OPC package: {0}")]
    InvalidPackage(String),
    /// The bytes are not a ZIP container at all (an HTML page saved as .vsdx, a legacy binary
    /// file, an empty file); the message already names what they are.
    #[error("{0}")]
    NotPackage(String),
    #[error("invalid XML: {0}")]
    Xml(String),
    #[error("package write failed: {0}")]
    Write(String),
}
