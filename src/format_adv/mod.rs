//! D3 Format Advanced module
//! Advanced number formatting, e.g., SI prefixes, locale, etc.

use num_format::{Locale, ToFormattedString};

/// D3.js: d3.formatPrefix (already implemented in format/format.rs)
/// Re-export for API parity.
pub use crate::format::format::format_prefix;

/// D3.js: d3.formatLocale
/// Returns a formatter closure for the given locale (e.g., "en", "fr", "de").
pub fn format_locale(locale: &str) -> impl Fn(u64) -> String {
    let loc = match locale {
        "en" => Locale::en,
        "fr" => Locale::fr,
        "de" => Locale::de,
        "es" => Locale::es,
        _ => Locale::en,
    };
    move |n: u64| n.to_formatted_string(&loc)
}
