pub mod problems;
pub mod util;
fn main() {
    //     "PAYPALISHIRING"
    // 3
    // let s = "PAYPALISHIRING".to_string();
    let s = "0123456789ABCD".to_string();
    let num_rows = 4;
    let res = problems::p0006_zigzag_conversion::convert(s, num_rows);
    println!("{}", res);

}
