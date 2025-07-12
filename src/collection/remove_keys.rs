use std::collections::HashMap;
use std::hash::Hash;

pub fn remove_keys<K, V, I>(map: &mut HashMap<K, V>, keys: I)
where
    K: Eq + Hash,
    I: IntoIterator<Item = K>,
{
    for k in keys {
        map.remove(&k);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_keys() {
        let mut m = HashMap::new();
        m.insert(1, "a");
        m.insert(2, "b");
        remove_keys(&mut m, vec![1]);
        assert!(!m.contains_key(&1));
        assert!(m.contains_key(&2));
    }
}
