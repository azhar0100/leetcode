use std::{cell::RefCell, rc::Rc};

use crate::util::treenode_leetcode::TreeNode;

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let p_val = &p.borrow().val.clone();
            let q_val = &q.borrow().val.clone();
            p_val == q_val
                && is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                && is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
        }
        _ => false,
    }
}
