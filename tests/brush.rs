//! Unit test for d3 brush
use rust_d3::brush::{brush_x, brush_y, BrushEvent, Brush};
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_brush_select_and_clear() {
    let mut brush = Brush::new(0.0..10.0);
    assert!(!brush.is_active());
    brush.select(2.0..5.0);
    assert_eq!(brush.selection, Some(2.0..5.0));
    assert!(brush.is_active());
    brush.clear();
    assert!(!brush.is_active());
    assert_eq!(brush.selection, None);
}

#[test]
fn test_brush_out_of_bounds() {
    let mut brush = Brush::new(0.0..10.0);
    brush.select(-1.0..5.0);
    assert_eq!(brush.selection, None);
    brush.select(2.0..11.0);
    assert_eq!(brush.selection, None);
}

#[test]
fn test_brush_x_and_y() {
    let bx = brush_x();
    let by = brush_y();
    assert_eq!(bx.extent, 0.0..1.0);
    assert_eq!(by.extent, 0.0..1.0);
}

#[test]
fn test_brush_filter_and_handle_size() {
    let mut brush = Brush::new(0.0..10.0);
    brush.filter(|| true);
    brush.handle_size(5.0);
    // No panic = pass
}
