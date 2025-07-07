//! Unit test for d3 zoom
use rust_d3::zoom::Zoom;

#[test]
fn test_zoom_zoom_by_and_pan_by() {
    let mut zoom = Zoom::new(0.5, 4.0);
    assert_eq!(zoom.scale, 1.0);
    zoom.zoom_by(2.0);
    assert_eq!(zoom.scale, 2.0);
    zoom.zoom_by(10.0);
    assert_eq!(zoom.scale, 4.0); // max_scale
    zoom.zoom_by(0.1);
    assert_eq!(zoom.scale, 0.5); // min_scale
    zoom.pan_by(5.0);
    assert_eq!(zoom.translate, 5.0);
    zoom.pan_by(-2.0);
    assert_eq!(zoom.translate, 3.0);
    zoom.reset();
    assert_eq!(zoom.scale, 1.0);
    assert_eq!(zoom.translate, 0.0);
}

#[test]
fn test_zoom_identity() {
    let z = Zoom::zoom_identity();
    assert_eq!(z.scale, 1.0);
    assert_eq!(z.translate, 0.0);
}

#[test]
fn test_zoom_transform() {
    let z = Zoom { scale: 2.0, translate: 3.0, min_scale: 1.0, max_scale: 4.0 };
    let (s, t) = z.zoom_transform();
    assert_eq!(s, 2.0);
    assert_eq!(t, 3.0);
}

#[test]
fn test_zoom_scale_extent() {
    let mut z = Zoom::new(0.5, 4.0);
    z.zoom_scale_extent(1.0, 2.0);
    assert_eq!(z.min_scale, 1.0);
    assert_eq!(z.max_scale, 2.0);
    assert!(z.scale >= 1.0 && z.scale <= 2.0);
}

#[test]
fn test_zoom_translate_extent() {
    let mut z = Zoom::new(1.0, 2.0);
    z.translate = 5.0;
    z.zoom_translate_extent(0.0, 3.0);
    assert_eq!(z.translate, 3.0);
    z.zoom_translate_extent(-2.0, 1.0);
    assert_eq!(z.translate, 1.0);
}

#[test]
fn test_zoom_on() {
    let mut z = Zoom::new(1.0, 2.0);
    z.on("zoom", || {}); // Should not panic
}
