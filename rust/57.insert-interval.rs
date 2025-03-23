/*
 * @lc app=leetcode id=57 lang=rust
 *
 * [57] Insert Interval
 */

// @lc code=start
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineSweepElement<T>
where
    T: Debug + Clone + PartialEq + Eq + PartialOrd + Ord,
{
    Start(T, usize),
    End(T, usize),
}

impl<T> PartialOrd for LineSweepElement<T>
where
    T: Debug + Clone + PartialEq + Eq + PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let value_a = match self {
            LineSweepElement::Start(value, _) => value,
            LineSweepElement::End(value, _) => value,
        };
        let value_b = match other {
            LineSweepElement::Start(value, _) => value,
            LineSweepElement::End(value, _) => value,
        };
        value_a.partial_cmp(value_b).map(|ordering| match ordering {
            std::cmp::Ordering::Equal => match (self, other) {
                (LineSweepElement::Start(_, _), LineSweepElement::Start(_, _)) => ordering,
                (LineSweepElement::Start(_, _), LineSweepElement::End(_, _)) => {
                    std::cmp::Ordering::Less
                }
                (LineSweepElement::End(_, _), LineSweepElement::Start(_, _)) => {
                    std::cmp::Ordering::Greater
                }
                (LineSweepElement::End(_, _), LineSweepElement::End(_, _)) => ordering,
            },
            _ => ordering,
        })
    }
}

impl<T> Ord for LineSweepElement<T>
where
    T: Debug + Clone + PartialEq + Eq + PartialOrd + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanState {
    pub opens: usize,
    pub start: Option<i32>,
    pub end: Option<i32>,
}

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Merge Intervals
    println!("{:?}", intervals);
    let mut interval_elements = intervals
        .iter()
        .enumerate()
        .flat_map(|(index, interval)| {
            vec![
                LineSweepElement::Start(interval[0], index),
                LineSweepElement::End(interval[1], index),
            ]
        })
        .collect::<Vec<LineSweepElement<i32>>>();
    interval_elements.sort();
    // println!(
    //     "{:?}",
    //     interval_elements
    //         .clone()
    //         .into_iter()
    //         .map(|x| match x {
    //             LineSweepElement::Start(s, _) => format!("Start({})", s),
    //             LineSweepElement::End(e, _) => format!("End({})", e),
    //         })
    //         .collect::<Vec<_>>()
    // );
    interval_elements
        .into_iter()
        .scan(
            ScanState {
                opens: 0,
                start: None,
                end: None,
            },
            |state, x| {
                match x {
                    LineSweepElement::Start(start, _) => {
                        state.opens += 1;
                        state.start = state.start.or(Some(start))
                    }
                    LineSweepElement::End(end, _) => {
                        state.opens -= 1;
                        state.end = Some(end)
                    }
                }
                match state.opens {
                    0 => {
                        let start = state.start.take();
                        let end = state.end.take();
                        Some(match (start, end) {
                            (Some(start), Some(end)) => Some((start, end)),
                            _ => None,
                        })
                    }
                    _ => Some(None),
                }
            },
        )
        .filter_map(|x| x)
        .map(|(x, y)| vec![x, y])
        .collect()
}

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let new_intervals = intervals.into_iter().chain(vec![new_interval]).collect();
    merge(new_intervals)
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        insert(intervals, new_interval)
    }
}
// @lc code=end
