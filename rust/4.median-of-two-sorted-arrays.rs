/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start

#[derive(Clone, Debug)]
pub enum FirstOrSecondPositionResult {
    First(usize),
    Second(usize),
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Direction {
    First,
    Second,
}

pub fn value_at_enum_idx_for_two_arrays<'a, T>(
    nums1: &'a [T],
    nums2: &'a [T],
    enum_idx: &'a FirstOrSecondPositionResult,
) -> Option<&'a T> {
    match enum_idx {
        FirstOrSecondPositionResult::First(first) => nums1.get(*first),
        FirstOrSecondPositionResult::Second(second) => nums2.get(*second),
    }
}

pub fn get_isize<'a, T>(nums: &'a [T], isize_idx: isize) -> Option<&'a T> {
    match isize_idx >= 0 {
        false => None,
        true => nums.get(isize_idx as usize),
    }
}

pub fn find_any_position_in_two_sorted_arrays<T>(
    nums1: &[T],
    nums2: &[T],
    position_to_find: usize,
) -> Option<FirstOrSecondPositionResult>
where
    T: Ord + Eq,
{
    let c: usize = position_to_find;
    let m: usize = nums1.len();
    let n: usize = nums2.len();
    let is_within_bounds = c < m + n;
    if !is_within_bounds {
        return None;
    }

    let c_isize = c as isize;
    let n_isize = n as isize;
    let min_position = (c_isize - n_isize).max(-1) as isize;
    let max_position = c.min(m - 1) as isize;
    let i_s = (min_position..(max_position + 1)).collect::<Vec<_>>();
    println!("i_s are {:?}", i_s);
    let partition_index = i_s
        .binary_search_by(|i| {
            let i = i.clone();
            let j = c_isize - i - 1;
            let a = get_isize(nums1, i);
            let b = get_isize(nums2, j);
            let direction = match (a, b) {
                (None, None) => None,
                (None, Some(_)) => Some(Direction::Second),
                (Some(_), None) => Some(Direction::First),
                (Some(a), Some(b)) => Some(match a.cmp(b) {
                    std::cmp::Ordering::Less => Direction::Second,
                    std::cmp::Ordering::Equal => Direction::First,
                    std::cmp::Ordering::Greater => Direction::First,
                }),
            };
            let prev_element = direction.map(|x| match x {
                Direction::First => (i - 1, j),
                Direction::Second => (i, j - 1),
            });
            match prev_element {
                Some((prev_i, prev_j)) => {
                    let prev_a = get_isize(nums1, prev_i);
                    let prev_b = get_isize(nums2, prev_j);
                    let actual_direction = match (prev_a, prev_b) {
                        (None, None) => None,
                        (None, Some(_)) => Some(Direction::Second),
                        (Some(_), None) => Some(Direction::First),
                        (Some(prev_a), Some(prev_b)) => Some(match prev_a.cmp(prev_b) {
                            std::cmp::Ordering::Less => Direction::Second,
                            std::cmp::Ordering::Equal => Direction::First,
                            std::cmp::Ordering::Greater => Direction::First,
                        }),
                    };
                    match actual_direction {
                        Some(actual_direction) => {
                            let direction =
                                direction.expect("This should be here, since prev_element is here");
                            match direction == actual_direction {
                                true => std::cmp::Ordering::Equal,
                                false => match direction {
                                    Direction::First => std::cmp::Ordering::Greater,
                                    Direction::Second => std::cmp::Ordering::Less,
                                },
                            }
                        }
                        None => std::cmp::Ordering::Equal,
                    }
                }
                None => std::cmp::Ordering::Equal,
            }
        })
        .ok();
    let i_at_partition_point = partition_index.map(|partition_index| i_s[partition_index]);
    let j_at_partition_point =
        i_at_partition_point.map(|i_at_partition_point| c_isize - 1 - i_at_partition_point);
    println!(
        "i,j is ({:?},{:?})",
        i_at_partition_point, j_at_partition_point
    );
    let value_a = i_at_partition_point
        .map(|i_at_partition_point| get_isize(&nums1, i_at_partition_point))
        .flatten();
    let value_b = j_at_partition_point
        .map(|j_at_partition_point| get_isize(&nums2, j_at_partition_point))
        .flatten();
    match (value_a, value_b) {
        (None, None) => None,
        (None, Some(_)) => Some(FirstOrSecondPositionResult::Second(
            j_at_partition_point.expect("If value_b exists, j_at_partition_point exists") as usize,
        )),
        (Some(_), None) => Some(FirstOrSecondPositionResult::First(
            i_at_partition_point.expect("If value_a exists, i_at_partition_point exists") as usize,
        )),
        (Some(a), Some(b)) => {
            let i_at_partition_point =
                i_at_partition_point.expect("if value_a exists, i_at_partition_point exists");
            let j_at_partition_point =
                j_at_partition_point.expect("if value_b exists, j_at_partition_point exists");
            let ordinal_c = position_to_find + 1;
            let ordinal_i = (i_at_partition_point + 1) as usize;
            let ordinal_j = (j_at_partition_point + 1) as usize;
            Some(match a.cmp(b) {
                std::cmp::Ordering::Less => {
                    FirstOrSecondPositionResult::Second(j_at_partition_point as usize)
                }
                std::cmp::Ordering::Equal => {
                    FirstOrSecondPositionResult::First(i_at_partition_point as usize)
                }
                std::cmp::Ordering::Greater => {
                    FirstOrSecondPositionResult::First(i_at_partition_point as usize)
                }
            })
            // let achieved_position_before_the_pair = ordinal_i + ordinal_j - 2;
            // match ordinal_c - achieved_position_before_the_pair {
            //     1 | 2 => match a.cmp(b) {
            //         std::cmp::Ordering::Less => Some(FirstOrSecondPositionResult::First(
            //             i_at_partition_point as usize,
            //         )),
            //         std::cmp::Ordering::Equal => Some(FirstOrSecondPositionResult::First(
            //             i_at_partition_point as usize,
            //         )),
            //         std::cmp::Ordering::Greater => Some(FirstOrSecondPositionResult::Second(
            //             j_at_partition_point as usize,
            //         )),
            //     },

            //     _ => panic!("This was never supposed to happen"),
            // }
        }
    }
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total_len = nums1.len() + nums2.len();
    match total_len % 2 == 1 {
        false => {
            let idx1 = total_len / 2 - 1;
            let idx2 = total_len / 2;
            let pos1 = find_any_position_in_two_sorted_arrays(&nums1, &nums2, idx1);
            let pos2 = find_any_position_in_two_sorted_arrays(&nums1, &nums2, idx2);
            let val1 = pos1
                .map(|pos1| {
                    value_at_enum_idx_for_two_arrays(&nums1, &nums2, &pos1)
                        .map(|x| x.clone() as f64)
                })
                .flatten();
            let val2 = pos2
                .map(|pos2| {
                    value_at_enum_idx_for_two_arrays(&nums1, &nums2, &pos2)
                        .map(|x| x.clone() as f64)
                })
                .flatten();
            println!("val1 is {:?}", val1);
            println!("val2 is {:?}", val2);
            let median = val1
                .map(|val1| val2.map(|val2| (val1 + val2) / 2.0))
                .flatten();
            median.expect("this should be there")
        }
        true => {
            let idx = (total_len - 1) / 2;
            let val = value_at_enum_idx_for_two_arrays(
                &nums1,
                &nums2,
                &find_any_position_in_two_sorted_arrays(&nums1, &nums2, idx).unwrap(),
            )
            .unwrap()
            .clone() as f64;
            println!("val is {:?}", val);
            val
        }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        find_median_sorted_arrays(nums1, nums2)
    }
}
// @lc code=end
