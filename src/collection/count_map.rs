// d3-collection: count_map implementation
// Return a map of counts by key (alias for count)

use std::collections::HashMap;

pub fn count_map<K: Eq + std::hash::Hash, V, F>(values: &[V], key_fn: F) -> HashMap<K, usize>
where
    F: Fn(&V) -> K,
{
    super::count::count(values, key_fn)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_map() {
        let data = vec!["a", "b", "a", "c", "b", "a"];
        let c = count_map(&data, |x| *x);
        assert_eq!(c[&"a"], 3);
        assert_eq!(c[&"b"], 2);
        assert_eq!(c[&"c"], 1);
    }
}
