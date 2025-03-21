use std::collections::HashMap;
// pub fn rotate(nums: &mut Vec<i32>, k: i32) {
//     let len = nums.len();
//     let effective_k = (k % (len as i32)) as usize;
//     let mut k_sized_vec = Vec::with_capacity(effective_k);
//     for o in (0..len).rev() {
//         let n = o + effective_k;
//         match n < len {
//             true => nums[n] = nums[o],
//             false => k_sized_vec.push(nums[o]),
//         }
//     }
//     for (i, val) in k_sized_vec.into_iter().rev().enumerate() {
//         nums[i] = val
//     }
// }

// pub fn rotate_constant_extra_space(nums: &mut Vec<i32>, constant_factor: i32, k: i32) {
//     let number_of_complete_constant_factor_rotations = k / constant_factor;
//     let last_rotation = k % constant_factor;
//     let rotation_number = (0..number_of_complete_constant_factor_rotations)
//         .map(|_| constant_factor)
//         .chain(std::iter::once(last_rotation));
//     for k in rotation_number{
//         rotate(nums, k);
//     }
// }

pub fn rotate_given_idxs_of_square_matrix<T>(
    matrix: &mut Vec<Vec<T>>,
    idxes: &[(usize, usize)],
    k: usize,
) where
    T: Copy,
{
    println!("rotating k {:?}, idxes: {:?}", k, idxes);
    let len = idxes.len();
    let effective_k = k % len;
    let mut k_sized_vec = Vec::with_capacity(effective_k);
    for o in (0..len).rev() {
        let n = o + effective_k;
        match n < len {
            true => matrix[idxes[n].0][idxes[n].1] = matrix[idxes[o].0][idxes[o].1],
            false => k_sized_vec.push(matrix[idxes[o].0][idxes[o].1]),
        }
    }
    for (i, val) in k_sized_vec.into_iter().rev().enumerate() {
        matrix[idxes[i].0][idxes[i].1] = val
    }
}

pub fn rotate_square_boundary(matrix: &mut Vec<Vec<i32>>, boundary_idx: usize) {
    let matrix_n = matrix.len();
    let matrix_boundary_length = matrix_n - (boundary_idx * 2);
    let n = matrix_boundary_length;
    let boundary_idxes = (0..4 * n)
        .filter_map(|i| {
            let div = i / n;
            let rem = i % n;
            match (div, rem) {
                (0, _) => Some((0, rem)),
                (1, 0) => None,
                (1, _) => Some((rem, n - 1)),
                (2, 0) => None,
                (2, _) => Some((n - 1, n - 1 - rem)),
                (3, 0) => None,
                (3, val) if val == n - 1 => None,
                (3, _) => Some((n - 1 - rem, 0)),
                _ => panic!("Invalid div: {}", div),
            }
        })
        .map(|(i, j)| (i + boundary_idx, j + boundary_idx));
    rotate_given_idxs_of_square_matrix(matrix, &boundary_idxes.collect::<Vec<_>>(), n - 1);
}

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let matrix_n = matrix.len();
    let boundary_idxes = (0..matrix_n / 2).map(|i| i);
    for boundary_idx in boundary_idxes {
        rotate_square_boundary(matrix, boundary_idx);
    }
}
