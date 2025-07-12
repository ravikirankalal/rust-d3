// d3-collection: from_entries implementation
// Construct a map from Vec<(K, V)>

use std::collections::HashMap;

pub fn from_entries<K: Eq + std::hash::Hash, V>(entries: Vec<(K, V)>) -> HashMap<K, V> {
    entries.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_entries() {
        let entries = vec![("a", 1), ("b", 2)];
        let m = from_entries(entries);
        assert_eq!(m["a"], 1);
        assert_eq!(m["b"], 2);
    }
}
