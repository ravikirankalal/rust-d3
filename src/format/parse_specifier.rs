use super::specifier::FormatSpecifier;

pub fn parse_specifier(spec: &str) -> FormatSpecifier {
    let mut fs = FormatSpecifier::default();
    eprintln!("[DEBUG] parse_specifier: spec='{}'", spec);
    let mut chars = spec.chars().peekable();
    let mut grouping_found = false;
    let mut comma_found = false;
    // First, scan for grouping and comma anywhere in the string
    for c in spec.chars() {
        if c == ',' {
            comma_found = true;
        } else if c == '_' {
            grouping_found = true;
        }
    }
    if comma_found {
        fs.comma = true;
    }
    if grouping_found {
        fs.grouping = Some('_');
    }
    // Accept multiple symbol specifiers ($, #) in any order before other options
    while let Some(&a) = chars.peek() {
        if matches!(a, '$' | '#') {
            fs.symbol = Some(a);
            chars.next();
        } else {
            break;
        }
    }
    // Fill and align
    let first = chars.peek().copied();
    let second = chars.clone().nth(1);
    if let (Some(a), Some(b)) = (first, second) {
        if matches!(b, '<' | '>' | '^' | '=') {
            fs.fill = Some(a);
            fs.align = Some(b);
            chars.next();
            chars.next();
        }
    }
    // Align only
    if fs.align.is_none() {
        if let Some(&a) = chars.peek() {
            if matches!(a, '<' | '>' | '^' | '=') {
                fs.align = Some(a);
                chars.next();
            }
        }
    }
    // Sign
    if let Some(&a) = chars.peek() {
        if matches!(a, '+' | '-' | ' ') {
            fs.sign = Some(a);
            chars.next();
        }
    }
    // Accept more symbol specifiers after sign/align
    while let Some(&a) = chars.peek() {
        if matches!(a, '$' | '#') {
            fs.symbol = Some(a);
            chars.next();
        } else {
            break;
        }
    }
    // Parentheses (accounting negative numbers)
    if let Some(&'(') = chars.peek() {
        fs.parentheses = true;
        chars.next();
    }
    // Zero
    if let Some(&'0') = chars.peek() {
        fs.zero = true;
        chars.next();
    }
    // Skip grouping/comma before width
    while let Some(&c) = chars.peek() {
        if c == ',' || c == '_' {
            chars.next();
        } else {
            break;
        }
    }
    // Skip any non-digit, non-dot, non-type chars before width
    while let Some(&c) = chars.peek() {
        if !c.is_ascii_digit() && c != '.' && c != 'f' && c != 'd' && c != 's' && c != 'e' && c != 'g' && c != 'r' && c != 'b' && c != 'o' && c != 'x' && c != 'X' && c != '%' && c != '\0' {
            chars.next();
        } else {
            break;
        }
    }
    // Width: parse consecutive digits only (must start with a digit)
    let mut width = String::new();
    if let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    width.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
        }
    }
    if !width.is_empty() {
        fs.width = width.parse().ok();
    }
    // Grouping/comma may appear after width, skip them
    while let Some(&c) = chars.peek() {
        if c == ',' || c == '_' {
            chars.next();
        } else {
            break;
        }
    }
    // Precision
    if let Some(&'.') = chars.peek() {
        chars.next();
        let mut num = String::new();
        while let Some(&d) = chars.peek() {
            if d.is_ascii_digit() {
                num.push(d);
                chars.next();
            } else if d == ',' || d == '_' {
                chars.next(); // skip grouping/comma
            } else {
                break;
            }
        }
        fs.precision = num.parse().ok();
    }
    // Type
    if let Some(&t) = chars.peek() {
        fs.ty = t;
    }
    eprintln!("[DEBUG] parse_specifier: parsed precision={:?} ty={:?}", fs.precision, fs.ty);
    // Locale and currency are not parsed from the spec string directly, but can be set by higher-level logic.
    fs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_and_align() {
        let fs = parse_specifier("*>10.2f");
        assert_eq!(fs.fill, Some('*'));
        assert_eq!(fs.align, Some('>'));
        assert_eq!(fs.width, Some(10));
        assert_eq!(fs.precision, Some(2));
        assert_eq!(fs.ty, 'f');
    }

    #[test]
    fn test_align_only() {
        let fs = parse_specifier("<10.2f");
        assert_eq!(fs.fill, None);
        assert_eq!(fs.align, Some('<'));
        assert_eq!(fs.width, Some(10));
        assert_eq!(fs.precision, Some(2));
        assert_eq!(fs.ty, 'f');
    }

    #[test]
    fn test_sign_and_zero() {
        let fs = parse_specifier("+010.2f");
        assert_eq!(fs.sign, Some('+'));
        assert_eq!(fs.zero, true);
        assert_eq!(fs.width, Some(10));
        assert_eq!(fs.precision, Some(2));
        assert_eq!(fs.ty, 'f');
    }

    #[test]
    fn test_symbol_and_parentheses() {
        let fs = parse_specifier("$(.2f");
        assert_eq!(fs.symbol, Some('$'));
        assert_eq!(fs.parentheses, true);
        assert_eq!(fs.precision, Some(2));
        assert_eq!(fs.ty, 'f');
    }

    #[test]
    fn test_grouping_and_comma() {
        let fs = parse_specifier(",.2f");
        assert_eq!(fs.comma, true);
        assert_eq!(fs.precision, Some(2));
        assert_eq!(fs.ty, 'f');
        let fs2 = parse_specifier("_.2f");
        assert_eq!(fs2.grouping, Some('_'));
        assert_eq!(fs2.precision, Some(2));
        assert_eq!(fs2.ty, 'f');
    }

    #[test]
    fn test_type_only() {
        let fs = parse_specifier("d");
        assert_eq!(fs.ty, 'd');
    }

    #[test]
    fn test_all_options() {
        let fs = parse_specifier("#<+0$(_10,.2f");
        assert_eq!(fs.symbol, Some('#'));
        assert_eq!(fs.align, Some('<'));
        assert_eq!(fs.sign, Some('+'));
        assert_eq!(fs.zero, true);
        assert_eq!(fs.symbol, Some('#'));
        assert_eq!(fs.parentheses, false); // No '('
        assert_eq!(fs.grouping, Some('_'));
        assert_eq!(fs.width, Some(10));
        assert_eq!(fs.comma, true);
        assert_eq!(fs.precision, Some(2));
        assert_eq!(fs.ty, 'f');
    }

    #[test]
    fn test_parentheses_and_currency() {
        let fs = parse_specifier("$(10.2f");
        assert_eq!(fs.symbol, Some('$'));
        assert_eq!(fs.parentheses, true);
        assert_eq!(fs.width, Some(10));
        assert_eq!(fs.precision, Some(2));
        assert_eq!(fs.ty, 'f');
    }

    #[test]
    fn test_no_type() {
        let fs = parse_specifier(".2");
        assert_eq!(fs.precision, Some(2));
        assert_eq!(fs.ty, '\0');
    }

    #[test]
    fn test_only_width() {
        let fs = parse_specifier("8");
        assert_eq!(fs.width, Some(8));
        assert_eq!(fs.ty, '\0');
    }

    #[test]
    fn test_only_precision() {
        let fs = parse_specifier(".3");
        assert_eq!(fs.precision, Some(3));
        assert_eq!(fs.ty, '\0');
    }

    #[test]
    fn test_fill_align_and_sign() {
        let fs = parse_specifier("_>+10.1f");
        assert_eq!(fs.fill, Some('_'));
        assert_eq!(fs.align, Some('>'));
        assert_eq!(fs.sign, Some('+'));
        assert_eq!(fs.width, Some(10));
        assert_eq!(fs.precision, Some(1));
        assert_eq!(fs.ty, 'f');
    }
}
