//! Unit test for d3 cluster_adv
use rust_d3::cluster_adv::{TreeNode, cluster_depth, cluster_leaf_count, cluster_height, cluster_leaves};

#[test]
fn test_cluster_depth() {
    let tree = TreeNode {
        value: 1,
        children: vec![
            TreeNode { value: 2, children: vec![] },
            TreeNode { value: 3, children: vec![
                TreeNode { value: 4, children: vec![] },
                TreeNode { value: 5, children: vec![] },
            ]},
        ],
    };
    assert_eq!(cluster_depth(&tree), 3);
}

#[test]
fn test_cluster_leaf_count() {
    let tree = TreeNode {
        value: 1,
        children: vec![
            TreeNode { value: 2, children: vec![] },
            TreeNode { value: 3, children: vec![
                TreeNode { value: 4, children: vec![] },
                TreeNode { value: 5, children: vec![] },
            ]},
        ],
    };
    assert_eq!(cluster_leaf_count(&tree), 3);
}

#[test]
fn test_cluster_height() {
    let tree = TreeNode {
        value: 1,
        children: vec![
            TreeNode { value: 2, children: vec![] },
            TreeNode { value: 3, children: vec![
                TreeNode { value: 4, children: vec![] },
                TreeNode { value: 5, children: vec![] },
            ]},
        ],
    };
    assert_eq!(cluster_height(&tree), 2);
}

#[test]
fn test_cluster_leaves() {
    let tree = TreeNode {
        value: 1,
        children: vec![
            TreeNode { value: 2, children: vec![] },
            TreeNode { value: 3, children: vec![
                TreeNode { value: 4, children: vec![] },
                TreeNode { value: 5, children: vec![] },
            ]},
        ],
    };
    let mut leaves = cluster_leaves(&tree);
    leaves.sort();
    assert_eq!(leaves, vec![2, 4, 5]);
}
