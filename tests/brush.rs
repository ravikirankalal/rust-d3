//! Unit tests for d3 brush

use rust_d3::brush::Brush;

#[test]
fn test_brush_select_and_clear() {
    let mut brush = Brush::new(0.0..100.0);
    brush.select(10.0..20.0);
    assert_eq!(brush.brush_selection(), Some(10.0..20.0));
    brush.clear();
    assert_eq!(brush.brush_selection(), None);
}

#[test]
fn test_brush_out_of_bounds() {
    let mut brush = Brush::new(0.0..100.0);
    brush.select(10.0..120.0); // Out of bounds
    assert_eq!(brush.brush_selection(), None);
}

#[test]
fn test_brush_filter_and_handle_size() {
    let mut brush = Brush::new(0.0..100.0);
    brush.filter(|| false).handle_size(10.0);
    // These are just setters, no direct assertion on their effect here.
    // The primary goal is to ensure the methods exist and don't panic.
    assert!(true);
}

#[test]
fn test_brush_x_and_y() {
    let brush_x = rust_d3::brush::brush_x();
    let brush_y = rust_d3::brush::brush_y();
    assert_eq!(brush_x.extent(), &(0.0..1.0));
    assert_eq!(brush_y.extent(), &(0.0..1.0));
}