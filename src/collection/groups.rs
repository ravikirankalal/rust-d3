// d3-collection: groups implementation
// Hierarchical grouping (multi-level)

use std::collections::HashMap;

pub fn groups<K, V, F>(values: &[V], key_fn: F) -> HashMap<K, Vec<&V>>
where
    K: Eq + std::hash::Hash,
    F: Fn(&V) -> K,
{
    let mut map: HashMap<K, Vec<&V>> = HashMap::new();
    for v in values {
        let k = key_fn(v);
        map.entry(k).or_default().push(v);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_groups() {
        let data = vec![1, 2, 2, 3];
        let g = groups(&data, |x| *x);
        assert_eq!(g[&1], vec![&1]);
        assert_eq!(g[&2], vec![&2, &2]);
        assert_eq!(g[&3], vec![&3]);
    }
}
