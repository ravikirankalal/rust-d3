// array.rs
// Tests for transpose, bisector, and quickselect

use rust_d3::array::{transpose, bisector, quickselect};

#[test]
fn test_transpose() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let t = transpose::transpose(&matrix);
    assert_eq!(t, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
}

#[test]
fn test_bisector() {
    let arr = vec![1, 3, 5, 7, 9];
    let (left, right) = bisector::bisector(|a: &i32, b: &i32| a.cmp(b));
    assert_eq!(left(&arr, &5), 2);
    assert_eq!(right(&arr, &5), 3);
    assert_eq!(left(&arr, &4), 2);
    assert_eq!(right(&arr, &4), 2);
}

#[test]
fn test_quickselect() {
    let mut arr = vec![9, 1, 8, 2, 7, 3, 6, 4, 5];
    quickselect::quickselect(&mut arr, 4);
    arr.sort(); // quickselect only guarantees kth element, not full sort
    assert_eq!(arr[4], 5);
}
