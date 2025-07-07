//! D3 DSV (CSV/TSV) module
//! Provides CSV/TSV parsing and serialization.

use serde_json::Value;

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
    /// d3-dsv: parseRows with row conversion function
    pub fn parse_rows_with<F>(&self, input: &str, row: F) -> Vec<Value>
    where
        F: Fn(&[String]) -> Value,
    {
        self.parse(input)
            .iter()
            .map(|row_vec| row(row_vec))
            .collect()
    }
    /// d3-dsv: formatRows with row conversion function
    pub fn format_rows_with<F>(&self, records: &[Vec<String>], row: F) -> String
    where
        F: Fn(&[String]) -> String,
    {
        records.iter().map(|row_vec| row(row_vec)).collect::<Vec<_>>().join("\n")
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

/// d3-dsv: autoType implementation (robust type inference)
pub fn auto_type(object: &mut [Vec<String>]) -> Vec<Vec<Value>> {
    object
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| {
                    let s_trimmed = cell.trim();
                    if s_trimmed.eq_ignore_ascii_case("null") {
                        Value::Null
                    } else if s_trimmed.is_empty() {
                        Value::from("")
                    } else if let Ok(n) = s_trimmed.parse::<i64>() {
                        Value::from(n)
                    } else if let Ok(f) = s_trimmed.parse::<f64>() {
                        Value::from(f)
                    } else if s_trimmed.eq_ignore_ascii_case("true") {
                        Value::from(true)
                    } else if s_trimmed.eq_ignore_ascii_case("false") {
                        Value::from(false)
                    } else {
                        Value::from(s_trimmed)
                    }
                })
                .collect()
        })
        .collect()
}
