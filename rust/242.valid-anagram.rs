/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {


pub fn is_anagram(s: String, t: String) -> bool {
    ({
        let s: &str = &s;
        s.chars().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|x| *x += 1).or_insert(1);
            acc
        })
    }) == {
        let s: &str = &t;
        s.chars().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|x| *x += 1).or_insert(1);
            acc
        })
    }
}
}
// @lc code=end

