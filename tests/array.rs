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