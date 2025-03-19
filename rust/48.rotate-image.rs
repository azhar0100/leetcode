/*
 * @lc app=leetcode id=48 lang=rust
 *
 * [48] Rotate Image
 */

// @lc code=start
use std::collections::HashMap;

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    let indices_iter = (0..n).flat_map(|i| {
        let i_clone = i.clone();
        (0..n).map(move |j| (i_clone, j))
    }).map(|(i,j)| {
        ((i,j),(j,n-1-i))
    });
    let mut index_history = HashMap::new();
    for (old, new) in indices_iter{
        
        let (i_old,j_old) = index_history.get(&old).unwrap_or(&old).clone();
        let (i_new,j_new) = index_history.get(&new).unwrap_or(&new).clone();
        println!("{:?} -> {:?}",(i_old,j_old), (i_new,j_new));
        // let a = matrix[i_old][j_old]
        // let b = matrix[i_new][j_new]
        // let a = b ^ a
        // let b = a ^ b
        // let a = b ^ a
        (matrix[i_old][j_old],matrix[i_new][j_new]) = (matrix[i_new][j_new],matrix[i_old][j_old]);
        index_history.insert(old, new);
        // let 
    };
    for i in (0..n){
        let row = matrix[i].as_mut_slice();
        for j in 0..(n/2) {
            (row[j],row[n-1-j]) = (row[n-1-j],row[j])
        }
    }
}


impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        rotate(matrix)        
    }
}
// @lc code=end

