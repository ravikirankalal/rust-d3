//! D3 Array Utilities module
//! Advanced array utilities, e.g., group, rollup, etc.
use std::collections::HashMap;

/// Groups values by a key function.
pub fn group<T, K, F>(data: &[T], key_fn: F) -> HashMap<K, Vec<&T>>
where
    K: std::cmp::Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    let mut map = HashMap::new();
    for item in data {
        map.entry(key_fn(item)).or_insert_with(Vec::new).push(item);
    }
    map
}

/// Rollup: groups values by a key function and reduces each group.
pub fn rollup<T, K, V, F, R>(data: &[T], key_fn: F, reduce_fn: R) -> HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
    F: Fn(&T) -> K,
    R: Fn(&[&T]) -> V,
{
    let grouped = group(data, &key_fn);
    grouped
        .into_iter()
        .map(|(k, v)| (k, reduce_fn(&v)))
        .collect()
}

/// TODO: Implement d3-array group/rollup/flatGroup/flatRollup/index/flatIndex for full parity
/// See: https://github.com/d3/d3-array

/// Returns a flat group (like d3.flatGroup)
pub fn flat_group<T, K, F>(data: &[T], key_fn: F) -> Vec<(K, Vec<&T>)>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    F: Fn(&T) -> K,
{
    let mut map = HashMap::new();
    for item in data {
        map.entry(key_fn(item)).or_insert_with(Vec::new).push(item);
    }
    map.into_iter().collect()
}

/// Returns a flat rollup (like d3.flatRollup)
pub fn flat_rollup<T, K, V, F, R>(data: &[T], key_fn: F, reduce_fn: R) -> Vec<(K, V)>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    F: Fn(&T) -> K,
    R: Fn(&[&T]) -> V,
{
    let mut map = HashMap::new();
    for item in data {
        map.entry(key_fn(item)).or_insert_with(Vec::new).push(item);
    }
    map.into_iter().map(|(k, v)| (k, reduce_fn(&v))).collect()
}

/// Returns an index (like d3.index)
pub fn index<T, K, F>(data: &[T], key_fn: F) -> HashMap<K, &T>
where
    K: std::cmp::Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    let mut map = HashMap::new();
    for item in data {
        map.insert(key_fn(item), item);
    }
    map
}

/// Returns a flat index (like d3.flatIndex)
pub fn flat_index<T, K, F>(data: &[T], key_fn: F) -> Vec<(K, &T)>
where
    K: std::cmp::Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    data.iter().map(|item| (key_fn(item), item)).collect()
}

/// Returns the index of the least element according to the comparator (like d3.scan)
pub fn scan<T, F>(data: &[T], mut cmp: F) -> Option<usize>
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    if data.is_empty() { return None; }
    let mut least = 0;
    for i in 1..data.len() {
        if cmp(&data[i], &data[least]) == std::cmp::Ordering::Less {
            least = i;
        }
    }
    Some(least)
}

/// Returns the least element (like d3.least)
pub fn least<T, F>(data: &[T], mut cmp: F) -> Option<&T>
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    data.iter().min_by(|a, b| cmp(a, b))
}

/// Returns the greatest element (like d3.greatest)
pub fn greatest<T, F>(data: &[T], mut cmp: F) -> Option<&T>
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    data.iter().max_by(|a, b| cmp(a, b))
}

/// Accurate floating-point summation (like d3.fsum) using Neumaier's algorithm
pub fn fsum<I: IntoIterator<Item = f64>>(iter: I) -> f64 {
    let mut sum = 0.0f64;
    let mut c = 0.0f64;
    for x in iter {
        let t = sum + x;
        if sum.abs() >= x.abs() {
            c += (sum - t) + x;
        } else {
            c += (x - t) + sum;
        }
        sum = t;
    }
    sum + c
}

/// Adder struct for incremental, accurate summation (like d3.Adder, Neumaier's algorithm)
pub struct Adder {
    sum: f64,
    c: f64,
}

impl Adder {
    pub fn new() -> Self {
        Self { sum: 0.0, c: 0.0 }
    }
    pub fn add(&mut self, x: f64) {
        let t = self.sum + x;
        if self.sum.abs() >= x.abs() {
            self.c += (self.sum - t) + x;
        } else {
            self.c += (x - t) + self.sum;
        }
        self.sum = t;
    }
    pub fn value(&self) -> f64 {
        self.sum + self.c
    }
}
