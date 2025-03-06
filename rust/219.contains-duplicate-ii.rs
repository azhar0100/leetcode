/*
 * @lc app=leetcode id=219 lang=rust
 *
 * [219] Contains Duplicate II
 */

// @lc code=start
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoublePointer {
    pub left: usize,
    pub right: usize,
}

impl DoublePointer {
    pub fn advance_both(&mut self) {
        self.left += 1;
        self.right += 1;
    }

    pub fn advance_right(&mut self) {
        self.right += 1;
    }

    pub fn advance_left(&mut self) {
        self.left += 1;
    }

    pub fn len(&self) -> isize {
        (self.right as isize) - (self.left as isize) + 1
    }

    pub fn jump_to(&mut self, idx: usize) {
        self.left = idx;
        self.right = idx;
    }

    pub fn jump_to_right(&mut self) {
        self.left = self.right;
    }

    pub fn jump_to_left(&mut self) {
        self.right = self.left;
    }

    pub fn jump_left_to(&mut self, idx: usize) {
        self.left = idx;
    }

    pub fn jump_right_to(&mut self, idx: usize) {
        self.right = idx;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafeDoublePointerError {
    LeftPointerExceedsMaxLen,
    RightPointerExceedsMaxLen,
    BothPointersExceedMaxLen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafeDoublePointer {
    pub double_pointer: DoublePointer,
    pub max_len: usize,
}

impl SafeDoublePointer {
    pub fn new(max_len: usize) -> Self {
        SafeDoublePointer {
            double_pointer: DoublePointer { left: 0, right: 0 },
            max_len,
        }
    }

    pub fn advance_left(&mut self) -> Result<(), SafeDoublePointerError> {
        if self.double_pointer.left >= (self.max_len - 1) {
            return Err(SafeDoublePointerError::LeftPointerExceedsMaxLen);
        }
        self.double_pointer.advance_left();
        Ok(())
    }

    pub fn advance_right(&mut self) -> Result<(), SafeDoublePointerError> {
        if self.double_pointer.right >= (self.max_len - 1) {
            return Err(SafeDoublePointerError::RightPointerExceedsMaxLen);
        }
        self.double_pointer.advance_right();
        Ok(())
    }

    pub fn retreat_left(&mut self) -> Result<(), SafeDoublePointerError> {
        if self.double_pointer.left == 0 {
            return Err(SafeDoublePointerError::LeftPointerExceedsMaxLen);
        }
        self.double_pointer.left -= 1;
        Ok(())
    }

    pub fn retreat_right(&mut self) -> Result<(), SafeDoublePointerError> {
        if self.double_pointer.right == 0 {
            return Err(SafeDoublePointerError::RightPointerExceedsMaxLen);
        }
        self.double_pointer.right -= 1;
        Ok(())
    }

    pub fn advance_both(&mut self) -> Result<(), SafeDoublePointerError> {
        self.advance_left()?;
        self.advance_right()?;
        Ok(())
    }

    pub fn jump_to(&mut self, idx: usize) -> Result<(), SafeDoublePointerError> {
        if idx >= self.max_len {
            return Err(SafeDoublePointerError::BothPointersExceedMaxLen);
        }
        self.double_pointer.jump_to(idx);
        Ok(())
    }

    pub fn jump_to_right(&mut self) {
        self.double_pointer.jump_to_right();
    }

    pub fn jump_to_left(&mut self) {
        self.double_pointer.jump_to_left();
    }

    pub fn jump_left_to(&mut self, idx: usize) -> Result<(), SafeDoublePointerError> {
        if idx >= self.max_len {
            return Err(SafeDoublePointerError::RightPointerExceedsMaxLen);
        }
        self.double_pointer.jump_left_to(idx);
        Ok(())
    }

    pub fn jump_right_to(&mut self, idx: usize) -> Result<(), SafeDoublePointerError> {
        if idx >= self.max_len {
            return Err(SafeDoublePointerError::RightPointerExceedsMaxLen);
        }
        self.double_pointer.jump_right_to(idx);
        Ok(())
    }

    pub fn len(&self) -> isize {
        self.double_pointer.len()
    }

    pub fn can_advance_left(&self) -> bool {
        self.double_pointer.left < self.max_len
    }

    pub fn can_advance_right(&self) -> bool {
        self.double_pointer.right < self.max_len
    }

    pub fn can_advance_both(&self) -> bool {
        self.can_advance_left() && self.can_advance_right()
    }
}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 {
            return false;
        };
        let mut pointer = SafeDoublePointer::new(nums.len());
        let mut running_map = HashSet::new();
        running_map.insert(nums[pointer.double_pointer.left].clone());
        loop {
            match pointer.len() != 1 {
                true => match pointer.len() > (k).try_into().unwrap() {
                    true => {
                        let left_value = nums[pointer.double_pointer.left];
                        running_map.remove(&left_value);
                        pointer.advance_left().unwrap()
                    }
                    false => match pointer.advance_right() {
                        Ok(_) => {
                            let right_value = nums.get(pointer.double_pointer.right).unwrap();
                            match running_map.insert(right_value.clone()) {
                                true => (),
                                false => return true,
                            }
                        }
                        Err(_) => return false,
                    },
                },
                false => match pointer.advance_right() {
                    Ok(_) => {
                        let right_value = nums.get(pointer.double_pointer.right).unwrap();
                        match running_map.insert(right_value.clone()) {
                            true => (),
                            false => return true,
                        }
                    }
                    Err(_) => return false,
                },
            }
        }
    }
}
// @lc code=end
