//! Construction refusals carry resource counts separately from evaluated cell errors.
use super::Error;
use std::fmt;

/// Failure to construct a document evaluator, before any cell is evaluated.
/// Converting to a ShapeSheet error explicitly discards the construction detail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildError {
    error: Error,
    detail: Option<String>,
}
impl BuildError {
    pub(super) fn other(error: Error) -> Self {
        Self { error, detail: None }
    }
}
impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(detail) = &self.detail { f.write_str(detail) } else { write!(f, "evaluator: {}", self.error) }
    }
}
impl std::error::Error for BuildError {}
impl From<BuildError> for Error {
    fn from(error: BuildError) -> Self {
        error.error
    }
}

#[derive(Clone, Copy)]
pub(super) struct Bound {
    name: &'static str,
    cap: usize,
    unit: &'static str,
    stage: &'static str,
    scopes: Option<usize>,
}
impl Bound {
    pub(super) const fn new(name: &'static str, cap: usize, unit: &'static str, stage: &'static str) -> Self {
        Self { name, cap, unit, stage, scopes: None }
    }
    pub(super) fn scopes(mut self, scopes: usize) -> Self {
        self.scopes = Some(scopes);
        self
    }
    pub(super) fn exceeded(self, count: u128) -> BuildError {
        let scopes = self.scopes.map(|n| format!("{} scopes, ", grouped(n as u128))).unwrap_or_default();
        BuildError {
            error: Error::Limit,
            detail: Some(format!(
                "evaluator: {} of {} exceeded while {} ({}{} {})",
                self.name,
                grouped(self.cap as u128),
                self.stage,
                scopes,
                grouped(count),
                self.unit
            )),
        }
    }
    pub(super) fn check(self, count: usize) -> Result<(), BuildError> {
        if count > self.cap { Err(self.exceeded(count as u128)) } else { Ok(()) }
    }
    pub(super) fn next(self, count: usize) -> Result<(), BuildError> {
        if count >= self.cap { Err(self.exceeded(count as u128 + 1)) } else { Ok(()) }
    }
    /// Keep arithmetic overflow distinct from the caller's later aggregate cap check.
    pub(super) fn add(self, a: usize, b: usize) -> Result<usize, BuildError> {
        a.checked_add(b).ok_or_else(|| self.exceeded(a as u128 + b as u128))
    }
    pub(super) fn increment_id(self, id: u32) -> Result<u32, BuildError> {
        id.checked_add(1).ok_or_else(|| self.exceeded(u128::from(id) + 1))
    }
}
fn grouped(value: u128) -> String {
    let digits = value.to_string();
    let mut result = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(c);
    }
    result
}

pub(super) const SOURCE_STAGE: &str = "indexing source cells";
pub(super) const INHERIT_STAGE: &str = "building shape inheritance";
pub(super) const FONT_STAGE: &str = "registering document and theme faces";
pub(super) const ROW_INDEX: Bound = Bound::new("row index cap", u32::MAX as usize, "indices", SOURCE_STAGE);
pub(super) const SECTION_INDEX: Bound = Bound::new("section index cap", u32::MAX as usize, "indices", SOURCE_STAGE);

pub(super) fn section_error(error: Error, section: &super::Section) -> BuildError {
    if error == Error::Limit { SECTION_INDEX.exceeded(u128::from(section.index.unwrap_or(0)) + 1) } else { BuildError::other(error) }
}
pub(super) fn cell_name_error(error: Error, section: &super::Section, row: &super::Row, position: usize) -> BuildError {
    if error != Error::Limit {
        return BuildError::other(error);
    }
    if section.name == "Geometry" && section.index == Some(u32::MAX) {
        return section_error(error, section);
    }
    if u32::try_from(position).is_err() {
        return ROW_INDEX.exceeded(position as u128);
    }
    ROW_INDEX.exceeded(row.index.map_or(position as u128, u128::from) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluate::{Evaluator, Scope};
    use misterdiagram_model::{Cell, Document, Page, Row, Section, Sheet, Style};

    #[test]
    fn colours_faces_names_indices_and_depth_have_construction_detail() {
        let mut document = Document { colors: (0..=65536).map(|i| (i, "#010203".into())).collect(), ..Default::default() };
        assert_eq!(
            Evaluator::new(&document).err().unwrap().to_string(),
            "evaluator: colours cap of 65,536 exceeded while reading document colours (65,537 colours)"
        );
        document.colors.clear();
        document.face_names = (0..=65536).map(|i| (i, "Original face".into())).collect();
        assert_eq!(
            Evaluator::new(&document).err().unwrap().to_string(),
            "evaluator: face cap of 65,536 exceeded while registering document and theme faces (65,537 faces)"
        );
        document.face_names.clear();
        document.document_sheet.cells.push(Cell { name: "x".repeat(65537), ..Default::default() });
        assert_eq!(
            Evaluator::new(&document).err().unwrap().to_string(),
            "evaluator: cell name byte cap of 65,536 exceeded while indexing source cells (65,537 bytes)"
        );
        document.document_sheet.cells.clear();
        document.document_sheet.sections.push(Section { name: "Geometry".into(), index: Some(u32::MAX), ..Default::default() });
        assert_eq!(
            Evaluator::new(&document).err().unwrap().to_string(),
            "evaluator: section index cap of 4,294,967,295 exceeded while indexing source cells (4,294,967,296 indices)"
        );
        document.document_sheet.sections = vec![Section {
            name: "Scratch".into(),
            rows: vec![Row { index: Some(u32::MAX), cells: vec![Cell { name: "A".into(), ..Default::default() }], ..Default::default() }],
            ..Default::default()
        }];
        assert_eq!(
            Evaluator::new(&document).err().unwrap().to_string(),
            "evaluator: row index cap of 4,294,967,295 exceeded while indexing source cells (4,294,967,296 indices)"
        );
        document.document_sheet = Sheet::default();
        document.styles = (0..=130).map(|id| Style { id, line_style: Some(id + 1), ..Default::default() }).collect();
        let error = Evaluator::new(&document).err().unwrap().to_string();
        assert_eq!(error, "evaluator: inheritance depth cap of 128 exceeded while building shape inheritance (132 scopes, 129 levels)");
    }

    #[test]
    fn scope_limits_and_other_errors_remain_distinct_from_cell_values() {
        let document = Document::default();
        let mut evaluator = Evaluator::new(&document).unwrap();
        // Populate empty sheets, not millions of cells or large XML fixtures.
        for i in 0..super::super::MAX_SCOPES - 1 {
            evaluator.sheets.insert(Scope::Style(i as u32), &document.document_sheet);
        }
        let error = evaluator.add_sheet(Scope::Page(1), &document.document_sheet).unwrap_err().to_string();
        assert_eq!(error, "evaluator: scope cap of 100,000 exceeded while registering sheets (100,001 scopes)");
        let duplicate = Document { pages: vec![Page::default(), Page::default()], ..Default::default() };
        let error = Evaluator::new(&duplicate).err().unwrap();
        assert_eq!(error.to_string(), "evaluator: #VALUE!");
        assert_eq!(Error::from(error), Error::Value);
        assert_eq!(super::super::Value::Error(Error::Limit).text(), "#LIMIT!");
        assert_eq!(grouped(u128::from(u64::MAX) + 1), "18,446,744,073,709,551,616");
    }
}
