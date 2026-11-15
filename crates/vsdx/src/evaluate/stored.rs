//! Restricted recovery for errors already present in a transaction's predecessor.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/cell-elementvisio-xml
use super::{Cell, Error, Evaluator, Key, Scope, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Clone)]
struct PriorError {
    formula: Option<String>,
    local_formula: Option<String>,
    saved: Option<Cell>,
}
/// A bounded, immutable classification made with strict predecessor evaluation.
/// Only entries with a usable saved value authorize recovery. Other old errors
/// retain provenance so an eligible ancestor cannot hide a newly failing child.
#[derive(Default)]
pub struct StoredErrors {
    cells: BTreeMap<Key, PriorError>,
    saved_count: usize,
    present: BTreeMap<Key, Cell>,
    scopes: BTreeSet<Scope>,
    virtual_cells: BTreeSet<Key>,
    page_names: BTreeMap<String, Scope>,
    master_names: BTreeMap<String, Scope>,
    shape_names: BTreeMap<(Scope, String), Scope>,
}
impl StoredErrors {
    pub fn is_empty(&self) -> bool {
        self.saved_count == 0
    }
    pub(super) fn reference_existed(&self, mut scope: Scope, reference: &str) -> bool {
        if reference.len() > 65536 {
            return false;
        }
        let Ok(segments) = super::split_reference(reference) else { return false };
        let Some((name, prefixes)) = segments.split_last() else { return false };
        for prefix in prefixes {
            let prefix = super::unquote(prefix);
            let target = if prefix.eq_ignore_ascii_case("TheDoc") {
                Some(Scope::Document)
            } else if prefix.eq_ignore_ascii_case("ThePage") {
                super::page_scope(scope)
            } else if let Some(name) = super::bracket_name(&prefix, "Pages") {
                self.page_names.get(&name.to_ascii_lowercase()).copied()
            } else if let Some(name) = super::bracket_name(&prefix, "Masters") {
                self.master_names.get(&name.to_ascii_lowercase()).copied()
            } else if prefix.eq_ignore_ascii_case("Styles") {
                Some(Scope::Style(u32::MAX))
            } else {
                super::page_scope(scope)
                    .or_else(|| matches!(scope, Scope::Style(_)).then_some(Scope::Style(u32::MAX)))
                    .and_then(|owner| self.shape_names.get(&(owner, prefix.to_ascii_lowercase())).copied())
            };
            let Some(target) = target else { return false };
            scope = target;
        }
        let name = super::unquote(name);
        super::indexed_key(&self.present, scope, &name).is_some()
            || self.virtual_cells.contains(&Key { scope, name: name.to_ascii_lowercase() })
            || (self.scopes.contains(&scope) && (super::defaults::cell(scope, &name).is_some() || name.eq_ignore_ascii_case("TheText")))
    }
}
impl Evaluator<'_> {
    /// Classify the predecessor without enabling recovery or retaining SETF effects.
    /// Call only after strict transaction publication fails.
    pub fn stored_errors(&mut self) -> Result<StoredErrors, Error> {
        let mut result = StoredErrors::default();
        let mut count = 0usize;
        let mut bytes = 0usize;
        for (names, scope, page) in self
            .document
            .pages
            .iter()
            .map(|p| ((&p.name, &p.name_u), Scope::Page(p.id), true))
            .chain(self.document.masters.iter().map(|m| ((&m.name, &m.name_u), Scope::Master(m.id), false)))
        {
            for name in [names.0, names.1] {
                bytes = bytes.saturating_add(name.len());
                if bytes > 64 * 1024 * 1024 {
                    return Err(Error::Limit);
                }
                let map = if page { &mut result.page_names } else { &mut result.master_names };
                map.entry(name.to_ascii_lowercase()).or_insert(scope);
            }
        }
        for (scope, id, name, name_u) in self
            .shapes
            .iter()
            .map(|(scope, shape)| (*scope, shape.id, &shape.name, &shape.name_u))
            .chain(self.document.styles.iter().map(|s| (Scope::Style(s.id), s.id, &s.name, &s.name_u)))
        {
            let owner = super::page_scope(scope).unwrap_or(Scope::Style(u32::MAX));
            for name in [format!("Sheet.{id}"), name.clone(), name_u.clone()] {
                bytes = bytes.saturating_add(name.len());
                if bytes > 64 * 1024 * 1024 {
                    return Err(Error::Limit);
                }
                result.shape_names.entry((owner, name.to_ascii_lowercase())).or_insert(scope);
            }
        }
        for scope in self.scopes().collect::<Vec<_>>() {
            result.scopes.insert(scope);
            for index in self.geometry_indices(scope) {
                let name = format!("geometry{}.path", index.saturating_add(1));
                bytes = bytes.saturating_add(name.len());
                if bytes > 64 * 1024 * 1024 || result.virtual_cells.len() >= 2_000_000 {
                    return Err(Error::Limit);
                }
                result.virtual_cells.insert(Key { scope, name });
            }
            for named in self.named_cells(scope).to_vec() {
                count = count.saturating_add(1);
                if count > 2_000_000 {
                    return Err(Error::Limit);
                }
                bytes = bytes.saturating_add(named.name.len()).saturating_add(named.cell.name.len());
                if bytes > 64 * 1024 * 1024 {
                    return Err(Error::Limit);
                }
                result
                    .present
                    .insert(Key { scope, name: named.name.to_ascii_lowercase() }, Cell { name: named.cell.name.clone(), ..Default::default() });
                let value = self.evaluate_snapshot(scope, &named.name);
                if value.error().is_none() {
                    continue;
                }
                let Some(key) = self.canonical_key(scope, &named.name) else { continue };
                let Some(cell) = self.cell(scope, &key.name) else { continue };
                let source = self.local_inh_cell(scope, &key.name).unwrap_or(cell);
                let formula = self.effective_formula(scope, &key.name);
                let local_formula = source.formula.clone();
                let saved = self.saved_value(scope, &key.name).map(|_| Cell {
                    name: source.name.clone(),
                    value: source.value.clone(),
                    formula: source.formula.clone(),
                    error: source.error.clone(),
                    units: source.units.clone(),
                    ..Default::default()
                });
                bytes = bytes
                    .saturating_add(key.name.len())
                    .saturating_add(formula.as_ref().map_or(0, String::len))
                    .saturating_add(local_formula.as_ref().map_or(0, String::len))
                    .saturating_add(saved.as_ref().map_or(0, |c| c.value.as_ref().map_or(0, String::len)));
                if bytes > 64 * 1024 * 1024 {
                    return Err(Error::Limit);
                }
                result.saved_count += usize::from(saved.is_some());
                result.cells.insert(key, PriorError { formula, local_formula, saved });
            }
        }
        Ok(result)
    }
    pub(super) fn mark_missing_reference(&mut self, scope: Scope, reference: &str) {
        if self.stored_errors.as_ref().is_some_and(|old| old.reference_existed(scope, reference)) {
            self.stored_new_error = true;
        }
    }
    /// Propagated lookup failures retain predecessor provenance. Existence queries
    /// and speculative lookups deliberately keep using reference_target directly.
    pub(super) fn checked_reference_target(&mut self, scope: Scope, reference: &str) -> Result<(Scope, String), Error> {
        let result = self.reference_target(scope, reference);
        if result.is_err() {
            self.mark_missing_reference(scope, reference);
        }
        result
    }
    /// Use only unchanged predecessor errors, never unrestricted read recovery.
    pub fn with_stored_errors(mut self, errors: Option<Arc<StoredErrors>>) -> Self {
        if errors.is_some() {
            self.values.clear();
        }
        self.stored_errors = errors;
        self
    }
    fn prior_error(&self, key: &Key) -> Option<&PriorError> {
        let prior = self.stored_errors.as_ref()?.cells.get(key)?;
        let cell = self.cell(key.scope, &key.name)?;
        let source = self.local_inh_cell(key.scope, &key.name).unwrap_or(cell);
        let provider = self.formula_origin(key.scope, &key.name).unwrap_or(key.scope);
        if self.overrides.contains_key(key) || self.overrides.contains_key(&Key { scope: provider, name: key.name.clone() }) {
            return None;
        }
        (self.effective_formula(key.scope, &key.name) == prior.formula && source.formula == prior.local_formula).then_some(prior)
    }
    /// The original serialized tuple, only after this cell actually needed recovery.
    pub(crate) fn stored_source(&self, scope: Scope, name: &str) -> Option<&Cell> {
        let key = self.canonical_key(scope, name)?;
        self.stored_used.contains(&key).then(|| self.prior_error(&key)?.saved.as_ref()).flatten()
    }
    pub(super) fn evaluate_stored(&mut self, scope: Scope, name: &str) -> Value {
        let parent_failure = self.stored_new_error;
        self.stored_new_error = false;
        let value = self.evaluate_inner_impl(scope, name);
        self.recover_stored_result(scope, name, value, parent_failure)
    }
    pub(super) fn recover_stored_result(&mut self, scope: Scope, name: &str, mut value: Value, parent_failure: bool) -> Value {
        let key = self.canonical_key(scope, name);
        if let Some(key) = &key {
            if value.error().is_some() {
                self.stored_new_error |= self.stored_blocked.contains(key) || self.prior_error(key).is_none();
                if !self.stored_new_error
                    && let Some(source) = self.prior_error(key).and_then(|p| p.saved.clone())
                {
                    let error = value.text();
                    let mut cache = source;
                    cache.formula = None;
                    cache.error = None;
                    value = self.cached_named_cell(scope, &key.name, &cache);
                    if value.error().is_none() && self.stored_used.insert(key.clone()) {
                        self.fallback_warning(scope, name, &error, "used the value saved in the file");
                    }
                    // The inner evaluator may have cached the strict error.
                    self.values.remove(key);
                }
            }
            if value.error().is_some() && self.stored_new_error {
                self.stored_blocked.insert(key.clone());
            } else {
                self.stored_blocked.remove(key);
            }
        }
        if key.is_none() && value.error().is_some() && self.stored_errors.as_ref().is_some_and(|old| old.reference_existed(scope, name)) {
            self.stored_new_error = true;
        }
        self.stored_new_error = parent_failure || (value.error().is_some() && self.stored_new_error);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use misterdiagram_model::Document;
    fn document(cells: &[(&str, Option<&str>, &str)]) -> Document {
        let mut d = Document::default();
        d.document_sheet.cells = cells
            .iter()
            .map(|(name, value, formula)| Cell {
                name: (*name).into(),
                value: value.map(str::to_owned),
                formula: Some((*formula).into()),
                ..Default::default()
            })
            .collect();
        d
    }
    fn classify(d: &Document) -> Arc<StoredErrors> {
        Arc::new(Evaluator::new(d).unwrap().stored_errors().unwrap())
    }
    #[test]
    fn stored_errors_ancestor_cannot_hide_a_new_failing_dependency() {
        let d = document(&[("A", Some("7"), "B+C"), ("B", Some("2"), "1/0"), ("C", Some("3"), "1")]);
        let stored = classify(&d);
        let mut after = d.clone();
        after.document_sheet.cells[2].formula = Some("1/0".into());
        let mut e = Evaluator::new(&after).unwrap().with_stored_errors(Some(stored));
        assert_eq!(e.evaluate(Scope::Document, "A"), Value::Error(Error::Div0));
        assert_eq!(e.evaluate(Scope::Document, "A"), Value::Error(Error::Div0));
    }
    #[test]
    fn stored_errors_saved_ancestor_can_cover_an_old_uncached_error() {
        let d = document(&[("A", Some("7"), "B+1"), ("B", None, "1/0")]);
        let stored = classify(&d);
        let mut e = Evaluator::new(&d).unwrap().with_stored_errors(Some(stored));
        assert_eq!(e.evaluate(Scope::Document, "A"), Value::Number(7.));
        assert_eq!(e.evaluate(Scope::Document, "B"), Value::Error(Error::Div0));
    }
    #[test]
    fn stored_errors_healed_formula_recalculates_and_defaults_do_not_qualify() {
        let d = document(&[("A", Some("7.000"), "1/B"), ("B", Some("0"), "0")]);
        let stored = classify(&d);
        let mut after = d.clone();
        after.document_sheet.cells[1].formula = Some("2".into());
        let mut e = Evaluator::new(&after).unwrap().with_stored_errors(Some(stored));
        assert_eq!(e.evaluate(Scope::Document, "A"), Value::Number(0.5));
        assert!(e.stored_source(Scope::Document, "A").is_none());
        let d = document(&[("LineWeight", None, "1/0")]);
        assert!(classify(&d).is_empty());
        assert!(Evaluator::new(&d).unwrap().with_cached_fallback().evaluate(Scope::Document, "LineWeight").error().is_none());
    }
    #[test]
    fn stored_errors_missing_reference_uses_only_a_real_saved_cell() {
        let d = document(&[("A", Some("4"), "Missing+1"), ("B", None, "Missing+1")]);
        let stored = classify(&d);
        let mut e = Evaluator::new(&d).unwrap().with_stored_errors(Some(stored));
        assert_eq!(e.evaluate(Scope::Document, "A"), Value::Number(4.));
        assert_eq!(e.evaluate(Scope::Document, "B"), Value::Error(Error::Ref));
    }
    #[test]
    fn stored_errors_inherited_formula_and_local_inh_must_both_stay_unchanged() {
        use misterdiagram_model::{Master, Page, Shape};
        let mut d = Document::default();
        let mut master = Shape { id: 1, ..Default::default() };
        master.sheet.cells.push(Cell { name: "Width".into(), formula: Some("1/0".into()), value: Some("4".into()), ..Default::default() });
        d.masters.push(Master { id: 5, shapes: vec![master].into(), ..Default::default() });
        let mut shape = Shape { id: 9, master: Some(5), master_shape: Some(1), ..Default::default() };
        shape.sheet.cells.push(Cell { name: "Width".into(), formula: Some("Inh".into()), value: Some("6".into()), ..Default::default() });
        d.pages.push(Page { id: 7, shapes: vec![shape].into(), ..Default::default() });
        let scope = Scope::PageShape { page: 7, shape: 9 };
        let stored = classify(&d);
        assert_eq!(Evaluator::new(&d).unwrap().with_stored_errors(Some(stored.clone())).evaluate(scope, "Width"), Value::Number(6.));
        let mut after = d.clone();
        after.masters[0].shapes[0].sheet.cells[0].formula = Some("2/0".into());
        assert_eq!(Evaluator::new(&after).unwrap().with_stored_errors(Some(stored.clone())).evaluate(scope, "Width"), Value::Error(Error::Div0));
        let mut after = d.clone();
        after.pages[0].shapes[0].sheet.cells[0].formula = Some("2/0".into());
        assert_eq!(Evaluator::new(&after).unwrap().with_stored_errors(Some(stored)).evaluate(scope, "Width"), Value::Error(Error::Div0));
    }
    #[test]
    fn stored_errors_ancestor_cannot_hide_a_deleted_dependency() {
        for formula in ["B+C", "B+SETF(\"C\",\"1\")"] {
            let d = document(&[("A", Some("7"), formula), ("B", Some("2"), "1/0"), ("C", Some("3"), "1")]);
            let stored = classify(&d);
            let mut after = d.clone();
            after.document_sheet.cells.pop();
            assert_eq!(Evaluator::new(&after).unwrap().with_stored_errors(Some(stored)).evaluate(Scope::Document, "A"), Value::Error(Error::Ref));
            // The same missing dependency already present before the edit is recoverable.
            let stored = classify(&after);
            assert_eq!(Evaluator::new(&after).unwrap().with_stored_errors(Some(stored)).evaluate(Scope::Document, "A"), Value::Number(7.));
        }
    }
    #[test]
    fn stored_errors_ancestor_cannot_hide_a_removed_reference_scope() {
        let mut d = document(&[("A", Some("7"), "B+Pages[Target]!Width"), ("B", Some("2"), "1/0")]);
        let mut page = misterdiagram_model::Page { id: 3, name: "Target".into(), ..Default::default() };
        page.sheet.cells.push(Cell { name: "Width".into(), value: Some("3".into()), ..Default::default() });
        d.pages.push(page);
        let stored = classify(&d);
        d.pages.clear();
        assert_eq!(Evaluator::new(&d).unwrap().with_stored_errors(Some(stored)).evaluate(Scope::Document, "A"), Value::Error(Error::Ref));
    }
    #[test]
    fn stored_errors_evalcell_keeps_new_errors_and_missing_targets_strict() {
        let d = document(&[("A", Some("7"), "B+EVALCELL(C)"), ("B", Some("2"), "1/0"), ("C", Some("3"), "1")]);
        let stored = classify(&d);
        let mut after = d.clone();
        after.document_sheet.cells[2].formula = Some("1/0".into());
        let mut e = Evaluator::new(&after).unwrap().with_stored_errors(Some(stored.clone()));
        assert_eq!(e.evaluate(Scope::Document, "A"), Value::Error(Error::Div0));
        after.document_sheet.cells.pop();
        assert_eq!(Evaluator::new(&after).unwrap().with_stored_errors(Some(stored)).evaluate(Scope::Document, "A"), Value::Error(Error::Ref));
        let stored = classify(&after);
        assert_eq!(Evaluator::new(&after).unwrap().with_stored_errors(Some(stored)).evaluate(Scope::Document, "A"), Value::Number(7.));
    }
    #[test]
    fn stored_errors_evalcell_frames_and_handled_errors_do_not_leak() {
        let mut d = document(&[
            ("A", Some("7"), "B+EVALCELL(Pages[Target]!Width)"),
            ("B", Some("2"), "1/0"),
            ("C", Some("3"), "ARG(\"x\")"),
            ("Handled", Some("7"), "IFERROR(EVALCELL(C,\"x\",1/0),4)"),
        ]);
        d.pages.push(misterdiagram_model::Page { id: 3, name: "Target".into(), ..Default::default() });
        d.pages[0].sheet.cells.push(Cell { name: "Width".into(), value: Some("3".into()), ..Default::default() });
        let stored = classify(&d);
        d.pages.clear();
        let mut e = Evaluator::new(&d).unwrap().with_stored_errors(Some(stored));
        assert_eq!(e.evaluate(Scope::Document, "A"), Value::Error(Error::Ref));
        assert_eq!(e.evaluate(Scope::Document, "Handled"), Value::Number(4.));
        for value in [2., 5.] {
            assert_eq!(e.eval_cell(Scope::Document, "C", &[("x".into(), Value::Number(value))]), Value::Number(value));
        }
    }
    #[test]
    fn stored_errors_path_reads_keep_removed_geometry_inputs_strict() {
        let mut d = document(&[("A", Some("7"), "B+PATHLENGTH(Pages[Target]!Sheet.9!Geometry1.Path)"), ("B", Some("2"), "1/0")]);
        let shape = misterdiagram_model::skeleton::rectangle(9, 1., 1., 2., 2., "").unwrap();
        d.pages.push(misterdiagram_model::Page { id: 3, name: "Target".into(), shapes: vec![shape].into(), ..Default::default() });
        let stored = classify(&d);
        let mut after = d.clone();
        let section = after.pages[0].shapes[0].sheet.sections.iter_mut().find(|s| s.name == "Geometry").unwrap();
        section.rows[0].cells.retain(|c| c.name != "X");
        assert_eq!(Evaluator::new(&after).unwrap().with_stored_errors(Some(stored.clone())).evaluate(Scope::Document, "A"), Value::Error(Error::Ref));
        after.pages[0].shapes[0].sheet.sections.retain(|s| s.name != "Geometry");
        assert_eq!(Evaluator::new(&after).unwrap().with_stored_errors(Some(stored)).evaluate(Scope::Document, "A"), Value::Error(Error::Ref));
        let stored = classify(&after);
        assert_eq!(Evaluator::new(&after).unwrap().with_stored_errors(Some(stored)).evaluate(Scope::Document, "A"), Value::Number(7.));
    }
}
