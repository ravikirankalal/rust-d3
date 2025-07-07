//! d3-array: Rust port starter template
//! Implements D3.js d3-array API: min, max, extent, sum, mean, median, etc.

/// Returns the minimum value in the slice, or None if empty.
pub fn min<T: PartialOrd + Copy>(values: &[T]) -> Option<T> {
    values.iter().cloned().min_by(|a, b| a.partial_cmp(b).unwrap())
}

/// Returns the maximum value in the slice, or None if empty.
pub fn max<T: PartialOrd + Copy>(values: &[T]) -> Option<T> {
    values.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap())
}

/// Returns the [min, max] extent of the slice, or None if empty.
pub fn extent<T: PartialOrd + Copy>(values: &[T]) -> Option<(T, T)> {
    let mut iter = values.iter().cloned();
    let first = iter.next()?;
    let (mut min, mut max) = (first, first);
    for v in iter {
        if v < min { min = v; }
        if v > max { max = v; }
    }
    Some((min, max))
}

/// Returns the sum of the slice.
pub fn sum<T>(values: &[T]) -> T
where
    T: Copy + std::iter::Sum<T>,
{
    values.iter().copied().sum()
}

/// Returns the mean (average) of the slice, or None if empty.
pub fn mean<T>(values: &[T]) -> Option<f64>
where
    T: Copy + Into<f64>,
{
    if values.is_empty() {
        None
    } else {
        Some(values.iter().copied().map(Into::into).sum::<f64>() / values.len() as f64)
    }
}

/// Returns the median of the slice, or None if empty.
pub fn median<T>(values: &[T]) -> Option<f64>
where
    T: Copy + PartialOrd + Into<f64>,
{
    let mut vals: Vec<T> = values.to_vec();
    let n = vals.len();
    if n == 0 {
        return None;
    }
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if n % 2 == 1 {
        Some(vals[n / 2].into())
    } else {
        let a = vals[n / 2 - 1].into();
        let b = vals[n / 2].into();
        Some((a + b) / 2.0)
    }
}

/// Returns the variance of the slice, or None if empty.
pub fn variance<T>(values: &[T]) -> Option<f64>
where
    T: Copy + Into<f64>,
{
    let n = values.len();
    if n == 0 {
        return None;
    }
    let mean = mean(values)?;
    let var = values.iter().copied().map(Into::into).map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
    Some(var)
}

/// Returns the standard deviation of the slice, or None if empty.
pub fn deviation<T>(values: &[T]) -> Option<f64>
where
    T: Copy + Into<f64>,
{
    variance(values).map(|v| v.sqrt())
}

/// Returns the quantile of the slice for a given p in [0, 1], or None if empty.
pub fn quantile<T>(values: &[T], p: f64) -> Option<f64>
where
    T: Copy + PartialOrd + Into<f64>,
{
    if values.is_empty() || !(0.0..=1.0).contains(&p) {
        return None;
    }
    let mut vals: Vec<T> = values.to_vec();
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = vals.len();
    let idx = p * (n as f64 - 1.0);
    let i = idx.floor() as usize;
    let frac = idx - i as f64;
    if i + 1 < n {
        let a = vals[i].into();
        let b = vals[i + 1].into();
        Some(a + (b - a) * frac)
    } else {
        Some(vals[i].into())
    }
}

/// Returns the index of the minimum value, or None if empty.
pub fn min_index<T: PartialOrd>(values: &[T]) -> Option<usize> {
    values.iter().enumerate().min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).map(|(i, _)| i)
}

/// Returns the index of the maximum value, or None if empty.
pub fn max_index<T: PartialOrd>(values: &[T]) -> Option<usize> {
    values.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).map(|(i, _)| i)
}

/// Returns a vector of evenly spaced values (like d3.range).
pub fn range(start: f64, stop: f64, step: f64) -> Vec<f64> {
    if step == 0.0 { return vec![]; }
    let mut result = vec![];
    let mut x = start;
    if step > 0.0 {
        while x < stop {
            result.push(x);
            x += step;
        }
    } else {
        while x > stop {
            result.push(x);
            x += step;
        }
    }
    result
}

/// Returns a vector of n+1 equally spaced ticks between start and stop (like d3.ticks).
pub fn ticks(start: f64, stop: f64, count: usize) -> Vec<f64> {
    if count == 0 { return vec![]; }
    let step = (stop - start) / count as f64;
    (0..=count).map(|i| start + i as f64 * step).collect()
}

/// Shuffles a mutable slice in place (Fisher-Yates shuffle).
pub fn shuffle<T>(values: &mut [T]) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rng = thread_rng();
    values.shuffle(&mut rng);
}

/// Returns the cumulative sum (cumsum) of the slice.
pub fn cumsum<T>(values: &[T]) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T> + Default,
{
    let mut result = Vec::with_capacity(values.len());
    let mut sum = T::default();
    for &v in values {
        sum = sum + v;
        result.push(sum);
    }
    result
}

/// Returns a vector of pairs of adjacent elements (like d3.pairs).
pub fn pairs<T: Copy>(values: &[T]) -> Vec<(T, T)> {
    values.windows(2).map(|w| (w[0], w[1])).collect()
}

/// Returns the transpose of a 2D vector (like d3.transpose).
pub fn transpose<T: Copy>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    if matrix.is_empty() { return vec![]; }
    let ncols = matrix[0].len();
    (0..ncols)
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect()
}

/// Returns the zipped version of multiple slices (like d3.zip).
pub fn zip<T: Copy>(vectors: &[&[T]]) -> Vec<Vec<T>> {
    if vectors.is_empty() { return vec![]; }
    let len = vectors[0].len();
    (0..len)
        .map(|i| vectors.iter().map(|v| v[i]).collect())
        .collect()
}

/// Returns the merged version of a slice of slices (like d3.merge).
pub fn merge<T: Copy>(vectors: &[&[T]]) -> Vec<T> {
    vectors.iter().flat_map(|v| v.iter().copied()).collect()
}

/// Returns the least element by comparator (like d3.least).
pub fn least<T, F>(values: &[T], mut compare: F) -> Option<&T>
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    values.iter().min_by(|a, b| compare(a, b))
}

/// Returns the greatest element by comparator (like d3.greatest).
pub fn greatest<T, F>(values: &[T], mut compare: F) -> Option<&T>
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    values.iter().max_by(|a, b| compare(a, b))
}

/// Returns the ascending comparator (like d3.ascending).
pub fn ascending<T: PartialOrd>(a: &T, b: &T) -> std::cmp::Ordering {
    a.partial_cmp(b).unwrap()
}

/// Returns the descending comparator (like d3.descending).
pub fn descending<T: PartialOrd>(a: &T, b: &T) -> std::cmp::Ordering {
    b.partial_cmp(a).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        let data = [3, 1, 4, 1, 5, 9];
        assert_eq!(min(&data), Some(1));
    }

    #[test]
    fn test_max() {
        let data = [3, 1, 4, 1, 5, 9];
        assert_eq!(max(&data), Some(9));
    }

    #[test]
    fn test_extent() {
        let data = [3, 1, 4, 1, 5, 9];
        assert_eq!(extent(&data), Some((1, 9)));
        let empty: [i32; 0] = [];
        assert_eq!(extent(&empty), None);
    }

    #[test]
    fn test_sum() {
        let data = [3, 1, 4, 1, 5, 9];
        assert_eq!(sum(&data), 23);
    }

    #[test]
    fn test_mean() {
        let data = [3.0, 1.0, 4.0, 1.0, 5.0, 9.0];
        assert_eq!(mean(&data), Some(23.0 / 6.0));
        let empty: [f64; 0] = [];
        assert_eq!(mean(&empty), None);
    }

    #[test]
    fn test_median() {
        let data = [3.0, 1.0, 4.0, 1.0, 5.0, 9.0];
        assert_eq!(median(&data), Some(3.5));
        let odd = [1.0, 2.0, 3.0];
        assert_eq!(median(&odd), Some(2.0));
        let empty: [f64; 0] = [];
        assert_eq!(median(&empty), None);
    }

    #[test]
    fn test_variance() {
        let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_eq!(variance(&data), Some(4.0));
    }

    #[test]
    fn test_deviation() {
        let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_eq!(deviation(&data), Some(2.0));
    }

    #[test]
    fn test_quantile() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(quantile(&data, 0.0), Some(1.0));
        assert_eq!(quantile(&data, 0.5), Some(3.0));
        assert_eq!(quantile(&data, 1.0), Some(5.0));
        assert_eq!(quantile(&data, 0.25), Some(2.0));
    }

    #[test]
    fn test_min_index() {
        let data = [3, 1, 4, 1, 5, 9];
        assert_eq!(min_index(&data), Some(1));
    }

    #[test]
    fn test_max_index() {
        let data = [3, 1, 4, 1, 5, 9];
        assert_eq!(max_index(&data), Some(5));
    }

    #[test]
    fn test_range() {
        fn approx_eq_vec(a: &[f64], b: &[f64], tol: f64) -> bool {
            a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < tol)
        }
        assert!(approx_eq_vec(&range(0.0, 5.0, 1.0), &[0.0, 1.0, 2.0, 3.0, 4.0], 1e-9));
        assert!(approx_eq_vec(&range(5.0, 0.0, -1.0), &[5.0, 4.0, 3.0, 2.0, 1.0], 1e-9));
        assert!(approx_eq_vec(&range(0.0, 1.0, 0.3), &[0.0, 0.3, 0.6, 0.9], 1e-9));
    }

    #[test]
    fn test_ticks() {
        let t = ticks(0.0, 10.0, 5);
        assert_eq!(t, vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);
    }

    #[test]
    fn test_shuffle() {
        let mut data = [1, 2, 3, 4, 5];
        let orig = data.clone();
        shuffle(&mut data);
        // The shuffled array should have the same elements as the original
        let mut orig_sorted = orig.to_vec();
        let mut data_sorted = data.to_vec();
        orig_sorted.sort();
        data_sorted.sort();
        assert_eq!(orig_sorted, data_sorted);
    }

    #[test]
    fn test_cumsum() {
        let data = [1, 2, 3, 4];
        assert_eq!(cumsum(&data), vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_pairs() {
        let data = [1, 2, 3, 4];
        assert_eq!(pairs(&data), vec![(1, 2), (2, 3), (3, 4)]);
    }

    #[test]
    fn test_transpose() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(transpose(&matrix), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }

    #[test]
    fn test_zip() {
        let a = [1, 2, 3];
        let b = [4, 5, 6];
        let zipped = zip(&[&a, &b]);
        assert_eq!(zipped, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    }

    #[test]
    fn test_merge() {
        let a = [1, 2];
        let b = [3, 4];
        assert_eq!(merge(&[&a, &b]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_least() {
        let data = [3, 1, 4, 1, 5, 9];
        assert_eq!(least(&data, ascending), Some(&1));
    }

    #[test]
    fn test_greatest() {
        let data = [3, 1, 4, 1, 5, 9];
        assert_eq!(greatest(&data, ascending), Some(&9));
    }

    #[test]
    fn test_ascending_descending() {
        assert_eq!(ascending(&1, &2), std::cmp::Ordering::Less);
        assert_eq!(descending(&1, &2), std::cmp::Ordering::Greater);
    }
}
