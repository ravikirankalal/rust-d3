#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "d3-brush not implemented yet")]
    fn test_brush_placeholder_panics() {
        brush_placeholder();
    }

    #[test]
    #[should_panic(expected = "d3-zoom not implemented yet")]
    fn test_zoom_placeholder_panics() {
        zoom_placeholder();
    }

    #[test]
    #[should_panic(expected = "d3-drag not implemented yet")]
    fn test_drag_placeholder_panics() {
        drag_placeholder();
    }
}
