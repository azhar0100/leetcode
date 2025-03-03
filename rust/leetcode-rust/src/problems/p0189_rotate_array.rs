use std::collections::HashMap;


pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let effective_k = (k % (len as i32)) as usize;
    let mut k_sized_vec = Vec::with_capacity(effective_k);
    for o in (0..len).rev(){
        let n = o+effective_k;
        match n < len{
            true => nums[n] = nums[o],
            false => k_sized_vec.push(nums[o]),
        }
    }
    for (i,val) in k_sized_vec.into_iter().rev().enumerate(){
        nums[i] = val
    }
}
