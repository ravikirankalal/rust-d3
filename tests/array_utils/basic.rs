//! Unit tests for d3 array_utils (group, rollup)

use rust_d3::array_utils::{group, rollup};
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
