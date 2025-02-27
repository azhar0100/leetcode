pub mod problems;
fn main() {
        // You can test your solution here
    let nums =  vec![2, 7, 11, 15];
    let target = 9;
    let result = problems::p0001_two_sum::Solution::two_sum(nums, target);
    println!("{:?}", result);
}
