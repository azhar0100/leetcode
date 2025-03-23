use super::p0056_merge_intervals::merge;



pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let new_intervals = intervals.into_iter().chain(vec![new_interval]).collect();
    merge(new_intervals)
}
