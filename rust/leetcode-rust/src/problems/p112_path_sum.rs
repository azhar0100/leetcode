use std::{cell::RefCell, rc::Rc};

use crate::util::treenode_leetcode::TreeNode;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    match root {
        Some(root) => {
            let root_node = root.borrow();
            match (&root_node.left, &root_node.right) {
                (None, None) => target_sum - root_node.val == 0,
                (None, Some(right)) => {
                    has_path_sum(Some(right.clone()), target_sum - root_node.val)
                }
                (Some(left), None) => has_path_sum(Some(left.clone()), target_sum - root_node.val),
                (Some(left), Some(right)) => {
                    has_path_sum(Some(left.clone()), target_sum - root_node.val)
                        || has_path_sum(Some(right.clone()), target_sum - root_node.val)
                }
            }
        }
        None => false,
    }
}
