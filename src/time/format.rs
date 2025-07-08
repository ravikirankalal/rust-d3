// d3-time-format stub
// Implements timeFormat and timeParse (strftime/strptime-like)

use chrono::{NaiveDateTime, Datelike, Timelike};

pub fn time_format(spec: &str, date: &NaiveDateTime) -> String {
    let mut out = String::new();
    let mut chars = spec.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let mut code = String::from("%");
            if let Some(&next) = chars.peek() {
                code.push(next);
                chars.next();
                out.push_str(&match code.as_str() {
                    "%Y" => format!("{:04}", date.year()),
                    "%m" => format!("{:02}", date.month()),
                    "%d" => format!("{:02}", date.day()),
                    "%H" => format!("{:02}", date.hour()),
                    "%M" => format!("{:02}", date.minute()),
                    "%S" => format!("{:02}", date.second()),
                    "%L" => format!("{:03}", date.and_utc().timestamp_subsec_millis()),
                    "%p" => if date.hour() < 12 { "AM".to_string() } else { "PM".to_string() },
                    "%a" => {
                        // Short weekday name
                        let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
                        let weekday = date.weekday().num_days_from_sunday() as usize;
                        days[weekday].to_string()
                    },
                    "%A" => {
                        // Full weekday name
                        let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
                        let weekday = date.weekday().num_days_from_sunday() as usize;
                        days[weekday].to_string()
                    },
                    "%b" => {
                        // Short month name
                        let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
                        let month = date.month0() as usize;
                        months[month].to_string()
                    },
                    "%B" => {
                        // Full month name
                        let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
                        let month = date.month0() as usize;
                        months[month].to_string()
                    },
                    "%w" => format!("{}", date.weekday().num_days_from_sunday()),
                    "%j" => format!("{:03}", date.ordinal()),
                    "%U" => format!("{:02}", date.iso_week().week()), // Not exact D3, but close
                    "%W" => format!("{:02}", date.iso_week().week()), // Not exact D3, but close
                    "%C" => format!("{:02}", date.year() / 100),
                    "%y" => format!("{:02}", date.year() % 100),
                    "%I" => {
                        let h = date.hour() % 12;
                        format!("{:02}", if h == 0 { 12 } else { h })
                    },
                    "%f" => format!("{:06}", date.and_utc().timestamp_subsec_micros()),
                    "%s" => format!("{}", date.and_utc().timestamp()),
                    // Add more specifiers as needed
                    _ => "[time_format stub]".to_string(),
                });
            } else {
                out.push('%');
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn time_parse(_spec: &str, _s: &str) -> Option<NaiveDateTime> {
    // TODO: Implement full specifier support
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn test_time_format_year() {
        let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(15, 30, 0).unwrap();
        assert_eq!(time_format("%Y", &d), "2025");
    }
    #[test]
    fn test_time_format_month() {
        let d = NaiveDate::from_ymd_opt(2025, 2, 8).unwrap().and_hms_opt(15, 30, 0).unwrap();
        assert_eq!(time_format("%m", &d), "02");
        assert_eq!(time_format("%b", &d), "Feb");
        assert_eq!(time_format("%B", &d), "February");
    }
    #[test]
    fn test_time_format_day() {
        let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(15, 30, 0).unwrap();
        assert_eq!(time_format("%d", &d), "08");
        assert_eq!(time_format("%a", &d), "Tue");
        assert_eq!(time_format("%A", &d), "Tuesday");
    }
    #[test]
    fn test_time_format_hour_minute_second() {
        let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_milli_opt(1, 2, 3, 4).unwrap();
        assert_eq!(time_format("%H", &d), "01");
        assert_eq!(time_format("%I", &d), "01");
        assert_eq!(time_format("%M", &d), "02");
        assert_eq!(time_format("%S", &d), "03");
        assert_eq!(time_format("%L", &d), "004");
        assert_eq!(time_format("%f", &d), "004000");
    }
    #[test]
    fn test_time_format_am_pm() {
        let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(1, 0, 0).unwrap();
        assert_eq!(time_format("%p", &d), "AM");
        let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(13, 0, 0).unwrap();
        assert_eq!(time_format("%p", &d), "PM");
    }
    #[test]
    fn test_time_format_composite() {
        let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(15, 2, 3).unwrap();
        assert_eq!(time_format("%Y-%m-%d %H:%M:%S", &d), "2025-07-08 15:02:03");
    }
}
