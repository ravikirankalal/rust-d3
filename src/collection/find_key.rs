use std::collections::HashMap;

pub fn find_key<K, V, F>(map: &HashMap<K, V>, pred: F) -> Option<&K>
where
    K: Eq + std::hash::Hash,
    F: Fn(&K) -> bool,
{
    map.keys().find(|k| pred(k))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_key() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        assert_eq!(find_key(&m, |k| *k == "b"), Some(&"b"));
        assert_eq!(find_key(&m, |k| *k == "z"), None);
    }
}
