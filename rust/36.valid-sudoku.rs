/*
 * @lc app=leetcode id=36 lang=rust
 *
 * [36] Valid Sudoku
 */

// @lc code=start
use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let board = board
        .into_iter()
        .map(|row| row.into_iter().map(|x| x.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut row_hashmaps = (0..9).map(|_| HashSet::new()).collect::<Vec<_>>();
    let mut col_hashmaps = (0..9).map(|_| HashSet::new()).collect::<Vec<_>>();
    let mut grp_hashmaps = (0..9).map(|_| HashSet::new()).collect::<Vec<_>>();
    for (i, board_row) in board.into_iter().enumerate() {
        for (j, board_item) in board_row.into_iter().enumerate() {
            match board_item {
                Some(board_item) => {
                    let grp_idx = 3 * (i / 3) + j / 3;

                    if !row_hashmaps[i].insert(board_item) {
                        return false;
                    }
                    if !col_hashmaps[j].insert(board_item) {
                        return false;
                    }
                    if !grp_hashmaps[grp_idx].insert(board_item) {
                        return false;
                    }
                }
                None => (),
            }
        }
    }
    true
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        is_valid_sudoku(board)
    }
}
// @lc code=end
