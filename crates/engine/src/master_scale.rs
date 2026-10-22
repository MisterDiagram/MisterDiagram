//! Measured/unmeasured master drops, witnessed in Brief 31b.
use crate::{Result, operation};
use misterdiagram_vsdx::evaluate::{Evaluator, Scope};
use serde_json::{Value, json};

#[derive(Clone, Copy)]
pub(crate) struct DrawingScale {
    page: f64,
    drawing: f64,
    kind: f64,
}
impl DrawingScale {
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/drawingscale-cell-page-properties-section
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/drawingscaletype-cell-page-properties-section
    pub(crate) fn read(e: &mut Evaluator<'_>, scope: Scope) -> Result<Self> {
        // An omitted master PageSheet has no drawing-scale declaration. Preserve
        // the unscaled 1:1 drop instead of using unrelated style-cell zeroes.
        let read = |e: &mut Evaluator<'_>, name: &str, default: f64| -> Result<f64> {
            if e.sheet(scope).is_none_or(|sheet| !sheet.cells.iter().any(|cell| cell.name.eq_ignore_ascii_case(name))) {
                Ok(default)
            } else {
                e.evaluate(scope, name).numeric().map_err(operation)
            }
        };
        let page = read(e, "PageScale", 1.)?;
        let drawing = read(e, "DrawingScale", 1.)?;
        let kind = read(e, "DrawingScaleType", 0.)?;
        if !page.is_finite()
            || !drawing.is_finite()
            || page <= 0.
            || drawing <= 0.
            || !kind.is_finite()
            || !(0. ..=5.).contains(&kind)
            || kind.fract() != 0.
        {
            return Err(operation("invalid master or page drawing scale"));
        }
        Ok(Self { page, drawing, kind })
    }
    /// The reference describes measured/unmeasured drops; the strict eightfold
    /// cutoff is a black-box observation, not stated by the reference.
    /// https://learn.microsoft.com/en-us/office/client-developer/visio/droponpagescale-cell-miscellaneous-section
    pub(crate) fn factor(self, destination: Self) -> Result<f64> {
        // Like scale types retain drawing size, regardless of numeric ratios.
        if (self.kind == 0.) == (destination.kind == 0.) {
            return Ok(1.);
        }
        let factor = (self.page / self.drawing) / (destination.page / destination.drawing);
        if !factor.is_finite() || factor <= 0. {
            return Err(operation("master drop scale overflow or underflow"));
        }
        Ok(if (0.125..=8.).contains(&factor) { 1. } else { factor })
    }
    pub(crate) fn json(self) -> Value {
        json!({"page_scale":self.page,"drawing_scale":self.drawing,"drawing_scale_type":self.kind})
    }
}
