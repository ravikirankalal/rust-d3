//! D3 DSV Advanced module
//! Advanced DSV parsing, e.g., custom delimiters, streaming, etc.

/// Parses DSV data with a custom delimiter.
pub fn parse_custom_dsv(data: &str, delimiter: char) -> Vec<Vec<String>> {
    data.lines()
        .map(|line| line.split(delimiter).map(|s| s.trim().to_string()).collect())
        .collect()
}
