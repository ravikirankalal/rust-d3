// d3-collection: map_keys implementation
// Map keys of a map

use std::collections::HashMap;

pub fn map_keys<K: Eq + std::hash::Hash, V: Clone, U: Eq + std::hash::Hash, F>(
    map: &HashMap<K, V>,
    f: F,
) -> HashMap<U, V>
where
    F: Fn(&K) -> U,
{
    map.iter().map(|(k, v)| (f(k), v.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_keys() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let mk = map_keys(&m, |k| format!("{}!", k));
        assert_eq!(mk["a!"], 1);
        assert_eq!(mk["b!"], 2);
    }
}
