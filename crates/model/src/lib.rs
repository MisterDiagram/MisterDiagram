//! Document model placeholders for the Phase 0 command scaffold.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShapeId(pub u32);
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Xfrm {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
impl Xfrm {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
}
