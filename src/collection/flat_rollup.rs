// d3-collection: flat_rollup implementation
// Flat grouping with aggregation (returns Vec<(K, R)>)

use std::collections::HashMap;

pub fn flat_rollup<K, V, R, F, G>(values: &[V], key_fn: F, reduce_fn: G) -> Vec<(K, R)>
where
    K: Eq + std::hash::Hash,
    F: Fn(&V) -> K,
    G: Fn(&[&V]) -> R,
{
    let mut map: HashMap<K, Vec<&V>> = HashMap::new();
    for v in values {
        let k = key_fn(v);
        map.entry(k).or_default().push(v);
    }
    map.into_iter().map(|(k, vs)| (k, reduce_fn(&vs))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flat_rollup() {
        let data = vec![1, 2, 2, 3];
        let mut r = flat_rollup(&data, |x| *x, |vs| vs.len());
        r.sort_by_key(|(k, _)| *k);
        assert_eq!(r, vec![(1, 1), (2, 2), (3, 1)]);
    }
}
