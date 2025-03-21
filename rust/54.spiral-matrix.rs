/*
 * @lc app=leetcode id=54 lang=rust
 *
 * [54] Spiral Matrix
 */

// @lc code=start
pub fn outer_spiral_of_matrix(matrix_size: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (m, n) = matrix_size;
    let perimeter = 2 * m + 2 * n;
    let spiral_order = (0..perimeter).map(move |i| {
        let diagonal_side = i / (m + n);
        let left_side_or_right_side = i % (m + n) / m;
        let side_number = diagonal_side * 2 + left_side_or_right_side;
        let number_in_side = match side_number {
            0 => i,
            1 => i - m,
            2 => i - m - n,
            3 => i - 2 * m - n,
            _ => panic!("Invalid side number: {}", side_number),
        };
        match side_number {
            0 => (0, number_in_side),
            1 => (number_in_side, n - 1),
            2 => (m - 1, n - 1 - number_in_side),
            3 => (m - 1 - number_in_side, 0),
            _ => panic!("Invalid side number: {}", side_number),
        }
    });
    spiral_order
}

pub fn spiral_order_idxes(matrix_size: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (m, n) = matrix_size;
    let smaller_dimension = m.min(n);
    let number_of_squares = smaller_dimension / 2;
    (0..number_of_squares)
        .flat_map(move |i| outer_spiral_of_matrix((m, n)).map(move |(x, y)| (x + i, y + i)))
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    // Spiral Matrix
    let m = matrix.len();
    let n = matrix[0].len();
    let spiral_order = spiral_order_idxes((m, n));
    spiral_order.map(|(i, j)| matrix[i][j]).collect()
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        spiral_order(matrix)        
    }
}
// @lc code=end

