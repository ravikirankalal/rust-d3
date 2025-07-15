use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

pub fn retain_keys<K, V, I>(map: &mut HashMap<K, V>, keys: I)
where
    K: Eq + Hash + Clone,
    I: IntoIterator<Item = K>,
{
    let key_set: HashSet<K> = keys.into_iter().collect();
    map.retain(|k, _| key_set.contains(k));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_retain_keys() {
        let mut m = HashMap::new();
        m.insert(1, "a");
        m.insert(2, "b");
        retain_keys(&mut m, vec![2]);
        assert!(!m.contains_key(&1));
        assert!(m.contains_key(&2));
    }
}
