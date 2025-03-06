/*
 * @lc app=leetcode id=205 lang=rust
 *
 * [205] Isomorphic Strings
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {

    pub fn is_isomorphic(s_str: String, t_str: String) -> bool {
        match s_str.len() == t_str.len() {
            true => {
                let mut dict = HashMap::new();
                s_str
                    .chars()
                    .zip(t_str.chars())
                    .all(|(s, t)| {
                        match dict.get(&s){
                            Some(&t_) => {
                                match t == t_{
                                    true => true,
                                    false => false,
                                }
                            },
                            None => {
                                dict.insert(s, t);
                                true
                            },
                        }
                    }
                )
            }
            false => false,
        }
    }
    
}
// @lc code=end

