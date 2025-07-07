#[cfg(test)]
mod tests {
    use rust_d3::hierarchy::Node;
    use rust_d3::{PackConfig, pack_with_config};

    #[test]
    fn test_pack_with_padding() {
        let mut root = Node::new("root");
        root.add_child(Node::new("a"));
        root.add_child(Node::new("b"));
        let config = PackConfig { radius: 10.0, padding: 2.0 };
        let packed = pack_with_config(&root, config);
        assert_eq!(packed.len(), 3);
        // Children should be separated by at least padding
        let a = &packed[1];
        let b = &packed[2];
        let dist = ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt();
        assert!(dist >= 2.0);
    }

    #[test]
    fn test_pack_with_zero_padding() {
        let mut root = Node::new("root");
        root.add_child(Node::new("a"));
        root.add_child(Node::new("b"));
        let config = PackConfig { radius: 10.0, padding: 0.0 };
        let packed = pack_with_config(&root, config);
        assert_eq!(packed.len(), 3);
    }

    #[test]
    fn test_pack_with_custom_radius() {
        let root = Node::new("root");
        let config = PackConfig { radius: 20.0, padding: 0.0 };
        let packed = pack_with_config(&root, config);
        assert_eq!(packed[0].r, 20.0);
    }
}
