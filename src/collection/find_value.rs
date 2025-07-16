use std::collections::HashMap;

pub fn find_value<K, V, F>(map: &HashMap<K, V>, pred: F) -> Option<&V>
where
    K: Eq + std::hash::Hash,
    F: Fn(&V) -> bool,
{
    map.values().find(|v| pred(v))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_value() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        assert_eq!(find_value(&m, |v| *v == 2), Some(&2));
        assert_eq!(find_value(&m, |v| *v == 3), None);
    }
}
