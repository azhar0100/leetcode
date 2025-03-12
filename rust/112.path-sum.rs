/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
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

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        has_path_sum(root,target_sum)        
    }
}
// @lc code=end

