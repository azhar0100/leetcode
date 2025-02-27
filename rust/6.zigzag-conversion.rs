/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let n: usize = num_rows as usize;
        let row_col_values = (0..s.len())
            .map(|i| {
                let period = 2 * (n - 1);
                let period_index = match period {
                    0 => 0,
                    _ => i / period,
                };
                let index_in_period = match period {
                    0 => i,
                    _ => i % period,
                };
                let base_col = period_index * (n - 1);
                match index_in_period < n {
                    true => (index_in_period, base_col),
                    false => (
                        match n {
                            0 => 0,
                            _ => n - 2 - index_in_period % n,
                        },
                        match n - 1 {
                            0 => 0,
                            _ => base_col + index_in_period % (n - 1),
                        },
                    ),
                }
            })
            .collect::<Vec<(usize, usize)>>();
        // println!("{:?}", row_col_values);
        let mut indices = (0..s.len()).collect::<Vec<usize>>();
        indices.sort_by_key(|i| {
            row_col_values
                .get(*i)
                .expect("They are not supposed to have more indices")
        });
        // println!("{:?}", indices);
        indices
            .iter()
            .map(|i| {
                s.chars()
                    .nth(*i)
                    .expect("They are not supposed to have more indices")
            })
            .collect()
    }
        
}
// @lc code=end

