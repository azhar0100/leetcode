use std::collections::HashSet;


pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut indices_can_jump = HashSet::new();
    indices_can_jump.insert(0);
    println!("{:?} indices can jump before loop",indices_can_jump);
    for (i, x) in nums.iter().enumerate() {
        if indices_can_jump.contains(&i) {
            for j in i..(i + *x as usize + 1).min(nums.len()) {
                indices_can_jump.insert(j);
            }
        }
        println!("{:?} indices can jump after loop iteration {:?}",indices_can_jump,i)
    }
    indices_can_jump.contains(&(nums.len() - 1))
}
