// D3 Array module: unified API for all array utilities
// All public API for d3-array parity in a single file.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// Grouping and rollup
pub fn group<T, K, F>(data: &[T], key_fn: F) -> HashMap<K, Vec<&T>>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut map = HashMap::new();
    for item in data {
        map.entry(key_fn(item)).or_insert_with(Vec::new).push(item);
    }
    map
}

pub fn rollup<T, K, V, F, R>(data: &[T], key_fn: F, reduce_fn: R) -> HashMap<K, V>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
    R: Fn(&[&T]) -> V,
{
    let grouped = group(data, &key_fn);
    grouped.into_iter().map(|(k, v)| (k, reduce_fn(&v))).collect()
}

pub fn flat_group<T, K, F>(data: &[T], key_fn: F) -> Vec<(K, Vec<&T>)>
where
    K: Eq + Hash + Clone,
    F: Fn(&T) -> K,
{
    let mut map = HashMap::new();
    for item in data {
        map.entry(key_fn(item)).or_insert_with(Vec::new).push(item);
    }
    map.into_iter().collect()
}

// Basic statistics
pub fn min<T: Ord + Copy>(data: &[T]) -> Option<T> {
    data.iter().copied().min()
}

pub fn max<T: Ord + Copy>(data: &[T]) -> Option<T> {
    data.iter().copied().max()
}

pub fn extent<T: Ord + Copy>(data: &[T]) -> Option<(T, T)> {
    if data.is_empty() { None } else { Some((min(data)?, max(data)?)) }
}

pub fn quantile<T: Into<f64> + Copy>(data: &[T], q: f64) -> Option<f64> {
    if data.is_empty() || q < 0.0 || q > 1.0 { return None; }
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

// Quantile for already-sorted arrays (no sort performed)
pub fn quantile_sorted<T: Into<f64> + Copy>(data: &[T], q: f64) -> Option<f64> {
    if data.is_empty() || q < 0.0 || q > 1.0 { return None; }
    let len = data.len();
    let pos = q * (len as f64 - 1.0);
    let lower = pos.floor() as usize;
    let upper = pos.ceil() as usize;
    let v: Vec<f64> = data.iter().map(|&x| x.into()).collect();
    if lower == upper {
        Some(v[lower])
    } else {
        let t = pos - lower as f64;
        Some(v[lower] * (1.0 - t) + v[upper] * t)
    }
}

// Quantile (inclusive interpolation, D3.js 7+)
pub fn quantile_sorted_inclusive<T: Into<f64> + Copy>(data: &[T], q: f64) -> Option<f64> {
    if data.is_empty() || q < 0.0 || q > 1.0 { return None; }
    let len = data.len();
    let pos = q * (len as f64 - 1.0);
    let lower = pos.floor() as usize;
    let upper = pos.ceil() as usize;
    let v: Vec<f64> = data.iter().map(|&x| x.into()).collect();
    if lower == upper {
        Some(v[lower])
    } else {
        let t = pos - lower as f64;
        Some(v[lower] * (1.0 - t) + v[upper] * t)
    }
}

// Quantile index (returns index of quantile value in unsorted array)
pub fn quantile_index<T: Into<f64> + Copy>(data: &[T], q: f64) -> Option<usize> {
    if data.is_empty() || q < 0.0 || q > 1.0 { return None; }
    let mut v: Vec<(usize, f64)> = data.iter().enumerate().map(|(i, &x)| (i, x.into())).collect();
    v.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let pos = q * (v.len() as f64 - 1.0);
    let lower = pos.floor() as usize;
    let upper = pos.ceil() as usize;
    if lower == upper {
        Some(v[lower].0)
    } else {
        let t = pos - lower as f64;
        if t < 0.5 { Some(v[lower].0) } else { Some(v[upper].0) }
    }
}

// Quantile index for already-sorted arrays (returns index)
pub fn quantile_sorted_index<T: Into<f64> + Copy>(data: &[T], q: f64) -> Option<usize> {
    if data.is_empty() || q < 0.0 || q > 1.0 { return None; }
    let len = data.len();
    let pos = q * (len as f64 - 1.0);
    let lower = pos.floor() as usize;
    let upper = pos.ceil() as usize;
    if lower == upper {
        Some(lower)
    } else {
        let t = pos - lower as f64;
        if t < 0.5 { Some(lower) } else { Some(upper) }
    }
}

// Freedman–Diaconis threshold function for histogram binning
pub fn freedman_diaconis_thresholds(data: &[f64], min: f64, max: f64) -> Vec<f64> {
    if data.len() < 2 { return vec![min, max]; }
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q1 = quantile_sorted(&sorted, 0.25).unwrap_or(min);
    let q3 = quantile_sorted(&sorted, 0.75).unwrap_or(max);
    let iqr = q3 - q1;
    let bin_width = if iqr == 0.0 { (max - min) / (data.len() as f64).sqrt() } else { 2.0 * iqr / (data.len() as f64).cbrt() };
    let bin_count = ((max - min) / bin_width).ceil().max(1.0) as usize;
    (0..=bin_count).map(|i| min + i as f64 * bin_width).collect()
}

// Scott's normal reference rule for histogram binning
pub fn scott_thresholds(data: &[f64], min: f64, max: f64) -> Vec<f64> {
    if data.len() < 2 { return vec![min, max]; }
    let stddev = deviation(data).unwrap_or(0.0);
    let bin_width = if stddev == 0.0 { (max - min) / (data.len() as f64).sqrt() } else { 3.5 * stddev / (data.len() as f64).cbrt() };
    let bin_count = ((max - min) / bin_width).ceil().max(1.0) as usize;
    (0..=bin_count).map(|i| min + i as f64 * bin_width).collect()
}

pub fn cumsum<T: Into<f64> + Copy>(data: &[T]) -> Vec<f64> {
    let mut sum = 0.0;
    data.iter().map(|&x| { sum += x.into(); sum }).collect()
}

pub fn range(start: f64, stop: f64, step: f64) -> Vec<f64> {
    let mut v = Vec::new();
    if step == 0.0 { return v; }
    let n = ((stop - start) / step).floor() as isize;
    if n <= 0 { return v; }
    for i in 0..n {
        v.push(start + i as f64 * step);
    }
    v
}

pub fn ticks(domain: (f64, f64), count: usize) -> Vec<f64> {
    let (start, stop) = domain;
    if count == 0 { return vec![]; }
    if count == 1 { return vec![start]; }
    if start == stop { return vec![start]; }
    let step = (stop - start) / (count as f64 - 1.0);
    (0..count).map(|i| start + i as f64 * step).collect()
}

pub fn min_index<T: Ord + Copy>(data: &[T]) -> Option<usize> {
    data.iter().enumerate().min_by_key(|&(_, v)| v).map(|(i, _)| i)
}

pub fn max_index<T: Ord + Copy>(data: &[T]) -> Option<usize> {
    data.iter().enumerate().max_by_key(|&(_, v)| v).map(|(i, _)| i)
}

pub fn sum<T: Into<f64> + Copy>(data: &[T]) -> f64 {
    data.iter().map(|&x| x.into()).sum()
}

pub fn mean<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    if data.is_empty() { None } else { Some(sum(data) / data.len() as f64) }
}

pub fn median<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    if data.is_empty() { return None; }
    let mut v: Vec<f64> = data.iter().map(|&x| x.into()).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = v.len() / 2;
    if v.len() % 2 == 0 {
        Some((v[mid - 1] + v[mid]) / 2.0)
    } else {
        Some(v[mid])
    }
}

pub fn mode<T: Ord + Copy + Hash>(data: &[T]) -> Option<T> {
    if data.is_empty() { return None; }
    let mut counts = HashMap::new();
    for &x in data {
        *counts.entry(x).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|&(_, count)| count).map(|(val, _)| val)
}

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

pub fn deviation<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    variance(data).map(|v| v.sqrt())
}

pub fn least<T: Ord + Copy>(data: &[T]) -> Option<T> {
    data.iter().copied().min()
}

pub fn greatest<T: Ord + Copy>(data: &[T]) -> Option<T> {
    data.iter().copied().max()
}

pub fn pairs<T: Copy>(data: &[T]) -> Vec<(T, T)> {
    data.windows(2).map(|w| (w[0], w[1])).collect()
}

pub fn transpose<T: Copy>(data: &[Vec<T>]) -> Vec<Vec<T>> {
    if data.is_empty() { return vec![]; }
    let len = data[0].len();
    (0..len).map(|i| data.iter().map(|row| row[i]).collect()).collect()
}

pub fn zip<T: Copy>(arrays: &[&[T]]) -> Vec<Vec<T>> {
    if arrays.is_empty() { return vec![]; }
    let len = arrays.iter().map(|arr| arr.len()).min().unwrap_or(0);
    (0..len).map(|i| arrays.iter().map(|arr| arr[i]).collect()).collect()
}

pub fn merge<T: Clone>(arrays: &[Vec<T>]) -> Vec<T> {
    arrays.iter().flat_map(|v| v.clone()).collect()
}

pub fn shuffle<T>(data: &mut [T]) {
    use rand::seq::SliceRandom;
    data.shuffle(&mut rand::rng());
}

pub fn permute<T: Copy>(data: &[T], indexes: &[usize]) -> Vec<T> {
    indexes.iter().map(|&i| data[i]).collect()
}

pub fn ascending<T: Ord>(a: T, b: T) -> i32 {
    a.cmp(&b) as i32
}

pub fn descending<T: Ord>(a: T, b: T) -> i32 {
    b.cmp(&a) as i32
}

// Set operations
pub fn union<T: Clone + Eq + Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let set: HashSet<_> = a.iter().cloned().chain(b.iter().cloned()).collect();
    set.into_iter().collect()
}

pub fn intersection<T: Clone + Eq + Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let set_a: HashSet<_> = a.iter().cloned().collect();
    let set_b: HashSet<_> = b.iter().cloned().collect();
    set_a.intersection(&set_b).cloned().collect()
}

pub fn difference<T: Clone + Eq + Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let set_a: HashSet<_> = a.iter().cloned().collect();
    let set_b: HashSet<_> = b.iter().cloned().collect();
    set_a.difference(&set_b).cloned().collect()
}

pub fn cross<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    let mut result = Vec::new();
    for x in a {
        for y in b {
            result.push((x.clone(), y.clone()));
        }
    }
    result
}

// Bisect
pub fn bisect_left<T: Ord>(data: &[T], x: &T) -> usize {
    let mut lo = 0;
    let mut hi = data.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if &data[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

pub fn bisect_right<T: Ord>(data: &[T], x: &T) -> usize {
    let mut lo = 0;
    let mut hi = data.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if &data[mid] <= x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

pub fn bisect_by<T, F>(data: &[T], x: &T, mut cmp: F) -> usize
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let mut lo = 0;
    let mut hi = data.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if cmp(&data[mid], x) == std::cmp::Ordering::Less {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

// Histogram/binning
#[derive(Debug, PartialEq)]
pub struct Bin<T> {
    pub x0: f64,
    pub x1: f64,
    pub data: Vec<T>,
}

pub fn sturges_thresholds(data: &[f64], min: f64, max: f64) -> Vec<f64> {
    if data.is_empty() { return Vec::new(); }
    let k = (data.len() as f64).log2().ceil() as usize + 1;
    let step = (max - min) / k as f64;
    (0..=k).map(|i| min + i as f64 * step).collect()
}

pub fn bin<T, V, F, G>(data: &[T], value: F, thresholds: G) -> Vec<Bin<T>>
where
    T: Clone,
    V: Into<f64>,
    F: Fn(&T) -> V,
    G: Fn(&[f64], f64, f64) -> Vec<f64>,
{
    if data.is_empty() { return Vec::new(); }
    let mut values: Vec<f64> = data.iter().map(|d| value(d).into()).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min_val = *values.first().unwrap();
    let max_val = *values.last().unwrap();
    let thresholds_vec = thresholds(&values, min_val, max_val);
    if thresholds_vec.is_empty() { return Vec::new(); }
    let mut bins: Vec<Bin<T>> = Vec::new();
    for i in 0..thresholds_vec.len() - 1 {
        bins.push(Bin { x0: thresholds_vec[i], x1: thresholds_vec[i + 1], data: Vec::new() });
    }
    for d in data.iter() {
        let val = value(d).into();
        let mut bin_index = -1;
        for (j, bin) in bins.iter().enumerate() {
            if val >= bin.x0 && (val < bin.x1 || (j == bins.len() - 1 && val == bin.x1)) {
                bin_index = j as isize;
                break;
            }
        }
        if bin_index != -1 {
            bins[bin_index as usize].data.push(d.clone());
        }
    }
    bins
}

// Blur
pub fn blur(data: &mut [f64], radius: f64) {
    if data.is_empty() || radius <= 0.0 { return; }
    let r = radius as usize;
    let n = data.len();
    for _ in 0..3 {
        let temp = data.to_vec();
        for i in 0..n {
            let mut sum = 0.0;
            let mut count = 0;
            for j in (i as isize - r as isize).max(0) as usize ..= (i + r).min(n - 1) {
                sum += temp[j];
                count += 1;
            }
            data[i] = sum / count as f64;
        }
    }
}

// Interner
pub struct Interner<T> {
    map: HashMap<T, T>,
}

impl<T> Interner<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Interner { map: HashMap::new() }
    }
    pub fn intern(&mut self, value: &T) -> &T {
        if self.map.contains_key(value) {
            self.map.get(value).unwrap()
        } else {
            self.map.insert(value.clone(), value.clone());
            self.map.get(value).unwrap()
        }
    }
}

// Tick step
pub fn tick_step(start: f64, stop: f64, count: usize) -> f64 {
    if count <= 1 || start == stop { return 0.0; }
    (stop - start) / (count as f64 - 1.0)
}

// Accurate floating-point summation (Kahan-Babuska-Neumaier)
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

// Returns the bisect index for x in a sorted array (d3.bisect)
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

// Quantile rank: returns the quantile (as a float) for a given value in a sorted array
pub fn quantile_rank<T: Into<f64> + Copy + PartialOrd>(data: &[T], x: T) -> Option<f64> {
    if data.is_empty() { return None; }
    let v: Vec<f64> = data.iter().map(|&y| y.into()).collect();
    let n = v.len() as f64;
    let mut idx = 0;
    while idx < v.len() && v[idx] < x.into() { idx += 1; }
    Some(idx as f64 / (n - 1.0))
}

// Quantile rank for sorted arrays (alias)
pub fn quantile_sorted_rank<T: Into<f64> + Copy + PartialOrd>(data: &[T], x: T) -> Option<f64> {
    quantile_rank(data, x)
}

// Quantile sorted index (inclusive)
pub fn quantile_sorted_index_inclusive<T: Into<f64> + Copy>(data: &[T], q: f64) -> Option<usize> {
    if data.is_empty() || q < 0.0 || q > 1.0 { return None; }
    let len = data.len();
    let pos = q * (len as f64 - 1.0);
    let lower = pos.floor() as usize;
    let upper = pos.ceil() as usize;
    if lower == upper {
        Some(lower)
    } else {
        let t = pos - lower as f64;
        if t < 0.5 { Some(lower) } else { Some(upper) }
    }
}

// Rice rule for histogram binning
pub fn rice_thresholds(data: &[f64], min: f64, max: f64) -> Vec<f64> {
    if data.is_empty() { return vec![min, max]; }
    let k = (2.0 * (data.len() as f64).cbrt()).ceil() as usize;
    let step = (max - min) / k as f64;
    (0..=k).map(|i| min + i as f64 * step).collect()
}

// Square-root rule for histogram binning
pub fn sqrt_thresholds(data: &[f64], min: f64, max: f64) -> Vec<f64> {
    if data.is_empty() { return vec![min, max]; }
    let k = (data.len() as f64).sqrt().ceil() as usize;
    let step = (max - min) / k as f64;
    (0..=k).map(|i| min + i as f64 * step).collect()
}
