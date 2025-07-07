//! Integration test for array_utils (group, rollup)

use rust_d3::array_utils::{group, rollup, flat_group, flat_rollup, index, flat_index};
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
fn test_flat_rollup() {
    let data = [1, 2, 3, 4];
    let fr = flat_rollup(&data, |x| x % 2, |group| group.iter().copied().sum::<i32>());
    assert!(fr.contains(&(0, 6))); // 2+4
    assert!(fr.contains(&(1, 4))); // 1+3
}

#[test]
fn test_index() {
    let data = [10, 20, 30];
    let idx = index(&data, |x| x / 10);
    assert_eq!(idx[&1], &10);
    assert_eq!(idx[&2], &20);
    assert_eq!(idx[&3], &30);
}

#[test]
fn test_flat_index() {
    let data = [10, 20, 30];
    let fidx = flat_index(&data, |x| x / 10);
    assert!(fidx.contains(&(1, &10)));
    assert!(fidx.contains(&(2, &20)));
    assert!(fidx.contains(&(3, &30)));
}
