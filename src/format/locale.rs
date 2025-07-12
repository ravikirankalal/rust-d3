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

impl Locale {
    pub fn en_us() -> Self {
        Locale {
            decimal: ".".to_string(),
            thousands: ",".to_string(),
            grouping: vec![3],
            currency: ("$".to_string(), "".to_string()),
        }
    }
    pub fn fr_fr() -> Self {
        Locale {
            decimal: ",".to_string(),
            thousands: " ".to_string(), // thin space
            grouping: vec![3],
            currency: ("".to_string(), " €".to_string()),
        }
    }
    pub fn from_tag(tag: &str) -> Self {
        match tag {
            "en-US" => Locale::en_us(),
            "fr-FR" => Locale::fr_fr(),
            _ => Locale::default(),
        }
    }
}
