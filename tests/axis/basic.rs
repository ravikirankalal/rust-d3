//! Unit tests for d3 axis

use rust_d3::scale::LinearScale;
use rust_d3::axis::Axis;

#[test]
fn test_axis_generate() {
    let scale = LinearScale::new((0.0, 10.0), (0.0, 100.0));
    let axis = Axis::new(5);
    let ticks = axis.generate(&scale);
    let expected = vec![
        (0.0, 0.0),
        (2.5, 25.0),
        (5.0, 50.0),
        (7.5, 75.0),
        (10.0, 100.0),
    ];
    assert_eq!(ticks, expected);
}
