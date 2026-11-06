//! Semantic text records accompany their unchanged legacy drawing operations.
//! https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-emf/dd585d0a-5d7c-4034-963a-1141af836972
use super::{Op, SceneBudget};
use misterdiagram_color::Rgba;
use misterdiagram_geom::{Affine, BezPath, Rect};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}
#[derive(Clone, Debug)]
pub struct MetaText {
    pub text: String,
    pub family: String,
    pub measured_family: String,
    pub weight: i32,
    pub italic: bool,
    pub size: f64,
    pub transform: Affine,
    pub anchor: TextAnchor,
    pub color: Rgba,
    /// Logical record rectangle and its mapping, separate from glyph placement.
    pub clip: Option<(Rect, Affine)>,
    /// Existing device-context clips are copied only for editable SVG serialization.
    pub dc_clips: Vec<(BezPath, bool)>,
    /// Following legacy Clip/Path/Pop operations replaced only by editable SVG.
    pub outline_ops: usize,
    pub outlines_only: bool,
    pub shape: Option<(u32, u32)>,
}
impl MetaText {
    pub(crate) fn charge(&self, budget: &mut SceneBudget) -> Result<(), String> {
        budget.charge_bytes(std::mem::size_of::<Self>())?;
        for s in [&self.text, &self.family, &self.measured_family] {
            budget.charge_bytes(s.capacity())?;
        }
        budget.charge_bytes(self.dc_clips.capacity().saturating_mul(std::mem::size_of::<(BezPath, bool)>()))?;
        for (p, _) in &self.dc_clips {
            budget.charge_bytes(std::mem::size_of_val(p.elements()).saturating_mul(2))?;
        }
        Ok(())
    }
    pub(crate) fn validate(&self, following: &[Op]) -> Result<(), String> {
        if self.text.len().saturating_add(self.family.len()).saturating_add(self.measured_family.len()) > 16 * 1024 * 1024
            || !self.size.is_finite()
            || !(0. ..=5000.).contains(&self.size)
            || self.transform.as_coeffs().iter().any(|v| !v.is_finite() || v.abs() > 1e6)
            || self.dc_clips.len() > 128
        {
            return Err("scene metafile text limit".into());
        }
        if let Some((r, m)) = self.clip {
            let b = m.transform_rect_bbox(r);
            if m.as_coeffs().iter().any(|v| !v.is_finite() || v.abs() > 1e6)
                || [r.x0, r.y0, r.x1, r.y1, b.x0, b.y0, b.x1, b.y1].iter().any(|v| !v.is_finite() || v.abs() > 1e6)
            {
                return Err("scene metafile text clip limit".into());
            }
        }
        let mut elements = 0usize;
        for (p, _) in &self.dc_clips {
            elements = elements.saturating_add(p.elements().len());
            if elements > 4_000_000
                || p.elements().iter().any(|e| match e {
                    kurbo::PathEl::MoveTo(p) | kurbo::PathEl::LineTo(p) => !point(*p),
                    kurbo::PathEl::QuadTo(a, b) => !point(*a) || !point(*b),
                    kurbo::PathEl::CurveTo(a, b, c) => !point(*a) || !point(*b) || !point(*c),
                    kurbo::PathEl::ClosePath => false,
                })
            {
                return Err("scene metafile text clip limit".into());
            }
        }
        let span = following.get(..self.outline_ops).ok_or("scene metafile text span limit")?;
        let mut depth = 0usize;
        for op in span {
            match op {
                Op::Clip { .. } => {
                    depth = depth.saturating_add(1);
                    if depth > 128 {
                        return Err("scene metafile text clip limit".into());
                    }
                }
                Op::Pop => depth = depth.checked_sub(1).ok_or("scene metafile text span limit")?,
                Op::Path { paint: super::Paint::Solid(_), .. } => {}
                _ => return Err("scene metafile text span limit".into()),
            }
        }
        if depth != 0 {
            return Err("scene metafile text span limit".into());
        }
        Ok(())
    }
    pub(crate) fn placed(mut self, m: Affine, shape: Option<(u32, u32)>) -> Self {
        self.transform = m * self.transform;
        if let Some((_, transform)) = &mut self.clip {
            *transform = m * *transform;
        }
        for (p, _) in &mut self.dc_clips {
            *p = m * &*p;
        }
        self.shape = shape;
        self
    }
}
fn point(p: kurbo::Point) -> bool {
    p.x.is_finite() && p.y.is_finite() && p.x.abs() <= 1e6 && p.y.abs() <= 1e6
}
