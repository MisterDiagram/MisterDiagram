//! Retained scene storage, charged between bounded shape constructions.
use super::{Op, Paint};

/// A shared export budget includes pixels, paths and text allocations. Fonts are
/// shared immutable resources owned by the global font database.
pub struct SceneBudget {
    remaining: usize,
    operations: usize,
}
impl Default for SceneBudget {
    fn default() -> Self {
        Self::new(256 * 1024 * 1024)
    }
}
impl SceneBudget {
    pub fn new(bytes: usize) -> Self {
        Self { remaining: bytes, operations: 0 }
    }
    pub(super) fn charge(&mut self, bytes: usize) -> Result<(), String> {
        self.remaining = self.remaining.checked_sub(bytes).ok_or("export scene memory limit")?;
        Ok(())
    }
    pub(crate) fn charge_ops(&mut self, ops: &[Op]) -> Result<(), String> {
        self.operations = self.operations.saturating_add(ops.len());
        if self.operations > 200_000 {
            return Err("export operation limit".into());
        }
        // Vec growth can retain twice the occupied operation/path storage.
        self.charge(std::mem::size_of_val(ops).saturating_mul(2))?;
        for op in ops {
            match op {
                Op::Image { pixels, .. } => self.charge(pixels.as_raw().capacity())?,
                Op::Path { path, paint } => {
                    self.charge(std::mem::size_of_val(path.elements()).saturating_mul(2))?;
                    if let Paint::Pattern { gradient: Some(g), .. } = paint {
                        self.charge(vector_bytes(&g.stops))?;
                    }
                }
                Op::Clip { path, .. } | Op::Stroke { path, .. } => {
                    self.charge(std::mem::size_of_val(path.elements()).saturating_mul(2))?;
                    if let Op::Stroke { stroke, .. } = op {
                        self.charge(stroke.dash_pattern.capacity().saturating_mul(std::mem::size_of::<f64>()))?;
                    }
                }
                Op::Text { layout, .. } => {
                    self.charge(vector_bytes(&layout.runs))?;
                    self.charge(vector_bytes(&layout.decos))?;
                    self.charge(vector_bytes(&layout.lines))?;
                    self.charge(vector_bytes(&layout.para_lines))?;
                    self.charge(vector_bytes(&layout.boundaries))?;
                    for boundary in &layout.boundaries {
                        self.charge(vector_bytes(boundary))?;
                    }
                    for run in &layout.runs {
                        self.charge(vector_bytes(&run.glyphs))?;
                        self.charge(vector_bytes(&run.cells))?;
                        self.charge(run.text.capacity())?;
                    }
                    for line in &layout.lines {
                        self.charge(vector_bytes(&line.caret_x))?;
                        self.charge(vector_bytes(&line.levels))?;
                        self.charge(vector_bytes(&line.edges))?;
                        self.charge(vector_bytes(&line.boundary))?;
                    }
                }
                Op::Bounds(_) | Op::Pop => {}
            }
        }
        Ok(())
    }
}
fn vector_bytes<T>(value: &Vec<T>) -> usize {
    value.capacity().saturating_mul(std::mem::size_of::<T>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::{Options, page_budgeted};
    use diagramcraft_geom::Affine;
    use std::sync::Arc;

    #[test]
    fn image_storage_is_charged_across_pages_and_within_one_page() {
        let image = Op::Image { pixels: Arc::new(image::RgbaImage::new(16, 16)), transform: Affine::IDENTITY };
        let charge = 1024 + std::mem::size_of::<Op>() * 2;
        let mut budget = SceneBudget::new(charge * 2 - 1);
        assert!(budget.charge_ops(std::slice::from_ref(&image)).is_ok());
        assert!(budget.charge_ops(std::slice::from_ref(&image)).is_err());
        let mut budget = SceneBudget::new(charge * 2 - 1);
        assert!(budget.charge_ops(&[image.clone(), image]).is_err());
    }

    #[test]
    fn page_construction_charges_paths_and_text_before_retaining_more_shapes() {
        let mut doc = diagramcraft_model::skeleton::document(false);
        doc.pages[0].shapes.push(diagramcraft_model::skeleton::rectangle(1, 1., 1., 2., 1., "Budget witness").unwrap());
        let mut budget = SceneBudget::new(usize::MAX);
        page_budgeted(&doc, 0, Options::default(), &mut budget).unwrap();
        let used = usize::MAX - budget.remaining;
        assert!(used > 1000);
        let mut budget = SceneBudget::new(used * 2 - 1);
        page_budgeted(&doc, 0, Options::default(), &mut budget).unwrap();
        assert!(page_budgeted(&doc, 0, Options::default(), &mut budget).unwrap_err().contains("memory limit"));
        doc.pages[0].shapes.push(diagramcraft_model::skeleton::rectangle(2, 4., 1., 2., 1., "Budget witness").unwrap());
        assert!(page_budgeted(&doc, 0, Options::default(), &mut SceneBudget::new(used)).unwrap_err().contains("memory limit"));
    }
}
