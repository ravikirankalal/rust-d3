// D3 Array module: unified API for all array utilities

pub mod array;

use std::collections::HashMap;

// Re-export all public items from array.rs
pub use self::array::*;

// --- D3 Array Utilities (defined in mod.rs) ---
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

// --- D3 Array Utils Advanced (defined in mod.rs) ---
/// Returns the bisect index for x in a sorted array (d3.bisect)
pub fn bisect<T: PartialOrd>(arr: &[T], x: &T) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = (low + high) / 2;
        if &arr[mid] < x {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// Returns the cross product of two arrays (d3.cross)
pub fn cross<T: Clone, U: Clone>(a: &[T], b: &[U]) -> Vec<(T, U)> {
    let mut result = Vec::new();
    for x in a {
        for y in b {
            result.push((x.clone(), y.clone()));
        }
    }
    result
}

/// Flattens a vector of vectors into a single vector.
pub fn flatten<T: Clone>(arrays: &[Vec<T>]) -> Vec<T> {
    arrays.iter().flat_map(|v| v.clone()).collect()
}

// --- Accurate floating-point summation (fsum, Adder) ---
/// Accurate floating-point summation (Kahan-Babuska-Neumaier)
pub fn fsum<I: IntoIterator<Item = f64>>(iter: I) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0;
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

pub struct Adder {
    sum: f64,
    c: f64,
}

impl Adder {
    pub fn new() -> Self {
        Adder { sum: 0.0, c: 0.0 }
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