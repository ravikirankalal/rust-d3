pub fn format_decimal(x: f64, precision: usize) -> String {
    format!("{:.*}", precision, x)
}
