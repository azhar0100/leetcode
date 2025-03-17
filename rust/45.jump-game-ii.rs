/*
 * @lc app=leetcode id=45 lang=rust
 *
 * [45] Jump Game II
 */

// @lc code=start
use std::collections::HashMap;
pub fn jump(nums: Vec<i32>) -> i32 {
    let mut jump_count = 0;
    let mut i = 0;
    while i < nums.len() - 1 {
        let current_jump_length = nums[i].clone() as usize;
        let current_slice = &nums[i..(i+current_jump_length+1).min(nums.len())];
        match current_slice.iter().enumerate().max_by_key(|(_,x)| **x).map(|(i,x)| i){
            Some(optimal_jump_position) => {
                let optimal_jump_position_canonical = (optimal_jump_position+i).min(nums.len()-1);
                i = optimal_jump_position_canonical;
                jump_count+=1;
            },
            None => break,
        }

    }
    jump_count
}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        jump(nums)
    }
}
// @lc code=end
