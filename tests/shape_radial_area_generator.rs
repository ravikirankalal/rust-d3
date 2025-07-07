use rust_d3::shape::RadialAreaGenerator;

#[test]
fn test_radial_area_generator() {
    let data = vec![(1.0, 0.5), (2.0, 1.0), (3.0, 1.5)];
    let radial_area = RadialAreaGenerator::new(|&(r, t)| (r, t));
    let points = radial_area.generate(&data);
    assert_eq!(points.len(), 6);
    assert_eq!(points[0], (1.0, 0.5));
    assert_eq!(points[3], (3.0, 0.0));
}
