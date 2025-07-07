//! Tests for d3 interaction (brush/zoom/drag placeholders)

use rust_d3::interaction::interaction::{brush_placeholder, zoom_placeholder, drag_placeholder};

#[test]
fn test_brush_placeholder() {
    assert_eq!(brush_placeholder(), "brush not implemented");
}

#[test]
fn test_zoom_placeholder() {
    assert_eq!(zoom_placeholder(), "zoom not implemented");
}

#[test]
fn test_drag_placeholder() {
    assert_eq!(drag_placeholder(), "drag not implemented");
}
