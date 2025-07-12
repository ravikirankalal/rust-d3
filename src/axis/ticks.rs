// d3-axis: Tick and TickFormat
pub type TickFormat = fn(f64) -> String;

#[derive(Debug, Clone)]
pub struct Tick {
    pub value: f64,
    pub label: String,
    pub position: f64,
}

impl Tick {
    pub fn new(value: f64, label: String, position: f64) -> Self {
        Self { value, label, position }
    }
}
