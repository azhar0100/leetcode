
// In src/problems/p0001_two_sum.rs
pub struct Solution;

use std::collections::{HashMap, HashSet};

use crate::util::double_pointer::SafeDoublePointer;
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

pub fn two_sum_double_pointer_on_sorted(nums: &[i32], target: i32) -> HashSet<(usize, usize)> {
    let mut double_pointer = SafeDoublePointer::new(nums.len());
    double_pointer.jump_right_to(nums.len()-1).expect("It should have this index");
    let mut result = HashSet::new();
    loop{
        let sum = nums[double_pointer.double_pointer.left] + nums[double_pointer.double_pointer.right];
        match sum.cmp(&target){
            std::cmp::Ordering::Equal => {
                result.insert((double_pointer.double_pointer.left, double_pointer.double_pointer.right));
                match double_pointer.advance_left(){
                    Ok(()) => (),
                    Err(_) => break,
                };
            },
            std::cmp::Ordering::Less => {
                match double_pointer.advance_left(){
                    Ok(()) => (),
                    Err(_) => break,
                };
            },
            std::cmp::Ordering::Greater => {
                match double_pointer.advance_right(){
                    Ok(()) => (),
                    Err(_) => break,
                };
            },
        }
    }
    result
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        two_sum(nums, target)            
    }
}
