//! Unit tests for d3 ColorScale
use rust_d3::color::ColorScale;

#[test]
fn test_color_scale() {
    let scale = ColorScale::new((0.0, 1.0), vec!["red".to_string(), "blue".to_string()]);
    assert_eq!(scale.scale(0.0), "red");
    assert_eq!(scale.scale(1.0), "blue");
    assert_eq!(scale.scale(0.5), "red"); // midpoint rounds down
}
