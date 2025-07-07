//! Unit test for d3 collection_adv advanced utilities
use rust_d3::collection_adv::*;
use std::collections::HashMap;

#[test]
fn test_group() {
    let data = vec!["a", "bb", "c", "dd", "eee"];
    let grouped = group(data.clone(), |s| s.len());
    assert_eq!(grouped.get(&1).unwrap(), &vec!["a", "c"]);
    assert_eq!(grouped.get(&2).unwrap(), &vec!["bb", "dd"]);
    assert_eq!(grouped.get(&3).unwrap(), &vec!["eee"]);
}

#[test]
fn test_rollup() {
    let data = vec!["a", "bb", "c", "dd", "eee"];
    let rolled = rollup(data.clone(), |s| s.len(), |v| v.len());
    assert_eq!(rolled.get(&1), Some(&2));
    assert_eq!(rolled.get(&2), Some(&2));
    assert_eq!(rolled.get(&3), Some(&1));
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
