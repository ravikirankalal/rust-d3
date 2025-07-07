//! Integration test for array_utils (group, rollup)

use rust_d3::array::{group, rollup, flat_group, fsum, Adder};
use std::collections::HashMap;

#[test]
fn test_group() {
    let data = [1, 2, 3, 4, 5, 6];
    let grouped = group(&data, |x| x % 2);
    assert_eq!(grouped[&0], vec![&2, &4, &6]);
    assert_eq!(grouped[&1], vec![&1, &3, &5]);
}

#[test]
fn test_rollup() {
    let data = [1, 2, 3, 4, 5, 6];
    let rolled = rollup(&data, |x| x % 2, |group| group.iter().copied().sum::<i32>());
    assert_eq!(rolled[&0], 12); // 2+4+6
    assert_eq!(rolled[&1], 9);  // 1+3+5
}

#[test]
fn test_flat_group() {
    let data = [1, 2, 3, 4];
    let fg = flat_group(&data, |x| x % 2);
    assert!(fg.contains(&(0, vec![&2, &4])));
    assert!(fg.contains(&(1, vec![&1, &3])));
}

#[test]
fn test_fsum() {
    let data = [1e100, 1.0, -1e100];
    let naive = data.iter().copied().sum::<f64>();
    let accurate = fsum(data);
    assert!(naive != 1.0); // naive sum is not accurate
    assert!((accurate - 1.0).abs() < 1e-12);
}

#[test]
fn test_adder() {
    let mut adder = Adder::new();
    adder.add(1e100);
    adder.add(1.0);
    adder.add(-1e100);
    assert!((adder.value() - 1.0).abs() < 1e-12);
}

#[test]
fn test_sum() {
    let data = [1, 2, 3, 4, 5];
    assert_eq!(rust_d3::array::sum(&data), 15.0);
    let empty: [i32; 0] = [];
    assert_eq!(rust_d3::array::sum(&empty), 0.0);
}

#[test]
fn test_mean() {
    let data = [2, 4, 6, 8];
    assert_eq!(rust_d3::array::mean(&data), Some(5.0));
    let empty: [i32; 0] = [];
    assert_eq!(rust_d3::array::mean(&empty), None);
}

#[test]
fn test_median() {
    let data = [1, 3, 5, 7, 9];
    assert_eq!(rust_d3::array::median(&data), Some(5.0));
    let even = [1, 2, 3, 4];
    assert_eq!(rust_d3::array::median(&even), Some(2.5));
    let empty: [i32; 0] = [];
    assert_eq!(rust_d3::array::median(&empty), None);
}

#[test]
fn test_mode() {
    let data = [1, 2, 2, 3, 4, 2, 5];
    assert_eq!(rust_d3::array::mode(&data), Some(2));
    let unique = [1, 2, 3, 4];
    assert!(rust_d3::array::mode(&unique).is_some()); // Any value is valid
    let empty: [i32; 0] = [];
    assert_eq!(rust_d3::array::mode(&empty), None);
}

#[test]
fn test_variance() {
    let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let var = rust_d3::array::variance(&data).unwrap();
    assert!((var - 4.0).abs() < 1e-6);
    let empty: [f64; 0] = [];
    assert_eq!(rust_d3::array::variance(&empty), None);
}

#[test]
fn test_deviation() {
    let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let dev = rust_d3::array::deviation(&data).unwrap();
    assert!((dev - 2.0).abs() < 1e-6);
    let empty: [f64; 0] = [];
    assert_eq!(rust_d3::array::deviation(&empty), None);
}

#[test]
fn test_pairs() {
    let data = [1, 2, 3, 4];
    let pairs = rust_d3::array::pairs(&data);
    assert_eq!(pairs, vec![(1, 2), (2, 3), (3, 4)]);
    let single = [1];
    assert!(rust_d3::array::pairs(&single).is_empty());
}

#[test]
fn test_transpose() {
    let data = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let transposed = rust_d3::array::transpose(&data);
    assert_eq!(transposed, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    let empty: Vec<Vec<i32>> = vec![];
    assert!(rust_d3::array::transpose(&empty).is_empty());
}

#[test]
fn test_zip() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let zipped = rust_d3::array::zip(&[&a, &b]);
    assert_eq!(zipped, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
    let empty: [&[i32]; 0] = [];
    assert!(rust_d3::array::zip(&empty).is_empty());
}

#[test]
fn test_least_greatest() {
    let data = [3, 1, 4, 2];
    assert_eq!(rust_d3::array::least(&data), Some(1));
    assert_eq!(rust_d3::array::greatest(&data), Some(4));
    let empty: [i32; 0] = [];
    assert_eq!(rust_d3::array::least(&empty), None);
    assert_eq!(rust_d3::array::greatest(&empty), None);
}

#[test]
fn test_shuffle_permute() {
    let mut data = [1, 2, 3, 4, 5];
    let orig = data.clone();
    rust_d3::array::shuffle(&mut data);
    // Shuffled array should have same elements as original
    let mut sorted = data.to_vec();
    sorted.sort();
    assert_eq!(sorted, orig);
    let perm = rust_d3::array::permute(&orig, &[4, 3, 2, 1, 0]);
    assert_eq!(perm, vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_ascending_descending() {
    assert_eq!(rust_d3::array::ascending(1, 2), -1);
    assert_eq!(rust_d3::array::ascending(2, 1), 1);
    assert_eq!(rust_d3::array::ascending(2, 2), 0);
    assert_eq!(rust_d3::array::descending(1, 2), 1);
    assert_eq!(rust_d3::array::descending(2, 1), -1);
    assert_eq!(rust_d3::array::descending(2, 2), 0);
}

#[test]
fn test_bisect() {
    let data = [1, 2, 4, 4, 5, 7];
    assert_eq!(rust_d3::array::bisect(&data, &4), 2);
    assert_eq!(rust_d3::array::bisect(&data, &3), 2);
    assert_eq!(rust_d3::array::bisect(&data, &8), 6);
    assert_eq!(rust_d3::array::bisect(&data, &0), 0);
}

#[test]
fn test_merge() {
    let arrays = vec![vec![1, 2], vec![3, 4], vec![5]];
    let merged = rust_d3::array::merge(&arrays);
    assert_eq!(merged, vec![1, 2, 3, 4, 5]);
    let empty: Vec<Vec<i32>> = vec![];
    assert!(rust_d3::array::merge(&empty).is_empty());
}

#[test]
fn test_cross() {
    let a = [1, 2];
    let b = ['a', 'b'];
    let cross = rust_d3::array::cross(&a, &b);
    assert_eq!(cross, vec![(1, 'a'), (1, 'b'), (2, 'a'), (2, 'b')]);
    let empty: [i32; 0] = [];
    assert!(rust_d3::array::cross(&empty, &b).is_empty());
    assert!(rust_d3::array::cross(&a, &empty).is_empty());
}