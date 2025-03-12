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


pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root{
        Some(root) => {
            let root_node = root.borrow();
            let left_node = root_node.left.clone();
            let right_node = root_node.right.clone();
            let find_min_on_side = |node_ref:Rc<RefCell<TreeNode>>| {
                let node = node_ref.borrow();
                let current_min = (node.val.clone() - root_node.val.clone()).abs();
                let side_min = get_minimum_difference(Some(node_ref.clone()));
                current_min.min(side_min)
            };
            let left_min = left_node.map(find_min_on_side).unwrap_or(i32::MAX);
            let right_min = right_node.map(find_min_on_side).unwrap_or(i32::MAX);
            left_min.min(right_min)
            
        },
        None => i32::MAX,
    }
}

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        get_minimum_difference(root)        
    }
}
// @lc code=end

