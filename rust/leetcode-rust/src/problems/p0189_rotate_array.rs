fn rotated_index(k: i32, n: usize, x: usize) -> usize {
    (((x as i32) + k) % (n as i32)) as usize
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    let old_positions = 0..n;
    let new_positions = (0..n).map(|x| rotated_index(k, n, x));
    match k.cmp(&0) {
        std::cmp::Ordering::Less => {
            old_positions
                .zip(new_positions)
                .for_each(|(o, n)| (nums[o], nums[n]) = (nums[n], nums[o]));
        }
        std::cmp::Ordering::Equal => (),
        std::cmp::Ordering::Greater => {
            old_positions
                .zip(new_positions)
                .rev()
                .for_each(|(o, n)| (nums[o], nums[n]) = (nums[n], nums[o]));
        }
    }
}
