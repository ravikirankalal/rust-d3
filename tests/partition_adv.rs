#[cfg(test)]
mod tests {
    use rust_d3::hierarchy::Node;
    use rust_d3::partition_adv::{PartitionConfig, partition_with_config};

    #[test]
    fn test_partition_with_padding() {
        let mut root = Node::new("root");
        root.add_child(Node::new("a"));
        root.add_child(Node::new("b"));
        let config = PartitionConfig { width: 100.0, height: 50.0, padding: 10.0 };
        let parts = partition_with_config(&root, config);
        assert_eq!(parts.len(), 3);
        // Check that children are separated by padding
        let a = &parts[1];
        let b = &parts[2];
        assert!((b.x0 - a.x1 - config.padding).abs() < 1e-6);
    }

    #[test]
    fn test_partition_with_zero_padding() {
        let mut root = Node::new("root");
        root.add_child(Node::new("a"));
        root.add_child(Node::new("b"));
        let config = PartitionConfig { width: 100.0, height: 50.0, padding: 0.0 };
        let parts = partition_with_config(&root, config);
        assert_eq!(parts.len(), 3);
        let a = &parts[1];
        let b = &parts[2];
        assert!((b.x0 - a.x1).abs() < 1e-6);
    }

    #[test]
    fn test_partition_with_custom_size() {
        let root = Node::new("root");
        let config = PartitionConfig { width: 200.0, height: 80.0, padding: 0.0 };
        let parts = partition_with_config(&root, config);
        assert_eq!(parts[0].x1, 200.0);
        assert_eq!(parts[0].y1, 80.0);
    }
}
