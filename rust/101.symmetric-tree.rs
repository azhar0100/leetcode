/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

pub fn is_same_tree_mirrored(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let p_val = &p.borrow().val.clone();
            let q_val = &q.borrow().val.clone();
            p_val == q_val
                && is_same_tree_mirrored(p.borrow().left.clone(), q.borrow().right.clone())
                && is_same_tree_mirrored(p.borrow().right.clone(), q.borrow().left.clone())
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

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_symmetric(root)
    }
}
// @lc code=end

