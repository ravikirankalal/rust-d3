//! d3-interpolate: String interpolation (number extraction only)

use regex::Regex;

/// Interpolate numbers in a string (e.g., "foo0" to "foo1")
pub fn interpolate_string(a: &str, b: &str, t: f64) -> String {
    let re = Regex::new(r"-?\d+(?:\.\d+)?").unwrap();
    let mut a_nums: Vec<f64> = re.find_iter(a).map(|m| m.as_str().parse().unwrap_or(0.0)).collect();
    let b_nums: Vec<f64> = re.find_iter(b).map(|m| m.as_str().parse().unwrap_or(0.0)).collect();
    for (i, b_num) in b_nums.iter().enumerate() {
        if i < a_nums.len() {
            a_nums[i] = a_nums[i] + (b_num - a_nums[i]) * t;
        }
    }
    let mut result = a.to_string();
    for num in a_nums {
        result = re.replace(&result, format!("{}", num)).to_string();
    }
    result
}
