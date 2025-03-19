use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut sorted_strs = strs
        .iter()
        .map(|x| {
            let mut char_vec = x.chars().collect::<Vec<_>>();
            char_vec.sort();
            char_vec.into_iter().collect::<String>()
        })
        .enumerate()
        .collect::<Vec<_>>();
    sorted_strs.sort_by_key(|(_, x)| x.clone());
    let fold_map = sorted_strs.iter().fold(HashMap::new(), |mut acc, &(i, ref x)| {
        let entries = acc.entry(x).or_insert(Vec::new());
        entries.push(i);
        acc
    });
    fold_map
        .into_iter()
        .map(|(_, v)| v.into_iter().map(|i| strs[i].clone()).collect::<Vec<_>>())
        .collect()
}
