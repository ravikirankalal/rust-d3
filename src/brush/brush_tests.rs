#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brush_extent() {
        let brush = Brush::new(0.0..10.0);
        assert_eq!(brush.extent(), &(0.0..10.0));
    }
    // TODO: Add tests for brushX and brushY when implemented
}
