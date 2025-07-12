pub fn format_octal(x: i64) -> String {
    format!("{:o}", x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_octal() {
        assert_eq!(format_octal(10), "12");
        assert_eq!(format_octal(255), "377");
    }
}
