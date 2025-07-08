// d3-scale: ScaleTime (stub)
#[derive(Debug, Clone)]
pub struct ScaleTime {
    pub domain: [chrono::NaiveDateTime; 2],
    pub range: [f64; 2],
}

impl ScaleTime {
    pub fn new(domain: [chrono::NaiveDateTime; 2], range: [f64; 2]) -> Self {
        Self { domain, range }
    }
    pub fn scale(&self, x: chrono::NaiveDateTime) -> f64 {
        let t = (x.and_utc().timestamp_millis() - self.domain[0].and_utc().timestamp_millis()) as f64 /
                (self.domain[1].and_utc().timestamp_millis() - self.domain[0].and_utc().timestamp_millis()) as f64;
        self.range[0] + t * (self.range[1] - self.range[0])
    }
    pub fn invert(&self, y: f64) -> chrono::NaiveDateTime {
        let t = (y - self.range[0]) / (self.range[1] - self.range[0]);
        let millis = self.domain[0].and_utc().timestamp_millis() as f64 + t * (self.domain[1].and_utc().timestamp_millis() - self.domain[0].and_utc().timestamp_millis()) as f64;
        chrono::DateTime::<chrono::Utc>::from_timestamp((millis / 1000.0) as i64, ((millis % 1000.0) * 1_000_000.0) as u32)
            .unwrap()
            .naive_utc()
    }
}
