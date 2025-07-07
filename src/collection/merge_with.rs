use std::collections::HashMap;

pub fn merge_with<K, V, F>(a: &HashMap<K, V>, b: &HashMap<K, V>, mut f: F) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
    F: FnMut(&K, &V, &V) -> V,
{
    let mut result = a.clone();
    for (k, v_b) in b.iter() {
        result.entry(k.clone())
            .and_modify(|v_a| *v_a = f(k, v_a, v_b))
            .or_insert_with(|| v_b.clone());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_with() {
        let mut a = HashMap::new();
        a.insert("x", 1);
        a.insert("y", 2);
        let mut b = HashMap::new();
        b.insert("y", 3);
        b.insert("z", 4);
        let merged = merge_with(&a, &b, |_, v1, v2| v1 + v2);
        assert_eq!(merged["x"], 1);
        assert_eq!(merged["y"], 5);
        assert_eq!(merged["z"], 4);
    }
}
