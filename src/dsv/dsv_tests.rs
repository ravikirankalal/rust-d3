#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_dsv_format_parse_and_format() {
        let csv = "a,b\n1,2\n3,4";
        let fmt = DsvFormat::new(',');
        let parsed = fmt.parse(csv);
        assert_eq!(parsed, vec![vec!["a".to_string(), "b".to_string()], vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]]);
        let formatted = fmt.format(&parsed);
        assert!(formatted.contains("a,b"));
    }

    #[test]
    fn test_parse_rows_with() {
        let tsv = "foo\tbar\n1\t2";
        let fmt = DsvFormat::new('\t');
        let rows = fmt.parse_rows_with(tsv, |row| json!(row));
        assert_eq!(rows[0], json!(["foo", "bar"]));
    }

    #[test]
    fn test_format_rows_with() {
        let fmt = DsvFormat::new(',');
        let data = vec![vec!["x".to_string(), "y".to_string()], vec!["1".to_string(), "2".to_string()]];
        let formatted = fmt.format_rows_with(&data, |row| row.join(","));
        assert!(formatted.contains("x,y"));
    }

    #[test]
    fn test_auto_type() {
        let mut data = vec![
            vec!["1".to_string(), "true".to_string(), "foo".to_string()],
            vec!["2.5".to_string(), "false".to_string(), "null".to_string()],
            vec!["".to_string(), "null".to_string(), " ".to_string()]
        ];
        let typed = auto_type(&mut data);
        assert_eq!(typed[0][0], json!(1));
        assert_eq!(typed[0][1], json!(true));
        assert_eq!(typed[0][2], json!("foo"));
        assert_eq!(typed[1][0], json!(2.5));
        assert_eq!(typed[1][1], json!(false));
        assert_eq!(typed[1][2], serde_json::Value::Null);
        // Additional checks for empty string and whitespace
        assert_eq!(typed[2][0], json!(""));
        assert_eq!(typed[2][1], serde_json::Value::Null);
        assert_eq!(typed[2][2], json!(""));
    }
}
