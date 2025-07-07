//! Unit test for d3 chord_adv (placeholder)
use rust_d3::chord_adv::*;
use rust_d3::chord_adv::chord_ribbon;
use std::f64::consts::PI;

#[test]
fn test_chord_ribbon_svg_path() {
    // 90-degree arc from 0 to PI/2, target from PI to 3*PI/2, radius 100
    let path = chord_ribbon(0.0, PI/2.0, PI, 3.0*PI/2.0, 100.0);
    // Accept both 0.000000 and -0.000000 for robust comparison
    let normalized = path.replace("-0.000000", "0.000000");
    let expected = "M100.000000,0.000000A100,100 0 0,1 0.000000,100.000000Q0,0 0.000000,-100.000000A100,100 0 0,1 -100.000000,0.000000Q0,0 100.000000,0.000000Z";
    assert_eq!(normalized, expected);
}
