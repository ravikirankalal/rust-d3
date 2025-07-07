//! D3 DSV (CSV/TSV) module
//! Provides CSV/TSV parsing and serialization.

/// Parse a delimiter-separated values (DSV) string into a vector of records.
pub fn parse_dsv(input: &str, delimiter: char) -> Vec<Vec<String>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.split(delimiter).map(|s| s.trim().to_string()).collect())
        .collect()
}

/// Serialize a vector of records into a DSV string.
pub fn to_dsv(records: &[Vec<String>], delimiter: char) -> String {
    records
        .iter()
        .map(|row| row.join(&delimiter.to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn parse_csv(input: &str) -> Vec<Vec<String>> {
    parse_dsv(input, ',')
}

pub fn to_csv(records: &[Vec<String>]) -> String {
    to_dsv(records, ',')
}

pub fn parse_tsv(input: &str) -> Vec<Vec<String>> {
    parse_dsv(input, '\t')
}

pub fn to_tsv(records: &[Vec<String>]) -> String {
    to_dsv(records, '\t')
}

/// Placeholder for d3-dsv API parity.
/// See: https://github.com/d3/d3-dsv#api-reference
/// TODO: Implement full API parity with d3-dsv (dsvFormat, parseRows, format, formatRows, autoType, etc.)

/// Placeholder for d3.dsvFormat(delimiter)
pub struct DsvFormat {
    delimiter: char,
}

impl DsvFormat {
    pub fn new(delimiter: char) -> Self {
        Self { delimiter }
    }
    pub fn parse(&self, input: &str) -> Vec<Vec<String>> {
        parse_dsv(input, self.delimiter)
    }
    pub fn format(&self, records: &[Vec<String>]) -> String {
        to_dsv(records, self.delimiter)
    }
    pub fn parse_rows(&self, input: &str) -> Vec<Vec<String>> {
        input.lines().map(|line| vec![line.to_string()]).collect()
    }
    pub fn format_rows(&self, records: &[Vec<String>]) -> String {
        records.iter().map(|row| row.join("	")).collect::<Vec<_>>().join("\n")
    }
}

pub fn dsv_format(delimiter: char) -> DsvFormat {
    DsvFormat::new(delimiter)
}

pub fn parse_rows(input: &str) -> Vec<Vec<String>> {
    input.lines().map(|line| vec![line.to_string()]).collect()
}

pub fn format(records: &[Vec<String>]) -> String {
    to_dsv(records, ',')
}

pub fn format_rows(records: &[Vec<String>]) -> String {
    records.iter().map(|row| row.join("\t")).collect::<Vec<_>>().join("\n")
}

/// Placeholder for d3.autoType
pub fn auto_type(object: &mut [Vec<String>]) {
    for row in object.iter_mut() {
        for cell in row.iter_mut() {
            let s = cell.trim();
            if s.eq_ignore_ascii_case("null") || s.is_empty() {
                *cell = String::from("");
            } else if let Ok(n) = s.parse::<f64>() {
                *cell = n.to_string();
            } else if s.eq_ignore_ascii_case("true") {
                *cell = String::from("true");
            } else if s.eq_ignore_ascii_case("false") {
                *cell = String::from("false");
            } // else leave as string
        }
    }
}
