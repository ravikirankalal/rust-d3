//! Unit test for d3 zoom
use rust_d3::zoom::Zoom;

#[test]
fn test_zoom_zoom_by_and_pan_by() {
    let mut z = Zoom::new(0.5, 4.0);
    assert_eq!(z.scale, 1.0);
    z.zoom_by(2.0);
    assert_eq!(z.scale, 2.0);
    z.zoom_by(10.0);
    assert_eq!(z.scale, 4.0); // max_scale
    z.zoom_by(0.1);
    assert_eq!(z.scale, 0.5); // min_scale
    z.pan_by(5.0);
    assert_eq!(z.translate, 5.0);
    z.pan_by(-2.0);
    assert_eq!(z.translate, 3.0);
    z.reset();
    assert_eq!(z.scale, 1.0);
    assert_eq!(z.translate, 0.0);
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
    let _z = Zoom::new(0.5, 4.0);
    // ...rest of the test...
}
