/*
 * @lc app=leetcode id=392 lang=rust
 *
 * [392] Is Subsequence
 */

// @lc code=start

pub fn is_subsequence_starting_from_index(s: &str, t: &str) -> bool {
    let first_s = s.get(0..1);
    let first_t = t.get(0..1);
    match (first_s, first_t) {
        (None, None) => true,
        (None, Some(_)) => true,
        (Some(_), None) => false,
        (Some(first_s), Some(first_t)) => {
            let last_s = s.get(1..);
            let last_t = t.get(1..);
            match (first_s == first_t, last_s, last_t) {
                (true, None, _) => true,
                (true, Some(_), None) => false,
                (true, Some(last_s), Some(last_t)) => {
                    is_subsequence_starting_from_index(last_s, last_t)
                }
                (false, Some(_last_s), Some(last_t)) => {
                    is_subsequence_starting_from_index(s, last_t)
                }
                (false, _, _) => false,
            }
        }
    }
}

pub fn is_subsequence(s: String, t: String) -> bool {
    is_subsequence_starting_from_index(s.as_str(), t.as_str())
}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        is_subsequence(s, t)
    }
}
// @lc code=end
