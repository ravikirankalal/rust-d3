// d3-collection: count_values implementation
// Return a Vec<(K, usize)> of counts

use std::collections::HashMap;

pub fn count_values<K: Eq + std::hash::Hash, V, F>(values: &[V], key_fn: F) -> Vec<(K, usize)>
where
    F: Fn(&V) -> K,
{
    let map = super::count::count(values, key_fn);
    map.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_values() {
        let data = vec!["a", "b", "a", "c", "b", "a"];
        let mut c = count_values(&data, |x| *x);
        c.sort_by_key(|(k, _)| *k);
        assert_eq!(c, vec![("a", 3), ("b", 2), ("c", 1)]);
    }
}
