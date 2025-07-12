// d3-collection: count implementation
// Count values by key

use std::collections::HashMap;

pub fn count<K: Eq + std::hash::Hash, V, F>(values: &[V], key_fn: F) -> HashMap<K, usize>
where
    F: Fn(&V) -> K,
{
    let mut map = HashMap::new();
    for v in values {
        let k = key_fn(v);
        *map.entry(k).or_insert(0) += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count() {
        let data = vec!["a", "b", "a", "c", "b", "a"];
        let c = count(&data, |x| *x);
        assert_eq!(c[&"a"], 3);
        assert_eq!(c[&"b"], 2);
        assert_eq!(c[&"c"], 1);
    }
}
