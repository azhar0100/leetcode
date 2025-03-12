/*
 * @lc app=leetcode id=222 lang=rust
 *
 * [222] Count Complete Tree Nodes
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

pub fn number_of_levels_in_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> usize{
    match root{
        Some(root) => {
            let root_node = root.borrow();
            1 + number_of_levels_in_complete_tree(root_node.left.clone())
        },
        None => 0,
    }
}

pub fn how_many_less_from_complete_tree(root:Option<Rc<RefCell<TreeNode>>>,current_level:usize, max_levels:usize) -> usize{
    match root{
        Some(root) => {
            let root_node = root.borrow();
            let left_node = (&root_node.left).clone();
            let right_node = (&root_node.right).clone();
            match (left_node,right_node){
                (None, None) => match current_level.cmp(&(max_levels-1)){
                    std::cmp::Ordering::Less => 2,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => panic!("Something wrong happened"),
                },
                (None, Some(_right)) => panic!("Not a complete tree"),
                (Some(left), None) => {
                    1
                },
                (Some(left), Some(right)) => {
                    match current_level.cmp(&(max_levels-1)){
                        std::cmp::Ordering::Less => {
                            let nodes_missing_on_right_side = how_many_less_from_complete_tree(Some(right), current_level+1, max_levels);
                            let total_levels_in_this_subtree = max_levels - current_level;
                            let total_nodes_in_this_subtree = 1_usize << total_levels_in_this_subtree;
                            let total_nodes_in_right_subtree = total_nodes_in_this_subtree >> 2;
                            match nodes_missing_on_right_side.cmp(&total_nodes_in_right_subtree){
                                std::cmp::Ordering::Less => nodes_missing_on_right_side,
                                std::cmp::Ordering::Equal => nodes_missing_on_right_side + how_many_less_from_complete_tree(Some(left), current_level+1, max_levels),
                                std::cmp::Ordering::Greater => panic!("Something wrong happened"),
                            }

                        },
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => panic!("Something wrong has happened"),
                    }
                },
            }
        },
        None => 0,
    }
}

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    root.map(|root| {
        let number_of_levels = number_of_levels_in_complete_tree(Some(root.clone()));
        let how_many_less = how_many_less_from_complete_tree(Some(root.clone()), 0, number_of_levels) as i32;
        (1 << number_of_levels) - 1 - how_many_less
    }).unwrap_or(0)
}
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        count_nodes(root)
    }
}
// @lc code=end

