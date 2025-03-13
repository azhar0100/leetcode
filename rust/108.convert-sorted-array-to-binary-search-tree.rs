/*
 * @lc app=leetcode id=108 lang=rust
 *
 * [108] Convert Sorted Array to Binary Search Tree
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

pub fn sorted_buffer_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let mid = nums.len() / 2;
    let mut root = TreeNode::new(nums[mid]);
    let left_part = &nums[..mid];
    let right_part = &nums[mid + 1..];
    root.left = match left_part.is_empty(){
        true => None,
        false => sorted_buffer_to_bst(left_part),
    };
    root.right = match right_part.is_empty(){
        true => None,
        false => sorted_buffer_to_bst(right_part),
    };
    Some(Rc::new(RefCell::new(root)))    
}


pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    sorted_buffer_to_bst(&nums)
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        sorted_array_to_bst(nums)
    }
}
// @lc code=end

