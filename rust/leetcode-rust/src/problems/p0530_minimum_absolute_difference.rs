use std::{cell::RefCell, i32, rc::Rc};

use crate::util::treenode_leetcode::TreeNode;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        Some(root) => {
            let mut result = vec![];
            let root_node = root.borrow();
            let left_node = root_node.left.clone();
            result.extend(inorder_traversal(left_node));
            let right_node = root_node.right.clone();
            result.push(root_node.val);
            result.extend(inorder_traversal(right_node));
            result
        }
        None => Vec::new(),
    }
}

pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    inorder_traversal(root)
        .windows(2)
        .map(|x| x[1] - x[0])
        .min()
        .unwrap_or(i32::MAX)
}
