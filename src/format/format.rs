use super::parse_specifier::parse_specifier;
use super::format_type::format_type;
use super::format_grouping::format_grouping;
use super::locale::Locale;

fn apply_fill_align_width(s: &str, fs: &super::specifier::FormatSpecifier, _is_negative: bool, _sign_char: Option<char>, parentheses: bool) -> String {
    let mut s = s.to_string();
    // Add sign if needed (skip if using parentheses)
    if !parentheses {
        if let Some(sign) = _sign_char {
            if !s.starts_with(sign) {
                s = format!("{}{}", sign, s);
            }
        }
    }
    let width = fs.width.unwrap_or(0);
    let fill = fs.fill.unwrap_or(' ');
    let align = fs.align.unwrap_or('>');
    let len = s.chars().count();
    // eprintln!("[DEBUG] apply_fill_align_width: s='{}', width={}, len={}, fill='{}', align='{}'", s, width, len, fill, align);
    if width > len {
        let pad = width - len;
        // eprintln!("[DEBUG] padding needed: pad={}", pad);
        match align {
            '<' => s = format!("{}{}", s, fill.to_string().repeat(pad)),
            '>' => s = format!("{}{}", fill.to_string().repeat(pad), s),
            '^' => {
                let left = pad / 2;
                let right = pad - left;
                s = format!("{}{}{}", fill.to_string().repeat(left), s, fill.to_string().repeat(right));
            },
            '=' => {
                // Pad after sign (if present)
                if !parentheses && _sign_char.is_some() && s.chars().count() > 1 {
                    let sign_char = s.chars().next().unwrap();
                    let rest: String = s.chars().skip(1).collect();
                    s = format!("{}{}{}", sign_char, fill.to_string().repeat(pad), rest);
                } else {
                    s = format!("{}{}", fill.to_string().repeat(pad), s);
                }
            },
            _ => s = format!("{}{}", fill.to_string().repeat(pad), s),
        }
        // eprintln!("[DEBUG] after padding: s='{}', len={}", s, s.chars().count());
    }
    s
}

pub fn format(spec: &str, x: f64) -> String {
    let fs = parse_specifier(spec);
    let locale = fs.locale.as_ref().map(|tag| Locale::from_tag(tag)).unwrap_or(Locale::default());
    let mut s = format_type(x, &fs);
    // Grouping: comma or underscore
    if fs.comma {
        s = format_grouping(&s, &locale.thousands);
    } else if let Some('_') = fs.grouping {
        s = s
            .chars()
            .rev()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("_")
            .chars()
            .rev()
            .collect();
    }
    // Currency
    if fs.symbol == Some('$') {
        let (prefix, suffix) = &locale.currency;
        s = format!("{}{}{}", prefix, s, suffix);
    }
    // Parentheses for negative numbers
    let mut parentheses = false;
    if fs.parentheses && x < 0.0 {
        s = format!("({})", s.trim_start_matches('-'));
        parentheses = true;
    }
    // Sign handling
    let is_negative = x.is_sign_negative() && x != 0.0;
    let sign_char = if is_negative {
        Some('-')
    } else if let Some(sign) = fs.sign {
        match sign {
            '+' => Some('+'),
            ' ' => Some(' '),
            _ => None,
        }
    } else {
        None
    };
    // Zero padding: if zero is set and width is set, use '0' as fill and '=' as align
    let mut fs_for_fill = fs.clone();
    if fs.zero && fs.width.is_some() {
        fs_for_fill.fill = Some('0');
        fs_for_fill.align = Some('=');
    }
    s = apply_fill_align_width(&s, &fs_for_fill, is_negative, sign_char, parentheses);
    s
}
