//! Unit tests for d3 OrdinalScale
use rust_d3::scale::OrdinalScale;

#[test]
fn test_ordinal_scale() {
    let scale = OrdinalScale::new(vec!["a", "b", "c"], vec![1, 2, 3]);
    assert_eq!(scale.scale(&"a"), Some(1));
    assert_eq!(scale.scale(&"b"), Some(2));
    assert_eq!(scale.scale(&"c"), Some(3));
    assert_eq!(scale.scale(&"d"), None);
}
