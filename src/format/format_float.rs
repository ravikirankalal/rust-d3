pub fn format_float(x: f64, precision: usize) -> String {
    format!("{:.*}", precision, x)
}
