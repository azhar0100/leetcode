use problems::p0015_3sum::three_sum_final;

pub mod problems;
pub mod util;
fn main() {
    //     "PAYPALISHIRING"
    // 3
    // let s = "PAYPALISHIRING".to_string();
    // let s = "0123456789ABCD".to_string();
    // let num_rows = 4;
    // let res = problems::p0006_zigzag_conversion::convert(s, num_rows);
    // println!("{}", res);
    let threesum_input = vec![-1,0,1,2,-1,-4];
    let result = three_sum_final(threesum_input);
    println!("{:?}", result);
}
