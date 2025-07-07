use rust_d3::shape::RadialLineGenerator;

#[test]
fn test_radial_line_generator() {
    let data = vec![(1.0, 0.5), (2.0, 1.0), (3.0, 1.5)];
    let radial_line = RadialLineGenerator::new(|&(r, t)| (r, t));
    let points = radial_line.generate(&data);
    assert_eq!(points.len(), 3);
    assert_eq!(points[0], (1.0, 0.5));
    assert_eq!(points[2], (3.0, 1.5));
}
