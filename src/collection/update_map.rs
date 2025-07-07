use std::collections::HashMap;

pub fn update_map<K, V, F>(map: &mut HashMap<K, V>, mut f: F)
where
    K: Eq + std::hash::Hash,
    F: FnMut(&K, &mut V),
{
    for (k, v) in map.iter_mut() {
        f(k, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_update_map() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        update_map(&mut m, |_, v| *v *= 10);
        assert_eq!(m["a"], 10);
        assert_eq!(m["b"], 20);
    }
}
