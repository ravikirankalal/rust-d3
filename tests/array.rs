//! Tests for d3 array utilities

use rust_d3::array::{min, max, extent, quantile, cumsum, range, ticks, min_index, max_index};

#[test]
fn test_min_max_extent() {
    let data = [3, 1, 4, 1, 5, 9];
    assert_eq!(min(&data), Some(1));
    assert_eq!(max(&data), Some(9));
    assert_eq!(extent(&data), Some((1, 9)));
    let empty: [i32; 0] = [];
    assert_eq!(min(&empty), None);
    assert_eq!(max(&empty), None);
    assert_eq!(extent(&empty), None);
    let single = [42];
    assert_eq!(min(&single), Some(42));
    assert_eq!(max(&single), Some(42));
    assert_eq!(extent(&single), Some((42, 42)));
}

#[test]
fn test_quantile() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(quantile(&data, 0.0), Some(1.0));
    assert_eq!(quantile(&data, 0.5), Some(3.0));
    assert_eq!(quantile(&data, 1.0), Some(5.0));
    assert_eq!(quantile(&data, -0.1), None);
    assert_eq!(quantile(&data, 1.1), None);
    let empty: [f64; 0] = [];
    assert_eq!(quantile(&empty, 0.5), None);
    let unsorted = [5.0, 1.0, 3.0];
    assert_eq!(quantile(&unsorted, 0.5), Some(3.0));
}

#[test]
fn test_cumsum() {
    let data = [1, 2, 3];
    assert_eq!(cumsum(&data), vec![1.0, 3.0, 6.0]);
    let empty: [i32; 0] = [];
    assert_eq!(cumsum(&empty), Vec::<f64>::new());
    let floats = [0.5, 1.5, 2.0];
    assert_eq!(cumsum(&floats), vec![0.5, 2.0, 4.0]);
}

fn approx_eq_vec(a: &[f64], b: &[f64], eps: f64) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < eps)
}

#[test]
fn test_range() {
    let r = range(0.0, 1.0, 0.2);
    assert!(approx_eq_vec(&r, &[0.0, 0.2, 0.4, 0.6, 0.8], 1e-9));
    assert_eq!(range(1.0, 1.0, 0.1), Vec::<f64>::new());
    assert_eq!(range(0.0, 1.0, 2.0), vec![0.0]);
}

#[test]
fn test_ticks() {
    assert_eq!(ticks((0.0, 1.0), 5), vec![0.0, 0.25, 0.5, 0.75, 1.0]);
    assert_eq!(ticks((1.0, 1.0), 5), vec![1.0]);
    assert_eq!(ticks((0.0, 1.0), 1), vec![0.0]);
}

#[test]
fn test_min_max_index() {
    let data = [3, 1, 4, 1, 5, 9];
    assert_eq!(min_index(&data), Some(1)); // first 1
    assert_eq!(max_index(&data), Some(5)); // 9
    let empty: [i32; 0] = [];
    assert_eq!(min_index(&empty), None);
    assert_eq!(max_index(&empty), None);
    let single = [42];
    assert_eq!(min_index(&single), Some(0));
    assert_eq!(max_index(&single), Some(0));
}
