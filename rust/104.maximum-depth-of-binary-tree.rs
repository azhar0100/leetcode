/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
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

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(root) => {
            let root_node_ref = root.borrow();
            match (&root_node_ref.left, &root_node_ref.right) {
                (None, None) => 1,
                (None, Some(right)) => 1 + max_depth(Some(right.clone())),
                (Some(left), None) => 1 + max_depth(Some(left.clone())),
                (Some(left), Some(right)) => {
                    1 + max_depth(Some(right.clone())).max(max_depth(Some(left.clone())))
                }
            }
        }
        None => 0,
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_depth(root)        
    }
}
// @lc code=end

