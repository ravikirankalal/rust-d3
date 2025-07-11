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
    pub fn ticks(&self, count: usize) -> Vec<chrono::NaiveDateTime> {
        let start = self.domain[0];
        let end = self.domain[1];
        let start_ms = start.and_utc().timestamp_millis();
        let end_ms = end.and_utc().timestamp_millis();
        let step = (end_ms - start_ms) / (count as i64 - 1);
        let mut ticks = Vec::new();
        for i in 0..count {
            let ms = start_ms + step * i as i64;
            let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(ms / 1000, ((ms % 1000) * 1_000_000) as u32)
                .unwrap()
                .naive_utc();
            ticks.push(dt);
        }
        ticks
    }
}
