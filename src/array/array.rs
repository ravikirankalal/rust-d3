// Array utilities implementation

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

// Returns the quantile of a sorted slice.
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

// Returns the cumulative sum of a slice.
pub fn cumsum<T: Into<f64> + Copy>(data: &[T]) -> Vec<f64> {
    let mut sum = 0.0;
    data.iter().map(|&x| {
        sum += x.into();
        sum
    }).collect()
}

// Returns a range of values (start, stop, step).
pub fn range(start: f64, stop: f64, step: f64) -> Vec<f64> {
    let mut v = Vec::new();
    let mut x = start;
    while x < stop {
        v.push(x);
        x += step;
    }
    v
}

// Returns tick values for a domain and count.
pub fn ticks(domain: (f64, f64), count: usize) -> Vec<f64> {
    let (start, stop) = domain;
    if count == 0 { return vec![]; }
    if count == 1 { return vec![start]; }
    if start == stop { return vec![start]; }
    let step = (stop - start) / (count as f64 - 1.0);
    (0..count).map(|i| start + i as f64 * step).collect()
}

/// Returns the index of the minimum value
pub fn min_index<T: Ord + Copy>(data: &[T]) -> Option<usize> {
    data.iter().enumerate().min_by_key(|&(_, v)| v).map(|(i, _)| i)
}

/// Returns the index of the maximum value
pub fn max_index<T: Ord + Copy>(data: &[T]) -> Option<usize> {
    data.iter().enumerate().max_by_key(|&(_, v)| v).map(|(i, _)| i)
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

/// Returns the deviation (standard deviation)
pub fn deviation<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    variance(data).map(|v| v.sqrt())
}

/// Returns the least element
pub fn least<T: Ord + Copy>(data: &[T]) -> Option<T> {
    data.iter().copied().min()
}

/// Returns the greatest element
pub fn greatest<T: Ord + Copy>(data: &[T]) -> Option<T> {
    data.iter().copied().max()
}

/// Returns the pairs of adjacent elements
pub fn pairs<T: Copy>(data: &[T]) -> Vec<(T, T)> {
    data.windows(2).map(|w| (w[0], w[1])).collect()
}

/// Returns the transpose of a 2D array
pub fn transpose<T: Copy>(data: &[Vec<T>]) -> Vec<Vec<T>> {
    if data.is_empty() { return vec![]; }
    let len = data[0].len();
    (0..len)
        .map(|i| data.iter().map(|row| row[i]).collect())
        .collect()
}

/// Returns the zip of multiple arrays
pub fn zip<T: Copy>(arrays: &[&[T]]) -> Vec<Vec<T>> {
    if arrays.is_empty() { return vec![]; }
    let len = arrays[0].len();
    (0..len)
        .map(|i| arrays.iter().map(|arr| arr[i]).collect())
        .collect()
}

/// Returns the merge of multiple arrays
pub fn merge<T: Clone>(arrays: &[Vec<T>]) -> Vec<T> {
    arrays.iter().flat_map(|v| v.clone()).collect()
}

/// Returns the shuffled array
pub fn shuffle<T>(data: &mut [T]) {
    use rand::seq::SliceRandom;
    data.shuffle(&mut rand::rngs::ThreadRng::default());
}

/// Returns the permuted array
pub fn permute<T: Copy>(data: &[T], indexes: &[usize]) -> Vec<T> {
    indexes.iter().map(|&i| data[i]).collect()
}

/// Returns the ascending order of two values
pub fn ascending<T: Ord>(a: T, b: T) -> i32 {
    a.cmp(&b) as i32
}

/// Returns the descending order of two values
pub fn descending<T: Ord>(a: T, b: T) -> i32 {
    b.cmp(&a) as i32
}

/// Returns the sum of the array (d3.sum)
pub fn sum<T: Into<f64> + Copy>(data: &[T]) -> f64 {
    data.iter().map(|&x| x.into()).sum()
}

/// Returns the mean of the array (d3.mean)
pub fn mean<T: Into<f64> + Copy>(data: &[T]) -> Option<f64> {
    if data.is_empty() { None } else { Some(sum(data) / data.len() as f64) }
}

/// Returns the median of the array (d3.median)
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

/// Represents a single bin in a histogram.
#[derive(Debug, PartialEq)]
pub struct Bin<T> {
    pub x0: f64,
    pub x1: f64,
    pub data: Vec<T>,
}

/// Generates thresholds for binning using Sturges' formula.
pub fn sturges_thresholds(data: &[f64], min: f64, max: f64) -> Vec<f64> {
    if data.is_empty() {
        return Vec::new();
    }
    let k = (data.len() as f64).log2().ceil() as usize + 1;
    let step = (max - min) / k as f64;
    (0..=k).map(|i| min + i as f64 * step).collect()
}

/// Computes the histogram (bins) for a given dataset.
///
/// # Arguments
/// * `data` - The input data.
/// * `value` - A function to extract the numeric value from each data point.
/// * `thresholds` - A function that generates the bin thresholds.
pub fn bin<T, V, F, G>(data: &[T], value: F, thresholds: G) -> Vec<Bin<T>>
where
    T: Clone,
    V: Into<f64>,
    F: Fn(&T) -> V,
    G: Fn(&[f64], f64, f64) -> Vec<f64>,
{
    if data.is_empty() {
        return Vec::new();
    }

    let mut values: Vec<f64> = data.iter().map(|d| value(d).into()).collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min_val = *values.first().unwrap();
    let max_val = *values.last().unwrap();

    let thresholds_vec = thresholds(&values, min_val, max_val);
    if thresholds_vec.is_empty() {
        return Vec::new();
    }

    let mut bins: Vec<Bin<T>> = Vec::new();
    for i in 0..thresholds_vec.len() - 1 {
        bins.push(Bin {
            x0: thresholds_vec[i],
            x1: thresholds_vec[i + 1],
            data: Vec::new(),
        });
    }

    for (_i, d) in data.iter().enumerate() {
        let val = value(d).into();
        let mut bin_index = -1;

        // Find the correct bin for the value
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

/// Applies a 1D blur to a mutable slice of f64 values.
/// This approximates a Gaussian blur by applying three iterations of a moving average (box filter).
///
/// # Arguments
/// * `data` - The mutable slice of f64 values to blur.
/// * `radius` - The blur radius. A larger radius means more blur.
pub fn blur(data: &mut [f64], radius: f64) {
    if data.is_empty() || radius <= 0.0 { return; }

    let r = radius as usize;
    let n = data.len();

    // Three passes of a box filter approximate a Gaussian blur
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

use std::collections::HashMap;
use std::hash::Hash;

/// A simple interner for values that are `Clone + Eq + Hash`.
pub struct Interner<T> {
    map: HashMap<T, T>,
}

impl<T> Interner<T>
where
    T: Clone + Eq + Hash,
{
    /// Creates a new, empty `Interner`.
    pub fn new() -> Self {
        Interner {
            map: HashMap::new(),
        }
    }

    /// Interns a value, returning a reference to the canonical instance.
    pub fn intern(&mut self, value: &T) -> &T {
        if self.map.contains_key(value) {
            self.map.get(value).unwrap()
        } else {
            self.map.insert(value.clone(), value.clone());
            self.map.get(value).unwrap()
        }
    }
}

use std::collections::HashSet;

/// Returns the union of two slices as a Vec (set union, unique values).
pub fn union<T: Clone + Eq + Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let set: HashSet<_> = a.iter().cloned().chain(b.iter().cloned()).collect();
    set.into_iter().collect()
}

/// Returns the intersection of two slices as a Vec (set intersection).
pub fn intersection<T: Clone + Eq + Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let set_a: HashSet<_> = a.iter().cloned().collect();
    let set_b: HashSet<_> = b.iter().cloned().collect();
    set_a.intersection(&set_b).cloned().collect()
}

/// Returns the difference of two slices as a Vec (set difference: a - b).
pub fn difference<T: Clone + Eq + Hash>(a: &[T], b: &[T]) -> Vec<T> {
    let set_a: HashSet<_> = a.iter().cloned().collect();
    let set_b: HashSet<_> = b.iter().cloned().collect();
    set_a.difference(&set_b).cloned().collect()
}

/// Returns the cartesian product (cross) of two slices.
pub fn cross<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    let mut result = Vec::new();
    for x in a {
        for y in b {
            result.push((x.clone(), y.clone()));
        }
    }
    result
}

/// Returns the index where to insert item x in a sorted slice to maintain order (bisect_left).
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

/// Returns the index where to insert item x in a sorted slice to maintain order (bisect_right).
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

/// Returns the index where to insert item x in a sorted slice using a custom comparator (bisect_by).
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

/// High-precision floating point sum (Kahan summation).
pub fn fsum(data: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0;
    for &x in data {
        let y = x - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}

/// Returns the step size between ticks for a domain and count (tickStep).
pub fn tick_step(start: f64, stop: f64, count: usize) -> f64 {
    if count == 0 { return 0.0; }
    (stop - start) / (count as f64 - 1.0)
}