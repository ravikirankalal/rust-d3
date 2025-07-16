//! Tests and usage examples for d3-hierarchy module

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_tree_layout_basic() {
        let mut root = Node::new("root");
        root.add_child(Node::new("a"));
        root.add_child(Node::new("b"));
        let tree = TreeLayout::new();
        tree.layout(&mut root);
        assert_eq!(root.depth, 0);
        assert_eq!(root.children[0].depth, 1);
        assert!(root.x.is_some() && root.y.is_some());
    }

    #[test]
    fn test_tree_layout_deep() {
        let mut root = Node::new("root");
        let mut c1 = Node::new("a");
        c1.add_child(Node::new("a1"));
        root.add_child(c1);
        let tree = TreeLayout::new();
        tree.layout(&mut root);
        assert_eq!(root.depth, 0);
        assert_eq!(root.children[0].children[0].depth, 2);
    }

    #[test]
    fn test_node_sum() {
        let mut root = Node::new(1);
        root.add_child(Node::new(2));
        root.add_child(Node::new(3));
        let sum = root.sum(&|v| *v as f64);
        assert_eq!(sum, 6.0);
        assert_eq!(root.value, Some(6.0));
    }

    #[test]
    fn test_each_traversal() {
        let mut root = Node::new(0);
        root.add_child(Node::new(1));
        root.add_child(Node::new(2));
        let mut count = 0;
        root.each(&mut |_n| {
            count += 1;
        });
        assert_eq!(count, 3);
    }

    #[test]
    fn test_treemap_layout_basic() {
        let mut root = Node::new("root");
        root.add_child(Node::new("a"));
        root.add_child(Node::new("b"));
        let treemap = TreemapLayout::new().size((200.0, 100.0));
        treemap.layout(&mut root);
        assert!(root.x.is_some() && root.y.is_some());
    }

    #[test]
    fn test_partition_layout_basic() {
        let mut root = Node::new("root");
        root.add_child(Node::new("a"));
        root.add_child(Node::new("b"));
        let partition = PartitionLayout::new().size((200.0, 100.0));
        partition.layout(&mut root);
        assert!(root.x.is_some() && root.y.is_some());
    }
}
