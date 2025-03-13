/*
 * @lc app=leetcode id=190 lang=rust
 *
 * [190] Reverse Bits
 */

// @lc code=start
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut y: u32 = 0;
        (0..32).for_each(|i| {
            y |= (x >> i & 1) << (31 - i);
        });
        y
    }
}
// @lc code=end
