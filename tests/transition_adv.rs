//! Tests for transition_adv module
use rust_d3::transition_adv::{transition_filter, transition_tween, TransitionAdvanced};

#[test]
fn test_transition_tween() {
    let mut values = Vec::new();
    transition_tween(0.0, 1.0, 32, |v| values.push(v));
    assert!(values.first().unwrap().abs() < 1e-6);
    assert!((values.last().unwrap() - 1.0).abs() < 1e-6);
    assert!(values.len() > 1);
}

#[test]
fn test_transition_filter_basic() {
    let data = vec![1, 2, 3, 4, 5];
    let filtered = transition_filter(&data, |&x| x % 2 == 0);
    assert_eq!(filtered, vec![2, 4]);
}

#[test]
fn test_transition_filter_empty() {
    let data: Vec<i32> = vec![];
    let filtered = transition_filter(&data, |_| true);
    assert!(filtered.is_empty());
}

#[test]
fn test_transition_advanced_chain() {
    let data = vec![1, 2, 3, 4, 5];
    let result = data
        .chain(|v| v.into_iter().filter(|&x| x > 2).collect())
        .chain(|v| v.into_iter().map(|x| x * 10).collect());
    assert_eq!(result, vec![30, 40, 50]);
}
