use std::{cell::RefCell, rc::Rc};

use crate::util::treenode_leetcode::TreeNode;

use super::p100_same_tree::is_same_tree;

pub fn is_same_tree_mirrored(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let p_val = &p.borrow().val.clone();
            let q_val = &q.borrow().val.clone();
            p_val == q_val
                && is_same_tree(p.borrow().left.clone(), q.borrow().right.clone())
                && is_same_tree(p.borrow().right.clone(), q.borrow().left.clone())
        }
        _ => false,
    }
}


pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(root) => {
            let root_node = root.borrow();
            match (&root_node.left, &root_node.right) {
                (None, None) => true,
                (None, Some(_)) => false,
                (Some(_), None) => false,
                (Some(left), Some(right)) => is_same_tree_mirrored(Some(left.clone()), Some(right.clone())),
            }
        }
        None => true,
    }
}
