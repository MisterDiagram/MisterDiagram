//! Copy-on-write ownership for unchanged package bytes and parsed part contents.
use std::{
    any::Any,
    fmt,
    ops::{Deref, DerefMut},
    sync::Arc,
};

/// An immutable allocation shared until its owner is edited.
/// https://learn.microsoft.com/en-us/office/client-developer/visio/introduction-to-the-visio-file-formatvsdx
pub struct Shared<T>(Arc<T>);
impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: Default> Default for Shared<T> {
    fn default() -> Self {
        Self(Arc::new(T::default()))
    }
}
impl<T: fmt::Debug> fmt::Debug for Shared<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl<T: PartialEq> PartialEq for Shared<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || self.0 == other.0
    }
}
impl<T> From<T> for Shared<T> {
    fn from(value: T) -> Self {
        Self(Arc::new(value))
    }
}
impl<T> From<Arc<T>> for Shared<T> {
    fn from(value: Arc<T>) -> Self {
        Self(value)
    }
}
impl<T> Deref for Shared<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T: Clone> DerefMut for Shared<T> {
    fn deref_mut(&mut self) -> &mut T {
        Arc::make_mut(&mut self.0)
    }
}
impl<T> Shared<T> {
    pub fn shared(&self) -> Arc<T> {
        self.0.clone()
    }
    pub fn allocation_id(&self) -> usize {
        Arc::as_ptr(&self.0) as usize
    }
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl<T> AsRef<[T]> for Shared<Vec<T>> {
    fn as_ref(&self) -> &[T] {
        self.0.as_slice()
    }
}
impl<'a, T> IntoIterator for &'a Shared<Vec<T>> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'a, T: Clone> IntoIterator for &'a mut Shared<Vec<T>> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Arc::make_mut(&mut self.0).iter_mut()
    }
}
impl<T: Clone> IntoIterator for Shared<Vec<T>> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        Arc::unwrap_or_clone(self.0).into_iter()
    }
}
/// Nonsemantic, typed reader proof. Only the reader can construct its private proof type.
/// A proof must also match the part's exact immutable byte allocation before use.
#[derive(Clone, Default)]
pub struct PartMemo(Option<Arc<dyn Any + Send + Sync>>);
impl PartMemo {
    pub fn new<T: Any + Send + Sync>(value: T) -> Self {
        Self(Some(Arc::new(value)))
    }
    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.0.as_ref()?.downcast_ref()
    }
}
impl fmt::Debug for PartMemo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PartMemo")
    }
}
impl PartialEq for PartMemo {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl<T> FromIterator<T> for Shared<Vec<T>> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Vec::from_iter(iter).into()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mutation_detaches_without_changing_retained_owners() {
        let original: Shared<Vec<u8>> = vec![1, 2, 3].into();
        let mut changed = original.clone();
        assert!(original.ptr_eq(&changed));
        changed.push(4);
        assert!(!original.ptr_eq(&changed));
        assert_eq!(original.as_ref(), &[1, 2, 3]);
        assert_eq!(changed.as_ref(), &[1, 2, 3, 4]);
    }
}
