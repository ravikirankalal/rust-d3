//! Tests for d3-polygon (Rust)

#[cfg(test)]
mod tests {
    use crate::{polygon_area, polygon_centroid, polygon_contains, polygon_hull, polygon_length};

    #[test]
    fn test_polygon_area_triangle() {
        let pts = [(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
        assert_eq!(polygon_area(&pts), 6.0);
    }
    #[test]
    fn test_polygon_centroid_triangle() {
        let pts = [(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
        let (cx, cy) = polygon_centroid(&pts);
        assert!((cx - 1.3333).abs() < 1e-3);
        assert!((cy - 1.0).abs() < 1e-3);
    }
    #[test]
    fn test_polygon_length_triangle() {
        let pts = [(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
        let len = polygon_length(&pts);
        assert!((len - (5.0 + 4.0 + 3.0)).abs() < 1e-6);
    }
    #[test]
    fn test_polygon_contains_inside() {
        let pts = [(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
        assert!(polygon_contains(&pts, 1.0, 1.0));
    }
    #[test]
    fn test_polygon_contains_outside() {
        let pts = [(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
        assert!(!polygon_contains(&pts, 5.0, 5.0));
    }
    #[test]
    fn test_polygon_hull_square() {
        let pts = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.5, 0.5)];
        let hull = polygon_hull(&pts);
        assert_eq!(hull.len(), 4);
        assert!(hull.contains(&(0.0, 0.0)));
        assert!(hull.contains(&(1.0, 0.0)));
        assert!(hull.contains(&(1.0, 1.0)));
        assert!(hull.contains(&(0.0, 1.0)));
    }
}
