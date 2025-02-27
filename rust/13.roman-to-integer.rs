/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        const RLE_VEC: Vec<(char, i32)> = Vec::new();
        let rle_values: Vec<(i32, i32)> = s
            .chars()
            .fold(RLE_VEC, |mut acc, c| match acc.last_mut() {
                Some((last_c, count)) if *last_c == c => {
                    let count_clone: i32 = count.clone();
                    let count_plus_1 = count_clone + 1;
                    acc.pop();
                    acc.push((c, count_plus_1));
                    acc
                }
                _ => {
                    acc.push((c, 1));
                    acc
                }
            })
            .iter()
            .map(|(c, count)| {
                (
                    match c {
                        'I' => 1,
                        'V' => 5,
                        'X' => 10,
                        'L' => 50,
                        'C' => 100,
                        'D' => 500,
                        'M' => 1000,
                        _ => 0,
                    },
                    count.clone(),
                )
            })
            .collect();
        rle_values
            .iter()
            .zip(rle_values.iter().zip(rle_values.iter().skip(1)).map(
                |((v1, _), (v2, _))| match v1 < v2 {
                    true => -1,
                    false => 1,
                },
            ))
            .map(|((v, count), sign)| v * count * sign)
            .sum()
    }
}
// @lc code=end
