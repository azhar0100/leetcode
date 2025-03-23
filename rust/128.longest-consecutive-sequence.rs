/*
 * @lc app=leetcode id=128 lang=rust
 *
 * [128] Longest Consecutive Sequence
 */

// @lc code=start
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanState {
    pub visited: HashSet<i32>,
    pub unvisited: HashSet<i32>,
    pub longest: i32,
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums_set: HashSet<i32> = nums.into_iter().collect();
    nums_set
        .iter()
        .fold(
            ScanState {
                visited: HashSet::new(),
                unvisited: nums_set.clone(),
                longest: 0,
            },
            |mut state, &num| match state.visited.contains(&num) {
                true => state,
                false => {
                    let mut current = num;
                    let mut current_longest = 1;
                    while state.unvisited.contains(&(current + 1)) {
                        current += 1;
                        current_longest += 1;
                        state.visited.insert(current);
                        state.unvisited.remove(&current);
                    }
                    current = num;
                    while state.unvisited.contains(&(current - 1)) {
                        current -= 1;
                        current_longest += 1;
                        state.visited.insert(current);
                        state.unvisited.remove(&current);
                    }
                    state.longest = state.longest.max(current_longest);
                    state.visited.insert(num);
                    state.unvisited.remove(&num);
                    state
                }
            },
        )
        .longest
}


impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        longest_consecutive(nums)
    }
}
// @lc code=end
