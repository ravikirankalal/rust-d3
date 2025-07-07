//! Unit tests for d3 symbol module (placeholder)
use rust_d3::symbol::*;

fn normalize_svg_path(s: &str) -> String {
    s.replace(",", " ")
        .split_whitespace()
        .map(|v| {
            v.parse::<f64>()
                .map(|f| format!("{:.6}", f))
                .unwrap_or_else(|_| v.to_string())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn test_symbol_circle() {
    // D3's symbolCircle(64) produces a circle of radius 4 (since r = sqrt(64/PI))
    let path = symbol_circle(64.0);
    let expected = format!(
        "M{:.6},0A{:.6},{:.6} 0 1,1 -{:.6},0A{:.6},{:.6} 0 1,1 {:.6},0Z",
        4.513516668382566, 4.513516668382566, 4.513516668382566, 4.513516668382566,
        4.513516668382566, 4.513516668382566, 4.513516668382566
    );
    assert_eq!(normalize_svg_path(&path), normalize_svg_path(&expected));
}
