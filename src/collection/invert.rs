// d3-collection: invert implementation
// Invert keys and values in a map (if possible)

use std::collections::HashMap;

pub fn invert<K: Eq + std::hash::Hash, V: Eq + std::hash::Hash + Clone>(
    map: &HashMap<K, V>,
) -> HashMap<V, K>
where
    K: Clone,
{
    map.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_invert() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let inv = invert(&m);
        assert_eq!(inv[&1], "a");
        assert_eq!(inv[&2], "b");
    }
}
