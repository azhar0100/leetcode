/*
 * @lc app=leetcode id=122 lang=rust
 *
 * [122] Best Time to Buy and Sell Stock II
 */

// @lc code=start
fn relu(x:i32) -> i32{
    match x > 0{
        true => x,
        false => 0,
    }
}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).map(|x| relu(x[1] - x[0])).sum()
    }
}
// @lc code=end

