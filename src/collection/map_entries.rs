use std::collections::HashMap;

pub fn map_entries<K, V, U, F>(map: &HashMap<K, V>, f: F) -> Vec<U>
where
    K: Eq + std::hash::Hash,
    F: Fn(&K, &V) -> U,
{
    map.iter().map(|(k, v)| f(k, v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_entries() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let v = map_entries(&m, |k, v| format!("{}-{}", k, v));
        assert!(v.contains(&"a-1".to_string()));
        assert!(v.contains(&"b-2".to_string()));
    }
}
