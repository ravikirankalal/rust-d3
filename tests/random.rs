//! Unit tests for d3 random
use rust_d3::random::{random_uniform, random_normal, random_exponential, random_bernoulli, random_bates, random_irwin_hall, random_log_normal};

#[test]
fn test_random_uniform() {
    let v = random_uniform(0.0, 1.0);
    assert!(v >= 0.0 && v < 1.0);
}

#[test]
fn test_random_normal() {
    let v = random_normal(0.0, 1.0);
    // Should be a real number, not NaN
    assert!(v.is_finite());
}

#[test]
fn test_random_exponential() {
    let v = random_exponential(1.0);
    assert!(v >= 0.0);
}

#[test]
fn test_random_bernoulli() {
    let mut true_found = false;
    let mut false_found = false;
    for _ in 0..100 {
        let b = random_bernoulli(0.5);
        if b { true_found = true; } else { false_found = true; }
    }
    assert!(true_found && false_found);
}

#[test]
fn test_random_bates() {
    let v = random_bates(10);
    assert!(v >= 0.0 && v <= 1.0);
}

#[test]
fn test_random_irwin_hall() {
    let v = random_irwin_hall(10);
    assert!(v >= 0.0 && v <= 10.0);
}

#[test]
fn test_random_log_normal() {
    let v = random_log_normal(0.0, 1.0);
    assert!(v > 0.0);
}
