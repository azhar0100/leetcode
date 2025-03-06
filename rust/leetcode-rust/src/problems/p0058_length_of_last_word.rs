pub fn length_of_last_word(s: String) -> i32 {
    s.trim()
        .split(' ')
        .last()
        .map(|x| x.len() as i32)
        .unwrap()
}
