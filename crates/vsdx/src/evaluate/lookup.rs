//! Bounded memoization of immutable source lookup, never evaluated values.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/about-cell-references
use super::Scope;
use misterdiagram_model::Cell;
use std::collections::HashMap;

#[derive(Default)]
pub(super) struct Baselines<'a> {
    scopes: HashMap<Scope, HashMap<String, Option<&'a Cell>>>,
    entries: usize,
    name_bytes: usize,
}
impl<'a> Baselines<'a> {
    pub fn get(&self, scope: Scope, name: &str) -> Option<Option<&'a Cell>> {
        self.scopes.get(&scope)?.get(name).copied()
    }
    pub fn insert(&mut self, scope: Scope, name: &str, value: Option<&'a Cell>) {
        // These are storage bounds only. Every miss, including saturation, runs
        // the original resolver, preserving its admission and error behavior.
        let Some(bytes) = self.name_bytes.checked_add(name.len()) else { return };
        if self.entries >= 131072 || bytes > 8 * 1024 * 1024 || name.len() > 65536 {
            return;
        }
        if self.scopes.entry(scope).or_default().insert(name.into(), value).is_none() {
            self.entries += 1;
            self.name_bytes = bytes;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn storage_saturation_keeps_hits_and_does_not_admit_unbounded_names() {
        let cell = Cell::default();
        let mut cache = Baselines::default();
        cache.insert(Scope::Page(0), "Width", Some(&cell));
        cache.insert(Scope::Page(0), "unknown", None);
        assert!(std::ptr::eq(cache.get(Scope::Page(0), "Width").unwrap().unwrap(), &cell));
        assert_eq!(cache.get(Scope::Page(0), "unknown"), Some(None));
        cache.insert(Scope::Page(0), &"x".repeat(65537), None);
        assert_eq!(cache.entries, 2);
        cache.entries = 131072;
        cache.insert(Scope::Page(0), "Height", Some(&cell));
        assert!(cache.get(Scope::Page(0), "Height").is_none());
        assert!(cache.get(Scope::Page(0), "Width").is_some());
        cache.entries = 2;
        cache.name_bytes = 8 * 1024 * 1024;
        cache.insert(Scope::Page(0), "Height", Some(&cell));
        assert!(cache.get(Scope::Page(0), "Height").is_none());
    }
}
