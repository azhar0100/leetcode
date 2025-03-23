use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanState {
    pub count: usize,
    pub num_sequence_hashmap: HashMap<i32, usize>,
    pub num_sequence_length_hashmap: HashMap<usize, (i32, i32)>,
}

pub fn intersection_of_two_ranges(range1: (i32, i32), range2: (i32, i32)) -> Option<(i32, i32)> {
    let (min1, max1) = range1;
    let (min2, max2) = range2;
    let min = min1.max(min2);
    let max = max1.min(max2);
    match min <= max {
        true => Some((min, max)),
        false => None,
    }
}

pub fn intersection_of_ranges(ranges: impl Iterator<Item = (i32, i32)>) -> Option<(i32, i32)> {
    ranges.fold(None, |acc, range| {
        acc.and_then(|acc_range| intersection_of_two_ranges(acc_range, range))
    })
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold(
            ScanState {
                count: 0,
                num_sequence_hashmap: HashMap::new(),
                num_sequence_length_hashmap: HashMap::new(),
            },
            |mut state, &num| {
                let existing_sequence_key_curr = state
                    .num_sequence_hashmap
                    .get(&num)
                    .map(|key| (key.clone(), num));
                let existing_sequence_key_prev = state
                    .num_sequence_hashmap
                    .get(&(num - 1))
                    .map(|key| (key.clone(), num - 1));
                let existing_sequence_key_next = state
                    .num_sequence_hashmap
                    .get(&(num + 1))
                    .map(|key| (key.clone(), num + 1));
                let existing_sequence_keys = existing_sequence_key_curr
                    .iter()
                    .chain(existing_sequence_key_prev.iter())
                    .chain(existing_sequence_key_next.iter())
                    .map(|(key, num)| (key.clone(), num.clone()))
                    .collect::<Vec<_>>();
                let existing_sequences = existing_sequence_keys
                    .iter()
                    .map(|(key, num)| {
                        (
                            state.num_sequence_length_hashmap.get(&key).unwrap().clone(),
                            num.clone(),
                        )
                    })
                    .collect::<Vec<_>>();
                let existing_sequences_or_empty = match existing_sequences.is_empty() {
                    true => vec![((num, num), num)],
                    false => existing_sequences,
                };
                let new_sequence =
                    intersection_of_ranges(existing_sequences_or_empty.iter().map(|&((x,y), _)| (x-1,y+1)))
                        .map(|(min, max)| (min, max))
                        .unwrap_or((num, num));
                state.count += 1;
                for &(_, num) in existing_sequences_or_empty.iter() {
                    state.num_sequence_hashmap.insert(num, state.count);
                }
                state
                    .num_sequence_length_hashmap
                    .insert(state.count, new_sequence);
                println!("state after {:?} => {:?}", num, state);
                state
            },
        )
        .num_sequence_length_hashmap
        .values()
        .max_by_key(|(min, max)| max - min + 1)
        .map(|(min, max)| max - min + 1)
        .unwrap_or(0) as i32
}
