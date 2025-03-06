use std::collections::{HashMap, HashSet};

use crate::util::double_pointer::SafeDoublePointer;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut pointer = SafeDoublePointer::new(nums.len());
    let mut running_map = HashSet::new();
    running_map.insert(nums[pointer.double_pointer.left].clone());
    loop{
        match pointer.len() != 1 {
            true => {
                match pointer.len() > (k+1).try_into().unwrap(){
                    true => {
                        let left_value = nums[pointer.double_pointer.left];
                        running_map.remove(&left_value);
                        pointer.advance_left().unwrap()
                    },
                    false => {
                        match pointer.advance_right(){
                            Ok(_) => {
                                let right_value = nums.get(pointer.double_pointer.right).unwrap();
                                match running_map.insert(right_value.clone()){
                                    true => (),
                                    false => return true,
                                }
                            },
                            Err(_) => return false,
                        }
                    },
                }
            }
            false => {
                    match pointer.advance_right(){
                        Ok(_) => {
                            let right_value = nums.get(pointer.double_pointer.right).unwrap();
                            match running_map.insert(right_value.clone()){
                                true => (),
                                false => return true,
                            }
                        },
                        Err(_) => return false,
                    }
                }
        }

    }
}