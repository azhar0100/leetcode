/*
 * @lc app=leetcode id=290 lang=rust
 *
 * [290] Word Pattern
 */

// @lc code=start

use std::collections::{HashMap, HashSet};

fn is_mapped(dict_s2t: &mut HashMap<char, char>, s: char, t: char) -> bool {
    dict_s2t.get(&s).map(|t_| t_ == &t).unwrap_or_else(|| {
        dict_s2t.insert(s, t);
        true
    })
}

pub fn is_isomorphic(s_str: String, t_str: String) -> bool {
    match s_str.len() == t_str.len() {
        true => {
            let mut dict_s2t = HashMap::new();
            let mut dict_t2s = HashMap::new();
            s_str
                .chars()
                .zip(t_str.chars())
                .all(|(s, t)| is_mapped(&mut dict_s2t, s, t) && is_mapped(&mut dict_t2s, t, s))
        }
        false => false,
    }
}

pub fn word_pattern(pattern: String, s: String) -> bool {
    let split = s.trim().split(' ');
    let pattern_dict = split
        .clone()
        .collect::<HashSet<&str>>()
        .into_iter()
        .enumerate()
        .map(|(i, split)| (split.to_string(), i))
        .collect::<HashMap<String, usize>>();
    let pattern_pattern = split
        .map(|split_i| ((pattern_dict.get(split_i).unwrap() + 65) as u8) as char)
        .collect();
    is_isomorphic(pattern, pattern_pattern)
}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        word_pattern(pattern, s)       
    }
}
// @lc code=end

