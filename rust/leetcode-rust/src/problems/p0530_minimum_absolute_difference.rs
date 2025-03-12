use std::{cell::RefCell, i32, rc::Rc};

use crate::util::treenode_leetcode::TreeNode;

pub fn get_minimum_difference_against_target(root: Option<Rc<RefCell<TreeNode>>>,target:i32,node_to_avoid:Rc<RefCell<TreeNode>>) -> Option<i32>{
    root.map(|root| -> Option<i32> {
        let is_node_to_avoid = root == node_to_avoid;
        let root_borrow = root.borrow();
        let root_diff = root_borrow.val - target;
        let root_abs_diff = match is_node_to_avoid{
            true => None,
            false => Some(root_diff.abs()),
        };

        // match root_diff.cmp(&0){
        //     std::cmp::Ordering::Less => {

        //     },
        //     std::cmp::Ordering::Equal => todo!(),
        //     std::cmp::Ordering::Greater => todo!(),
        // }



        let left_abs_diff = root_borrow.left.clone().map(|x| (x.borrow().val.clone() - target).abs());
        let right_abs_diff = root_borrow.right.clone().map(|x| (x.borrow().val.clone() - target).abs());
        let option_cmp = |x_val:Option<i32>,y_val:Option<i32>| -> std::cmp::Ordering {
            match (x_val,y_val){
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(x_val), Some(y_val)) => x_val.cmp(&y_val),
            }
        };
        let directions_to_check = match (option_cmp(left_abs_diff,root_abs_diff),option_cmp(root_abs_diff,right_abs_diff),option_cmp(left_abs_diff,right_abs_diff)){
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less, std::cmp::Ordering::Less) => vec![1],
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => panic!("impossible"),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => vec![],
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => todo!(),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => todo!(),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => todo!(),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => todo!(),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => todo!(),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => todo!(),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less, std::cmp::Ordering::Less) => todo!(),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => todo!(),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => todo!(),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => todo!(),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => todo!(),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => todo!(),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => todo!(),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => todo!(),
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => todo!(),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less, std::cmp::Ordering::Less) => todo!(),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => todo!(),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => todo!(),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => todo!(),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => todo!(),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => todo!(),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => todo!(),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => todo!(),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => todo!(),
        };
        directions_to_check.into_iter().map(|x| match x{
            0 => root_abs_diff,
            1 => left_abs_diff.map(|left_abs_diff| get_minimum_difference_against_target(root_borrow.left.clone(), target, node_to_avoid.clone()).map(|x| left_abs_diff.min(x))).flatten(),
            2 => right_abs_diff.map(|right_abs_diff| get_minimum_difference_against_target(root_borrow.right.clone(), target, node_to_avoid.clone()).map(|x| right_abs_diff.min(x))).flatten(),
            _ => panic!("Something wrong happened")
        }).filter_map(|x| x).min()


        // let vec_repr = vec![Some(root_abs_diff),left_abs_diff,right_abs_diff];
        // let mut indices: Vec<usize> = vec![0,1,2];
        // indices.sort_by(|x,y| {
        //     let (x_val,y_val) = (vec_repr[*x],vec_repr[*y]);
        //     match (x_val,y_val){
        //         (None, None) => std::cmp::Ordering::Equal,
        //         (None, Some(_)) => std::cmp::Ordering::Greater,
        //         (Some(_), None) => std::cmp::Ordering::Less,
        //         (Some(x_val), Some(y_val)) => x_val.cmp(&y_val),
        //     }
        // });
        // let first_second_third
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