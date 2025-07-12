// Port of d3-array's intern to Rust
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Returns a reference to the value in the set if present, or inserts and returns a reference to the new value.
pub fn intern_set<'a, T: Eq + Hash + Clone>(set: &'a mut HashSet<T>, value: T) -> &'a T {
    if !set.contains(&value) {
        set.insert(value.clone());
    }
    set.get(&value).unwrap()
}

/// Returns a reference to the value in the map if present, or inserts and returns a reference to the new value.
pub fn intern_map<'a, K: Eq + Hash + Clone, V>(map: &'a mut HashMap<K, V>, key: K, default: V) -> &'a V {
    map.entry(key).or_insert(default)
}
