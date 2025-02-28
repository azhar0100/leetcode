/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
use std::collections::{HashSet,HashMap};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        if self.double_pointer.left >= (self.max_len-1) {
            return Err(SafeDoublePointerError::LeftPointerExceedsMaxLen);
        }
        self.double_pointer.advance_left();
        Ok(())
    }

    pub fn advance_right(&mut self) -> Result<(), SafeDoublePointerError> {
        if self.double_pointer.right >= (self.max_len-1) {
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

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        three_sum_final(nums)
    }
}
// @lc code=end

