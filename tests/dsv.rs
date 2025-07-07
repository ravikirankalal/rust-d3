//! Unit test for d3 dsv
use rust_d3::dsv::{parse_csv, to_csv, parse_tsv, to_tsv};
use rust_d3::dsv::{dsv_format, parse_rows, format, format_rows};

#[test]
fn test_parse_and_to_csv() {
    let csv = "a,b,c\n1,2,3\n4,5,6";
    let records = parse_csv(csv);
    assert_eq!(records, vec![
        vec!["a","b","c"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>(),
        vec!["1","2","3"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>(),
        vec!["4","5","6"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>(),
    ]);
    let out = to_csv(&records);
    assert!(out.contains("a,b,c"));
}

#[test]
fn test_parse_and_to_tsv() {
    let tsv = "a\tb\tc\n1\t2\t3";
    let records = parse_tsv(tsv);
    assert_eq!(records[0], vec!["a","b","c"].into_iter().map(|s| s.to_string()).collect::<Vec<String>>());
    let out = to_tsv(&records);
    assert!(out.contains("a\tb\tc"));
}

#[test]
fn test_dsv_format_struct() {
    let dsv = dsv_format(';');
    let data = "x;y;z\n7;8;9";
    let records = dsv.parse(data);
    assert_eq!(records[0], vec!["x", "y", "z"]);
    let out = dsv.format(&records);
    assert!(out.contains("x;y;z"));
}

#[test]
fn test_parse_rows_and_format_rows() {
    let data = "row1\nrow2\nrow3";
    let rows = parse_rows(data);
    assert_eq!(rows.len(), 3);
    let out = format_rows(&rows);
    assert!(out.contains("row1"));
}

#[test]
fn test_format_function() {
    let records = vec![vec!["foo".to_string(), "bar".to_string()]];
    let out = format(&records);
    assert_eq!(out, "foo,bar");
}

#[test]
fn test_auto_type() {
    use rust_d3::dsv::auto_type;
    let mut data = vec![
        vec!["42".to_string(), "true".to_string(), "null".to_string(), "foo".to_string()],
        vec!["3.14".to_string(), "false".to_string(), "".to_string(), "bar".to_string()],
    ];
    auto_type(&mut data);
    assert_eq!(data[0][0], "42");
    assert_eq!(data[0][1], "true");
    assert_eq!(data[0][2], "");
    assert_eq!(data[0][3], "foo");
    assert_eq!(data[1][0], "3.14");
    assert_eq!(data[1][1], "false");
    assert_eq!(data[1][2], "");
    assert_eq!(data[1][3], "bar");
}
