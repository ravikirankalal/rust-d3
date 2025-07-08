pub fn format_prefix(x: f64, precision: usize) -> String {
    let abs = x.abs();
    let (value, suffix) = if abs >= 1_000_000.0 {
        (x / 1_000_000.0, "M")
    } else if abs >= 1_000.0 {
        (x / 1_000.0, "k")
    } else {
        (x, "")
    };
    // eprintln!("[DEBUG] x: {x}, value: {value}, suffix: {suffix}, precision: {precision}");
    if suffix.is_empty() {
        // For no prefix, show all significant digits
        let s = format!("{:.*}", precision.saturating_sub(1), value);
        // eprintln!("[DEBUG] (no prefix) formatted: {s}");
        if s.contains('.') {
            let s = s.trim_end_matches('0').trim_end_matches('.');
            // eprintln!("[DEBUG] (no prefix) trimmed: {s}");
            return s.to_string();
        }
        return s;
    } else {
        // Always produce the requested number of significant digits
        let int_digits = if value.abs() >= 1.0 {
            value.abs().log10().floor() as usize + 1
        } else {
            1
        };
        let decimals = if precision > int_digits { precision - int_digits } else { 0 };
        // eprintln!("[DEBUG] int_digits: {int_digits}, decimals: {decimals}");
        let mut s = format!("{:.*}", decimals, value);
        // eprintln!("[DEBUG] initial formatted: {s}");
        // Only trim .0 for whole numbers if precision <= 2 (D3.js behavior)
        if s.contains('.') && s.split('.').nth(1).unwrap_or("").chars().all(|c| c == '0') && precision <= 2 {
            let trimmed = s.trim_end_matches('0').trim_end_matches('.');
            // eprintln!("[DEBUG] trim .0 for whole (precision<=2): {s} -> {trimmed}");
            s = trimmed.to_string();
        } else if s.contains('.') {
            let mut trimmed = s.as_str();
            loop {
                if trimmed.ends_with('0') && trimmed.contains('.') {
                    let without = &trimmed[..trimmed.len()-1];
                    let sigdigs = without.chars().filter(|c| c.is_ascii_digit()).count();
                    // eprintln!("[DEBUG] trimming 0: {trimmed} -> {without}, sigdigs: {sigdigs}");
                    if sigdigs >= precision {
                        trimmed = without;
                        continue;
                    }
                }
                if trimmed.ends_with('.') {
                    let without = &trimmed[..trimmed.len()-1];
                    let sigdigs = without.chars().filter(|c| c.is_ascii_digit()).count();
                    // eprintln!("[DEBUG] trimming .: {trimmed} -> {without}, sigdigs: {sigdigs}");
                    if sigdigs >= precision {
                        trimmed = without;
                        continue;
                    }
                }
                break;
            }
            s = trimmed.to_string();
            // eprintln!("[DEBUG] after trimming: {s}");
        }
        let result = format!("{}{}", s, suffix);
        // eprintln!("[DEBUG] final result: {result}");
        result
    }
}
