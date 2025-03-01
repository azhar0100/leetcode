use core::panic;

#[derive(Clone, Debug)]
pub enum FirstOrSecondPositionResult {
    First(usize),
    Second(usize),
}

pub fn value_at_enum_idx_for_two_arrays<'a, T>(nums1: &'a [T],nums2: &'a [T], enum_idx:&'a FirstOrSecondPositionResult) -> Option<&'a T>{
    match enum_idx{
        FirstOrSecondPositionResult::First(first) => {
            nums1.get(*first)
        },
        FirstOrSecondPositionResult::Second(second) => {
            nums2.get(*second)
        },
    }

}

pub fn get_isize<'a, T>(nums: &'a [T], isize_idx:isize) -> Option<&'a T>{
    match isize_idx >= 0{
        false => None,
        true => nums.get(isize_idx as usize)
    }

}

pub fn find_any_position_in_two_sorted_arrays<T>(
    nums1: &[T],
    nums2: &[T],
    position_to_find: usize,
) -> Option<FirstOrSecondPositionResult>
where
    T: Ord + Eq,
{
    let c: usize = position_to_find;
    let m: usize = nums1.len();
    let n: usize = nums2.len();
    let is_within_bounds = c < m + n;
    if !is_within_bounds {
        return None;
    }

    let c_isize = c as isize;
    let n_isize = n as isize;
    let min_position = (c_isize - n_isize).max(-1) as isize;
    let max_position = c.min(m - 1) as isize;
    let i_s = (min_position..(max_position + 1)).collect::<Vec<_>>();
    println!("i_s are {:?}", i_s);
    let partition_index_raw = i_s.partition_point(|i| {
        let j = c_isize - i - 1;
        let a = get_isize(nums1, *i);
        let b = get_isize(nums2, j);
        match (a,b){
            (None, None) => false,
            (None, Some(_)) => true,
            (Some(_), None) => false,
            (Some(a), Some(b)) => b > a,
        }
    });
    let partition_index = match partition_index_raw >= i_s.len(){
        true => partition_index_raw - 1,
        false => partition_index_raw,
    };

    let i_at_partition_point = i_s[partition_index];
    let j_at_partition_point = c_isize - 1 - i_at_partition_point;
    println!(
        "i,j is ({:?},{:?})",
        i_at_partition_point, j_at_partition_point
    );
    let value_a = get_isize(&nums1,i_at_partition_point);
    let value_b = get_isize(&nums2,j_at_partition_point);
    match (value_a,value_b) {
        (None, None) => None,
        (None, Some(_)) => Some(FirstOrSecondPositionResult::Second(j_at_partition_point as usize)),
        (Some(_), None) => Some(FirstOrSecondPositionResult::First(i_at_partition_point as usize)),
        (Some(a), Some(b)) => {
            let ordinal_c = position_to_find + 1;
            let ordinal_i = (i_at_partition_point + 1) as usize;
            let ordinal_j = (j_at_partition_point + 1) as usize;
            let achieved_position_before_the_pair = ordinal_i + ordinal_j - 2;
            match ordinal_c - achieved_position_before_the_pair{
                1 => match a.cmp(b) {
                    std::cmp::Ordering::Less => Some(FirstOrSecondPositionResult::First(i_at_partition_point as usize)),
                    std::cmp::Ordering::Equal =>Some(FirstOrSecondPositionResult::First(i_at_partition_point as usize)),
                    std::cmp::Ordering::Greater => Some(FirstOrSecondPositionResult::Second(j_at_partition_point as usize)),
                },
                2 => match a.cmp(b) {
                    std::cmp::Ordering::Less => Some(FirstOrSecondPositionResult::Second(j_at_partition_point as usize)),
                    std::cmp::Ordering::Equal => Some(FirstOrSecondPositionResult::Second(j_at_partition_point as usize)),
                    std::cmp::Ordering::Greater => Some(FirstOrSecondPositionResult::First(i_at_partition_point as usize)),
                },
                _ => panic!("This was never supposed to happen")
            }
        },
    }
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total_len = nums1.len() + nums2.len();
    match total_len % 2 == 1{
        false => {
            let idx1 = total_len/2 -1;
            let idx2 = total_len/2;
            println!("idx1 is {:?},idx2 is {:?}",idx1,idx2);
            let pos1 = find_any_position_in_two_sorted_arrays(&nums1, &nums2, idx1).unwrap();
            let pos2 = find_any_position_in_two_sorted_arrays(&nums1, &nums2, idx2).unwrap();
            println!("pos1 is {:?},pos2 is {:?}",pos1,pos2);
            let val1 = value_at_enum_idx_for_two_arrays(&nums1,&nums2, &pos1).unwrap().clone() as f64;
            let val2 = value_at_enum_idx_for_two_arrays(&nums1,&nums2, &pos2).unwrap().clone() as f64;
            println!("val1 is {:?},val2 is {:?}",val1,val2);
            (val1 + val2)/2.0
        },
        true => {
            let idx = (total_len-1)/2;
            let val = value_at_enum_idx_for_two_arrays(&nums1,&nums2, &find_any_position_in_two_sorted_arrays(&nums1, &nums2, idx).unwrap()).unwrap().clone() as f64;
            println!("val is {:?}",val);
            val
        },
    }
}
