// d3-collection: set implementation
// See: https://github.com/d3/d3-collection#set

use std::collections::HashSet;

pub struct Set<T> {
    inner: HashSet<T>,
}

impl<T: Eq + std::hash::Hash> Set<T> {
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: HashSet::with_capacity(cap),
        }
    }
    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value)
    }
    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }
    pub fn remove(&mut self, value: &T) -> bool {
        self.inner.remove(value)
    }
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_basic() {
        let mut s = Set::new();
        assert!(s.insert("a"));
        assert!(s.insert("b"));
        assert!(!s.insert("a"));
        assert!(s.contains(&"a"));
        assert!(s.contains(&"b"));
        assert!(!s.contains(&"c"));
        assert_eq!(s.len(), 2);
        assert!(!s.is_empty());
        assert!(s.remove(&"a"));
        assert!(!s.contains(&"a"));
        assert_eq!(s.len(), 1);
    }
}
