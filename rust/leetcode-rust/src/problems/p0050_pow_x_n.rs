pub fn my_pow_64bit(x: f64, n: i64) -> f64 {
    let reduction_param = 100;
    match n.cmp(&0) {
        std::cmp::Ordering::Less => 1.0 / my_pow_64bit(x, -n),
        std::cmp::Ordering::Equal => 1.0,
        std::cmp::Ordering::Greater => match n > reduction_param {
            true => {
                let tenth_pow = my_pow_64bit(x, reduction_param);
                my_pow_64bit(tenth_pow, n / reduction_param) * my_pow_64bit(x, n % reduction_param)
            }
            false => (0..n).fold(1.0, |acc, i| acc * x),
        },
    }
}
pub fn my_pow(x: f64, n: i32) -> f64 {
    my_pow_64bit(x, n as i64)    
}
