fn relu(x:i32) -> i32{
    match x > 0{
        true => x,
        false => 0,
    }
}
pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices.windows(2).map(|window| relu(window[1] - window[0])).sum()
}
