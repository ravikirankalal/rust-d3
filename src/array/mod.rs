// D3 Array module: unified API for all array utilities

use std::collections::HashMap;

// --- Core array functions ---

pub fn min<T: Ord + Copy>(data: &[T]) -> Option<T> {
    data.iter().copied().min()
}

pub fn max<T: Ord + Copy>(data: &[T]) -> Option<T> {
    data.iter().copied().max()
}

pub fn extent<T: Ord + Copy>(data: &[T]) -> Option<(T, T)> {
    if data.is_empty() {
        None
    } else {
        Some((min(data)?, max(data)?))
    }
}

pub fn quantile<T: Into<f64> + Copy>(data: &[T], q: f64) -> Option<f64> {
    if data.is_empty() || q < 0.0 || q > 1.0 {
        return None;
    }
    let mut v: Vec<f64> = data.iter().map(|&x| x.into()).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pos = q * (v.len() as f64 - 1.0);
    let lower = pos.floor() as usize;
    let upper = pos.ceil() as usize;
    if lower == upper {
        Some(v[lower])
    } else {
        let t = pos - lower as f64;
        Some(v[lower] * (1.0 - t) + v[upper] * t)
    }
}

pub fn cumsum<T: Into<f64> + Copy>(data: &[T]) -> Vec<f64> {
    let mut sum = 0.0;
    data.iter().map(|&x| {
        sum += x.into();
        sum
    }).collect()
}

// --- D3 Array Utilities (from array_utils) ---
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

// --- D3 Array Utils Advanced (from array_utils_adv) ---
/// D3.js: d3.bisect (stub)
pub fn bisect<T: PartialOrd>(_arr: &[T], _x: &T) -> usize {
    // TODO: Implement bisect logic
    0
}

/// D3.js: d3.merge (stub)
pub fn merge<T: Clone>(_arrays: &[Vec<T>]) -> Vec<T> {
    // TODO: Implement merge logic
    Vec::new()
}

/// D3.js: d3.cross (stub)
pub fn cross<T: Clone, U: Clone>(_a: &[T], _b: &[U]) -> Vec<(T, U)> {
    // TODO: Implement cross logic
    Vec::new()
}

/// Flattens a vector of vectors into a single vector.
pub fn flatten<T: Clone>(arrays: &[Vec<T>]) -> Vec<T> {
    arrays.iter().flat_map(|v| v.clone()).collect()
}

/// Returns a range of numbers from start (inclusive) to stop (exclusive) by step.
pub fn range(start: f64, stop: f64, step: f64) -> Vec<f64> {
    let mut v = Vec::new();
    let mut x = start;
    while x < stop {
        v.push(x);
        x += step;
    }
    v
}

/// Returns n ticks between a and b (inclusive).
pub fn ticks(domain: (f64, f64), n: usize) -> Vec<f64> {
    let (a, b) = domain;
    if n == 0 { return vec![]; }
    if n == 1 { return vec![a]; }
    if a == b { return vec![a]; }
    let step = (b - a) / (n as f64 - 1.0);
    (0..n).map(|i| a + i as f64 * step).collect()
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

// --- D3 Array Index Utilities ---
/// Returns the index of the minimum value
pub fn min_index<T: Ord + Copy>(data: &[T]) -> Option<usize> {
    data.iter().enumerate().min_by_key(|&(_, v)| v).map(|(i, _)| i)
}

/// Returns the index of the maximum value
pub fn max_index<T: Ord + Copy>(data: &[T]) -> Option<usize> {
    data.iter().enumerate().max_by_key(|&(_, v)| v).map(|(i, _)| i)
}
