/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start

use std::collections::{HashMap, HashSet};
pub fn three_sum(nums: Vec<i32>) -> HashSet<(i32, i32, i32)> {
    let mut result = HashSet::new();
    for (i, num) in nums.iter().enumerate() {
        let sum_indices = {
            let nums = nums.clone();
            let target = -num;
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
            .into_iter()
            .map(|x| x as usize)
            .collect::<Vec<_>>();
        for j in sum_indices.iter() {
            for k in sum_indices.iter() {
                if i != *j && i != *k && j != k {
                    let mut triplet = vec![nums[i], nums[*j], nums[*k]];
                    triplet.sort();
                    let triplet_sum: i32 = triplet.iter().sum();
                    if triplet_sum == 0 {
                        result.insert((triplet[0], triplet[1], triplet[2]));
                    }
                }
            }
        }
    }
    result
}

pub fn three_sum_final(nums: Vec<i32>) -> Vec<Vec<i32>> {
    three_sum(nums).into_iter().map(|(i, j, k)| vec![i as i32, j as i32, k as i32]).collect()
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        three_sum_final(nums)
    }
}
// @lc code=end

