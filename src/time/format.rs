// d3-time-format stub
// Implements timeFormat and timeParse (strftime/strptime-like)

use crate::time::locale::TimeLocale;
use chrono::{Datelike, Local, NaiveDateTime, TimeZone, Timelike, Utc};

/// Format a date/time using a specifier string and locale.
pub fn time_format_with_locale<Tz: TimeZone>(
    spec: &str,
    date: &NaiveDateTime,
    locale: &TimeLocale,
    _is_utc: bool,
) -> String {
    let mut out = String::new();
    let mut chars = spec.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let mut code = String::from("%");
            // Support for padding modifiers: %-d, %_d, %0d, etc.
            let mut pad = None;
            if let Some(&next) = chars.peek() {
                if next == '-' || next == '_' || next == '0' {
                    pad = Some(next);
                    code.push(next);
                    chars.next();
                }
            }
            if let Some(&next) = chars.peek() {
                code.push(next);
                chars.next();
                out.push_str(&match code.as_str() {
                    "%Y" => format!("{:04}", date.year()),
                    "%m" => format!("{:02}", date.month()),
                    "%d" | "%-d" | "%_d" | "%0d" => match pad {
                        Some('-') => format!("{}", date.day()),
                        Some('_') => format!("{:2}", date.day()),
                        Some('0') => format!("{:02}", date.day()),
                        _ => format!("{:02}", date.day()),
                    },
                    "%e" => format!("{:2}", date.day()),
                    "%H" => format!("{:02}", date.hour()),
                    "%M" => format!("{:02}", date.minute()),
                    "%S" => format!("{:02}", date.second()),
                    "%L" => format!("{:03}", date.and_utc().timestamp_subsec_millis()),
                    "%f" => format!("{:06}", date.and_utc().timestamp_subsec_micros()),
                    "%p" => {
                        if date.hour() < 12 {
                            locale.am.to_string()
                        } else {
                            locale.pm.to_string()
                        }
                    }
                    "%P" => {
                        if date.hour() < 12 {
                            locale.am.to_lowercase()
                        } else {
                            locale.pm.to_lowercase()
                        }
                    }
                    "%a" => {
                        let weekday = date.weekday().num_days_from_sunday() as usize;
                        locale.short_days[weekday].to_string()
                    }
                    "%A" => {
                        let weekday = date.weekday().num_days_from_sunday() as usize;
                        locale.days[weekday].to_string()
                    }
                    "%b" => {
                        let month = date.month0() as usize;
                        locale.short_months[month].to_string()
                    }
                    "%B" => {
                        let month = date.month0() as usize;
                        locale.months[month].to_string()
                    }
                    "%w" => format!("{}", date.weekday().num_days_from_sunday()),
                    "%j" => format!("{:03}", date.ordinal()),
                    "%U" => format!("{:02}", date.iso_week().week()),
                    "%W" => format!("{:02}", date.iso_week().week()),
                    "%V" => format!("{:02}", date.iso_week().week()),
                    "%C" => format!("{:02}", date.year() / 100),
                    "%y" => format!("{:02}", date.year() % 100),
                    "%I" => {
                        let h = date.hour() % 12;
                        format!("{:02}", if h == 0 { 12 } else { h })
                    }
                    "%s" => format!("{}", date.and_utc().timestamp()),
                    "%Q" => format!("{:03}", (date.month0() / 3) + 1), // Quarter
                    "%q" => format!("{}", (date.month0() / 3) + 1),
                    "%D" => time_format_with_locale::<Tz>("%m/%d/%y", date, locale, _is_utc),
                    "%F" => time_format_with_locale::<Tz>("%Y-%m-%d", date, locale, _is_utc),
                    "%R" => time_format_with_locale::<Tz>("%H:%M", date, locale, _is_utc),
                    "%T" => time_format_with_locale::<Tz>("%H:%M:%S", date, locale, _is_utc),
                    "%r" => time_format_with_locale::<Tz>("%I:%M:%S %p", date, locale, _is_utc),
                    "%c" => {
                        time_format_with_locale::<Tz>("%a %b %e %H:%M:%S %Y", date, locale, _is_utc)
                    }
                    "%x" => time_format_with_locale::<Tz>("%m/%d/%y", date, locale, _is_utc),
                    "%X" => time_format_with_locale::<Tz>("%H:%M:%S", date, locale, _is_utc),
                    "%Z" => {
                        if _is_utc {
                            "UTC".to_string()
                        } else {
                            Local::now().format("%Z").to_string()
                        }
                    }
                    "%z" => {
                        if _is_utc {
                            "+0000".to_string()
                        } else {
                            Local::now().format("%z").to_string()
                        }
                    }
                    "%%" => "%".to_string(),
                    // ...add more specifiers as needed...
                    _ => "[time_format stub]".to_string(),
                });
            } else {
                out.push('%');
            }
        } else if c == '\\' {
            // Escaping: allow \\% to output a literal %
            if let Some('%') = chars.peek() {
                chars.next();
                out.push('%');
            } else {
                out.push('\\');
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Format a date/time using the default locale (backward compatible)
pub fn time_format(spec: &str, date: &NaiveDateTime) -> String {
    time_format_with_locale::<Utc>(spec, date, &TimeLocale::default(), true)
}

/// Parse a date/time string using a specifier and locale (stub)
pub fn time_parse_with_locale(
    spec: &str,
    s: &str,
    locale: &TimeLocale,
    _is_utc: bool,
) -> Option<NaiveDateTime> {
    use chrono::NaiveDate;
    let mut year = None;
    let mut month = None;
    let mut day = None;
    let mut hour = Some(0);
    let mut minute = Some(0);
    let mut second = Some(0);
    let mut millis = Some(0);
    let mut ampm = None;
    let mut idx = 0;
    let chars: Vec<char> = s.chars().collect();
    let spec_chars: Vec<char> = spec.chars().collect();
    let mut i = 0;
    while i < spec_chars.len() {
        if spec_chars[i] == '%' {
            i += 1;
            if i >= spec_chars.len() {
                break;
            }
            let code = spec_chars[i];
            match code {
                'Y' => {
                    if idx + 4 > chars.len() {
                        return None;
                    }
                    year = s[idx..idx + 4].parse().ok();
                    idx += 4;
                }
                'y' => {
                    if idx + 2 > chars.len() {
                        return None;
                    }
                    let y: i32 = s[idx..idx + 2].parse().ok()?;
                    year = Some(if y < 50 { 2000 + y } else { 1900 + y });
                    idx += 2;
                }
                'm' => {
                    if idx + 2 > chars.len() {
                        return None;
                    }
                    month = s[idx..idx + 2].parse().ok();
                    idx += 2;
                }
                'b' => {
                    // Short month name
                    let mut found = false;
                    for (i, &name) in locale.short_months.iter().enumerate() {
                        if s[idx..].starts_with(name) {
                            month = Some(i as u32 + 1);
                            idx += name.len();
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return None;
                    }
                }
                'B' => {
                    // Full month name, allow whitespace before/after
                    let mut found = false;
                    let s_trim = s[idx..].trim_start();
                    let offset = s[idx..].len() - s_trim.len();
                    for (i, &name) in locale.months.iter().enumerate() {
                        if s_trim.starts_with(name) {
                            month = Some(i as u32 + 1);
                            idx += offset + name.len();
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return None;
                    }
                }
                'd' | 'e' => {
                    // Accept 1 or 2 digit day
                    let mut len = 2;
                    if idx + 2 > chars.len() || !s[idx..idx + 2].chars().all(|c| c.is_ascii_digit())
                    {
                        len = 1;
                    }
                    if idx + len > chars.len() {
                        return None;
                    }
                    day = s[idx..idx + len].trim().parse().ok();
                    idx += len;
                }
                'H' => {
                    if idx + 2 > chars.len() {
                        return None;
                    }
                    hour = s[idx..idx + 2].parse().ok();
                    idx += 2;
                }
                'I' => {
                    if idx + 2 > chars.len() {
                        return None;
                    }
                    hour = s[idx..idx + 2].parse().ok();
                    idx += 2;
                }
                'M' => {
                    if idx + 2 > chars.len() {
                        return None;
                    }
                    minute = s[idx..idx + 2].parse().ok();
                    idx += 2;
                }
                'S' => {
                    if idx + 2 > chars.len() {
                        return None;
                    }
                    second = s[idx..idx + 2].parse().ok();
                    idx += 2;
                }
                'L' => {
                    if idx + 3 > chars.len() {
                        return None;
                    }
                    millis = s[idx..idx + 3].parse().ok();
                    idx += 3;
                }
                'f' => {
                    if idx + 6 > chars.len() {
                        return None;
                    }
                    millis = s[idx..idx + 6].parse().ok().map(|v: u32| v / 1000);
                    idx += 6;
                }
                'p' | 'P' => {
                    if s[idx..].starts_with(locale.am)
                        || s[idx..].starts_with(&locale.am.to_lowercase())
                    {
                        ampm = Some("AM");
                        idx += locale.am.len();
                    } else if s[idx..].starts_with(locale.pm)
                        || s[idx..].starts_with(&locale.pm.to_lowercase())
                    {
                        ampm = Some("PM");
                        idx += locale.pm.len();
                    } else {
                        return None;
                    }
                }
                'j' => {
                    if idx + 3 > chars.len() {
                        return None;
                    }
                    let ordinal: u32 = s[idx..idx + 3].parse().ok()?;
                    let y = year.unwrap_or(1970);
                    let d = NaiveDate::from_yo_opt(y, ordinal)?;
                    year = Some(d.year());
                    month = Some(d.month());
                    day = Some(d.day());
                    idx += 3;
                }
                's' => {
                    // Unix timestamp
                    let mut end = idx;
                    while end < chars.len() && chars[end].is_ascii_digit() {
                        end += 1;
                    }
                    let ts: i64 = s[idx..end].parse().ok()?;
                    let ndt = chrono::Utc.timestamp_opt(ts, 0).single()?;
                    return Some(ndt.naive_utc());
                }
                // Add more specifiers as needed
                _ => return None,
            }
        } else {
            // Match literal
            if idx >= chars.len() || chars[idx] != spec_chars[i] {
                return None;
            }
            idx += 1;
        }
        i += 1;
    }
    // Post-process AM/PM
    let mut hour_val = hour.unwrap_or(0);
    if let Some(ap) = ampm {
        if ap == "PM" && hour_val < 12 {
            hour_val += 12;
        } else if ap == "AM" && hour_val == 12 {
            hour_val = 0;
        }
    }
    let y = year.unwrap_or(1970);
    let m = month.unwrap_or(1);
    let d = day.unwrap_or(1);
    let h = hour_val;
    let min = minute.unwrap_or(0);
    let s = second.unwrap_or(0);
    let ms = millis.unwrap_or(0);
    NaiveDate::from_ymd_opt(y, m, d).and_then(|date| date.and_hms_milli_opt(h, min, s, ms))
}

/// Parse a date/time string using the default locale (stub)
pub fn time_parse(spec: &str, s: &str) -> Option<NaiveDateTime> {
    time_parse_with_locale(spec, s, &TimeLocale::default(), true)
}

/// Predefined formatters (stubs)
pub fn iso_format(date: &NaiveDateTime) -> String {
    date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

pub fn default_format(date: &NaiveDateTime) -> String {
    time_format("%c", date)
}

/// Multi-format (tick formatting) stub
pub fn multi_format<'a, F: Fn(&NaiveDateTime) -> bool>(
    formats: Vec<(&'a str, F)>,
    date: &NaiveDateTime,
) -> String {
    for (spec, pred) in formats {
        if pred(date) {
            return time_format(spec, date);
        }
    }
    time_format("%c", date)
}

/// Allow custom locale injection in the future
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn test_time_format_year() {
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(15, 30, 0)
            .unwrap();
        assert_eq!(time_format("%Y", &d), "2025");
    }
    #[test]
    fn test_time_format_month() {
        let d = NaiveDate::from_ymd_opt(2025, 2, 8)
            .unwrap()
            .and_hms_opt(15, 30, 0)
            .unwrap();
        assert_eq!(time_format("%m", &d), "02");
        assert_eq!(time_format("%b", &d), "Feb");
        assert_eq!(time_format("%B", &d), "February");
    }
    #[test]
    fn test_time_format_day() {
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(15, 30, 0)
            .unwrap();
        assert_eq!(time_format("%d", &d), "08");
        assert_eq!(time_format("%a", &d), "Tue");
        assert_eq!(time_format("%A", &d), "Tuesday");
    }
    #[test]
    fn test_time_format_hour_minute_second() {
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_milli_opt(1, 2, 3, 4)
            .unwrap();
        assert_eq!(time_format("%H", &d), "01");
        assert_eq!(time_format("%I", &d), "01");
        assert_eq!(time_format("%M", &d), "02");
        assert_eq!(time_format("%S", &d), "03");
        assert_eq!(time_format("%L", &d), "004");
        assert_eq!(time_format("%f", &d), "004000");
    }
    #[test]
    fn test_time_format_am_pm() {
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(1, 0, 0)
            .unwrap();
        assert_eq!(time_format("%p", &d), "AM");
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(13, 0, 0)
            .unwrap();
        assert_eq!(time_format("%p", &d), "PM");
    }
    #[test]
    fn test_time_format_composite() {
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(15, 2, 3)
            .unwrap();
        assert_eq!(time_format("%Y-%m-%d %H:%M:%S", &d), "2025-07-08 15:02:03");
    }
    #[test]
    fn test_time_parse_basic() {
        let s = "2025-07-08 15:30:00";
        let spec = "%Y-%m-%d %H:%M:%S";
        let dt = time_parse(spec, s).unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 7);
        assert_eq!(dt.day(), 8);
        assert_eq!(dt.hour(), 15);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 0);
    }
    #[test]
    fn test_time_parse_short_month() {
        let s = "08-Feb-25 01:02:03";
        let spec = "%d-%b-%y %H:%M:%S";
        let dt = time_parse(spec, s).unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 2);
        assert_eq!(dt.day(), 8);
        assert_eq!(dt.hour(), 1);
        assert_eq!(dt.minute(), 2);
        assert_eq!(dt.second(), 3);
    }
    #[test]
    fn test_time_parse_full_month() {
        let s = "8 February 2025 13:02:03";
        let spec = "%d %B %Y %H:%M:%S";
        let dt = time_parse(spec, s).unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 2);
        assert_eq!(dt.day(), 8);
        assert_eq!(dt.hour(), 13);
        assert_eq!(dt.minute(), 2);
        assert_eq!(dt.second(), 3);
    }
    #[test]
    fn test_time_parse_am_pm() {
        let s = "08-02-2025 01:02:03 PM";
        let spec = "%d-%m-%Y %I:%M:%S %p";
        let dt = time_parse(spec, s).unwrap();
        assert_eq!(dt.hour(), 13);
        let s = "08-02-2025 12:02:03 AM";
        let dt = time_parse(spec, s).unwrap();
        assert_eq!(dt.hour(), 0);
    }
    #[test]
    fn test_time_parse_millis() {
        let s = "2025-07-08 15:30:00.123";
        let spec = "%Y-%m-%d %H:%M:%S.%L";
        let dt = time_parse(spec, s).unwrap();
        assert_eq!(dt.and_utc().timestamp_subsec_millis(), 123);
    }
    #[test]
    fn test_time_parse_unix_timestamp() {
        let dt = time_parse("%s", "1752105600").unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 7);
        assert_eq!(dt.day(), 10);
    }
    #[test]
    fn test_time_parse_ordinal() {
        let s = "2025 189";
        let spec = "%Y %j";
        let dt = time_parse(spec, s).unwrap();
        assert_eq!(dt.month(), 7);
        assert_eq!(dt.day(), 8);
    }
    #[test]
    fn test_time_parse_fail() {
        assert!(time_parse("%Y-%m-%d", "not-a-date").is_none());
        assert!(time_parse("%d-%b-%y", "31-Foo-99").is_none());
    }
}
