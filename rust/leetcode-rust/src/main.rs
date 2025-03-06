use problems::{p0205_isomorphic_strings::is_isomorphic, p0290_word_pattern::word_pattern};

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

    // let nums1 = vec![];
    // let nums2 = vec![1];
    // // let result = problems::p0004_median_of_two_sorted_arrays::find_any_position_in_two_sorted_arrays(&nums1,&nums2,1);
    // let result = problems::p0004_median_of_two_sorted_arrays::find_median_sorted_arrays(nums1,nums2);
    // println!("{:?}",result)
    // let mut nums = vec![1,2,3,4,5,6,7];
    // let k = 3;
    // println!("nums before: {:?}",nums);
    // problems::p0189_rotate_array::rotate(&mut nums, k);
    // println!("nums after: {:?}",nums);

    // let mut nums = vec![1,2,3,4];
    // let result = problems::p0238_product_of_array_except_self::product_except_self(nums);
    // println!("{:?}",result)
    // let citations = vec![1,3,1,5];
    // let result = problems::p0274_h_index::h_index(citations);
    // println!("{:?}",result);
    let test_string1 = "abba".to_string();
    let test_string2 = "dog cat cat dog".to_string();
    let result = word_pattern(test_string1, test_string2);
    println!("{:?}",result);
}
