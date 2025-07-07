use rust_d3::shape::ArcGenerator;

#[test]
fn test_arc_generator() {
    let arc = ArcGenerator::new()
        .inner_radius(10.0)
        .outer_radius(20.0)
        .start_angle(0.0)
        .end_angle(std::f64::consts::PI);
    let path = arc.generate();
    assert!(path.contains("A20")); // Should contain outer radius
    assert!(path.contains("A10")); // Should contain inner radius
}
