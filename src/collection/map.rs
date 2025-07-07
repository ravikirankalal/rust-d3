// d3-collection: map implementation
// See: https://github.com/d3/d3-collection#map

use std::collections::HashMap;

pub struct Map<K, V> {
    inner: HashMap<K, V>,
}

impl<K: Eq + std::hash::Hash, V> Map<K, V> {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self { inner: HashMap::with_capacity(cap) }
    }
    pub fn insert(&mut self, key: K, value: V) {
        self.inner.insert(key, value);
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys()
    }
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.values()
    }
    pub fn entries(&self) -> impl Iterator<Item = (&K, &V)> {
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
    fn test_map_basic() {
        let mut m = Map::new();
        m.insert("a", 1);
        m.insert("b", 2);
        assert_eq!(m.get(&"a"), Some(&1));
        assert_eq!(m.get(&"b"), Some(&2));
        assert_eq!(m.get(&"c"), None);
        assert_eq!(m.len(), 2);
        assert!(!m.is_empty());
        assert_eq!(m.remove(&"a"), Some(1));
        assert_eq!(m.get(&"a"), None);
        assert_eq!(m.len(), 1);
    }
}
