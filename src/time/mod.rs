// d3-time parity root
// Re-export all submodules here as you implement them

pub mod format;
pub use format::time_format;
pub mod locale;
pub use locale::TimeLocale;

use chrono::{NaiveDateTime, Duration, Datelike, Timelike};

// Interval traits and stubs
pub trait TimeInterval {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime;
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime;
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime;
    fn range(&self, start: NaiveDateTime, stop: NaiveDateTime, step: i32) -> Vec<NaiveDateTime>;
    fn count(&self, start: NaiveDateTime, stop: NaiveDateTime) -> i32 { 0 }
    fn every(&self, _step: i32) -> Option<Self> where Self: Sized { None }
}

macro_rules! interval_stub {
    ($name:ident, $duration:expr) => {
        pub struct $name;
        impl TimeInterval for $name {
            fn floor(&self, date: NaiveDateTime) -> NaiveDateTime { date - Duration::nanoseconds(date.timestamp_subsec_nanos() as i64 % $duration) }
            fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime { self.floor(date) + Duration::nanoseconds($duration) }
            fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime { date + Duration::nanoseconds($duration * step as i64) }
            fn range(&self, start: NaiveDateTime, stop: NaiveDateTime, step: i32) -> Vec<NaiveDateTime> {
                let mut v = Vec::new();
                let mut d = self.floor(start);
                while d < stop {
                    v.push(d);
                    d = self.offset(d, step);
                }
                v
            }
        }
    };
}

interval_stub!(Second, 1_000_000_000);
interval_stub!(Minute, 60_000_000_000);
interval_stub!(Hour, 3_600_000_000_000);
// Day interval (midnight)
pub struct Day;
impl TimeInterval for Day {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap().and_hms_opt(0, 0, 0).unwrap()
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        self.floor(date) + chrono::Duration::days(1)
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        date + chrono::Duration::days(step as i64)
    }
    fn range(&self, start: NaiveDateTime, stop: NaiveDateTime, step: i32) -> Vec<NaiveDateTime> {
        let mut v = Vec::new();
        let mut d = self.floor(start);
        while d < stop {
            v.push(d);
            d = self.offset(d, step);
        }
        v
    }
}

// Week interval (starts on Sunday, like D3)
pub struct Week;
impl TimeInterval for Week {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        let weekday = date.weekday().num_days_from_sunday() as i64;
        let d = date.date() - chrono::Duration::days(weekday);
        d.and_hms_opt(0, 0, 0).unwrap()
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        self.floor(date) + chrono::Duration::days(7)
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        date + chrono::Duration::days(7 * step as i64)
    }
    fn range(&self, start: NaiveDateTime, stop: NaiveDateTime, step: i32) -> Vec<NaiveDateTime> {
        let mut v = Vec::new();
        let mut d = self.floor(start);
        while d < stop {
            v.push(d);
            d = self.offset(d, step);
        }
        v
    }
}

// Month interval (first of month)
pub struct Month;
impl TimeInterval for Month {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        let mut year = date.year();
        let mut month = date.month() + 1;
        if month > 12 {
            year += 1;
            month = 1;
        }
        chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        let mut year = date.year();
        let mut month = date.month() as i32 + step;
        while month > 12 {
            year += 1;
            month -= 12;
        }
        while month < 1 {
            year -= 1;
            month += 12;
        }
        chrono::NaiveDate::from_ymd_opt(year, month as u32, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
    }
    fn range(&self, start: NaiveDateTime, stop: NaiveDateTime, step: i32) -> Vec<NaiveDateTime> {
        let mut v = Vec::new();
        let mut d = self.floor(start);
        while d < stop {
            v.push(d);
            d = self.offset(d, step);
        }
        v
    }
}

// Year interval (first of year)
pub struct Year;
impl TimeInterval for Year {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(date.year() + step, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap()
    }
    fn range(&self, start: NaiveDateTime, stop: NaiveDateTime, step: i32) -> Vec<NaiveDateTime> {
        let mut v = Vec::new();
        let mut d = self.floor(start);
        while d < stop {
            v.push(d);
            d = self.offset(d, step);
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn test_day_interval() {
        let day = Day;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(15, 30, 0).unwrap();
        assert_eq!(day.floor(d), NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(0, 0, 0).unwrap());
    }
    #[test]
    fn test_week_interval() {
        let week = Week;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(15, 30, 0).unwrap();
        assert_eq!(week.floor(d), NaiveDate::from_ymd_opt(2025, 7, 6).unwrap().and_hms_opt(0, 0, 0).unwrap());
    }
    #[test]
    fn test_month_interval() {
        let month = Month;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(15, 30, 0).unwrap();
        assert_eq!(month.floor(d), NaiveDate::from_ymd_opt(2025, 7, 1).unwrap().and_hms_opt(0, 0, 0).unwrap());
    }
    #[test]
    fn test_year_interval() {
        let year = Year;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(15, 30, 0).unwrap();
        assert_eq!(year.floor(d), NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap());
    }
}
