//! Comprehensive tests for d3 Selection
use rust_d3::selection::Selection;

#[test]
fn test_map_filter() {
    let sel = Selection::new(vec![1, 2, 3]);
    let mapped = sel.map(|x| x * 2);
    assert_eq!(mapped.data(), &vec![2, 4, 6]);
    let filtered = sel.filter(|x| *x > 1);
    assert_eq!(filtered.data(), &vec![2, 3]);
}

#[test]
fn test_select_select_all() {
    let sel = Selection::new(vec![1, 2, 3, 4]);
    let s = sel.select(|x| *x == 2);
    assert_eq!(s.data(), &vec![2]);
    let all = sel.select_all(|x| *x % 2 == 0);
    assert_eq!(all.data(), &vec![2, 4]);
}

#[test]
fn test_size_empty_nodes_node() {
    let sel = Selection::new(vec![1, 2, 3]);
    assert_eq!(sel.size(), 3);
    assert!(!sel.empty());
    assert_eq!(sel.nodes(), vec![&1, &2, &3]);
    assert_eq!(sel.node(), Some(&1));
    let empty = Selection::<i32>::new(vec![]);
    assert!(empty.empty());
    assert_eq!(empty.node(), None);
}

#[test]
fn test_call_each() {
    let sel = Selection::new(vec![10, 20, 30]);
    let mut sum = 0;
    sel.each(|x, _i| sum += x);
    assert_eq!(sum, 60);
    let mut called = false;
    sel.call(|s| {
        assert_eq!(s.size(), 3);
        called = true;
    });
    assert!(called);
}

#[test]
fn test_sort_by_merge() {
    let sel = Selection::new(vec![3, 1, 2]);
    let sorted = sel.sort_by(|a, b| a.cmp(b));
    assert_eq!(sorted.data(), &vec![1, 2, 3]);
    let other = Selection::new(vec![4, 5]);
    let merged = sel.merge(&other);
    assert_eq!(merged.data(), &vec![3, 1, 2, 4, 5]);
}
