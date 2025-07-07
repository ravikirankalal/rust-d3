//! Tests for d3-collection utilities
use rust_d3::collection::{group, rollup, keys, values, entries};
use std::collections::HashMap;

#[test]
fn test_collection_group() {
    let data = vec![1, 2, 3, 4, 5, 6];
    let grouped = group(data.clone(), |x| x % 2);
    assert_eq!(grouped[&0], vec![2, 4, 6]);
    assert_eq!(grouped[&1], vec![1, 3, 5]);
}

#[test]
fn test_collection_rollup() {
    let data = vec![1, 2, 3, 4, 5, 6];
    let rolled = rollup(data.clone(), |x| x % 2, |group| group.iter().sum::<i32>());
    assert_eq!(rolled[&0], 12); // 2+4+6
    assert_eq!(rolled[&1], 9);  // 1+3+5
}

#[test]
fn test_keys_values_entries() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    let mut k = keys(&map);
    k.sort();
    assert_eq!(k, vec!["a", "b"]);
    let mut v = values(&map);
    v.sort();
    assert_eq!(v, vec![1, 2]);
    let mut e = entries(&map);
    e.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(e, vec![("a", 1), ("b", 2)]);
}
