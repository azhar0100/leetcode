/*
 * @lc app=leetcode id=134 lang=rust
 *
 * [134] Gas Station
 */

// @lc code=start
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> Option<i32> {
    // Gas Station
    let n = match gas.len() == cost.len() {
        true => gas.len(),
        false => panic!("lengths don't match"),
    };
    let gas_minus_cost_vec = gas.iter().zip(cost.iter()).map(|(g, c)| g - c);
    match gas_minus_cost_vec.clone().sum::<i32>() < 0 {
        true => None,
        false => {
            // let change_indices = gas_minus_cost_vec.iter().enumerate().scan(
            //     std::cmp::Ordering::Less,
            //     |ordering, (i, x)| {
            //         let new_ordering = x.cmp(&0);
            //         match &new_ordering == ordering {
            //             true => Some(None),
            //             false => {
            //                 *ordering = new_ordering.clone();
            //                 Some(Some((i, new_ordering)))
            //             }
            //         }
            //     },
            // );
            let index = gas_minus_cost_vec
                .scan(0, |acc, x| {
                    *acc += x;
                    Some(*acc)
                })
                .enumerate()
                .min_by_key(|(_, k)| *k)
                .map(|(i, _)| i)?;
            Some(((index + 1) % n) as i32)
        }
    }
}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        can_complete_circuit(gas, cost).unwrap_or(-1)
    }
}
// @lc code=end
