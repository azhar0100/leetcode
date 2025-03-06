use std::collections::{HashMap, HashSet};

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
    super::p0205_isomorphic_strings::is_isomorphic(pattern, pattern_pattern)
}
