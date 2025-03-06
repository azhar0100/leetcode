use std::collections::HashMap;

fn is_mapped(dict_s2t: &mut HashMap<char, char>, s: char, t: char) -> bool {
    dict_s2t.get(&s).map(|t_| t_ == &t).unwrap_or_else(|| {
        dict_s2t.insert(s, t);
        true
    })
}

pub fn is_isomorphic(s_str: String, t_str: String) -> bool {
    match s_str.len() == t_str.len() {
        true => {
            let mut dict_s2t = HashMap::new();
            let mut dict_t2s = HashMap::new();
            s_str
                .chars()
                .zip(t_str.chars())
                .all(|(s, t)| is_mapped(&mut dict_s2t, s, t) && is_mapped(&mut dict_t2s, t, s))
        }
        false => false,
    }
}
