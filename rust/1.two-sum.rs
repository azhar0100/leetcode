/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start

use std::collections::HashMap;
pub fn search_sorted(nums: &[i32], target: i32) -> Option<usize> {
    nums.get(nums.partition_point(|&x| x < target))
        .map(|&x| match x == target {
            true => Some(nums.partition_point(|&x| x < target)),
            false => None,
        })
        .flatten()
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap_indices = HashMap::new();
    let mut result = Vec::new();
    nums.iter().enumerate().for_each(|(i, &x)|{
        let complement = target - x;
        hashmap_indices.get(&complement).map(|&j|{
            result.push(i as i32);
            result.push(j as i32);
        });
        hashmap_indices.insert(x, i);        
    });
    result
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        two_sum(nums, target)            
    }
}

// @lc code=end
