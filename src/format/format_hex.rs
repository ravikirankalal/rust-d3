pub fn format_hex(x: i64, upper: bool) -> String {
    if upper {
        format!("{:X}", x)
    } else {
        format!("{:x}", x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_hex() {
        assert_eq!(format_hex(255, false), "ff");
        assert_eq!(format_hex(255, true), "FF");
    }
}
