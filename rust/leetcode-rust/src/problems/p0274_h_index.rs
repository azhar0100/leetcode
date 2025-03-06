use std::collections::HashMap;

pub fn h_index(citations: Vec<i32>) -> i32 {
    let buckets: HashMap<i32, i32> = citations.iter().fold(HashMap::new(), |mut acc, k| {
        acc.entry(*k).and_modify(|x| *x += 1).or_insert(1);
        acc
    });
    let mut unique_citations_numbers = buckets.keys().cloned().collect::<Vec<_>>();
    unique_citations_numbers.sort();
    unique_citations_numbers
        .iter()
        .rev()
        .scan(0, |acc, &x| {
            *acc = *acc + buckets.get(&x).unwrap();
            Some((*acc, x.clone() as i32))
        })
        .map(|(at_least_count, key): (i32, i32)| (key, at_least_count))
        .collect::<HashMap<i32, i32>>()
        .into_iter()
        .map(|(k, v)| k.min(v))
        .max()
        .unwrap()
}
