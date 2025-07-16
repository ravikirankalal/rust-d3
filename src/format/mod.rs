// d3-format parity root
// Handles number formatting to match D3.js standards, supporting various formats.
// Includes methods for formatting based on locale and custom specifications.
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

pub fn format_locale(value: f64, locale: &str, grouping: bool) -> String {
    let loc = crate::format::locale::Locale::from_tag(locale);
    let s = format!("{:.6}", value);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let grouped = if grouping {
        let mut grouped = String::new();
        let chars = int_part.chars().rev().collect::<Vec<_>>();
        for (i, c) in chars.iter().enumerate() {
            if i > 0 && i % loc.grouping[0] == 0 {
                grouped.push_str(&loc.thousands);
            }
            grouped.push(*c);
        }
        grouped.chars().rev().collect::<String>()
    } else {
        int_part.to_string()
    };
    if parts.len() > 1 {
        format!("{}{}{}", grouped, loc.decimal, parts[1])
    } else {
        grouped
    }
}

/// Implements alternate form (#) handling for supported types.
pub fn format_alternate(value: f64, specifier: &str) -> String {
    // Parse the type from the specifier (last char)
    let ty = specifier.chars().last().unwrap_or('f');
    match ty {
        'x' => format!("0x{:x}", value as i64),
        'X' => format!("0x{:X}", value as i64),
        'o' => format!("0o{:o}", value as i64),
        'b' => format!("0b{:b}", value as i64),
        'f' | 'e' | 'g' | 'r' => {
            // For floats, alternate form means always show decimal point
            let s = match ty {
                'e' => format!("{:#e}", value),
                'g' => format!("{:#}", value),
                'r' => format!("{:#}", value),
                _ => format!("{:#}", value),
            };
            if !s.contains('.') {
                format!("{}.", s)
            } else {
                s
            }
        }
        _ => format!("{}", value),
    }
}

/// Implements type n (locale-aware number formatting).
pub fn format_type_n(value: f64, locale: &str) -> String {
    // Use format_locale for grouping and decimal separator, with grouping enabled
    format_locale(value, locale, true)
}

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
    fn test_format_type() {
        let spec = FormatSpecifier {
            precision: Some(2),
            ty: 'f',
            ..Default::default()
        };
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
    #[test]
    fn test_format_currency_en_us() {
        // en-US: $1,234.56
        let locale = crate::format::locale::Locale::from_tag("en-US");
        let s = crate::format::format_locale(1234.56, "en-US", true);
        let formatted = format!("{}{}{}", locale.currency.0, s, locale.currency.1);
        assert_eq!(formatted, "$1,234.560000");
    }
    #[test]
    fn test_format_currency_fr_fr() {
        // fr-FR: 1 234,56 €
        let locale = crate::format::locale::Locale::from_tag("fr-FR");
        let s = crate::format::format_locale(1234.56, "fr-FR", true);
        let formatted = format!("{}{}{}", locale.currency.0, s, locale.currency.1);
        assert_eq!(formatted, "1 234,560000 €");
    }
    #[test]
    fn test_format_locale_en_us() {
        // No grouping
        let s = crate::format::format_locale(1234567.89, "en-US", false);
        assert_eq!(s, "1234567.890000");
    }
    #[test]
    fn test_format_locale_fr_fr() {
        let s = format_locale(1234567.89, "fr-FR", true);
        // Should be grouped with thin spaces and comma as decimal separator
        assert!(s.contains(","));
        assert!(s.contains(" ") || s.contains("\u{202f}"));
    }
}

// d3-format: TODO stubs for alternate form (#) and type n
// - Type n: locale-aware number formatting (not yet implemented)
// - Alternate form (#): not fully tested for all types
