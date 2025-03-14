/*
 * @lc app=leetcode id=45 lang=rust
 *
 * [45] Jump Game II
 */

// @lc code=start
use std::collections::HashMap;

// use std::collections::HashSet;
pub fn can_jump_from_this_index(
    nums: &[i32],
    starting_index: usize,
    accumulator: &mut HashMap<usize, Option<i32>>,
) -> Option<i32> {
    // println!("accumulator")
    match accumulator.get(&starting_index) {
        Some(memoized) => *memoized,
        None => {
            println!("Working on starting index {:?}", starting_index);
            nums.get(starting_index)
                .map(|jump| (*jump as usize + starting_index).min(nums.len() - 1))
                .map(
                    |max_index| match max_index == nums.len() - 1 {
                        true => {
                            accumulator.insert(starting_index, Some(1));
                            Some(1)
                        }
                        false => {
                            accumulator.insert(starting_index, None);
                            (starting_index + 1..max_index + 1)
                                .rev()
                                .filter_map(|new_start_position| {
                                    can_jump_from_this_index(nums, new_start_position, accumulator)
                                        .map(|x| x + 1)
                                })
                                .min()
                        }
                    }, /* match max_index == nums.len() - 1 {
                           true => Some(true),
                           false => {
                               accumulator.insert(starting_index, false);
                               Some(
                                   (starting_index + 1..max_index + 1)
                                       .rev()
                                       .filter_map(|new_start_position| {
                                           can_jump_from_this_index(nums, new_start_position, accumulator)
                                       })
                                       .any(|x| x),
                               )
                           }
                       } */
                )
                .flatten()
        }
    }
}
pub fn jump(nums: Vec<i32>) -> i32 {
    match nums.len(){
        1 => 0,
        _ => can_jump_from_this_index(&nums, 0, &mut HashMap::new()).unwrap()
    }
    
}


impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        jump(nums)
    }
}
// @lc code=end

