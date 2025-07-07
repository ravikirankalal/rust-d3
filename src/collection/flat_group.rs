// d3-collection: flat_group implementation
// Flat grouping (returns Vec<(K, Vec<&V>)>)

use std::collections::HashMap;

pub fn flat_group<K, V, F>(values: &[V], key_fn: F) -> Vec<(K, Vec<&V>)>
where
    K: Eq + std::hash::Hash,
    F: Fn(&V) -> K,
{
    let mut map: HashMap<K, Vec<&V>> = HashMap::new();
    for v in values {
        let k = key_fn(v);
        map.entry(k).or_default().push(v);
    }
    map.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flat_group() {
        let data = vec![1, 2, 2, 3];
        let mut g = flat_group(&data, |x| *x);
        g.sort_by_key(|(k, _)| *k);
        assert_eq!(g[0], (1, vec![&1]));
        assert_eq!(g[1], (2, vec![&2, &2]));
        assert_eq!(g[2], (3, vec![&3]));
    }
}
