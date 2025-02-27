pub mod problems;
fn main() {
        // You can test your solution here
    let nums =  vec![3,2,4]    ;
    let target = 6;
    let result = problems::p0001_two_sum::Solution::two_sum(nums, target);
    println!("{:?}", result);
}
