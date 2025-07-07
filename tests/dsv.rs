//! Unit tests for d3 dsv

use rust_d3::dsv::{parse_csv, to_csv, parse_tsv, to_tsv, auto_type, parse_rows, format, format_rows};
use rust_d3::dsv::dsv::DsvFormat;
use serde_json::Value;

#[test]
fn test_parse_and_to_csv() {
    let csv_string = "a,b,c\n1,2,3\n4,5,6";
    let parsed = parse_csv(csv_string);
    assert_eq!(parsed, vec![vec!["a".to_string(), "b".to_string(), "c".to_string()], vec!["1".to_string(), "2".to_string(), "3".to_string()], vec!["4".to_string(), "5".to_string(), "6".to_string()]]);
    let formatted = to_csv(&parsed);
    assert_eq!(formatted, csv_string);
}

#[test]
fn test_parse_and_to_tsv() {
    let tsv_string = "a\tb\tc\n1\t2\t3\n4\t5\t6";
    let parsed = parse_tsv(tsv_string);
    assert_eq!(parsed, vec![vec!["a".to_string(), "b".to_string(), "c".to_string()], vec!["1".to_string(), "2".to_string(), "3".to_string()], vec!["4".to_string(), "5".to_string(), "6".to_string()]]);
    let formatted = to_tsv(&parsed);
    assert_eq!(formatted, tsv_string);
}

#[test]
fn test_dsv_format_struct() {
    let csv_string = "a,b,c\n1,2,3";
    let fmt = DsvFormat::new(',');
    let parsed = fmt.parse(csv_string);
    assert_eq!(parsed, vec![vec!["a".to_string(), "b".to_string(), "c".to_string()], vec!["1".to_string(), "2".to_string(), "3".to_string()]]);
    let formatted = fmt.format(&parsed);
    assert_eq!(formatted, csv_string);
}

#[test]
fn test_parse_rows_and_format_rows() {
    let input = "row1\nrow2";
    let parsed = parse_rows(input);
    assert_eq!(parsed, vec![vec!["row1".to_string()], vec!["row2".to_string()]]);

    let records = vec![vec!["a".to_string(), "b".to_string()], vec!["c".to_string(), "d".to_string()]];
    let formatted = format_rows(&records);
    assert_eq!(formatted, "a\tb\nc\td");
}

#[test]
fn test_format_function() {
    let records = vec![vec!["a".to_string(), "b".to_string()], vec!["c".to_string(), "d".to_string()]];
    let formatted = format(&records);
    assert_eq!(formatted, "a,b\nc,d");
}

#[test]
fn test_auto_type() {
    let mut data = vec![
        vec!["1".to_string(), "2.5".to_string(), "true".to_string(), "hello".to_string(), "null".to_string(), "".to_string()],
    ];
    let typed = auto_type(&mut data);
    assert_eq!(typed[0][0], Value::from(1));
    assert_eq!(typed[0][1], Value::from(2.5));
    assert_eq!(typed[0][2], Value::from(true));
    assert_eq!(typed[0][3], Value::from("hello"));
    assert_eq!(typed[0][4], Value::Null);
    assert_eq!(typed[0][5], Value::from(""));
}
