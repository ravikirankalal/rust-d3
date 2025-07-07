// d3-collection: filter_map implementation
// Filter a map by predicate

use std::collections::HashMap;

pub fn filter_map<K: Eq + std::hash::Hash + Clone, V: Clone, F>(map: &HashMap<K, V>, pred: F) -> HashMap<K, V>
where
    F: Fn(&K, &V) -> bool,
{
    map.iter().filter(|(k, v)| pred(k, v)).map(|(k, v)| (k.clone(), v.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_filter_map() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        m.insert("c", 3);
        let f = filter_map(&m, |_, v| *v % 2 == 1);
        assert_eq!(f.len(), 2);
        assert!(f.contains_key("a"));
        assert!(f.contains_key("c"));
    }
}
