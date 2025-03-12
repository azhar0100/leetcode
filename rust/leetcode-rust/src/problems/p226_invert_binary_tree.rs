use std::{cell::RefCell, rc::Rc};

use crate::util::treenode_leetcode::TreeNode;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root{
        Some(root) => {
            Some(Rc::new(RefCell::new(TreeNode{
                val: root.borrow().val.clone(),
                left: invert_tree(root.borrow().right.clone()),
                right: invert_tree(root.borrow().left.clone()),
            })))
        },
        None => None,
    }
        
}