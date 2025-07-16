use std::collections::HashMap;

pub fn partition_map<K, V, F>(map: &HashMap<K, V>, pred: F) -> (HashMap<K, V>, HashMap<K, V>)
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
    F: Fn(&K, &V) -> bool,
{
    let mut left = HashMap::new();
    let mut right = HashMap::new();
    for (k, v) in map.iter() {
        if pred(k, v) {
            left.insert(k.clone(), v.clone());
        } else {
            right.insert(k.clone(), v.clone());
        }
    }
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_partition_map() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let (even, odd) = partition_map(&m, |_, v| *v % 2 == 0);
        assert_eq!(even.get("b"), Some(&2));
        assert_eq!(odd.get("a"), Some(&1));
    }
}
