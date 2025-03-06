use std::collections::HashSet;

pub fn happy_process(n: i32) -> i32 {
    (0..((n as f64).log10().floor() as u32 + 1))
        .map(|i| n / 10_i32.pow(i) % 10)
        .map(|x| x * x)
        .sum()
}

pub fn is_happy(n: i32) -> bool {
    match n == 1 {
        true => true,
        false => {
            let mut already_seen: HashSet<_> = HashSet::new();
            let mut n_loop = n;
            loop {
                let seen = happy_process(n_loop);
                match seen == 1 {
                    true => return true,
                    false => match already_seen.contains(&seen) {
                        true => return false,
                        false => {
                            already_seen.insert(seen);
                            n_loop = seen
                        }
                    },
                }
            }
        }
    }
}
