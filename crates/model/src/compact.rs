//! Compact, copy-on-write XML strings and sorted attribute tables.
//! https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
use std::{
    ops::Deref,
    sync::{Arc, OnceLock},
};

type Pair = (Arc<String>, Arc<String>);
type BorrowedPair<'a> = (&'a String, &'a String);

/// Small tables have no tree-node or hash-bucket minimum allocation. Parsed tables
/// share their immutable strings with the typed model; authoring uses copy-on-write.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attributes(Vec<Pair>);
impl Attributes {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.binary_search_by(|(k, _)| k.as_str().cmp(key)).ok().and_then(|i| self.0.get(i)).map(|(_, v)| v.as_ref())
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut String> {
        let i = self.0.binary_search_by(|(k, _)| k.as_str().cmp(key)).ok()?;
        self.0.get_mut(i).map(|(_, v)| Arc::make_mut(v))
    }
    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        match self.0.binary_search_by(|(k, _)| k.as_ref().cmp(&key)) {
            Ok(i) => self.0.get_mut(i).map(|(_, v)| Arc::unwrap_or_clone(std::mem::replace(v, Arc::new(value)))),
            Err(i) => {
                self.0.insert(i, (Arc::new(key), Arc::new(value)));
                None
            }
        }
    }
    pub fn remove(&mut self, key: &str) -> Option<String> {
        let i = self.0.binary_search_by(|(k, _)| k.as_str().cmp(key)).ok()?;
        Some(Arc::unwrap_or_clone(self.0.remove(i).1))
    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn iter(&self) -> impl Iterator<Item = BorrowedPair<'_>> {
        self.0.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.iter().map(|(k, _)| k.as_ref())
    }
    pub fn retain(&mut self, mut f: impl FnMut(&String, &mut String) -> bool) {
        self.0.retain_mut(|(k, v)| f(k.as_ref(), Arc::make_mut(v)));
    }
    /// Bulk construction avoids quadratic insertion for wide XML elements.
    pub fn shared(mut values: Vec<Pair>) -> Self {
        values.sort_by(|a, b| a.0.cmp(&b.0));
        values.dedup_by(|later, earlier| {
            if later.0 == earlier.0 {
                earlier.1 = later.1.clone();
                true
            } else {
                false
            }
        });
        values.shrink_to_fit();
        Self(values)
    }
}
impl FromIterator<(String, String)> for Attributes {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(it: T) -> Self {
        Self::shared(it.into_iter().map(|(k, v)| (Arc::new(k), Arc::new(v))).collect())
    }
}
impl<const N: usize> From<[(String, String); N]> for Attributes {
    fn from(v: [(String, String); N]) -> Self {
        v.into_iter().collect()
    }
}
impl IntoIterator for Attributes {
    type Item = (String, String);
    type IntoIter = std::iter::Map<std::vec::IntoIter<Pair>, fn(Pair) -> Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|(k, v)| (Arc::unwrap_or_clone(k), Arc::unwrap_or_clone(v)))
    }
}
impl<'a> IntoIterator for &'a Attributes {
    type Item = BorrowedPair<'a>;
    type IntoIter = std::iter::Map<std::slice::Iter<'a, Pair>, fn(&'a Pair) -> Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }
}

/// One-word shared handle for interned XML names and resolved namespace URIs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XmlString(Arc<String>);
impl XmlString {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn shared(&self) -> Arc<String> {
        self.0.clone()
    }
}
impl Default for XmlString {
    fn default() -> Self {
        static EMPTY: OnceLock<Arc<String>> = OnceLock::new();
        Self(EMPTY.get_or_init(|| Arc::new(String::new())).clone())
    }
}
impl Deref for XmlString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}
impl From<&str> for XmlString {
    fn from(s: &str) -> Self {
        Self(Arc::new(s.to_string()))
    }
}
impl From<String> for XmlString {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}
impl From<Arc<String>> for XmlString {
    fn from(s: Arc<String>) -> Self {
        Self(s)
    }
}
impl PartialEq<str> for XmlString {
    fn eq(&self, s: &str) -> bool {
        self.as_str() == s
    }
}
impl PartialEq<&str> for XmlString {
    fn eq(&self, s: &&str) -> bool {
        self.as_str() == *s
    }
}
impl std::fmt::Display for XmlString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compact_tables_keep_last_value_and_isolate_mutation() {
        let mut a =
            Attributes::from([("z".into(), "1".into()), ("a".into(), "old".into()), ("a".into(), "middle".into()), ("a".into(), "new".into())]);
        let b = a.clone();
        assert_eq!(a.get("a").unwrap(), "new");
        a.get_mut("a").unwrap().push('!');
        assert_eq!(b.get("a").unwrap(), "new");
        assert_eq!(a.keys().map(String::as_str).collect::<Vec<_>>(), ["a", "z"]);
        assert_eq!(a.remove("z"), Some("1".into()));
    }
}
