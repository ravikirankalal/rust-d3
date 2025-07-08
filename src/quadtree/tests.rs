//! Tests for d3-quadtree (Rust)

#[cfg(test)]
mod tests {
    use crate::Quadtree;

    #[test]
    fn test_quadtree_insert_and_len() {
        let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
        qt.insert(1.0, 2.0, "a");
        qt.insert(3.0, 4.0, "b");
        assert_eq!(qt.len(), 2);
    }

    #[test]
    fn test_quadtree_find() {
        let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
        qt.insert(1.0, 2.0, "a");
        qt.insert(3.0, 4.0, "b");
        let found = qt.find(1.0, 2.0, 0.1);
        assert!(found.is_some());
        assert_eq!(found.unwrap().2, "a");
        let not_found = qt.find(9.0, 9.0, 0.5);
        assert!(not_found.is_none());
    }
    #[test]
    fn test_quadtree_remove() {
        let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
        qt.insert(1.0, 2.0, "a");
        qt.insert(3.0, 4.0, "b");
        let removed = qt.remove(1.0, 2.0, 0.1);
        assert_eq!(removed, Some("a"));
        assert_eq!(qt.len(), 1);
        let not_removed = qt.remove(9.0, 9.0, 0.5);
        assert_eq!(not_removed, None);
    }
    #[test]
    fn test_quadtree_visit() {
        let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
        qt.insert(1.0, 2.0, 1);
        qt.insert(3.0, 4.0, 2);
        let mut sum = 0;
        qt.visit(|(_, _, v)| sum += v);
        assert_eq!(sum, 3);
    }
    #[test]
    fn test_quadtree_subdivision() {
        let mut qt = Quadtree::new((0.0, 0.0, 8.0, 8.0));
        for i in 0..10 {
            qt.insert(i as f64, i as f64, i);
        }
        // After enough inserts, root should be internal
        match &qt.root {
            Some(crate::quadtree::Node::Internal(_)) => (),
            _ => panic!("Quadtree did not subdivide as expected"),
        }
    }
    #[test]
    fn test_quadtree_clear() {
        let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
        qt.insert(1.0, 2.0, 1);
        qt.clear();
        assert!(qt.root.is_none());
    }
    #[test]
    fn test_quadtree_query_range() {
        let mut qt = Quadtree::new((0.0, 0.0, 10.0, 10.0));
        qt.insert(1.0, 2.0, "a");
        qt.insert(3.0, 4.0, "b");
        qt.insert(8.0, 8.0, "c");
        let found = qt.query_range((0.0, 0.0, 5.0, 5.0));
        let vals: Vec<_> = found.iter().map(|p| p.2).collect();
        assert!(vals.contains(&"a"));
        assert!(vals.contains(&"b"));
        assert!(!vals.contains(&"c"));
    }
}
