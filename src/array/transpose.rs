//! Transpose a matrix (array of arrays), like d3-array's transpose.
//!
//! # Example
//! ```
//! let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
//! let t = rust_d3::transpose(&matrix);
//! assert_eq!(t, vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
//! ```

/// Returns the transpose of a matrix (array of arrays).
pub fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return vec![];
    }
    let ncols = matrix[0].len();
    let mut result = vec![Vec::with_capacity(matrix.len()); ncols];
    for row in matrix {
        for (j, val) in row.iter().enumerate() {
            result[j].push(val.clone());
        }
    }
    result
}
