// d3-time parity root
// Re-export all submodules here as you implement them

pub mod format;
pub use format::{
    default_format, iso_format, multi_format, time_format_with_locale, time_parse_with_locale,
};
pub mod locale;
pub use locale::TimeLocale;

use chrono::TimeZone;
use chrono::{Datelike, Duration, NaiveDateTime, Utc};

// Stubs for D3 time formatting and intervals
pub fn time_format_stub(_specifier: &str) -> impl Fn(&str) -> String {
    |_x| String::from("[time_format stub]")
}

pub struct TimeIntervalStub;
impl TimeIntervalStub {
    pub fn floor(&self, _date: &str) -> String {
        String::from("[floor stub]")
    }
    pub fn ceil(&self, _date: &str) -> String {
        String::from("[ceil stub]")
    }
    pub fn offset(&self, _date: &str, _step: i32) -> String {
        String::from("[offset stub]")
    }
    pub fn range(&self, _start: &str, _stop: &str, _step: i32) -> Vec<String> {
        vec![String::from("[range stub]")]
    }
}

// UTC and local formatters (stubs)
pub fn utc_format(spec: &str, date: &chrono::NaiveDateTime) -> String {
    format::time_format_with_locale::<chrono::Utc>(spec, date, &locale::TimeLocale::default(), true)
}

pub fn local_format(spec: &str, date: &chrono::NaiveDateTime) -> String {
    format::time_format_with_locale::<chrono::Local>(
        spec,
        date,
        &locale::TimeLocale::default(),
        false,
    )
}

// Interval traits and stubs
pub trait TimeInterval {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime;
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime;
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime;
    fn range(&self, start: NaiveDateTime, stop: NaiveDateTime, step: i32) -> Vec<NaiveDateTime>;
    fn count(&self, _start: NaiveDateTime, _stop: NaiveDateTime) -> i32 {
        0
    }
    fn every(&self, _step: i32) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
}

#[derive(Default, Clone)]
pub struct Second;
impl TimeInterval for Second {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        date - Duration::nanoseconds(date.and_utc().timestamp_subsec_nanos() as i64 % 1_000_000_000)
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        self.floor(date) + Duration::nanoseconds(1_000_000_000)
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        date + Duration::nanoseconds(1_000_000_000 * step as i64)
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

#[derive(Default, Clone)]
pub struct Minute;
impl TimeInterval for Minute {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        date - Duration::nanoseconds(
            date.and_utc().timestamp_subsec_nanos() as i64 % 60_000_000_000,
        )
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        self.floor(date) + Duration::nanoseconds(60_000_000_000)
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        date + Duration::nanoseconds(60_000_000_000 * step as i64)
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

#[derive(Default, Clone)]
pub struct Hour;
impl TimeInterval for Hour {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        date - Duration::nanoseconds(
            date.and_utc().timestamp_subsec_nanos() as i64 % 3_600_000_000_000,
        )
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        self.floor(date) + Duration::nanoseconds(3_600_000_000_000)
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        date + Duration::nanoseconds(3_600_000_000_000 * step as i64)
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

// Day interval (midnight)
#[derive(Default, Clone)]
pub struct Day;
impl TimeInterval for Day {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), date.day())
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        self.floor(date) + chrono::Duration::try_days(1).unwrap()
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        date + chrono::Duration::try_days(step as i64).unwrap()
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

// UTC Day interval (midnight in UTC)
#[derive(Default, Clone)]
pub struct UtcDay;
impl TimeInterval for UtcDay {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        // Convert NaiveDateTime to DateTime<Utc> using the recommended method
        let dt_utc = Utc.from_utc_datetime(&date);
        let d = dt_utc.date_naive();
        d.and_hms_opt(0, 0, 0).unwrap()
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        self.floor(date) + chrono::Duration::try_days(1).unwrap()
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        date + chrono::Duration::try_days(step as i64).unwrap()
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
#[derive(Default, Clone)]
pub struct Week;
impl TimeInterval for Week {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        let weekday = date.weekday().num_days_from_sunday() as i64;
        let d = date.date() - chrono::Duration::try_days(weekday).unwrap();
        d.and_hms_opt(0, 0, 0).unwrap()
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        self.floor(date) + chrono::Duration::try_days(7).unwrap()
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        date + chrono::Duration::try_days(7 * step as i64).unwrap()
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
#[derive(Default, Clone)]
pub struct Month;
impl TimeInterval for Month {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        let mut year = date.year();
        let mut month = date.month() + 1;
        if month > 12 {
            year += 1;
            month = 1;
        }
        chrono::NaiveDate::from_ymd_opt(year, month, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
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
        chrono::NaiveDate::from_ymd_opt(year, month as u32, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
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
#[derive(Default, Clone)]
pub struct Year;
impl TimeInterval for Year {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(date.year(), 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(date.year() + 1, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        let new_year = date.year() + step;
        // Handle edge cases for year overflow/underflow
        if new_year < 1 || new_year > 9999 {
            return date; // Return original date if year is out of bounds
        }
        chrono::NaiveDate::from_ymd_opt(new_year, 1, 1)
            .unwrap_or(date.date())
            .and_hms_opt(0, 0, 0)
            .unwrap_or(date)
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

// --- D3.js timeEvery/interval.every parity ---

/// Returns an interval that advances by the given step, or None if step < 1.
pub fn time_every<I: TimeInterval + Default>(step: i32) -> Option<Every<I>> {
    if step < 1 {
        return None;
    }
    Some(Every {
        interval: I::default(),
        step,
    })
}

/// An interval that advances by a given step (e.g., every 2 days).
pub struct Every<I: TimeInterval> {
    interval: I,
    step: i32,
}

impl<I: TimeInterval + Clone> TimeInterval for Every<I> {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        let base = self.interval.floor(date);
        let mut count = 0;
        while self
            .interval
            .offset(self.interval.floor(date), -self.step * count)
            < base
        {
            count += 1;
        }
        self.interval.offset(base, self.step * count)
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        let f = self.floor(date);
        if f < date { self.offset(f, 1) } else { f }
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        self.interval.offset(date, self.step * step)
    }
    fn range(&self, start: NaiveDateTime, stop: NaiveDateTime, _step: i32) -> Vec<NaiveDateTime> {
        let mut v = Vec::new();
        let mut d = self.ceil(start);
        while d < stop {
            v.push(d);
            d = self.offset(d, 1);
        }
        v
    }
    fn count(&self, start: NaiveDateTime, stop: NaiveDateTime) -> i32 {
        let mut count = 0;
        let mut d = self.ceil(start);
        while d < stop {
            count += 1;
            d = self.offset(d, 1);
        }
        count
    }
    fn every(&self, step: i32) -> Option<Self>
    where
        Self: Sized,
    {
        if step < 1 {
            None
        } else {
            Some(Self {
                interval: self.interval.clone(),
                step: self.step * step,
            })
        }
    }
}

macro_rules! impl_every {
    ($name:ident) => {
        impl $name {
            pub fn every(step: i32) -> Option<Every<$name>> {
                time_every::<$name>(step)
            }
        }
    };
}

impl_every!(Second);
impl_every!(Minute);
impl_every!(Hour);
impl_every!(Day);
impl_every!(Week);
impl_every!(Month);
impl_every!(Year);

// Custom week start intervals
#[derive(Default, Clone)]
pub struct Sunday;
impl TimeInterval for Sunday {
    fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
        let weekday = date.weekday().num_days_from_sunday() as i64;
        let d = date.date() - chrono::Duration::try_days(weekday).unwrap();
        d.and_hms_opt(0, 0, 0).unwrap()
    }
    fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
        self.floor(date) + chrono::Duration::try_days(7).unwrap()
    }
    fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
        date + chrono::Duration::try_days(7 * step as i64).unwrap()
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

macro_rules! week_start_interval {
    ($name:ident, $weekday:expr) => {
        #[derive(Default, Clone)]
        pub struct $name;
        impl TimeInterval for $name {
            fn floor(&self, date: NaiveDateTime) -> NaiveDateTime {
                let weekday = (7 + date.weekday().num_days_from_sunday() as i64 - $weekday) % 7;
                let d = date.date() - chrono::Duration::try_days(weekday).unwrap();
                d.and_hms_opt(0, 0, 0).unwrap()
            }
            fn ceil(&self, date: NaiveDateTime) -> NaiveDateTime {
                self.floor(date) + chrono::Duration::try_days(7).unwrap()
            }
            fn offset(&self, date: NaiveDateTime, step: i32) -> NaiveDateTime {
                date + chrono::Duration::try_days(7 * step as i64).unwrap()
            }
            fn range(
                &self,
                start: NaiveDateTime,
                stop: NaiveDateTime,
                step: i32,
            ) -> Vec<NaiveDateTime> {
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

week_start_interval!(Monday, 1);
week_start_interval!(Tuesday, 2);
week_start_interval!(Wednesday, 3);
week_start_interval!(Thursday, 4);
week_start_interval!(Friday, 5);
week_start_interval!(Saturday, 6);

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeZone, Utc};
    #[test]
    fn test_day_interval() {
        let day = Day;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(15, 30, 0)
            .unwrap();
        assert_eq!(
            day.floor(d),
            NaiveDate::from_ymd_opt(2025, 7, 8)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }
    #[test]
    fn test_utc_day_interval() {
        let utc_day = UtcDay;
        let d = Utc
            .with_ymd_and_hms(2025, 7, 8, 15, 30, 0)
            .unwrap()
            .naive_utc();
        assert_eq!(
            utc_day.floor(d),
            Utc.with_ymd_and_hms(2025, 7, 8, 0, 0, 0)
                .unwrap()
                .naive_utc()
        );
    }
    #[test]
    fn test_week_interval() {
        let week = Week;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(15, 30, 0)
            .unwrap();
        assert_eq!(
            week.floor(d),
            NaiveDate::from_ymd_opt(2025, 7, 6)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }
    #[test]
    fn test_month_interval() {
        let month = Month;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(15, 30, 0)
            .unwrap();
        assert_eq!(
            month.floor(d),
            NaiveDate::from_ymd_opt(2025, 7, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }
    #[test]
    fn test_year_interval() {
        let year = Year;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(15, 30, 0)
            .unwrap();
        assert_eq!(
            year.floor(d),
            NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }
    #[test]
    fn test_time_format_stub() {
        let f = time_format_stub("%Y-%m-%d");
        assert_eq!(f("2025-07-08"), "[time_format stub]");
    }
    #[test]
    fn test_time_interval_stub() {
        let interval = TimeIntervalStub;
        assert_eq!(interval.floor("2025-07-08"), "[floor stub]");
        assert_eq!(interval.ceil("2025-07-08"), "[ceil stub]");
        assert_eq!(interval.offset("2025-07-08", 1), "[offset stub]");
        assert_eq!(
            interval.range("2025-07-08", "2025-07-09", 1),
            vec!["[range stub]".to_string()]
        );
    }
    #[test]
    fn test_every_day() {
        let every_2_days = Day::every(2).unwrap();
        let start = NaiveDate::from_ymd_opt(2025, 7, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let stop = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let range = every_2_days.range(start, stop, 1);
        assert_eq!(range.len(), 4);
        assert_eq!(range[0], start);
        assert_eq!(range[1], start + chrono::Duration::try_days(2).unwrap());
        assert_eq!(range[2], start + chrono::Duration::try_days(4).unwrap());
        assert_eq!(range[3], start + chrono::Duration::try_days(6).unwrap());
    }
    #[test]
    fn test_time_every_fn() {
        let every_3_weeks = time_every::<Week>(3).unwrap();
        let start = NaiveDate::from_ymd_opt(2025, 7, 6)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let stop = NaiveDate::from_ymd_opt(2025, 8, 17)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let range = every_3_weeks.range(start, stop, 1);
        assert_eq!(range.len(), 2);
        assert_eq!(range[0], start);
        assert_eq!(range[1], start + chrono::Duration::try_days(21).unwrap());
    }
    #[test]
    fn test_monday_interval() {
        let monday = Monday;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(15, 30, 0)
            .unwrap(); // Tuesday
        assert_eq!(
            monday.floor(d),
            NaiveDate::from_ymd_opt(2025, 7, 7)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }
    #[test]
    fn test_sunday_interval() {
        let sunday = Sunday;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(15, 30, 0)
            .unwrap(); // Tuesday
        assert_eq!(
            sunday.floor(d),
            NaiveDate::from_ymd_opt(2025, 7, 6)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }
    #[test]
    fn test_thursday_interval() {
        let thursday = Thursday;
        let d = NaiveDate::from_ymd_opt(2025, 7, 8)
            .unwrap()
            .and_hms_opt(15, 30, 0)
            .unwrap(); // Tuesday
        assert_eq!(
            thursday.floor(d),
            NaiveDate::from_ymd_opt(2025, 7, 3)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }
}
