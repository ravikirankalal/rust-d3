pub fn format_exponential(x: f64, precision: usize) -> String {
    format!("{:.*e}", precision, x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_exponential() {
        assert_eq!(format_exponential(1234.0, 2), "1.23e3");
        assert_eq!(format_exponential(0.01234, 2), "1.23e-2");
    }
}
