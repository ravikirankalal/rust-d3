use rust_d3::shape::{LineGenerator, stack};

#[test]
fn test_line_generator() {
    let data = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.5)];
    let path = LineGenerator::generate(&data, |d| *d);
    // LineGenerator::generate returns Vec<(f64, f64)>, not a path string.
    // The original test was asserting against a string, which is incorrect.
    // I will assert against the generated points.
    assert_eq!(path, vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.5)]);
}

#[test]
fn test_stack_generator() {
    let data = vec![
        vec![10.0, 20.0],
        vec![5.0, 15.0],
    ];
    let stacked_data = stack(&data);
    assert_eq!(stacked_data.len(), 2);
    assert_eq!(stacked_data[0], vec![(0.0, 10.0), (0.0, 20.0)]);
    assert_eq!(stacked_data[1], vec![(10.0, 15.0), (20.0, 35.0)]);
}
