use std::{cell::RefCell, rc::Rc};

use problems::{p0055_jump_game::can_jump, p0080_remove_duplicates_from_sorted_array_2::remove_duplicates, p0134_gas_station::can_complete_circuit, p0222_count_complete_tree_nodes::count_nodes, p45_jump_game_2::jump};
use util::treenode_leetcode::TreeNode;


pub mod problems;
pub mod util;
fn main() {
    //     "PAYPALISHIRING"
    // 3
    // let s = "PAYPALISHIRING".to_string();
    // let s = "0123456789ABCD".to_string();
    // let num_rows = 4;
    // let res = problems::p0006_zigzag_conversion::convert(s, num_rows);
    // println!("{}", res);
    // let threesum_input = vec![-1,0,1,2,-1,-4];
    // let result = three_sum_final(threesum_input);
    // println!("{:?}", result);

    // let nums1 = vec![];
    // let nums2 = vec![1];
    // // let result = problems::p0004_median_of_two_sorted_arrays::find_any_position_in_two_sorted_arrays(&nums1,&nums2,1);
    // let result = problems::p0004_median_of_two_sorted_arrays::find_median_sorted_arrays(nums1,nums2);
    // println!("{:?}",result)
    // let mut nums = vec![1,2,3,4,5,6,7];
    // let k = 3;
    // println!("nums before: {:?}",nums);
    // problems::p0189_rotate_array::rotate(&mut nums, k);
    // println!("nums after: {:?}",nums);

    // let mut nums = vec![1,2,3,4];
    // let result = problems::p0238_product_of_array_except_self::product_except_self(nums);
    // println!("{:?}",result)
    // let citations = vec![1,3,1,5];
    // let result = problems::p0274_h_index::h_index(citations);
    // println!("{:?}",result);
    // let test_string1 = "abba".to_string();
    // let test_string2 = "dog cat cat dog".to_string();
    // let result = word_pattern(test_string1, test_string2);
    // println!("{:?}",result);
    // let res = problems::p0219_contains_duplicate_ii::contains_nearby_duplicate(vec![1,2,3,1,2,3],2);
    // println!("{:?}",res)
    // let input_val = Some(Rc::new(RefCell::new(TreeNode {
    //     val: 1,
    //     left: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 2,
    //         left: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 4,
    //             left: None,
    //             right: None,
    //         }))),
    //         right: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 5,
    //             left: None,
    //             right: None,
    //         }))),
    //     }))),
    //     right: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 3,
    //         left: Some(Rc::new(RefCell::new(TreeNode {
    //             val: 6,
    //             left: None,
    //             right: None,
    //         }))),
    //         right: None,
    //     }))),
    // })));
    // let res = count_nodes(input_val);
    // println!("{:?}", res)
    // let res = remove_duplicates(&mut vec![1,1,1,2,2,3]);
    // let res = jump(vec![2,1]);
    // println!("{:?}",res);
    let res = can_complete_circuit(vec![2,3,4], vec![3,4,3]);
    println!("{:?}",res)
    
}
