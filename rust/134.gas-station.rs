/*
 * @lc app=leetcode id=134 lang=rust
 *
 * [134] Gas Station
 */

// @lc code=start
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    // Gas Station
    let n = match gas.len() == cost.len() {
        true => gas.len(),
        false => panic!("lengths don't match"),
    };
    (0..n)
        .filter_map(|i| {
            let indices_iterator = (0..n).map(|j| j % n);
            let mut gas_current = 0;
            for j in indices_iterator{
                gas_current += gas[j];
                if gas_current < cost[j]{
                    return None;
                }else{
                    gas_current -= cost[j]
                }
            };
            Some(i)
        })
        .next()
        .map(|x| x as i32)
        .unwrap()
}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        can_complete_circuit(gas, cost)        
    }
}
// @lc code=end

