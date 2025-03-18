/*
 * @lc app=leetcode id=151 lang=rust
 *
 * [151] Reverse Words in a String
 */

// @lc code=start
pub fn reverse_words(s: String) -> String {
    s.split(" ")
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}



impl Solution {
    pub fn reverse_words(s: String) -> String {
        reverse_words(s)        
    }
}
// @lc code=end

