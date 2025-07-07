//! D3 Collection module
//! Provides collection utilities (see d3-collection in JS).

use std::collections::HashMap;
use std::hash::Hash;

/// Groups values by one or more keys, returning a nested map (like d3.group).
pub fn group<T, K, F>(values: impl IntoIterator<Item = T>, key_fn: F) -> HashMap<K, Vec<T>>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();
    for v in values {
        let k = key_fn(&v);
        map.entry(k).or_default().push(v);
    }
    map
}

/// Groups and reduces values by keys, returning a nested map of reduced values (like d3.rollup).
pub fn rollup<T, K, V, F, R>(
    values: impl IntoIterator<Item = T>,
    key_fn: F,
    reduce_fn: R,
) -> HashMap<K, V>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
    R: Fn(Vec<T>) -> V,
{
    let grouped = group(values, key_fn);
    grouped
        .into_iter()
        .map(|(k, vs)| (k, reduce_fn(vs)))
        .collect()
}

/// Returns the keys of a map as a Vec (like Object.keys).
pub fn keys<K: Clone, V>(map: &HashMap<K, V>) -> Vec<K> {
    map.keys().cloned().collect()
}

/// Returns the values of a map as a Vec (like Object.values).
pub fn values<K, V: Clone>(map: &HashMap<K, V>) -> Vec<V> {
    map.values().cloned().collect()
}

/// Returns the entries of a map as a Vec of (K, V) pairs (like Object.entries).
pub fn entries<K: Clone, V: Clone>(map: &HashMap<K, V>) -> Vec<(K, V)> {
    map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}
