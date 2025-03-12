/*
 * @lc app=leetcode id=637 lang=rust
 *
 * [637] Average of Levels in Binary Tree
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
use std::{cell::RefCell, collections::HashMap, rc::Rc};
#[derive(Debug, Clone)]
pub struct LevelData {
    sum: f64,
    count: usize,
}

pub fn traverse_binary_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
    accumulator: &mut HashMap<usize, LevelData>,
    current_level: usize,
) {
    match root {
        Some(root) => {
            let root_node = root.borrow();
            let level_data = accumulator
                .entry(current_level)
                .or_insert(LevelData { sum: 0.0, count: 0 });
            level_data.count += 1;
            level_data.sum += root_node.val as f64;
            traverse_binary_tree(root_node.left.clone(), accumulator, current_level + 1);
            traverse_binary_tree(root_node.right.clone(), accumulator, current_level + 1);
        }
        None => (),
    }
}

pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    let mut accumulator = HashMap::new();
    traverse_binary_tree(root, &mut accumulator, 0);
    let mut level_averages_map = accumulator
        .into_iter()
        .map(|(k, level_data)| (k, level_data.sum / (level_data.count as f64)))
        .collect::<Vec<_>>();
    level_averages_map.sort_by_key(|(l, _)| l.clone());
    level_averages_map.into_iter().map(|(_, v)| v).collect()
}
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        average_of_levels(root)        
    }
}
// @lc code=end

