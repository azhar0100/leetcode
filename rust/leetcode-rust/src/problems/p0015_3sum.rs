use std::collections::HashSet;

use crate::problems::p0001_two_sum::two_sum_better_return_format;

use super::p0001_two_sum::two_sum_double_pointer_on_sorted;

pub fn three_sum(nums_arg: Vec<i32>) -> HashSet<(i32, i32, i32)> {
    let mut nums = nums_arg
        .into_iter()
        // .collect::<HashSet<i32>>()
        // .into_iter()
        .collect::<Vec<_>>();
    nums.sort();
    let mut result = HashSet::new();
    for (i, num) in nums.iter().enumerate() {
        let sum_indices = two_sum_double_pointer_on_sorted(&nums, -num);
        // println!("{:?} sum_indices for num {:?}", sum_indices, num);
        for (j, k) in sum_indices.iter() {
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
    result
}

pub fn three_sum_final(nums: Vec<i32>) -> Vec<Vec<i32>> {
    three_sum(nums)
        .into_iter()
        .map(|(i, j, k)| vec![i as i32, j as i32, k as i32])
        .collect()
}
