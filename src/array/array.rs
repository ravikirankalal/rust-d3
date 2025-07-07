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
    if count < 2 || start == stop {
        return vec![start];
    }
    let step = (stop - start) / (count as f64 - 1.0);
    (0..count).map(|i| start + i as f64 * step).collect()
}

/// TODO: Implement d3-array missing functions for full parity
/// See: https://github.com/d3/d3-array

/// Returns the index of the minimum value
pub fn min_index<T: Ord + Copy>(data: &[T]) -> Option<usize> {
    data.iter().enumerate().min_by_key(|&(_, v)| v).map(|(i, _)| i)
}

/// Returns the index of the maximum value
pub fn max_index<T: Ord + Copy>(data: &[T]) -> Option<usize> {
    data.iter().enumerate().max_by_key(|&(_, v)| v).map(|(i, _)| i)
}

/// Returns the mode (most common value)
pub fn mode<T: Ord + Copy>(_data: &[T]) -> Option<T> {
    // TODO: Implement
    None
}

/// Returns the deviation (standard deviation)
pub fn deviation<T: Into<f64> + Copy>(_data: &[T]) -> Option<f64> {
    // TODO: Implement
    None
}

/// Returns the least element
pub fn least<T: Ord + Copy>(_data: &[T]) -> Option<T> {
    // TODO: Implement
    None
}

/// Returns the greatest element
pub fn greatest<T: Ord + Copy>(_data: &[T]) -> Option<T> {
    // TODO: Implement
    None
}

/// Returns the pairs of adjacent elements
pub fn pairs<T: Copy>(_data: &[T]) -> Vec<(T, T)> {
    // TODO: Implement
    vec![]
}

/// Returns the transpose of a 2D array
pub fn transpose<T: Copy>(_data: &[Vec<T>]) -> Vec<Vec<T>> {
    // TODO: Implement
    vec![]
}

/// Returns the zip of multiple arrays
pub fn zip<T: Copy>(_arrays: &[&[T]]) -> Vec<Vec<T>> {
    // TODO: Implement
    vec![]
}

/// Returns the merge of multiple arrays
pub fn merge<T: Copy>(_arrays: &[&[T]]) -> Vec<T> {
    // TODO: Implement
    vec![]
}

/// Returns the shuffled array
pub fn shuffle<T: Copy>(_data: &mut [T]) {
    // TODO: Implement
}

/// Returns the permuted array
pub fn permute<T: Copy>(_data: &[T], _indexes: &[usize]) -> Vec<T> {
    // TODO: Implement
    vec![]
}

/// Returns the ascending order of two values
pub fn ascending<T: Ord>(_a: T, _b: T) -> i32 {
    // TODO: Implement
    0
}

/// Returns the descending order of two values
pub fn descending<T: Ord>(_a: T, _b: T) -> i32 {
    // TODO: Implement
    0
}
