use std::{cell::RefCell, i32, rc::Rc};

use crate::util::treenode_leetcode::TreeNode;

pub fn get_minimum_difference_against_target(root: Option<Rc<RefCell<TreeNode>>>,target:i32) -> Option<i32>{
    root.map(|root| {
        let root_abs_diff = (root.borrow().val - target).abs();
        let left_abs_diff = get_minimum_difference_against_target(root.borrow().left.clone(),target);
        let right_abs_diff = get_minimum_difference_against_target(root.borrow().right.clone(),target);

        vec![Some(root_abs_diff),left_abs_diff,right_abs_diff].into_iter().filter_map(|x| x).min()
    }).flatten()
}

pub fn get_minimum_difference_overall(node:Option<Rc<RefCell<TreeNode>>>,root:Option<Rc<RefCell<TreeNode>>>) -> i32{
    match node{
        Some(node) => {
            let borrowed_node = node.borrow();
            let node_val = borrowed_node.val;
            let node_target_diff = get_minimum_difference_against_target(root.clone(),node_val).unwrap_or(i32::MAX);
            let left_val = get_minimum_difference_overall(borrowed_node.left.clone(),root.clone());
            let right_val = get_minimum_difference_overall(borrowed_node.right.clone(),root.clone());
            vec![node_target_diff,left_val,right_val].into_iter().min().unwrap()
        },
        None => i32::MAX,
    }
}

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