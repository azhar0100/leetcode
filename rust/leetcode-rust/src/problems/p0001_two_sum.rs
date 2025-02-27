// In src/problems/p0001_two_sum.rs
pub struct Solution;

pub fn search_sorted(
    nums: &[i32],
    target: i32,
    left: Option<usize>,
    right: Option<usize>,
) -> Option<usize> {
    let left = left.unwrap_or(0);
    let right = right.unwrap_or(nums.len() - 1);
    let half = (left + right) / 2;
    let value_at_half = nums.get(half);
    value_at_half
        .map(|&x| match x.cmp(&target) {
            std::cmp::Ordering::Equal => Some(half),
            std::cmp::Ordering::Less => search_sorted(nums, target, Some(half + 1), Some(right)),
            std::cmp::Ordering::Greater => search_sorted(nums, target, Some(left), Some(half - 1)),
        })
        .flatten()
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        nums.iter()
            .enumerate()
            .find_map(|(i, &x)| {
                let y = target - x;
                search_sorted(&sorted_nums, y, None, None).map(|j| vec![i as i32, j as i32])
            })
            .unwrap()
    }
}
