use std::cmp::Ord;
use std::collections::HashMap;

pub fn map_to_vec<K, V>(map: &HashMap<K, V>) -> Vec<V>
where
    K: Ord + Clone,
    V: Clone,
{
    let mut v: Vec<_> = map.iter().collect();
    v.sort_by(|a, b| a.0.cmp(b.0));
    v.into_iter().map(|(_, v)| v.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_to_vec() {
        let mut m = HashMap::new();
        m.insert(2, "b");
        m.insert(1, "a");
        let v = map_to_vec(&m);
        assert_eq!(v, vec!["a", "b"]);
    }
}
