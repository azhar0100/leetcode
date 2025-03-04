/*
 * @lc app=leetcode id=274 lang=rust
 *
 * [274] H-Index
 */

// @lc code=start
use std::collections::{HashMap, HashSet};

pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut unique_citations_numbers = citations
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    unique_citations_numbers.sort();
    let buckets: HashMap<i32, u32> = citations.iter().fold(HashMap::new(), |mut acc, k| {
        acc.entry(*k).and_modify(|x| *x = *x + 1);
        acc
    });
    let bucket_numbers = unique_citations_numbers
        .iter()
        .rev()
        .scan(0, |mut acc, k| {
            *acc = *acc + buckets.get(k).unwrap();
            Some(acc.clone())
        })
        .enumerate()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    // fn f((i, k): (usize, &u32)) -> u32 {
    //     let one_based_idx = i + 1;
    //     k.min(one_based_idx as u32)
    // }
    bucket_numbers
        .iter()
        .map(|(i, k)| {
            let one_based_idx = (i + 1) as u32;
            k.min(&one_based_idx).clone()
        })
        .max()
        .unwrap() as i32
}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        h_index(citations)        
    }
}
// @lc code=end

