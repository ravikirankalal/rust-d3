// d3-collection: index implementation
// Creates a map from an array using a key function

use std::collections::HashMap;

pub fn index<K, V, F>(values: &[V], key_fn: F) -> HashMap<K, &V>
where
    K: Eq + std::hash::Hash,
    F: Fn(&V) -> K,
{
    let mut map = HashMap::new();
    for v in values {
        map.insert(key_fn(v), v);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_index() {
        let data = vec!["a", "b", "c"];
        let idx = index(&data, |x| *x);
        assert_eq!(idx[&"a"], &"a");
        assert_eq!(idx[&"b"], &"b");
        assert_eq!(idx[&"c"], &"c");
    }
}
