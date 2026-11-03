//! Implicit display units observed in field/brief66-2026-11-03/implicit-native.
use super::{Evaluator, Scope};
impl Evaluator<'_> {
    /// A missing saved U does not make a typed ShapeSheet cell dimensionless.
    /// Drawing lengths use internal inches even on metric and scaled pages.
    /// Explicit U is resolved by the caller before this fallback.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/format-function
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/width-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/height-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/pinx-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/piny-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/locpinx-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/locpiny-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/angle-cell-shape-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/txtwidth-cell-text-transform-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/size-cell-character-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/lineweight-cell-line-format-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/indleft-cell-paragraph-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/x-cell-geometry-section
    pub(super) fn implicit_unit(&self, scope: Scope, name: &str) -> Option<&'static str> {
        if name.len() > 128 {
            return None;
        }
        let shape = matches!(scope, Scope::PageShape { .. } | Scope::MasterShape { .. });
        let style = matches!(scope, Scope::Style(_));
        if !shape && !style {
            return None;
        }
        let name = name.to_ascii_lowercase();
        match name.as_str() {
            "width" if shape => return Some("IN"),
            "height" | "pinx" | "piny" | "locpinx" | "locpiny" | "txtwidth" if shape => return Some("IN"),
            "angle" if shape => return Some("DEG"),
            "lineweight" => return Some("PT"),
            _ => {}
        }
        let row_suffix = |tail: &str| {
            if tail.is_empty() {
                return true;
            }
            let tail = tail.strip_prefix('[').and_then(|value| value.strip_suffix(']')).unwrap_or(tail);
            tail.parse::<u32>().is_ok_and(|n| n > 0)
        };
        if let Some(tail) = name.strip_prefix("char.size")
            && row_suffix(tail)
        {
            return Some("PT");
        }
        if let Some(tail) = name.strip_prefix("para.indleft")
            && row_suffix(tail)
        {
            return Some("IN");
        }
        let (section, coordinate) = name.strip_prefix("geometry")?.split_once('.')?;
        let index = section.parse::<u32>().ok()?.checked_sub(1)?;
        let row = coordinate.strip_prefix('x')?.parse::<u32>().ok()?;
        match self.geometry_row_type(scope, index, row)? {
            "MoveTo" | "LineTo" | "ArcTo" | "EllipticalArcTo" | "Ellipse" | "InfiniteLine" | "PolylineTo" | "NURBSTo" | "SplineStart"
            | "SplineKnot" => Some("IN"),
            _ => None,
        }
    }
}
