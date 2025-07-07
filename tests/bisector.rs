//! Unit tests for d3 bisector
use rust_d3::bisector::{bisect_left, bisect_right, Bisector};

#[test]
fn test_bisect_left() {
    let arr = [1, 2, 4, 4, 5, 7];
    assert_eq!(bisect_left(&arr, &4), 2);
    assert_eq!(bisect_left(&arr, &6), 5);
    assert_eq!(bisect_left(&arr, &0), 0);
}

#[test]
fn test_bisect_right() {
    let arr = [1, 2, 4, 4, 5, 7];
    assert_eq!(bisect_right(&arr, &4), 4);
    assert_eq!(bisect_right(&arr, &6), 5);
    assert_eq!(bisect_right(&arr, &0), 0);
}

#[test]
fn test_bisector_left_right_center_with_accessor() {
    let arr = [(1, "a"), (2, "b"), (4, "c"), (4, "d"), (5, "e")];
    let accessor = |x: &(i32, &str)| x.0;
    assert_eq!(Bisector::left(&arr, &4, accessor), 2);
    assert_eq!(Bisector::right(&arr, &4, accessor), 4);
    assert_eq!(Bisector::center(&arr, &4, accessor), 3);
}
