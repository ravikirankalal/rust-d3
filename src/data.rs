//! Data structures and manipulation utilities for charts

use serde::{Deserialize, Serialize};

/// A basic data point with a label and numeric value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub label: String,
    pub value: f64,
}

impl DataPoint {
    /// Create a new data point
    pub fn new(label: &str, value: f64) -> Self {
        Self {
            label: label.to_string(),
            value,
        }
    }
}

/// A data point with x and y coordinates for scatter plots and line charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// Create a new 2D point
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Utility functions for data manipulation
pub struct DataUtils;

impl DataUtils {
    /// Find the minimum value in a dataset
    pub fn min(data: &[DataPoint]) -> Option<f64> {
        data.iter()
            .map(|d| d.value)
            .fold(None, |acc, x| Some(acc.map_or(x, |acc| acc.min(x))))
    }

    /// Find the maximum value in a dataset
    pub fn max(data: &[DataPoint]) -> Option<f64> {
        data.iter()
            .map(|d| d.value)
            .fold(None, |acc, x| Some(acc.map_or(x, |acc| acc.max(x))))
    }

    /// Calculate the sum of all values
    pub fn sum(data: &[DataPoint]) -> f64 {
        data.iter().map(|d| d.value).sum()
    }

    /// Find min and max for 2D points
    pub fn extent_2d(data: &[Point2D]) -> Option<(Point2D, Point2D)> {
        if data.is_empty() {
            return None;
        }

        let mut min_x = data[0].x;
        let mut max_x = data[0].x;
        let mut min_y = data[0].y;
        let mut max_y = data[0].y;

        for point in data.iter().skip(1) {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }

        Some((Point2D::new(min_x, min_y), Point2D::new(max_x, max_y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_point_creation() {
        let point = DataPoint::new("test", 42.0);
        assert_eq!(point.label, "test");
        assert_eq!(point.value, 42.0);
    }

    #[test]
    fn test_point_2d_creation() {
        let point = Point2D::new(1.0, 2.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
    }

    #[test]
    fn test_data_utils_min_max() {
        let data = vec![
            DataPoint::new("A", 10.0),
            DataPoint::new("B", 5.0),
            DataPoint::new("C", 15.0),
        ];

        assert_eq!(DataUtils::min(&data), Some(5.0));
        assert_eq!(DataUtils::max(&data), Some(15.0));
        assert_eq!(DataUtils::sum(&data), 30.0);
    }

    #[test]
    fn test_extent_2d() {
        let data = vec![
            Point2D::new(1.0, 2.0),
            Point2D::new(3.0, 1.0),
            Point2D::new(2.0, 4.0),
        ];

        let (min, max) = DataUtils::extent_2d(&data).unwrap();
        assert_eq!(min.x, 1.0);
        assert_eq!(min.y, 1.0);
        assert_eq!(max.x, 3.0);
        assert_eq!(max.y, 4.0);
    }
}
