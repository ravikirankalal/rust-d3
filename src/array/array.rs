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
    data.shuffle(&mut rand::thread_rng());
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