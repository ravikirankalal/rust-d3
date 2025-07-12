// d3-time-format locale stub

pub struct TimeLocale {
    pub months: [&'static str; 12],
    pub short_months: [&'static str; 12],
    pub days: [&'static str; 7],
    pub short_days: [&'static str; 7],
    pub am: &'static str,
    pub pm: &'static str,
}

impl Default for TimeLocale {
    fn default() -> Self {
        Self {
            months: ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
            short_months: ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"],
            days: ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"],
            short_days: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
            am: "AM",
            pm: "PM",
        }
    }
}

impl TimeLocale {
    pub fn en_us() -> Self {
        Self::default()
    }
    // Add more built-in locales as needed
}

// Allow custom locale injection in the future
