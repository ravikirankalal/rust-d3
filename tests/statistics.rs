//! Unit tests for d3 statistics (re-exported)
use rust_d3::statistics::{mean, sum, median, variance};
use rust_d3::statistics::{deviation, quantile, cumsum, scan};

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

#[test]
fn test_deviation() {
    let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let dev = deviation(&data).unwrap();
    println!("Deviation: {}", dev);
    // Sample standard deviation should be approximately 2.138089935299395
    assert!((dev - 2.138089935299395).abs() < 1e-6);
    let empty: [f64; 0] = [];
    assert_eq!(deviation(&empty), None);
    let single = [42.0];
    assert_eq!(deviation(&single), None);
}

#[test]
fn test_quantile() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(quantile(&data, 0.0), Some(1.0));
    assert_eq!(quantile(&data, 0.25), Some(2.0));
    assert_eq!(quantile(&data, 0.5), Some(3.0));
    assert_eq!(quantile(&data, 0.75), Some(4.0));
    assert_eq!(quantile(&data, 1.0), Some(5.0));
    let empty: [f64; 0] = [];
    assert_eq!(quantile(&empty, 0.5), None);
    assert_eq!(quantile(&data, -0.1), None);
    assert_eq!(quantile(&data, 1.1), None);
}

#[test]
fn test_cumsum() {
    let data = [1, 2, 3, 4];
    assert_eq!(cumsum(&data), vec![1.0, 3.0, 6.0, 10.0]);
    let data = [0.5, 1.5, 2.0];
    assert_eq!(cumsum(&data), vec![0.5, 2.0, 4.0]);
    let empty: [f64; 0] = [];
    assert_eq!(cumsum(&empty), Vec::<f64>::new());
}

#[test]
fn test_scan() {
    let data = [3, 1, 4, 1, 5, 9];
    assert_eq!(scan(&data), Some(1)); // first minimum index
    let data = [10, 20, 30];
    assert_eq!(scan(&data), Some(0));
    let empty: [i32; 0] = [];
    assert_eq!(scan(&empty), None);
}

#[test]
fn test_statistics_edge_cases() {
    // Single element
    let data_f = [42.0];
    assert_eq!(mean(&data_f), Some(42.0));
    assert_eq!(sum(&data_f), Some(42.0));
    assert_eq!(variance(&data_f), None); // Not enough data for variance
    let data_i = [42];
    assert_eq!(median(&data_i), Some(42.0));
    // Negative and mixed values
    let data_f = [-1.0, 0.0, 1.0];
    assert_eq!(mean(&data_f), Some(0.0));
    assert_eq!(sum(&data_f), Some(0.0));
    let data_i = [-1, 0, 1];
    assert_eq!(median(&data_i), Some(0.0));
    // Repeated values
    let data_f = [2.0, 2.0, 2.0];
    assert_eq!(mean(&data_f), Some(2.0));
    assert_eq!(variance(&data_f), Some(0.0));
    let data_i = [2, 2, 2];
    assert_eq!(median(&data_i), Some(2.0));
    // Integer input
    let data = [1, 2, 3, 4];
    assert_eq!(mean(&data), Some(2.5));
    assert_eq!(sum(&data), Some(10.0));
    assert_eq!(median(&data), Some(2.5));
    assert!((variance(&data).unwrap() - 1.6666667).abs() < 1e-6);
}

#[test]
fn test_statistics_nan_and_inf() {
    let data = [1.0, std::f64::NAN, 3.0];
    // mean and sum should ignore NaN, but our implementation will propagate it
    assert!(mean(&data).unwrap().is_nan());
    assert!(sum(&data).unwrap().is_nan());
    let data = [1.0, std::f64::INFINITY, 3.0];
    assert!(mean(&data).unwrap().is_infinite());
    assert!(sum(&data).unwrap().is_infinite());
    let data = [std::f64::INFINITY, std::f64::NEG_INFINITY];
    assert!(variance(&data).unwrap().is_nan());
}

#[test]
fn test_statistics_all_elements_equal() {
    let data = [7, 7, 7, 7];
    assert_eq!(mean(&data), Some(7.0));
    assert_eq!(variance(&data), Some(0.0));
    assert_eq!(median(&data), Some(7.0));
}

#[test]
fn test_statistics_negative_and_floating_edge_cases() {
    let data = [-1, -2, -3, -4];
    assert_eq!(mean(&data), Some(-2.5));
    assert_eq!(median(&data), Some(-2.5));
    assert!((variance(&data).unwrap() - 1.6666667).abs() < 1e-6);
    let data = [1e-10, 2e-10, 3e-10, 4e-10];
    assert!((mean(&data).unwrap() - 2.5e-10).abs() < 1e-20);
    assert!((variance(&data).unwrap() - 1.6666667e-20).abs() < 1e-26);
}
