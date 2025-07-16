// d3-collection: map_map implementation
// Map values of a map

use std::collections::HashMap;

pub fn map_map<K: Eq + std::hash::Hash + Clone, V, K2: Eq + std::hash::Hash, U, F>(
    map: &HashMap<K, V>,
    f: F,
) -> HashMap<K2, U>
where
    F: Fn(&K, &V) -> (K2, U),
{
    map.iter().map(|(k, v)| f(k, v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_map() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let mm = map_map(&m, |k, v| ((*k).to_string(), v * 10));
        assert_eq!(mm["a"], 10);
        assert_eq!(mm["b"], 20);
    }
}
