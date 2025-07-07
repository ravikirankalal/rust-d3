//! Unit tests for d3 array utilities

use rust_d3::array::{min, max, extent, quantile, cumsum, range, ticks};

#[test]
fn test_min() {
    let data = [3, 1, 4, 1, 5, 9];
    assert_eq!(min(&data), Some(1));
    let empty: [i32; 0] = [];
    assert_eq!(min(&empty), None);
}

#[test]
fn test_max() {
    let data = [3, 1, 4, 1, 5, 9];
    assert_eq!(max(&data), Some(9));
    let empty: [i32; 0] = [];
    assert_eq!(max(&empty), None);
}

#[test]
fn test_extent() {
    let data = [3, 1, 4, 1, 5, 9];
    assert_eq!(extent(&data), Some((1, 9)));
    let empty: [i32; 0] = [];
    assert_eq!(extent(&empty), None);
}

#[test]
fn test_quantile() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(quantile(&data, 0.0), Some(1.0));
    assert_eq!(quantile(&data, 0.5), Some(3.0));
    assert_eq!(quantile(&data, 1.0), Some(5.0));
    assert_eq!(quantile(&[], 0.5), None);
}

#[test]
fn test_cumsum() {
    let data = [1, 2, 3];
    let result = cumsum(&data);
    assert_eq!(result, vec![1.0, 3.0, 6.0]);
}

#[test]
fn test_range() {
    let r = range(0.0, 1.0, 0.2);
    assert_eq!(r, vec![0.0, 0.2, 0.4, 0.6, 0.8]);
}

#[test]
fn test_ticks() {
    let t = ticks((0.0, 1.0), 5);
    assert_eq!(t, vec![0.0, 0.25, 0.5, 0.75, 1.0]);
    let t2 = ticks((1.0, 1.0), 5);
    assert_eq!(t2, vec![1.0]);
}
