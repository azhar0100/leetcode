/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x.cmp(&0) {
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => true,
            std::cmp::Ordering::Greater => {
                (0..((x as f64).log10().floor() as u32 + 1))
                    .map(|i| {
                        (x / 10_i32.pow(i) % 10) == (x / 10_i32.pow(((x as f64).log10().floor() as u32 + 1) - i - 1) % 10)
                    })
                    .all(|x| x)
            }
        }
    }        
}
// @lc code=end
