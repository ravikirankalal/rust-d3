// d3-collection: values implementation
// Returns the values of a map-like object as a Vec

use std::collections::HashMap;

pub fn values<K: Eq + std::hash::Hash, V: Clone>(map: &HashMap<K, V>) -> Vec<V> {
    map.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_values() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let mut v = values(&m);
        v.sort();
        assert_eq!(v, vec![1, 2]);
    }
}
