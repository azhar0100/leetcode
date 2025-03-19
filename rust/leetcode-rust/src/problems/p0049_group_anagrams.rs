use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    strs.iter()
        .map(|x| {
            let mut char_vec = x.chars().collect::<Vec<_>>();
            char_vec.sort();
            char_vec.into_iter().collect::<String>()
        })
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, x)| {
            let entries = acc.entry(x).or_insert(Vec::new());
            entries.push(i);
            acc
        })
        .into_iter()
        .map(|(_, v)| v.into_iter().map(|i| strs[i].clone()).collect::<Vec<_>>())
        .collect()
}
