use rust_d3::{Stack, StackedSeries};

#[derive(Clone, Debug, PartialEq)]
struct Row {
    category: &'static str,
    value: f64,
}

#[test]
fn test_stack_builder_keys_value_order_offset_metadata() {
    let data = vec![
        Row { category: "A", value: 1.0 },
        Row { category: "B", value: 2.0 },
        Row { category: "A", value: 3.0 },
        Row { category: "B", value: 4.0 },
    ];
    let keys = vec!["A", "B"];
    let stack = Stack::new()
        .keys(keys.clone())
        .value_fn(|row: &Row, key: &&str| if row.category == *key { row.value } else { 0.0 });
    let result = stack.stack(&data);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].meta.key, "A");
    assert_eq!(result[1].meta.key, "B");
    // Check metadata and stacking
    for series in &result {
        assert!(keys.contains(&series.meta.key));
        for (y0, y1, row) in &series.points {
            assert!(y1 >= y0);
            assert!(row.category == series.meta.key || *y1 - *y0 == 0.0);
        }
    }
}
