use num_format::{Locale, ToFormattedString};

pub fn format_comma(n: u64) -> String {
    n.to_formatted_string(&Locale::en)
}

pub fn format_fixed(n: f64, digits: usize) -> String {
    format!("{:.*}", digits, n)
}

/// d3-format: format specifier parser (very basic)
pub struct FormatSpecifier {
    pub precision: Option<usize>,
    pub comma: bool,
    pub percent: bool,
}

impl FormatSpecifier {
    pub fn parse(spec: &str) -> Self {
        let comma = spec.contains(",");
        let percent = spec.contains("%");
        let precision = spec.split('.').nth(1).and_then(|s| s.chars().take_while(|c| c.is_digit(10)).collect::<String>().parse().ok());
        Self { precision, comma, percent }
    }
}

/// d3-format: format function (supports comma, percent, precision)
pub fn format(specifier: &str) -> impl Fn(f64) -> String {
    let spec = FormatSpecifier::parse(specifier);
    move |n: f64| {
        let mut val = if let Some(p) = spec.precision {
            format!("{:.*}", p, n)
        } else {
            n.to_string()
        };
        if spec.comma {
            if let Ok(int_val) = val.parse::<u64>() {
                val = int_val.to_formatted_string(&Locale::en);
            }
        }
        if spec.percent {
            if let Ok(f) = val.parse::<f64>() {
                val = format!("{}%", f * 100.0);
            }
        }
        val
    }
}

/// d3-format: formatPrefix (very basic SI prefix for thousands)
pub fn format_prefix(_specifier: &str, value: f64) -> impl Fn(f64) -> String {
    let prefix = if value.abs() >= 1_000_000.0 {
        "M"
    } else if value.abs() >= 1_000.0 {
        "k"
    } else {
        ""
    };
    move |n: f64| {
        let scaled = if prefix == "M" {
            n / 1_000_000.0
        } else if prefix == "k" {
            n / 1_000.0
        } else {
            n
        };
        format!("{:.2}{}", scaled, prefix)
    }
}
