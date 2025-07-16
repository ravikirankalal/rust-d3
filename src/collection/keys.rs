// d3-collection: keys implementation
// Returns the keys of a map-like object as a Vec

use std::collections::HashMap;

pub fn keys<K: Clone + Eq + std::hash::Hash, V>(map: &HashMap<K, V>) -> Vec<K> {
    map.keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_keys() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let mut k = keys(&m);
        k.sort();
        assert_eq!(k, vec!["a", "b"]);
    }
}
