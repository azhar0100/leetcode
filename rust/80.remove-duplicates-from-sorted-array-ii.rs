/*
 * @lc app=leetcode id=80 lang=rust
 *
 * [80] Remove Duplicates from Sorted Array II
 */

// @lc code=start
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // let the_window_filter = nums.windows(3).enumerate().map(|(i,window)| {
    //     println!("{:?} {:?}",i,window);
    //     let a = &window[0];
    //     let b = &window[1];
    //     let c = &window[2];
    //     (i,a == b && b == c)
    // });
    let mut k = 0;
    for i in 0..nums.len() {
        if i - k + 2 >= nums.len() {
            break;
        }
        let a = &nums[i - k + 0];
        let b = &nums[i - k + 1];
        let c = &nums[i - k + 2];
        let should_remove = a == b && b == c;
        if should_remove {
            nums.remove(i - k);
            k += 1;
        }
    }
    // println!("{:?}",nums);
    nums.len() as i32
}


impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        remove_duplicates(nums)        
    }
}
// @lc code=end

