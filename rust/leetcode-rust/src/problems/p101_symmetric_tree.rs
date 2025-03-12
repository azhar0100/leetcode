use std::{cell::RefCell, rc::Rc};

use crate::util::treenode_leetcode::TreeNode;

use super::p100_same_tree::is_same_tree;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(root) => {
            let root_node = root.borrow();
            match (&root_node.left, &root_node.right) {
                (None, None) => true,
                (None, Some(_)) => false,
                (Some(_), None) => false,
                (Some(left), Some(right)) => is_same_tree(Some(left.clone()), Some(right.clone())),
            }
        }
        None => true,
    }
}
