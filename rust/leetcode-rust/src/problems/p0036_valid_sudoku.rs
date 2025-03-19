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
                    let grp_idx = 3 * (i / 3) + j/3;
                    println!("{:?} {:?} {:?}",i,j,grp_idx);
                    let row_condition = !row_hashmaps[i].insert(board_item);
                    let col_condition = !col_hashmaps[j].insert(board_item);
                    let grp_condition = !grp_hashmaps[grp_idx].insert(board_item);
                    let all_condition = row_condition || col_condition || grp_condition;
                    if all_condition{
                        return false;
                    }
                }
                None => (),
            }
        }
    }
    true
}
