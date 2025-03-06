/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim()
            .split(' ')
            .last()
            .map(|x| x.len() as i32)
            .unwrap()
    }
}
// @lc code=end
