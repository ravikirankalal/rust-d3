pub fn format_percent(x: f64, precision: usize) -> String {
    format!("{:.*}%", precision, x * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_percent() {
        assert_eq!(format_percent(0.1234, 1), "12.3%");
        assert_eq!(format_percent(1.0, 0), "100%");
    }
}
