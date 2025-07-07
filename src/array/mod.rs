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

/// Returns the merge of multiple arrays (d3.merge)
pub fn merge<T: Clone>(arrays: &[Vec<T>]) -> Vec<T> {
    arrays.iter().flat_map(|v| v.clone()).collect()
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

// --- Additional statistics functions ---
/// Returns the sum of the array elements.
pub fn sum<T: Into<f64> + Copy>(data: &[T]) -> f64 {
    data.iter().copied().map(Into::into).sum()
}

/// Returns the mean (average) of the array elements.
pub fn mean<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    let n = data.len();
    if n == 0 {
        None
    } else {
        Some(sum(data) / n as f64)
    }
}

/// Returns the median of the array elements.
pub fn median<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    let n = data.len();
    if n == 0 {
        return None;
    }
    let mut v: Vec<f64> = data.iter().map(|&x| x.into()).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = n as f64 / 2.0;
    if n % 2 == 0 {
        Some((v[mid as usize - 1] + v[mid as usize]) / 2.0)
    } else {
        Some(v[mid.floor() as usize])
    }
}

/// Returns the mode (most common value)
pub fn mode<T: Ord + Copy + std::hash::Hash>(data: &[T]) -> Option<T> {
    use std::collections::HashMap;
    if data.is_empty() { return None; }
    let mut counts = HashMap::new();
    for &x in data {
        *counts.entry(x).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|&(_, count)| count).map(|(val, _)| val)
}

/// Returns the variance of the array (d3.variance, population variance)
pub fn variance<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    let n = data.len();
    if n == 0 { return None; }
    let mean = data.iter().map(|&x| x.into()).sum::<f64>() / n as f64;
    let var = data.iter().map(|&x| {
        let d = x.into() - mean;
        d * d
    }).sum::<f64>() / n as f64;
    Some(var)
}

/// Returns the deviation (standard deviation) of the array (d3.deviation)
pub fn deviation<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    variance(data).map(|v| v.sqrt())
}

/// Returns the pairs of adjacent elements (d3.pairs)
pub fn pairs<T: Copy>(data: &[T]) -> Vec<(T, T)> {
    data.windows(2).map(|w| (w[0], w[1])).collect()
}

/// Returns the transpose of a 2D array (d3.transpose)
pub fn transpose<T: Copy>(data: &[Vec<T>]) -> Vec<Vec<T>> {
    if data.is_empty() { return vec![]; }
    let len = data[0].len();
    (0..len)
        .map(|i| data.iter().map(|row| row[i]).collect())
        .collect()
}

/// Returns the zip of multiple arrays (d3.zip)
pub fn zip<T: Copy>(arrays: &[&[T]]) -> Vec<Vec<T>> {
    if arrays.is_empty() { return vec![]; }
    let len = arrays[0].len();
    (0..len)
        .map(|i| arrays.iter().map(|arr| arr[i]).collect())
        .collect()
}

/// Returns the least element according to Ord (d3.least)
pub fn least<T: Ord + Copy>(data: &[T]) -> Option<T> {
    data.iter().copied().min()
}

/// Returns the greatest element according to Ord (d3.greatest)
pub fn greatest<T: Ord + Copy>(data: &[T]) -> Option<T> {
    data.iter().copied().max()
}

/// Shuffles the array in place (d3.shuffle)
pub fn shuffle<T>(data: &mut [T]) {
    use rand::seq::SliceRandom;
    data.shuffle(&mut rand::rngs::ThreadRng::default());
}

/// Returns a new array with elements permuted by indexes (d3.permute)
pub fn permute<T: Copy>(data: &[T], indexes: &[usize]) -> Vec<T> {
    indexes.iter().map(|&i| data[i]).collect()
}

/// Returns -1, 0, or 1 for ascending order (d3.ascending)
pub fn ascending<T: Ord>(a: T, b: T) -> i32 {
    a.cmp(&b) as i32
}

/// Returns -1, 0, or 1 for descending order (d3.descending)
pub fn descending<T: Ord>(a: T, b: T) -> i32 {
    b.cmp(&a) as i32
}
