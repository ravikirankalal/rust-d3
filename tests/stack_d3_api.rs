//! Unit tests for d3-stack D3.js API methods
use rust_d3::stack::{stack_order_none, stack_offset_none};

#[test]
fn test_stack_order_none() {
    // Should not panic and should be callable
    stack_order_none::<f64>();
}

#[test]
fn test_stack_offset_none() {
    // Should not panic and should be callable
    stack_offset_none::<f64>();
}
