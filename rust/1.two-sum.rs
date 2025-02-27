/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashSet;
pub fn search_sorted(nums: &[i32], target: i32) -> Option<usize> {
    nums.get(nums.partition_point(|&x| x < target))
        .map(|&x| match x == target {
            true => Some(nums.partition_point(|&x| x < target)),
            false => None,
        })
        .flatten()
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted_nums_idxs = (0..nums.len()).collect::<Vec<usize>>();
        sorted_nums_idxs.sort_by_key(|&i| nums[i]);
        sorted_nums_idxs
            .iter()
            .map(|&i| nums[i])
            .collect::<Vec<i32>>()
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| {
                let y = target - x;
                search_sorted(
                    &sorted_nums_idxs
                        .iter()
                        .map(|&i| nums[i])
                        .collect::<Vec<i32>>(),
                    y,
                )
                .map(|j| match i == j {
                    true => None,
                    false => Some(std::iter::once(i).chain(std::iter::once(j))),
                }).flatten()
            })
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|i| sorted_nums_idxs[i as usize] as i32)
            .collect()
            
    }
}

// @lc code=end
