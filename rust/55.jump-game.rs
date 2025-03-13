/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 */

// @lc code=start
use std::collections::HashSet;
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut indices_can_jump = HashSet::new();
    indices_can_jump.insert(0);
    for (i, x) in nums.iter().enumerate() {
        if indices_can_jump.contains(&i) {
            for j in i..(i + *x as usize + 1).min(nums.len()) {
                indices_can_jump.insert(j);
            }
        }
        if indices_can_jump.contains(&(nums.len() - 1)){
            return true
        }
        
    }
    indices_can_jump.contains(&(nums.len() - 1))
}


impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        can_jump(nums)
    }
}
// @lc code=end
