use rust_d3::array_utils::{scan, least, greatest};

#[test]
fn test_scan() {
    let data = [3, 1, 4, 1, 5, 9];
    let idx = scan(&data, |a, b| a.cmp(b));
    assert_eq!(idx, Some(1)); // first minimum
}

#[test]
fn test_least() {
    let data = [3, 1, 4, 1, 5, 9];
    let val = least(&data, |a, b| a.cmp(b));
    assert_eq!(val, Some(&1));
}

#[test]
fn test_greatest() {
    let data = [3, 1, 4, 1, 5, 9];
    let val = greatest(&data, |a, b| a.cmp(b));
    assert_eq!(val, Some(&9));
}
