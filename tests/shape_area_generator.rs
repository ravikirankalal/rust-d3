use rust_d3::shape::AreaGenerator;

#[test]
fn test_area_generator() {
    let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)];
    let area = AreaGenerator::new(|&(x, y)| (x, y));
    let points = area.generate(&data);
    assert_eq!(points.len(), 6);
    // First half is original, second half is baseline
    assert_eq!(points[0], (0.0, 1.0));
    assert_eq!(points[3], (2.0, 0.0));
}
