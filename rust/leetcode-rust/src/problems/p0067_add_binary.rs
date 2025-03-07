fn char_to_bool(x: char) -> bool {
    match x {
        '0' => false,
        '1' => true,
        _ => panic!("This should not happen"),
    }
}

fn bool_to_char(x: bool) -> char {
    match x {
        true => '1',
        false => '0',
    }
}

pub fn add_binary(a: String, b: String) -> String {
    let n = a.len().max(b.len());
    let a_bin = a.chars().map(char_to_bool).rev();
    let b_bin = b.chars().map(char_to_bool).rev();
    let a_bin_padded = a_bin
        .map(|x| Some(x))
        .chain((a.len()..(n + 1)).map(|x| None));
    let b_bin_padded = b_bin
        .map(|x| Some(x))
        .chain((b.len()..(n + 1)).map(|x| None));
    let full_added = a_bin_padded
        .zip(b_bin_padded)
        .scan(false, |mut acc, (x, y)| {
            let x_bin = x.unwrap_or(false);
            let y_bin = y.unwrap_or(false);
            let (full_add, full_carry) = match (x_bin, y_bin, *acc) {
                (true, true, true) => (true, true),
                (true, true, false) => (false, true),
                (true, false, true) => (false, true),
                (true, false, false) => (true, false),
                (false, true, true) => (false, true),
                (false, true, false) => (true, false),
                (false, false, true) => (true, false),
                (false, false, false) => (false, false),
            };
            *acc = full_carry;
            Some(full_add)
        });
    let res:String = full_added
        .collect::<Vec<bool>>()
        .into_iter()
        .rev()
        .skip_while(|x| !x)
        .map(bool_to_char)
        .collect();
    match res.len() == 0{
        true => "0".to_string(),
        false => res,
    }
}
