/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoublePointer {
    pub left: usize,
    pub right: usize,
}

impl DoublePointer{
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
pub enum SafeDoublePointerError{
    LeftPointerExceedsMaxLen,
    RightPointerExceedsMaxLen,
    BothPointersExceedMaxLen,
}

pub struct SafeDoublePointer{
    pub double_pointer: DoublePointer,
    pub max_len: usize,
}

impl SafeDoublePointer{
    pub fn new(max_len:usize) -> Self {
        SafeDoublePointer{
            double_pointer: DoublePointer{left: 0, right: 0},
            max_len,
        }
    }

    pub fn advance_left(&mut self) -> Result<(), SafeDoublePointerError> {
        if self.double_pointer.left >= self.max_len {
            return Err(SafeDoublePointerError::LeftPointerExceedsMaxLen);
        }
        self.double_pointer.advance_left();
        Ok(())
    }

    pub fn advance_right(&mut self) -> Result<(), SafeDoublePointerError> {
        if self.double_pointer.right >= self.max_len {
            return Err(SafeDoublePointerError::RightPointerExceedsMaxLen);
        }
        self.double_pointer.advance_right();
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

use std::collections::HashSet;
impl Solution {
    
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hashset_stack: HashSet<char> = HashSet::new();
        const INIT_VALUE: usize = 0;
        let mut pointer = SafeDoublePointer::new(s.len());
        let mut max_length = INIT_VALUE;
        loop {
            let current_end_pointer = pointer.double_pointer.right;
            let current_char = match s.chars().nth(current_end_pointer){
                Some(c) => c,
                None => break,
            };
            match hashset_stack.contains(&current_char) {
                true => {
                    let left_character = s.chars().nth(pointer.double_pointer.left).unwrap();
                    hashset_stack.remove(&left_character);
                    match pointer.advance_left() {
                        Ok(()) => (),
                        Err(_) => break,
                    }
                }
                false => {
                    hashset_stack.insert(current_char);
                    max_length = max_length.max(pointer.len().abs() as usize);
                    match pointer.advance_right() {
                        Ok(()) => (),
                        Err(_) => break,
                    }
                }
            };
        }
        max_length as i32
    }
    
}
// @lc code=end
