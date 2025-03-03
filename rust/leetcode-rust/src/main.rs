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
    // let threesum_input = vec![-1,0,1,2,-1,-4];
    // let result = three_sum_final(threesum_input);
    // println!("{:?}", result);

    let nums1 = vec![1,3];
    let nums2 = vec![2];
    // let result = problems::p0004_median_of_two_sorted_arrays::find_any_position_in_two_sorted_arrays(&nums1,&nums2,1);
    let result = problems::p0004_median_of_two_sorted_arrays::find_median_sorted_arrays(nums1,nums2);
    println!("{:?}",result)
}
