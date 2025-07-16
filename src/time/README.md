# rust-d3::time Module

The `time` module provides Rust implementations of D3.js time intervals, step intervals, custom week start intervals, and UTC intervals, enabling flexible date/time manipulation, range generation, and step-based iteration for time series and calendar data.

## Features

- **Time Intervals**: Second, Minute, Hour, Day, Week, Month, Year
- **Step Intervals**: Use `.every(n)` or `time_every::<Interval>(n)` to get intervals that advance by a custom step (e.g., every 2 days, every 3 weeks)
- **Custom Week Start Intervals**: Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday (e.g., `Monday` for ISO weeks)
- **UTC Intervals**: UtcDay (pattern for UtcWeek, UtcMonth, etc.)
- **Range Generation**: Generate Vecs of `chrono::NaiveDateTime` for any interval and step
- **Trait-based**: All intervals implement the `TimeInterval` trait for composability
- **D3 Parity**: API and behavior closely match D3.js's d3-time

---

## Usage Examples

### 1. Basic Interval Usage

```rust
use rust_d3::time::{Day, TimeInterval};
use chrono::NaiveDate;

let day = Day;
let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(15, 30, 0).unwrap();
assert_eq!(day.floor(d), NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(0, 0, 0).unwrap());
```

### 2. Step Intervals with `.every()`

```rust
use rust_d3::time::Day;
use chrono::NaiveDate;

let every_2_days = Day::every(2).unwrap();
let start = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
let stop = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(0, 0, 0).unwrap();
let range = every_2_days.range(start, stop, 1);
assert_eq!(range, vec![
    start,
    start + chrono::Duration::days(2),
    start + chrono::Duration::days(4),
    start + chrono::Duration::days(6),
]);
```

### 3. Custom Week Start Intervals

```rust
use rust_d3::time::Monday;
use chrono::NaiveDate;

let monday = Monday;
let d = NaiveDate::from_ymd_opt(2025, 7, 8).unwrap().and_hms_opt(15, 30, 0).unwrap(); // Tuesday
assert_eq!(monday.floor(d), NaiveDate::from_ymd_opt(2025, 7, 7).unwrap().and_hms_opt(0, 0, 0).unwrap());
```

### 4. UTC Intervals

```rust
use rust_d3::time::UtcDay;
use chrono::Utc;

let utc_day = UtcDay;
let d = Utc.with_ymd_and_hms(2025, 7, 8, 15, 30, 0).unwrap().naive_utc();
assert_eq!(utc_day.floor(d), Utc.with_ymd_and_hms(2025, 7, 8, 0, 0, 0).unwrap().naive_utc());
```

### 5. Generic Step Intervals with `time_every` Function

```rust
use rust_d3::time::{time_every, Week};
use chrono::NaiveDate;

let every_3_weeks = time_every::<Week>(3).unwrap();
let start = NaiveDate::from_ymd_opt(2025, 7, 6).unwrap().and_hms_opt(0, 0, 0).unwrap();
let stop = NaiveDate::from_ymd_opt(2025, 8, 17).unwrap().and_hms_opt(0, 0, 0).unwrap();
let range = every_3_weeks.range(start, stop, 1);
assert_eq!(range, vec![
    start,
    start + chrono::Duration::days(21),
]);
```

---

## API

- `TimeInterval` trait: floor, ceil, offset, range, count, every
- `Second`, `Minute`, `Hour`, `Day`, `Week`, `Month`, `Year`, `UtcDay`, `Monday`, ...: interval types
- `.every(n)`: returns a step interval (e.g., every 2 days)
- `time_every::<Interval>(n)`: generic step interval

---

## License
MIT
