pub fn format_grouping(s: &str, thousands: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut count = 0;
    for c in chars.iter().rev() {
        if count != 0 && count % 3 == 0 {
            result.push_str(thousands);
        }
        result.push(*c);
        count += 1;
    }
    result.chars().rev().collect()
}
