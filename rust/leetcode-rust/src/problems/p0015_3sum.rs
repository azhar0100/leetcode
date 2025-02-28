
use crate::problems::p0001_two_sum::two_sum_better_return_format;
use super::p0001_two_sum::two_sum_double_pointer_on_sorted;

use std::collections::{HashMap, HashSet};
use crate::util::double_pointer::SafeDoublePointer;

pub fn three_sum(nums_arg: Vec<i32>) -> HashSet<(i32, i32, i32)> {
    let nums_multiplicity: HashMap<i32, usize> = nums_arg.iter().fold(HashMap::new(), |mut acc, &x| {
        acc.entry(x).and_modify(|e| *e += 1).or_insert(1);
        acc
    });
    let mut nums = nums_multiplicity
        .keys()
        .map(|&x| x.clone())
        .into_iter()
        .collect::<Vec<_>>();
    nums.sort();
    let mut result = HashSet::new();
    for (i, num) in nums.iter().enumerate() {
        let sum_indices = {
            let nums: &[i32] = &nums;
            let target = -num;
            let mut double_pointer = SafeDoublePointer::new(nums.len());
            double_pointer
                .jump_right_to(nums.len() - 1)
                .expect("jump_right_to failed");
            let mut result = HashSet::new();
            'outer_loop: while double_pointer.len() >= 0 {
                // println!("{:?}", double_pointer);
                let sum =
                    nums[double_pointer.double_pointer.left] + nums[double_pointer.double_pointer.right];
                match sum.cmp(&target) {
                    std::cmp::Ordering::Equal => {
                        result.insert((
                            double_pointer.double_pointer.left,
                            double_pointer.double_pointer.right,
                        ));
                        match double_pointer.advance_left() {
                            Ok(()) => (),
                            Err(_) => break 'outer_loop,
                        };
                    }
                    std::cmp::Ordering::Less => {
                        match double_pointer.advance_left() {
                            Ok(()) => (),
                            Err(_) => break 'outer_loop,
                        };
                    }
                    std::cmp::Ordering::Greater => {
                        match double_pointer.retreat_right() {
                            Ok(()) => (),
                            Err(_) => break 'outer_loop,
                        };
                    }
                }
            }
            // let values_at_result_indices: Vec<_> =
            //     result.iter().map(|(i, j)| (nums[*i], nums[*j])).collect();
            // println!(
            //     "Two sum result for {:?},{:?} is {:?} which maps to {:?}",
            //     nums, target, result, values_at_result_indices
            // );
            result
        };
        // println!("{:?} sum_indices for num {:?}", sum_indices, num);
        for (j, k) in sum_indices.iter() {
            let triplet = vec![nums[i], nums[*j], nums[*k]];
            let triplet_multiplicity: HashMap<i32, usize> = triplet.iter().fold(HashMap::new(), |mut acc, &x| {
                acc.entry(x).and_modify(|e| *e += 1).or_insert(1);
                acc
            });
            let is_triplet_multiplicity_valid = triplet_multiplicity.iter().all(|(x, &x_multiplicity)| {
                match nums_multiplicity.get(x) {
                    Some(&x_multiplicity_in_nums) => x_multiplicity <= x_multiplicity_in_nums,
                    None => false,
                }
            });
            if is_triplet_multiplicity_valid {
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
