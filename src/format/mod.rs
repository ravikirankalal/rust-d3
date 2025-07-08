// d3-format parity root
// Re-export all submodules here as you implement them

pub mod specifier;
pub use specifier::FormatSpecifier;
pub mod parse_specifier;
pub use parse_specifier::parse_specifier;
pub mod format_decimal;
pub use format_decimal::format_decimal;
pub mod format_integer;
pub use format_integer::format_integer;
pub mod format_float;
pub use format_float::format_float;
pub mod format_grouping;
pub use format_grouping::format_grouping;
pub mod format_prefix;
pub use format_prefix::format_prefix;
pub mod format_type;
pub use format_type::format_type;
pub mod format;
pub use format::format;
pub mod locale;
pub use locale::Locale;

// D3-time and d3-time-format modules

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_decimal() {
        assert_eq!(format_decimal(12345.678, 2), "12345.68");
    }
    #[test]
    fn test_format_integer() {
        assert_eq!(format_integer(12345), "12345");
    }
    #[test]
    fn test_format_float() {
        assert_eq!(format_float(12345.678, 3), "12345.678");
    }
    #[test]
    fn test_format_grouping() {
        assert_eq!(format_grouping("1234567", ","), "1,234,567");
    }
    #[test]
    fn test_format_prefix() {
        // D3.js: .2s on 1234.0 => "1.2k", .3s on 1234.0 => "1.23k"
        assert_eq!(format_prefix(1234.0, 3), "1.23k");
        assert_eq!(format_prefix(1234.0, 2), "1.2k");
        assert_eq!(format_prefix(1_000_000.0, 3), "1.00M");
        assert_eq!(format_prefix(1_000_000.0, 2), "1M");
    }
    #[test]
    fn test_parse_specifier() {
        let spec = parse_specifier(".2f");
        assert_eq!(spec.precision, Some(2));
        assert_eq!(spec.ty, 'f');
    }
    #[test]
    fn test_format_type() {
        let spec = FormatSpecifier { precision: Some(2), ty: 'f', ..Default::default() };
        assert_eq!(format_type(123.456, &spec), "123.46");
    }
    #[test]
    fn test_format() {
        assert_eq!(format(".2f", 123.456), "123.46");
        assert_eq!(format(",.0f", 12345.0), "12,345");
        assert_eq!(format(".2s", 1234.0), "1.2k");
        assert_eq!(format(".1s", 1234.0), "1k");
        // Underscore grouping
        assert_eq!(format("_.0f", 12345.0), "12_345");
        // Parentheses for negative numbers
        assert_eq!(format("(.2f", -123.45), "(123.45)");
    }
    #[test]
    fn test_format_fill_align() {
        // Fill with * and right align
        assert_eq!(super::format("*>10.2f", 12.3), "*****12.30"); // 5 stars + 5 chars = 10
        // Left align
        assert_eq!(super::format("<10.2f", 12.3), "12.30     ");
        // Center align
        assert_eq!(super::format("^10.2f", 12.3), "  12.30   ");
    }
    #[test]
    fn test_format_width() {
        assert_eq!(super::format("10.2f", 12.3), "     12.30"); // 5 spaces + 5 chars = 10
    }
    #[test]
    fn test_format_sign() {
        assert_eq!(super::format("+10.2f", 12.3), "    +12.30");
        assert_eq!(super::format("+10.2f", -12.3), "    -12.30");
        assert_eq!(super::format(" 10.2f", 12.3), "     12.30");
    }
    #[test]
    fn test_format_zero_padding() {
        assert_eq!(format("010.2f", 12.3), "0000012.30");
    }
    #[test]
    fn test_format_types() {
        assert_eq!(format(".2e", 1234.0), "1.23e3");
        assert_eq!(format(".2g", 1234.0), "1.2e3");
        assert_eq!(format(".2r", 1234.0), "1.2e3");
        assert_eq!(format("b", 10.0), "1010");
        assert_eq!(format("o", 10.0), "12");
        assert_eq!(format("x", 255.0), "ff");
        assert_eq!(format("X", 255.0), "FF");
        assert_eq!(format("%", 0.123), "12.3%");
    }
    #[test]
    fn test_format_special_values() {
        assert_eq!(format(".2f", f64::NAN), "NaN");
        assert_eq!(format(".2f", f64::INFINITY), "inf");
        assert_eq!(format(".2f", f64::NEG_INFINITY), "-inf");
        assert_eq!(format(".2f", -0.0), "-0.00");
    }
    #[test]
    fn test_format_si_precision() {
        assert_eq!(format(".2s", 1234.0), "1.2k");
    }
    #[test]
    fn test_format_currency_locale_stub() {
        // These are stubs for future implementation
        let _ = format("$,.2f", 1234.56); // Should eventually be "$1,234.56"
        let _ = format(".2f", 1234.56); // Locale stub
    }
}

pub fn format_locale(value: f64, locale: &str) -> String {
    // Minimal stub: support en-US and fr-FR for demo, fallback to en-US
    match locale {
        "fr-FR" => {
            let s = format!("{:.6}", value).replace('.', ",");
            // Add thin space for thousands
            let parts: Vec<&str> = s.split(',').collect();
            let int_part = parts[0];
            let mut grouped = String::new();
            let mut chars = int_part.chars().rev().collect::<Vec<_>>();
            for (i, c) in chars.iter().enumerate() {
                if i > 0 && i % 3 == 0 {
                    grouped.push(' ');
                }
                grouped.push(*c);
            }
            let grouped = grouped.chars().rev().collect::<String>();
            if parts.len() > 1 {
                format!("{}", grouped + "," + parts[1])
            } else {
                grouped
            }
        }
        _ => format!("{:.6}", value),
    }
}

// d3-format: TODO stubs for alternate form (#) and type n
// - Type n: locale-aware number formatting (not yet implemented)
// - Alternate form (#): not fully tested for all types
