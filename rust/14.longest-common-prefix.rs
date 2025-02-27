/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let min_length = strs.iter().map(|s| s.len()).min().unwrap_or(0);
        let mut prefix = String::new();
        for i in 0..min_length {
            let c = strs.get(0).map(|x| x.chars().nth(i)).flatten();
            match c {
                Some(c) => match strs.iter().all(|s| s.chars().nth(i) == Some(c)) {
                    true => prefix.push(c),
                    false => break,
                },
                None => break,
            }
        }
        prefix
    }
}
// @lc code=end
