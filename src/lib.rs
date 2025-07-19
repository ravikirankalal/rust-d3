// Only export the array module for isolated testing
pub mod array;
pub mod axis;
pub mod chord;
pub mod collection;
pub mod color;
pub mod contour;
pub mod dispatch;
pub mod ease;
pub mod format;
pub mod geojson;
pub mod hierarchy;
pub mod interpolate;
pub mod path;
pub mod polygon;
pub mod quadtree;
pub mod random;
pub mod scale;
pub mod scale_chromatic;
pub mod selection;
pub mod shape;
pub mod time;
pub mod timer;

pub use array::bisector::bisector;
pub use array::quickselect::quickselect;
pub use array::transpose::transpose;
pub use axis::Axis;
pub use polygon::*;
pub use quadtree::*;
pub use selection::Selection;
pub use shape::{Symbol, SymbolType};

/// Formats a floating-point value for SVG coordinates.
/// Rounds to 6 decimal places and trims trailing zeros to avoid
/// long floats that break exact-string tests and bloat DOM.
pub fn px(v: f64) -> String {
    // Round to 6 decimal places
    let rounded = (v * 1_000_000.0).round() / 1_000_000.0;
    
    // Handle -0.0 case by checking if the result is effectively zero
    let effective_value = if rounded.abs() < 1e-9 { 0.0 } else { rounded };
    
    // Format with up to 6 decimal places
    let formatted = format!("{:.6}", effective_value);
    
    // Trim trailing zeros and decimal point if needed
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    
    trimmed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_px_integer_values() {
        assert_eq!(px(0.0), "0");
        assert_eq!(px(1.0), "1");
        assert_eq!(px(10.0), "10");
        assert_eq!(px(-5.0), "-5");
    }

    #[test]
    fn test_px_decimal_values() {
        assert_eq!(px(1.5), "1.5");
        assert_eq!(px(3.14), "3.14");
        assert_eq!(px(-2.75), "-2.75");
    }

    #[test]
    fn test_px_trailing_zeros_trimmed() {
        assert_eq!(px(1.50000), "1.5");
        assert_eq!(px(2.100000), "2.1");
        assert_eq!(px(3.000000), "3");
    }

    #[test]
    fn test_px_precision_rounding() {
        // Values that would normally have more than 6 decimal places
        assert_eq!(px(1.0/3.0), "0.333333");  // 1/3 = 0.333333...
        assert_eq!(px(1.0/7.0), "0.142857");  // 1/7 = 0.142857...
        assert_eq!(px(3.1415926535), "3.141593"); // pi rounded to 6 decimals
    }

    #[test]
    fn test_px_very_small_values() {
        assert_eq!(px(0.000001), "0.000001");   // Right at the precision limit
        assert_eq!(px(0.0000001), "0");        // Below precision, rounds to 0  
        assert_eq!(px(0.00000001), "0");       // Below precision, rounds to 0
        assert_eq!(px(1e-8), "0");            // Scientific notation, below precision
    }

    #[test]
    fn test_px_edge_cases() {
        assert_eq!(px(0.999999), "0.999999");
        assert_eq!(px(0.9999999), "1");     // Rounds to 1 due to precision limit
        assert_eq!(px(-0.0), "0");          // -0.0 becomes "0"
    }
}

// Example integration: use hierarchy with shape for tree visualization
// use crate::hierarchy::{Node, TreeLayout};
// use crate::shape::{Line, LinearCurve};
//
// let mut root = Node::new((0.0, 0.0));
// root.add_child(Node::new((1.0, 1.0)));
// let tree = TreeLayout::new();
// tree.layout(&mut root);
// let mut line = Line::new()
//     .x(|d, _| d.0)
//     .y(|d, _| d.1)
//     .curve(LinearCurve::default());
// let path = line.generate(&[root.data, root.children[0].data]);
// println!("SVG Path: {}", path);
