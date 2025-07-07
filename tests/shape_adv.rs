#[cfg(test)]
mod tests {
    use rust_d3::shape_adv::shape_adv::{area, arc};

    #[test]
    fn test_area() {
        let data = vec![1.0, 2.0, 3.0];
        let result = area(&data, |x| (*x, *x));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_arc() {
        let arc_points = arc(1.0, 2.0, 0.0, std::f64::consts::PI);
        assert!(!arc_points.is_empty());
    }
}
