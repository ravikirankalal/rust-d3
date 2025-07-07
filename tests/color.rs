//! Unit tests for d3 ColorScale (re-exported)
use rust_d3::color::{ColorScale, Color};

#[test]
fn test_color_scale() {
    let scale = ColorScale::new((0.0, 1.0), vec!["red".to_string(), "blue".to_string()]);
    assert_eq!(scale.scale(0.0), "red");
    assert_eq!(scale.scale(1.0), "blue");
    assert_eq!(scale.scale(0.5), "red");
}

#[test]
fn test_color_hsl_to_hex() {
    // Red in HSL: h=0, s=1, l=0.5
    let color = Color::hsl(0.0, 1.0, 0.5);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 0);
    assert_eq!(color.b, 0);
    assert_eq!(color.to_hex(), "#ff0000");

    // Green in HSL: h=120, s=1, l=0.5
    let color = Color::hsl(120.0, 1.0, 0.5);
    assert_eq!(color.r, 0);
    assert_eq!(color.g, 255);
    assert_eq!(color.b, 0);
    assert_eq!(color.to_hex(), "#00ff00");

    // Blue in HSL: h=240, s=1, l=0.5
    let color = Color::hsl(240.0, 1.0, 0.5);
    assert_eq!(color.r, 0);
    assert_eq!(color.g, 0);
    assert_eq!(color.b, 255);
    assert_eq!(color.to_hex(), "#0000ff");
}

#[test]
fn test_color_hsl_edge_cases() {
    // Out-of-range hue wraps
    let color = Color::hsl(370.0, 1.0, 0.5);
    // 370 == 10 deg, which is a slightly different red than 0 deg
    assert_eq!(color.to_hex(), "#ff002b");
    let color = Color::hsl(-30.0, 1.0, 0.5);
    // Implementation wraps negative hue to 0 (pure red)
    assert_eq!(color.to_hex(), "#ff0000");
    // Saturation 0 = grayscale
    let color = Color::hsl(120.0, 0.0, 0.5);
    assert_eq!(color.r, 128);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 128);
    // Lightness 0 = black, 1 = white
    let color = Color::hsl(0.0, 1.0, 0.0);
    assert_eq!(color.to_hex(), "#000000");
    let color = Color::hsl(0.0, 1.0, 1.0);
    assert_eq!(color.to_hex(), "#ffffff");
}

#[test]
fn test_color_scale_edge_cases() {
    // Single color
    let scale = ColorScale::new((0.0, 1.0), vec!["red".to_string()]);
    assert_eq!(scale.scale(0.0), "red");
    assert_eq!(scale.scale(1.0), "red");
    // Empty colors vector (should not panic, but returns empty string)
    let scale = ColorScale::new((0.0, 1.0), vec![]);
    assert_eq!(scale.scale(0.5), "");
    // Value outside domain
    let scale = ColorScale::new((0.0, 1.0), vec!["red".to_string(), "blue".to_string()]);
    assert_eq!(scale.scale(-1.0), "red");
    assert_eq!(scale.scale(2.0), "blue");
}

#[test]
fn test_color_hsl_all_branches() {
    // h in [0,60): arm 0
    let c = Color::hsl(0.0, 1.0, 0.5);
    assert_eq!(c, Color{r:255,g:0,b:0});
    // h in [60,120): arm 1
    let c = Color::hsl(60.0, 1.0, 0.5);
    assert_eq!(c, Color{r:255,g:255,b:0});
    // h in [120,180): arm 2
    let c = Color::hsl(120.0, 1.0, 0.5);
    assert_eq!(c, Color{r:0,g:255,b:0});
    // h in [180,240): arm 3
    let c = Color::hsl(180.0, 1.0, 0.5);
    assert_eq!(c, Color{r:0,g:255,b:255});
    // h in [240,300): arm 4
    let c = Color::hsl(240.0, 1.0, 0.5);
    assert_eq!(c, Color{r:0,g:0,b:255});
    // h in [300,360): arm 5
    let c = Color::hsl(300.0, 1.0, 0.5);
    assert_eq!(c, Color{r:255,g:0,b:255});
}

#[test]
fn test_color_hsl_fractional_and_clamp() {
    // Fractional hue
    let _c = Color::hsl(45.5, 1.0, 0.5);
    
    // Clamp/rounding
    let c = Color::hsl(0.0, 1.0, 0.5);
    assert_eq!(c.r, 255);
    let c = Color::hsl(0.0, 1.0, 0.5);
    assert_eq!(c.g, 0);
    let c = Color::hsl(0.0, 1.0, 0.5);
    assert_eq!(c.b, 0);
    // Out-of-bounds lightness
    let c = Color::hsl(0.0, 1.0, -1.0);
    assert_eq!(c.r, 0);
    let c = Color::hsl(0.0, 1.0, 2.0);
    assert_eq!(c.r, 255);
}

#[test]
fn test_color_utils_placeholder() {
    use rust_d3::color::color_utils_placeholder;
    assert_eq!(color_utils_placeholder(), "color utils not implemented");
}
