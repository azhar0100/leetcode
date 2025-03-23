/*
 * @lc app=leetcode id=54 lang=rust
 *
 * [54] Spiral Matrix
 */

// @lc code=start
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirstSideOrSecondSide {
    FirstSide,
    SecondSide,
}

pub fn outer_spiral_of_matrix(matrix_size: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (m, n) = matrix_size;
    let perimeter = 2 * m + 2 * n;
    let spiral_order = (0..perimeter).filter_map(move |i| {
        let side_of_rectangle = match i / (perimeter / 2) {
            0 => FirstSideOrSecondSide::FirstSide,
            1 => FirstSideOrSecondSide::SecondSide,
            _ => return None,
        };
        let side_of_l_shape = match i % (perimeter / 2) < n {
            true => FirstSideOrSecondSide::FirstSide,
            false => FirstSideOrSecondSide::SecondSide,
        };
        match (side_of_rectangle, side_of_l_shape) {
            (FirstSideOrSecondSide::FirstSide, FirstSideOrSecondSide::FirstSide) => {
                let j = i;
                Some((0, j))
            }
            (FirstSideOrSecondSide::FirstSide, FirstSideOrSecondSide::SecondSide) => {
                let j = i - n;
                Some((j, n - 1))
            }
            (FirstSideOrSecondSide::SecondSide, FirstSideOrSecondSide::FirstSide) => {
                let j = i % (m + n);
                Some((m - 1, n - 1 - j))
            }
            (FirstSideOrSecondSide::SecondSide, FirstSideOrSecondSide::SecondSide) => {
                let j = i % (m + n) - n;
                let res_tuple = (m - 1 - j, 0);
                match res_tuple {
                    (0, 0) => None,
                    _ => Some(res_tuple),
                }
            }
            _ => None,
        }
    });
    spiral_order
}

pub fn spiral_order_idxes(matrix_size: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (m, n) = matrix_size;
    let smaller_dimension = m.min(n);
    let number_of_squares = match smaller_dimension % 2 {
        0 => smaller_dimension / 2,
        1 => smaller_dimension / 2 + 1,
        _ => panic!("Invalid smaller_dimension: {}", smaller_dimension),
    };
    (0..number_of_squares).flat_map(move |i| {
        let new_matrix_size = (m - 2 * i, n - 2 * i);
        match new_matrix_size {
            (0, _) | (_, 0) => {
                Box::new(std::iter::empty()) as Box<dyn Iterator<Item = (usize, usize)>>
            }
            (1, _) => Box::new((0..new_matrix_size.1).map(move |j| (i, i + j))),
            (_, 1) => Box::new((0..new_matrix_size.0).map(move |j| (i + j, i))),
            _ => {
                Box::new(outer_spiral_of_matrix(new_matrix_size).map(move |(x, y)| (x + i, y + i)))
            }
        }
    })
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    // Spiral Matrix
    spiral_order_idxes((matrix.len(), matrix[0].len()))
        .scan(None, |acc, idx| match acc {
            Some(prev_idx) => match prev_idx == &idx {
                true => {
                    *acc = None;
                    Some(None)
                }
                false => {
                    *acc = Some(idx);
                    Some(Some(idx))
                }
            },
            None => {
                *acc = Some(idx);
                Some(Some(idx))
            }
        })
        .filter_map(|x| x)
        .map(|(i, j)| matrix[i][j])
        .collect()
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        spiral_order(matrix)        
    }
}
// @lc code=end

