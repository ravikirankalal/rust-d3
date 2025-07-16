// d3-collection: entries implementation
// Returns the entries of a map-like object as a Vec<(K, V)>

use std::collections::HashMap;

pub fn entries<K: Clone + Eq + std::hash::Hash, V: Clone>(map: &HashMap<K, V>) -> Vec<(K, V)> {
    map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_entries() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let mut e = entries(&m);
        e.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(e, vec![("a", 1), ("b", 2)]);
    }
}
