//! Unit tests for d3 LineGenerator (re-exported)
use rust_d3::shape::LineGenerator;

#[test]
fn test_line_generator() {
    let data = vec![(1.0, 2.0), (3.0, 4.0)];
    let line = LineGenerator::generate(&data, |&(x, y)| (x, y));
    assert_eq!(line, data);
}
