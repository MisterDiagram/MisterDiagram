//! Complete bounded source-cell comparison, including cells absent from COM traversal.
use crate::evaluate::{Evaluator, Scope};
use diagramcraft_model::{Cell, Document, Row, Section, Shape, Sheet};
use diagramcraft_sheet::Value;
use std::collections::BTreeMap;
pub const FLOOR: f64 = 99.;
const MAX_BYTES: usize = 64 * 1024 * 1024;
const MAX_ROWS: usize = 2_000_000;

#[derive(Debug, Default)]
pub struct Report {
    pub matched: u64,
    pub total: u64,
    pub failures: BTreeMap<String, u64>,
}
impl Report {
    pub fn percent(&self) -> f64 {
        if self.total == 0 { 0. } else { 100. * self.matched as f64 / self.total as f64 }
    }
    pub fn passed(&self) -> bool {
        self.total > 0 && self.percent() >= FLOOR
    }
    pub fn merge(&mut self, other: Self) {
        self.matched = self.matched.saturating_add(other.matched);
        self.total = self.total.saturating_add(other.total);
        for (name, count) in other.failures {
            let name = if self.failures.len() >= 4096 && !self.failures.contains_key(&name) { "<additional names>".into() } else { name };
            let entry = self.failures.entry(name).or_default();
            *entry = entry.saturating_add(count);
        }
    }
    fn failed(&mut self, name: &str) {
        let name = if self.failures.len() >= 4096 && !self.failures.contains_key(name) { "<additional names>" } else { name };
        *self.failures.entry(name.into()).or_default() += 1;
    }
}

struct Entry<'a> {
    name: String,
    cell: &'a Cell,
    scope: Scope,
    active: bool,
    ambiguous: bool,
    section: Option<&'a Section>,
    row: Option<&'a Row>,
}
#[derive(Default)]
struct Index<'a> {
    rows: BTreeMap<Vec<usize>, Entry<'a>>,
    bytes: usize,
}
impl<'a> Index<'a> {
    fn cell(
        &mut self,
        key: Vec<usize>,
        name: String,
        cell: &'a Cell,
        scope: Scope,
        active: bool,
        metadata: (Option<&'a Section>, Option<&'a Row>),
    ) -> Result<(), String> {
        self.bytes = self.bytes.checked_add(name.len().saturating_add(key.len().saturating_mul(8))).ok_or("roundtrip index overflow")?;
        if self.bytes > MAX_BYTES || self.rows.len() >= MAX_ROWS || name.len() > 65536 {
            return Err("roundtrip index limit".into());
        }
        self.rows.insert(key, Entry { name, cell, scope, active, ambiguous: false, section: metadata.0, row: metadata.1 });
        Ok(())
    }
    fn sheet(&mut self, key: &[usize], sheet: &'a Sheet, scope: Scope, active: bool) -> Result<(), String> {
        for (i, c) in sheet.cells.iter().enumerate() {
            let mut k = key.to_vec();
            k.extend([0, i]);
            self.cell(k, c.name.clone(), c, scope, active, (None, None))?;
        }
        for (i, s) in sheet.sections.iter().enumerate() {
            for (j, c) in s.cells.iter().enumerate() {
                let mut k = key.to_vec();
                k.extend([1, i, 0, j]);
                let prefix = crate::evaluate::section_prefix(s).map_err(|e| e.to_string())?;
                self.cell(k, format!("{prefix}.{}", c.name), c, scope, active && !s.deleted, (Some(s), None))?;
            }
            for (j, r) in s.rows.iter().enumerate() {
                for (n, c) in r.cells.iter().enumerate() {
                    let mut k = key.to_vec();
                    k.extend([1, i, 1, j, n]);
                    let name = crate::evaluate::cell_name(s, r, j, c).map_err(|e| e.to_string())?;
                    self.cell(k, name, c, scope, active && !s.deleted && !r.deleted, (Some(s), Some(r)))?;
                }
            }
        }
        Ok(())
    }
    fn shapes(&mut self, key: &[usize], shapes: &'a [Shape], parent: Scope, active: bool) -> Result<(), String> {
        for (i, s) in shapes.iter().enumerate() {
            let mut key = key.to_vec();
            key.extend([2, i]);
            let scope = match parent {
                Scope::Page(p) => Scope::PageShape { page: p, shape: s.id },
                Scope::Master(m) => Scope::MasterShape { master: m, shape: s.id },
                _ => return Err("shape scope".into()),
            };
            let active = active && s.sheet.attributes.get("Del").is_none_or(|d| d != "1");
            self.sheet(&key, &s.sheet, scope, active)?;
            self.shapes(&key, &s.shapes, parent, active)?;
        }
        Ok(())
    }
    fn document(d: &'a Document) -> Result<Self, String> {
        // Validate before recursion, allocation, or evaluator indexing. Numeric positional
        // identities never clone a group name for each of its descendants.
        crate::write::validate_document(d).map_err(|e| e.to_string())?;
        let mut out = Self::default();
        out.sheet(&[0], &d.document_sheet, Scope::Document, true)?;
        for (i, s) in d.styles.iter().enumerate() {
            out.sheet(&[1, i], &s.sheet, Scope::Style(s.id), true)?;
        }
        for (i, p) in d.pages.iter().enumerate() {
            out.sheet(&[2, i], &p.sheet, Scope::Page(p.id), true)?;
            out.shapes(&[2, i], &p.shapes, Scope::Page(p.id), true)?;
        }
        for (i, m) in d.masters.iter().enumerate() {
            out.sheet(&[3, i], &m.sheet, Scope::Master(m.id), true)?;
            out.shapes(&[3, i], &m.shapes, Scope::Master(m.id), true)?;
        }
        // Only duplicate local canonical names are ambiguous. An ordinary Inh
        // source legitimately resolves to a different provider cell.
        let mut counts = BTreeMap::new();
        for e in out.rows.values().filter(|e| e.active) {
            *counts.entry((e.scope, e.name.to_ascii_lowercase())).or_insert(0usize) += 1;
        }
        for e in out.rows.values_mut() {
            e.ambiguous = counts.get(&(e.scope, e.name.to_ascii_lowercase())).is_some_and(|n| *n > 1);
        }
        Ok(out)
    }
}
fn value_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a.is_finite() && b.is_finite() && (a - b).abs() <= 1e-6,
        (Value::Color { rgb: a, .. }, Value::Color { rgb: b, .. }) => a == b,
        _ => a == b,
    }
}
fn equal(a: &Entry<'_>, b: &Entry<'_>, ea: &mut Evaluator<'_>, eb: &mut Evaluator<'_>) -> bool {
    let section = |s: Option<&Section>| s.map(|s| (s.name.clone(), s.index, s.deleted));
    let row = |r: Option<&Row>| r.map(|r| (r.name.clone(), r.index, r.row_type.clone(), r.deleted));
    if a.scope != b.scope
        || section(a.section) != section(b.section)
        || row(a.row) != row(b.row)
        || a.name != b.name
        || a.cell.units != b.cell.units
        || a.active != b.active
    {
        return false;
    }
    if !a.active || a.ambiguous || b.ambiguous {
        // Masked duplicate canonical names must compare their own source data.
        return a.cell == b.cell;
    }
    let va = ea.evaluate_snapshot(a.scope, &a.name);
    let vb = eb.evaluate_snapshot(b.scope, &b.name);
    let fa = ea.effective_formula(a.scope, &a.name);
    let fb = eb.effective_formula(b.scope, &b.name);
    fa == fb && value_equal(&va, &vb) && (!matches!(va, Value::Error(_)) || a.cell == b.cell)
}
/// Compare all local source cells, without COM omissions, hook or volatile exclusions.
/// The denominator is the union of positional source identities. Deleted cells compare
/// literally; active cells compare effective formulas, units and typed evaluated values.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
pub fn compare(
    before: &Document,
    after: &Document,
    theme_before: Option<&dyn diagramcraft_sheet::host::ThemeHost>,
    theme_after: Option<&dyn diagramcraft_sheet::host::ThemeHost>,
    text: Option<&dyn diagramcraft_model::text::TextHost>,
) -> Result<Report, String> {
    let mut a = Index::document(before)?;
    let b = Index::document(after)?;
    let mut ea = Evaluator::new(before).map_err(|e| e.to_string())?;
    let mut eb = Evaluator::new(after).map_err(|e| e.to_string())?;
    if let Some(h) = theme_before {
        ea = ea.with_theme_host(h);
    }
    if let Some(h) = theme_after {
        eb = eb.with_theme_host(h);
    }
    if let Some(h) = text {
        ea = ea.with_text_host(h);
        eb = eb.with_text_host(h);
    }
    let mut report = Report { total: a.rows.len() as u64, ..Report::default() };
    for (key, b) in b.rows {
        if let Some(a) = a.rows.remove(&key) {
            if equal(&a, &b, &mut ea, &mut eb) {
                report.matched += 1;
            } else {
                report.failed(&b.name);
            }
        } else {
            report.total += 1;
            report.failed(&b.name);
        }
    }
    for a in a.rows.values() {
        report.failed(&a.name);
    }
    Ok(report)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_comparison_includes_master_pages_unknown_cells_and_deleted_rows() {
        let mut a = diagramcraft_model::skeleton::document(false);
        a.masters[0].sheet.cells.push(Cell { name: "UnknownPrivateCell".into(), value: Some("1".into()), ..Cell::default() });
        let mut b = a.clone();
        b.masters[0].sheet.cells.last_mut().unwrap().value = Some("2".into());
        let r = compare(&a, &b, None, None, None).unwrap();
        assert_eq!(r.failures["UnknownPrivateCell"], 1);
        assert!(compare(&a, &a, None, None, None).unwrap().passed());
        let mut b = a.clone();
        b.masters[0].id += 100;
        assert!(compare(&a, &b, None, None, None).unwrap().matched < compare(&a, &a, None, None, None).unwrap().matched);
    }
    #[test]
    fn inherited_provider_is_not_a_duplicate_local_cell() {
        let mut a = diagramcraft_model::skeleton::document(false);
        a.pages[0].shapes.push(Shape {
            id: 1,
            master: Some(a.masters[0].id),
            sheet: Sheet {
                cells: vec![Cell { name: "Width".into(), formula: Some("Inh".into()), value: Some("999".into()), ..Cell::default() }],
                ..Sheet::default()
            },
            ..Shape::default()
        });
        let written = crate::write(&a).unwrap();
        let b = crate::import(&written.bytes).unwrap();
        let r = compare(&a, &b, None, None, None).unwrap();
        assert!(!r.failures.contains_key("Width"), "{:?}", r.failures);
    }
    #[test]
    fn duplicate_cell_names_cannot_hide_an_earlier_change() {
        let mut a = diagramcraft_model::skeleton::document(false);
        for value in ["1", "2"] {
            a.document_sheet.cells.push(Cell { name: "Width".into(), value: Some(value.into()), ..Cell::default() });
        }
        let mut b = a.clone();
        let n = b.document_sheet.cells.len();
        b.document_sheet.cells[n - 2].value = Some("999".into());
        let r = compare(&a, &b, None, None, None).unwrap();
        assert_eq!(r.failures["Width"], 1);
    }
    #[test]
    fn hostile_group_names_are_not_repeated_in_descendant_identities() {
        let mut d = diagramcraft_model::skeleton::document(false);
        let mut s = Shape { name_u: "x".repeat(64000), ..Shape::default() };
        s.shapes = (1..90001).map(|id| Shape { id, ..Shape::default() }).collect();
        d.pages[0].shapes.push(s);
        let i = Index::document(&d).unwrap();
        assert!(i.bytes < 1024 * 1024);
        d.document_sheet.cells[0].value = Some("x".repeat(17 * 1024 * 1024));
        assert!(Index::document(&d).is_err());
    }
}
