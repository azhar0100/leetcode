pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len().clone();
    let n = matrix[0].len().clone();
    let zero_indices = (0..m).flat_map(|i| {
        let i_clone = i.clone();
        (0..n).map(move |j| (i_clone,j))
    }).filter_map(|(i,j)| match &matrix[i][j] == &0{
        true => Some((i,j)),
        false => None
    }).collect::<Vec<_>>();
    for (i,j) in zero_indices{
        for k in 0..n{
            println!("Setting {:?} to 0", (i,k));
            matrix[i][k] = 0;
        }
        for k in 0..m{
            println!("Setting {:?} to 0", (k,j));
            matrix[k][j] = 0;
        }
    }
}
