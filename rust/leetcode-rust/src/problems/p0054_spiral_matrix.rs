pub fn outer_spiral_of_matrix(matrix_size: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (m, n) = matrix_size;
    let perimeter = 2 * m + 2 * n;
    let spiral_order = (0..perimeter).filter_map(move |i| {
        let second_turn = i >= perimeter;
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
        let is_last_item_in_side = match (diagonal_side, left_side_or_right_side) {
            (0, 0) => number_in_side == m - 1,
            (0, 1) => number_in_side == n - 1,
            (1, 0) => number_in_side == m - 1,
            (1, 1) => number_in_side == n - 1,
            _ => panic!(
                "Invalid diagonal_side: {}, left_side_or_right_side: {}",
                diagonal_side, left_side_or_right_side
            ),
        };
        match is_last_item_in_side {
            true => None,
            false => Some(match side_number {
                0 => (0, number_in_side),
                1 => (number_in_side, n - 1),
                2 => (m - 1, n - 1 - number_in_side),
                3 => (m - 1 - number_in_side, 0),
                _ => panic!("Invalid side number: {}", side_number),
            }),
        }
    });
    spiral_order
}

pub fn spiral_order_idxes(matrix_size: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (m, n) = matrix_size;
    let smaller_dimension = m.min(n);
    let number_of_squares = smaller_dimension / 2;
    (0..number_of_squares)
        .flat_map(move |i| outer_spiral_of_matrix((m - 2*i, n - 2*i)).map(move |(x, y)| (x + i, y + i)))
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    // Spiral Matrix
    let m = matrix.len();
    let n = matrix[0].len();
    let spiral_order = spiral_order_idxes((m, n))
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
        .filter_map(|x| x);
    let spiral_order_vec: Vec<_> = spiral_order.collect();
    println!("{:?}", spiral_order_vec);

    spiral_order_vec
        .into_iter()
        .map(|(i, j)| matrix[i][j])
        .collect()
}
