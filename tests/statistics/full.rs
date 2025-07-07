//! Unit tests for d3 statistics (re-exported)
use rust_d3::statistics::{mean, sum, median, variance};

#[test]
fn test_mean() {
    let data = [1.0, 2.0, 3.0, 4.0];
    assert_eq!(mean(&data), Some(2.5));
    let empty: [f64; 0] = [];
    assert_eq!(mean(&empty), None);
}

#[test]
fn test_sum() {
    let data = [1.0, 2.0, 3.0];
    assert_eq!(sum(&data), Some(6.0));
    let empty: [f64; 0] = [];
    assert_eq!(sum(&empty), None);
}

#[test]
fn test_median() {
    let data = [1, 2, 3, 4];
    assert_eq!(median(&data), Some(2.5));
    let odd = [1, 2, 3];
    assert_eq!(median(&odd), Some(2.0));
    let empty: [i32; 0] = [];
    assert_eq!(median(&empty), None);
}

#[test]
fn test_variance() {
    let data = [1.0, 2.0, 3.0, 4.0];
    assert!((variance(&data).unwrap() - 1.6666667).abs() < 1e-6);
    let empty: [f64; 0] = [];
    assert_eq!(variance(&empty), None);
}
