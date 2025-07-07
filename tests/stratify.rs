#[cfg(test)]
mod tests {
    use rust_d3::stratify::{stratify, FlatNode, stratify_parent_id};
    use rust_d3::hierarchy::hierarchy::Node;

    #[test]
    fn test_stratify_basic() {
        let data = vec![
            FlatNode { id: "a".to_string(), parent_id: None, value: 1 },
            FlatNode { id: "b".to_string(), parent_id: Some("a".to_string()), value: 2 },
        ];
        let tree = stratify(data);
        assert!(tree.is_some());
        let root = tree.unwrap();
        assert_eq!(root.value, 1);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].value, 2);
    }

    #[test]
    fn test_stratify_parent_id_custom_struct() {
        #[derive(Clone)]
        struct MyNode { id: &'static str, parent: Option<&'static str>, val: i32 }
        let data = vec![
            MyNode { id: "root", parent: None, val: 10 },
            MyNode { id: "child", parent: Some("root"), val: 20 },
        ];
        let builder = stratify_parent_id(
            |n: &MyNode| n.id.to_string(),
            |n: &MyNode| n.parent.map(|p| p.to_string()),
        );
        let tree = builder.build(&data);
        assert!(tree.is_some());
        let root = tree.unwrap();
        assert_eq!(root.value.val, 10);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].value.val, 20);
    }
}
