use std::{cell::RefCell, rc::Rc};

use crate::util::treenode_leetcode::TreeNode;

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
