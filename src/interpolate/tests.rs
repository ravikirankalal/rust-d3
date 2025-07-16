//! d3-interpolate tests and examples

#[cfg(test)]
mod tests {
    use crate::interpolate::{
        interpolate_number, interpolate_array, interpolate_string, interpolate_rgb, interpolate_hsl
    };

    #[test]
    fn test_interpolate_number() {
        assert_eq!(interpolate_number(0.0, 10.0, 0.5), 5.0);
    }

    #[test]
    fn test_interpolate_number_basic() {
        assert_eq!(interpolate_number(1.0, 3.0, 0.5), 2.0);
    }

    #[test]
    fn test_interpolate_array() {
        let a = [0.0, 1.0];
        let b = [10.0, 11.0];
        let r = interpolate_array(&a, &b, 0.5);
        assert_eq!(r, vec![5.0, 6.0]);
    }

    #[test]
    fn test_interpolate_array_mixed() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let r = interpolate_array(&a, &b, 0.25);
        assert_eq!(r, vec![1.75, 2.75, 3.75]);
    }

    #[test]
    fn test_interpolate_string() {
        let s = interpolate_string("foo0bar", "foo10bar", 0.5);
        assert_eq!(s, "foo5bar");
    }

    #[test]
    fn test_interpolate_rgb() {
        let c = interpolate_rgb("#ff0000", "#00ff00", 0.5);
        assert_eq!(c, "#808000");
    }

    #[test]
    fn test_interpolate_rgb_edges() {
        assert_eq!(interpolate_rgb("#000000", "#ffffff", 0.0), "#000000");
        assert_eq!(interpolate_rgb("#000000", "#ffffff", 1.0), "#ffffff");
    }

    #[test]
    fn test_interpolate_hsl() {
        let c = interpolate_hsl("#ff0000", "#00ff00", 0.5);
        // Should be a yellowish color between red and green
        assert!(c.starts_with("#"));
    }

    #[test]
    fn test_interpolate_hsl_edges() {
        assert_eq!(interpolate_hsl("#ff0000", "#00ff00", 0.0), "#ff0000");
        assert_eq!(interpolate_hsl("#ff0000", "#00ff00", 1.0), "#00ff00");
    }
}
