use rust_d3::shape::PieGenerator;

#[test]
fn test_pie_generator() {
    let data = vec![1.0, 2.0, 3.0];
    let pie = PieGenerator::new(|x: &f64| *x);
    let angles = pie.generate(&data);
    assert_eq!(angles.len(), 3);
    let total_angle: f64 = angles.iter().map(|(start, end)| end - start).sum();
    assert!((total_angle - std::f64::consts::TAU).abs() < 1e-12);
}
