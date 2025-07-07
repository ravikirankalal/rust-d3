use std::collections::HashMap;

pub fn map_filter<K, V, U, F>(map: &HashMap<K, V>, f: F) -> HashMap<K, U>
where
    K: Eq + std::hash::Hash + Clone,
    F: Fn(&K, &V) -> Option<U>,
{
    map.iter()
        .filter_map(|(k, v)| f(k, v).map(|u| (k.clone(), u)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map_filter() {
        let mut m = HashMap::new();
        m.insert("a", 1);
        m.insert("b", 2);
        let mf = map_filter(&m, |_, v| if *v % 2 == 0 { Some(v * 10) } else { None });
        assert_eq!(mf.get("a"), None);
        assert_eq!(mf.get("b"), Some(&20));
    }
}
