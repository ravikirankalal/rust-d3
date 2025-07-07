//! Unit tests for d3 extent
use rust_d3::extent::extent;

#[test]
fn test_extent() {
    let data = [3, 1, 4, 1, 5, 9];
    assert_eq!(extent(&data), Some((1, 9)));
    let empty: [i32; 0] = [];
    assert_eq!(extent(&empty), None);
}
