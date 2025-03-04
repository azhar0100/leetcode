pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let prefixes = nums.iter().scan(1, |acc, x| {
        *acc = *acc * (*x);
        Some(acc.clone())
    });
    let suffixes = nums
        .iter()
        .rev()
        .scan(1, |acc, x| {
            *acc = *acc * (*x);
            Some(acc.clone())
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev();
    // let prefixes_iter = std::iter::once(None).chain(prefixes.map(|x| Some(x))).skip(1).take(nums.len());
    // let suffixes_iter = suffixes.map(|x| Some(x)).chain(std::iter::once(None)).take(nums.len()).skip(1);
    let prefixes_iter = prefixes.map(|x|Some(x));
    let suffixes_iter = suffixes.map(|x|Some(x));
    println!("{:?}", nums);
    println!("{:?}", prefixes_iter.clone().collect::<Vec<_>>());
    println!("{:?}", suffixes_iter.clone().collect::<Vec<_>>());
    prefixes_iter
        .zip(suffixes_iter)
        .map(|(p, s)| match (p,s){
            (None, None) => panic!("Impossible"),
            (None, Some(s)) => s,
            (Some(p), None) => p,
            (Some(p), Some(s)) => p*s,
        })
        .collect()
}
