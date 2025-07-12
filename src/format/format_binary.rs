pub fn format_binary(x: i64) -> String {
    format!("{:b}", x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_binary() {
        assert_eq!(format_binary(10), "1010");
        assert_eq!(format_binary(255), "11111111");
    }
}
