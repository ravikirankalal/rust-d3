#[cfg(test)]
mod tests {
    use rust_d3::axis::util::TransformBuilder;

    #[test]
    fn test_transform_builder_basic() {
        let transform = TransformBuilder::new()
            .translate(10.0, 20.0)
            .build();
        assert_eq!(transform, "translate(10,20)");
    }

    #[test]
    fn test_transform_builder_with_rotation() {
        let transform = TransformBuilder::new()
            .translate(10.0, 20.0)
            .rotate(45.0, 100.0, 50.0)
            .build();
        assert_eq!(transform, "translate(10,20) rotate(45 100 50)");
    }

    #[test]
    fn test_transform_builder_with_scale() {
        let transform = TransformBuilder::new()
            .translate(10.0, 20.0)
            .scale(2.0, 1.5)
            .build();
        assert_eq!(transform, "translate(10,20) scale(2,1.5)");
    }

    #[test]
    fn test_transform_builder_all_transforms() {
        let transform = TransformBuilder::new()
            .translate(10.0, 20.0)
            .rotate(45.0, 100.0, 50.0)
            .scale(2.0, 1.5)
            .build();
        assert_eq!(transform, "translate(10,20) rotate(45 100 50) scale(2,1.5)");
    }

    #[test]
    fn test_transform_builder_with_existing() {
        let transform = TransformBuilder::with_existing(Some("matrix(1,0,0,1,0,0)".to_string()))
            .translate(10.0, 20.0)
            .build();
        assert_eq!(transform, "matrix(1,0,0,1,0,0) translate(10,20)");
    }

    #[test]
    fn test_transform_builder_empty_returns_default() {
        let transform = TransformBuilder::new().build();
        assert_eq!(transform, "translate(0,0)");
    }

    #[test]
    fn test_transform_builder_zero_translate_still_outputs() {
        let transform = TransformBuilder::new()
            .translate(0.0, 0.0)
            .build();
        assert_eq!(transform, "translate(0,0)");
    }

    #[test]
    fn test_transform_builder_with_empty_existing() {
        let transform = TransformBuilder::with_existing(Some("".to_string()))
            .translate(5.0, 10.0)
            .build();
        assert_eq!(transform, "translate(5,10)");
    }

    #[test]
    fn test_transform_builder_with_none_existing() {
        let transform = TransformBuilder::with_existing(None)
            .translate(5.0, 10.0)
            .build();
        assert_eq!(transform, "translate(5,10)");
    }
}
