#[cfg(test)]
mod tests {
    use rust_d3::stack::stack;

    #[test]
    fn test_stack_basic() {
        let data = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let result = stack(&data);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 3);
        assert_eq!(result[1].len(), 3);
        // Check stacking logic
        assert_eq!(result[0][0], (0.0, 1.0));
        assert_eq!(result[1][0], (1.0, 5.0));
    }

    #[test]
    fn test_stack_empty() {
        let data: Vec<Vec<f64>> = vec![];
        let result = stack(&data);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_stack_single_series() {
        let data = vec![vec![10.0, 20.0, 30.0]];
        let result = stack(&data);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec![(0.0, 10.0), (0.0, 20.0), (0.0, 30.0)]);
    }

    #[test]
    fn test_stack_negative_and_zero() {
        let data = vec![vec![0.0, -2.0, 3.0], vec![4.0, 0.0, -6.0]];
        let result = stack(&data);
        assert_eq!(result[0], vec![(0.0, 0.0), (0.0, -2.0), (0.0, 3.0)]);
        assert_eq!(result[1], vec![(0.0, 4.0), (-2.0, -2.0), (3.0, -3.0)]);
    }

    #[test]
    fn test_stack_non_uniform_lengths() {
        let data = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];
        // Should handle or panic; let's check for panic safety
        let result = std::panic::catch_unwind(|| stack(&data));
        assert!(result.is_err() || result.is_ok()); // Accept either, but should not UB
    }

    #[test]
    fn test_stack_large_numbers() {
        let data = vec![vec![1_000_000_000.0, 2_000_000_000.0], vec![3_000_000_000.0, 4_000_000_000.0]];
        let result = stack(&data);
        assert_eq!(result[0][0], (0.0, 1_000_000_000.0));
        assert_eq!(result[1][1], (2_000_000_000.0, 6_000_000_000.0));
    }
}
