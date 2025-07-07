// d3-collection: merge_maps implementation
// Merge two maps (second map overwrites first on key conflict)

use std::collections::HashMap;

pub fn merge_maps<K: Eq + std::hash::Hash + Clone, V: Clone>(a: &HashMap<K, V>, b: &HashMap<K, V>) -> HashMap<K, V> {
    let mut merged = a.clone();
    for (k, v) in b.iter() {
        merged.insert(k.clone(), v.clone());
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_maps() {
        let mut a = HashMap::new();
        a.insert("a", 1);
        a.insert("b", 2);
        let mut b = HashMap::new();
        b.insert("b", 3);
        b.insert("c", 4);
        let m = merge_maps(&a, &b);
        assert_eq!(m["a"], 1);
        assert_eq!(m["b"], 3);
        assert_eq!(m["c"], 4);
    }
}
