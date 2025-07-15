// d3-collection: map_values implementation
// Map values of a map, keeping keys

use std::collections::HashMap;

pub fn map_values<K: Eq + std::hash::Hash + Clone, V, U, F>(
    map: &HashMap<K, V>,
    f: F,
) -> HashMap<K, U>
where
    F: Fn(&V) -> U,
{
    map.iter().map(|(k, v)| (k.clone(), f(v))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_values() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let mv = map_values(&m, |v| v * 10);
        assert_eq!(mv["a"], 10);
        assert_eq!(mv["b"], 20);
    }
}
