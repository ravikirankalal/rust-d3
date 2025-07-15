// d3-collection: rollup implementation
// Groups and reduces values by key

use std::collections::HashMap;

pub fn rollup<K, V, G, R, F>(values: &[V], key_fn: G, reduce_fn: F) -> HashMap<K, R>
where
    K: Eq + std::hash::Hash,
    G: Fn(&V) -> K,
    F: Fn(&[&V]) -> R,
{
    let mut groups: HashMap<K, Vec<&V>> = HashMap::new();
    for v in values {
        let k = key_fn(v);
        groups.entry(k).or_default().push(v);
    }
    groups
        .into_iter()
        .map(|(k, vs)| (k, reduce_fn(&vs)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rollup_sum() {
        let data = vec![1, 2, 2, 3, 3, 3];
        let result = rollup(&data, |x| *x, |vs| vs.len());
        assert_eq!(result[&1], 1);
        assert_eq!(result[&2], 2);
        assert_eq!(result[&3], 3);
    }
}
