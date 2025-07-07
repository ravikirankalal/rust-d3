#[cfg(test)]
mod tests {
    use rust_d3::stack::{stack};
    use rust_d3::stack_adv::{
        stack_offset_none, stack_offset_expand, stack_offset_silhouette, stack_offset_wiggle,
        stack_offset_diverging,
        stack_order_none, stack_order_reverse, stack_order_ascending, stack_order_descending, stack_order_appearance, stack_order_inside_out
    };

    const DATA: [[f64; 3]; 2] = [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ];

    fn stacked_data() -> Vec<Vec<f64>> {
        DATA.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn stack_offset_none_works() {
        let mut result = stack(&stacked_data());
        stack_offset_none(&mut result);
        assert_eq!(result[0][0], (0.0, 1.0));
        assert_eq!(result[1][0], (1.0, 5.0));
    }

    #[test]
    fn stack_offset_expand_sums_to_one() {
        let mut result = stack(&stacked_data());
        stack_offset_expand(&mut result);
        for i in 0..3 {
            let sum: f64 = result.iter().map(|s| s[i].1 - s[i].0).sum();
            assert!((sum - 1.0).abs() < 1e-6, "sum at x={i} is {sum}");
        }
    }

    #[test]
    fn stack_offset_silhouette_centered() {
        let mut result = stack(&stacked_data());
        stack_offset_silhouette(&mut result);
        for i in 0..3 {
            let min = result.iter().map(|s| s[i].0).fold(f64::INFINITY, f64::min);
            let max = result.iter().map(|s| s[i].1).fold(f64::NEG_INFINITY, f64::max);
            let center = (min + max) / 2.0;
            assert!(center.abs() < 1e-6, "center at x={i} is {center}");
        }
    }

    #[test]
    fn stack_offset_wiggle_valid() {
        let mut result = stack(&stacked_data());
        stack_offset_wiggle(&mut result);
        for (si, s) in result.iter().enumerate() {
            for (xi, &(a, b)) in s.iter().enumerate() {
                assert!(a.is_finite() && b.is_finite(), "invalid at series={si}, x={xi}");
            }
        }
    }

    #[test]
    fn stack_offset_diverging_positive_and_negative() {
        let mut result = vec![
            vec![(0.0, 2.0), (0.0, -1.0)],
            vec![(0.0, -3.0), (0.0, 4.0)],
        ];
        stack_offset_diverging(&mut result);
        // At x=0: first is positive, second is negative
        assert_eq!(result[0][0], (0.0, 2.0));
        assert_eq!(result[1][0], (0.0, -3.0));
        // At x=1: first is negative, second is positive
        assert_eq!(result[0][1], (0.0, -1.0));
        assert_eq!(result[1][1], (0.0, 4.0));
    }

    #[test]
    fn stack_order_functions() {
        let series = vec![vec![(0.0, 1.0), (0.0, 2.0)], vec![(0.0, 3.0), (0.0, 4.0)], vec![(0.0, 0.5), (0.0, 0.2)]];
        assert_eq!(stack_order_none(&series), vec![0, 1, 2]);
        assert_eq!(stack_order_reverse(&series), vec![2, 1, 0]);
        // Ascending: smallest sum at bottom
        let asc = stack_order_ascending(&series);
        assert_eq!(asc[0], 2); // smallest sum
        // Descending: largest sum at bottom
        let desc = stack_order_descending(&series);
        assert_eq!(desc[0], 1); // largest sum
        // Appearance: by max value
        let app = stack_order_appearance(&series);
        assert!(app.contains(&1));
        // Inside-out: alternates from center
        let inside_out = stack_order_inside_out(&series);
        assert_eq!(inside_out.len(), 3);
    }
}
