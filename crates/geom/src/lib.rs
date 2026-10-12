//! Geometry in inches: page space is y-up, origin bottom-left.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
pub use kurbo::{Affine, Vec2};
/// Length in page-space inches.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Inches(pub f64);
