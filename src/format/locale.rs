#[derive(Debug, Clone)]
pub struct Locale {
    pub decimal: String,
    pub thousands: String,
    pub grouping: Vec<usize>,
    pub currency: (String, String), // (prefix, suffix)
}

impl Default for Locale {
    fn default() -> Self {
        Locale {
            decimal: ".".to_string(),
            thousands: ",".to_string(),
            grouping: vec![3],
            currency: ("".to_string(), "".to_string()),
        }
    }
}
