/*
 * @lc app=leetcode id=530 lang=rust
 *
 * [530] Minimum Absolute Difference in BST
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
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        get_minimum_difference(root)
    }
}
// @lc code=end

