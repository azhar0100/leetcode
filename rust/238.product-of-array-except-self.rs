/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 */

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let prefixes = nums
            .iter()
            .scan(1 as i32, |acc, x| {
                *acc = *acc * x;
                Some(*acc)
            })
            .map(|x| Some(x))
            .skip(1);
        let suffixes = nums
            .iter()
            .rev()
            .scan(1 as i32, |acc, x| {
                *acc = *acc * x;
                Some(*acc)
            })
            .map(|x| Some(x))
            .skip(1);
        let prefix_iter = std::iter::once(None).chain(prefixes);
        let suffix_iter = suffixes.chain(std::iter::once(None));

        prefix_iter
            .zip(suffix_iter)
            .map(|(p, s)| match (p, s) {
                (None, None) => panic!("This should never happen"),
                (None, Some(s)) => s,
                (Some(p), None) => p,
                (Some(p), Some(s)) => p * s,
            })
            .collect()
    }
}
// @lc code=end
