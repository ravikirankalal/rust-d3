// d3-scale: scaleUtc (UTC time scale)

use chrono::{DateTime, Duration, TimeZone, Utc};

#[derive(Debug, Clone)]
pub struct ScaleUtc {
    domain: [DateTime<Utc>; 2],
    range: [f64; 2],
    clamp: bool,
    nice: bool,
}

impl ScaleUtc {
    pub fn new(domain: [DateTime<Utc>; 2], range: [f64; 2]) -> Self {
        Self {
            domain,
            range,
            clamp: false,
            nice: false,
        }
    }
    pub fn domain(&self) -> [DateTime<Utc>; 2] {
        self.domain
    }
    pub fn range(&self) -> [f64; 2] {
        self.range
    }
    pub fn clamp(mut self, clamp: bool) -> Self {
        self.clamp = clamp;
        self
    }
    pub fn nice(mut self) -> Self {
        self.nice = true;
        self
    }
    pub fn scale(&self, t: DateTime<Utc>) -> f64 {
        let (d0, d1) = (
            self.domain[0].timestamp() as f64,
            self.domain[1].timestamp() as f64,
        );
        let (r0, r1) = (self.range[0], self.range[1]);
        let mut v = t.timestamp() as f64;
        if self.clamp {
            v = v.max(d0).min(d1);
        }
        if (d1 - d0).abs() < std::f64::EPSILON {
            return r0;
        }
        r0 + (r1 - r0) * (v - d0) / (d1 - d0)
    }
    pub fn invert(&self, r: f64) -> DateTime<Utc> {
        let (d0, d1) = (
            self.domain[0].timestamp() as f64,
            self.domain[1].timestamp() as f64,
        );
        let (r0, r1) = (self.range[0], self.range[1]);
        let mut v = if (r1 - r0).abs() < std::f64::EPSILON {
            d0
        } else {
            d0 + (d1 - d0) * (r - r0) / (r1 - r0)
        };
        if self.clamp {
            v = v.max(d0).min(d1);
        }
        // Use unwrap_or_else to avoid panic if timestamp is out of range
        Utc.timestamp_opt(v as i64, 0)
            .single()
            .unwrap_or_else(|| self.domain[0])
    }
    pub fn ticks(&self, count: usize) -> Vec<DateTime<Utc>> {
        let (d0, d1) = (self.domain[0], self.domain[1]);
        let mut ticks = Vec::new();
        let total_secs = d1.timestamp() - d0.timestamp();
        if count == 0 || total_secs <= 0 {
            return ticks;
        }
        let step = (total_secs as f64 / count as f64).ceil() as i64;
        if step <= 0 {
            ticks.push(d0);
            if d0 != d1 {
                ticks.push(d1);
            }
            return ticks;
        }
        let mut t = d0;
        while t <= d1 {
            ticks.push(t);
            match t.checked_add_signed(Duration::try_seconds(step).unwrap()) {
                Some(next) => t = next,
                None => break,
            }
        }
        if let Some(last) = ticks.last() {
            if *last < d1 {
                ticks.push(d1);
            }
        }
        ticks
    }
    pub fn tick_format(&self, _count: usize, fmt: &str) -> impl Fn(&DateTime<Utc>) -> String + '_ {
        let fmt = fmt.to_string();
        move |d| d.format(&fmt).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    #[test]
    fn test_scale_utc() {
        let d0 = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let d1 = Utc.with_ymd_and_hms(2020, 1, 2, 0, 0, 0).unwrap();
        let s = ScaleUtc::new([d0, d1], [0.0, 100.0]);
        assert_eq!(s.scale(d0), 0.0);
        assert_eq!(s.scale(d1), 100.0);
        let mid = d0 + chrono::Duration::try_seconds(43200).unwrap();
        assert_eq!(s.scale(mid), 50.0);
        assert_eq!(s.invert(50.0), mid);
    }
    #[test]
    fn test_ticks() {
        let d0 = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let d1 = Utc.with_ymd_and_hms(2020, 1, 2, 0, 0, 0).unwrap();
        let s = ScaleUtc::new([d0, d1], [0.0, 100.0]);
        let ticks = s.ticks(4);
        assert!(ticks.len() >= 4);
    }
    #[test]
    fn test_tick_format() {
        let d0 = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let s = ScaleUtc::new([d0, d0], [0.0, 100.0]);
        let fmt = s.tick_format(5, "%Y-%m-%d");
        assert_eq!(fmt(&d0), "2020-01-01");
    }
}
