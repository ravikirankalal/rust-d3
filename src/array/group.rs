use std::collections::HashMap;
use std::hash::Hash;

pub fn group<T, K, F>(array: &[T], key_fn: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();
    for item in array {
        let key = key_fn(item);
        map.entry(key).or_insert_with(Vec::new).push(item.clone());
    }
    map
}
