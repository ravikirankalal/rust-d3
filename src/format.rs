//! D3-format: Rust port with D3.js API parity
//! Implements format, format_prefix, and format_specifier.

/// FormatSpecifier: parsed representation of a D3.js format specifier string.
#[derive(Debug, Clone, PartialEq)]
pub struct FormatSpecifier {
    pub fill: Option<char>,
    pub align: Option<char>,
    pub sign: Option<char>,
    pub symbol: Option<char>,
    pub zero: bool,
    pub width: Option<usize>,
    pub comma: bool,
    pub precision: Option<usize>,
    pub trim: bool,
    pub ty: Option<char>,
}

impl FormatSpecifier {
    /// Parse a D3.js-style format specifier string (e.g. ",.2f")
    pub fn parse(spec: &str) -> Result<Self, String> {
        // Basic parser for e.g. "+.2f", ",.2f", ">10.2f", etc.
        // Extract first two chars for fill/align
        let mut chars = spec.chars();
        let first = chars.next();
        let second = chars.next();
        let mut fill: Option<char> = None;
        let mut align: Option<char> = None;
        let mut rest = String::new();
        // Fill/align parsing block
        let mut chars = spec.chars();
        let first = chars.next();
        let second = chars.next();
        let (fill, align, rest) = if let (Some(a), Some(b)) = (first, second) {
            if b == '<' || b == '>' || b == '^' || b == '=' {
                (Some(a), Some(b), chars.collect::<String>())
            } else if a == '<' || a == '>' || a == '^' || a == '=' {
                (None, Some(a), {
                    let mut s = String::new();
                    s.push(b);
                    s.extend(chars);
                    s
                })
            } else {
                let mut s = String::new();
                s.push(a);
                s.push(b);
                s.extend(chars);
                (None, None, s)
            }
        } else if let Some(a) = first {
            if a == '<' || a == '>' || a == '^' || a == '=' {
                (None, Some(a), chars.collect::<String>())
            } else {
                let mut s = String::new();
                s.push(a);
                s.extend(chars);
                (None, None, s)
            }
        } else {
            (None, None, String::new())
        };
        // Now, after fill/align extraction, rest contains the rest of the specifier
        // Continue parsing width, zero, etc. from rest as before
        let mut chars = rest.chars().peekable();
        let mut sign = None;
        let symbol = None;
        let mut zero = false;
        let mut width = None;
        let mut comma = false;
        let mut precision = None;
        let mut trim = false;
        let mut ty = None;
        // Parse sign if present
        if let Some(&c) = chars.peek() {
            if c == '+' || c == '-' || c == ' ' || c == '(' {
                sign = Some(c);
                chars.next();
            }
        }
        // Parse zero-padding flag (0) before width
        if let Some(&c) = chars.peek() {
            if c == '0' {
                zero = true;
                chars.next();
            }
        }
        // Parse width
        let mut width_digits = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                width_digits.push(c);
                chars.next();
            } else { break; }
        }
        if !width_digits.is_empty() {
            width = Some(width_digits.parse().unwrap());
        }
        while let Some(&c) = chars.peek() {
            match c {
                '~' => { trim = true; chars.next(); },
                ',' => { comma = true; chars.next(); },
                '.' => {
                    chars.next();
                    let mut digits = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_ascii_digit() {
                            digits.push(d);
                            chars.next();
                        } else { break; }
                    }
                    if !digits.is_empty() {
                        precision = Some(digits.parse().unwrap());
                    }
                },
                'f' | 'e' | 'g' | 'r' | 's' | '%' => {
                    ty = Some(c);
                    chars.next();
                },
                _ => { chars.next(); },
            }
        }
        Ok(FormatSpecifier {
            fill, align, sign, symbol, zero, width, comma, precision, trim, ty
        })
    }
}

/// Returns a formatter closure for the given specifier string.
pub fn format(spec: &str) -> Result<impl Fn(f64) -> String, String> {
    let specifier = FormatSpecifier::parse(spec)?;
    Ok(move |value: f64| {
        let mut s = match (specifier.ty, specifier.precision, specifier.comma) {
            (Some('f'), Some(p), true) => format_with_grouping(value, p),
            (Some('f'), Some(p), false) => format!("{:.*}", p, value),
            (Some('e'), Some(p), _) => format!("{:.*e}", p, value),
            (Some('e'), None, _) => format!("{:e}", value),
            (Some('g'), Some(p), _) => format_general(value, p),
            (Some('g'), None, _) => format_general(value, 6),
            (Some('s'), Some(p), _) => format_si(value, p),
            (Some('s'), None, _) => format_si(value, 2),
            (Some('%'), Some(p), _) => format!("{:.*}%", p, value * 100.0),
            (Some('%'), None, _) => format!("{}%", value * 100.0),
            _ => format!("{}", value),
        };
        // Handle trim
        if specifier.trim {
            if let Some(dot) = s.find('.') {
                let mut end = s.len();
                while end > dot && s.as_bytes()[end - 1] == b'0' { end -= 1; }
                if end > dot && s.as_bytes()[end - 1] == b'.' { end -= 1; }
                s.truncate(end);
            }
        }
        // Handle sign and zero/parenthesis logic
        let mut sign_str = String::new();
        let abs_s = s.trim_start_matches('-').to_string();
        // Treat -0.0 as negative for formatting
        let is_negative = value.is_sign_negative() || (value == 0.0 && 1.0 / value == f64::NEG_INFINITY);
        if let Some(sign) = specifier.sign {
            match sign {
                '+' => {
                    if is_negative {
                        sign_str.push('-');
                    } else {
                        sign_str.push('+');
                    }
                },
                ' ' => {
                    if is_negative {
                        sign_str.push('-');
                    } else {
                        sign_str.push(' ');
                    }
                },
                '(' => {
                    if is_negative {
                        // Parenthesis sign, handled after padding
                    } else {
                        sign_str.clear();
                    }
                },
                _ => {
                    if is_negative {
                        sign_str.push('-');
                    }
                }
            }
        } else if is_negative {
            sign_str.push('-');
        }
        // Handle zero-padding and alignment
        let width = specifier.width.unwrap_or(0);
        let fill = specifier.fill.unwrap_or(' ');
        let mut align = specifier.align.unwrap_or('>');
        // D3.js: If zero is set and no explicit align, align should default to '='
        if specifier.zero && specifier.align.is_none() {
            align = '=';
        }
        let align_for_zero = if fill == '0' && (align == '>' || align == '=' || specifier.align.is_none()) { '=' } else { align };
        // Zero-padding after sign, before digits for align = '='
        if align_for_zero == '=' && width > sign_str.len() + abs_s.len() {
            let pad = width - sign_str.len() - abs_s.len();
            let mut out = format!("{}{}{}", sign_str, "0".repeat(pad), abs_s);
            if specifier.sign == Some('(') && is_negative {
                out = format!("({})", out.trim_start_matches('-'));
            }
            return out;
        }
        // For other alignments, use fill (could be '0')
        let content = format!("{}{}", sign_str, abs_s);
        // For custom fill, if fill is not ' ', pad to width even if width == content.len()
        let pad = if width > content.len() { width - content.len() } else { 0 };
        let result = if pad > 0 {
            match align {
                '<' => format!("{}{}", content, fill.to_string().repeat(pad)),
                '>' => format!("{}{}", fill.to_string().repeat(pad), content),
                '^' => {
                    let left = pad / 2;
                    let right = pad - left;
                    format!("{}{}{}", fill.to_string().repeat(left), content, fill.to_string().repeat(right))
                },
                _ => format!("{}{}", fill.to_string().repeat(pad), content),
            }
        } else {
            content
        };
        // Parenthesis sign wrapping for negative numbers (non-zero-padding cases)
        if specifier.sign == Some('(') && is_negative {
            return format!("({})", result.trim_start_matches('-'));
        }
        // For parenthesis sign and positive, just output as normal
        if specifier.sign == Some('(') && !is_negative {
            return abs_s;
        }
        return result;
    })
}

fn format_with_grouping(value: f64, precision: usize) -> String {
    let s = format!("{:.*}", precision, value);
    let mut parts = s.split('.');
    let int_part = parts.next().unwrap_or("");
    let frac_part = parts.next();
    let grouped = group_int(int_part);
    match frac_part {
        Some(frac) => format!("{}.{}", grouped, frac),
        None => grouped,
    }
}

fn group_int(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut count = 0;
    for c in chars.iter().rev() {
        if count != 0 && count % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, *c);
        count += 1;
    }
    result
}

/// SI-prefix formatting (e.g. 1.2k, 3.4M)
pub fn format_prefix(spec: &str, value: f64) -> Result<String, String> {
    // Parse the specifier
    let spec = FormatSpecifier::parse(spec)?;
    // SI prefixes from 1e-24 to 1e24
    const PREFIXES: [&str; 17] = [
        "y", "z", "a", "f", "p", "n", "μ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y"
    ];
    let mut v = value.abs();
    let mut exp = 0;
    if v == 0.0 {
        return Ok(format!("{:.*}", spec.precision.unwrap_or(2), value));
    }
    while v >= 1000.0 && exp < 8 {
        v /= 1000.0;
        exp += 1;
    }
    while v < 1.0 && exp > -8 {
        v *= 1000.0;
        exp -= 1;
    }
    let prefix = PREFIXES[(exp + 8) as usize];
    let sign = if value < 0.0 { "-" } else { "" };
    let precision = spec.precision.unwrap_or(2);
    let mut s = format!("{}{:.*}{}", sign, precision, v, prefix);
    // Apply width, fill, align if present
    if let Some(width) = spec.width {
        let fill = spec.fill.unwrap_or(' ');
        let align = spec.align.unwrap_or('>');
        if s.len() < width {
            let pad = width - s.len();
            match align {
                '<' => s = format!("{}{}", s, fill.to_string().repeat(pad)),
                '>' => s = format!("{}{}", fill.to_string().repeat(pad), s),
                '^' => {
                    let left = pad / 2;
                    let right = pad - left;
                    s = format!("{}{}{}", fill.to_string().repeat(left), s, fill.to_string().repeat(right));
                },
                _ => {},
            }
        }
    }
    Ok(s)
}

fn format_si(value: f64, precision: usize) -> String {
    // SI prefixes from 1e-24 to 1e24
    const PREFIXES: [&str; 17] = [
        "y", "z", "a", "f", "p", "n", "μ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y"
    ];
    let mut v = value.abs();
    let mut exp = 0;
    if v == 0.0 {
        return format!("{:.*}", precision, value);
    }
    while v >= 1000.0 && exp < 8 {
        v /= 1000.0;
        exp += 1;
    }
    while v < 1.0 && exp > -8 {
        v *= 1000.0;
        exp -= 1;
    }
    let prefix = PREFIXES[(exp + 8) as usize];
    let sign = if value < 0.0 { "-" } else { "" };
    format!("{}{:.*}{}", sign, precision, v, prefix)
}

fn format_general(value: f64, precision: usize) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    let abs = value.abs();
    let sci = abs < 1e-4 || abs >= 1e3;
    if sci {
        // Scientific notation, precision = significant digits
        let s = format!("{:.*e}", precision.saturating_sub(1), value);
        // Remove trailing zeros and dot
        let s = s.replace("e", "e");
        let mut parts = s.split('e');
        let mut mant = parts.next().unwrap_or("").to_string();
        let exp = parts.next().unwrap_or("");
        if mant.contains('.') {
            while mant.ends_with('0') { mant.pop(); }
            if mant.ends_with('.') { mant.pop(); }
        }
        format!("{}e{}", mant, exp)
    } else {
        // Fixed, precision = significant digits
        let digits = if abs >= 1.0 {
            precision.saturating_sub((abs.log10().floor() as usize) + 1)
        } else {
            let mut zeros = 0;
            let mut x = abs;
            while x < 1.0 {
                x *= 10.0;
                zeros += 1;
                if zeros > 20 { break; }
            }
            precision + zeros - 1
        };
        let s = format!("{:.*}", digits, value);
        // Remove trailing zeros and dot
        let mut s = s;
        if s.contains('.') {
            while s.ends_with('0') { s.pop(); }
            if s.ends_with('.') { s.pop(); }
        }
        s
    }
}
