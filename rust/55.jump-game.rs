/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 */

// @lc code=start
use std::collections::HashMap;

// use std::collections::HashSet;
pub fn can_jump_from_this_index(
    nums: &[i32],
    starting_index: usize,
    accumulator: &mut HashMap<usize, bool>,
) -> Option<bool> {
    // println!("accumulator")
    match accumulator.get(&starting_index) {
        Some(memoized) => Some(*memoized),
        None => {
            println!("Working on starting index {:?}", starting_index);
            nums.get(starting_index)
                .map(|jump| (*jump as usize + starting_index).min(nums.len() - 1))
                .map(|max_index| match max_index == nums.len() - 1 {
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
                })
                .flatten()
        }
    }
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    can_jump_from_this_index(&nums, 0, &mut HashMap::new()).unwrap_or(false)
    // let mut indices_can_jump = HashSet::new();
    // indices_can_jump.insert(0);
    // println!("{:?} indices can jump before loop", indices_can_jump);
    // for (i, x) in nums.iter().enumerate() {
    //     if indices_can_jump.contains(&i) {
    //         for j in i..(i + *x as usize + 1).min(nums.len()) {
    //             indices_can_jump.insert(j);
    //         }
    //     }
    //     println!(
    //         "{:?} indices can jump after loop iteration {:?}",
    //         indices_can_jump, i
    //     )
    // }
    // indices_can_jump.contains(&(nums.len() - 1))
}


impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        can_jump(nums)
    }
}
// @lc code=end
