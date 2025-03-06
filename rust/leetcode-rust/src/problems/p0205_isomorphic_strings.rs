use std::collections::HashMap;

pub fn is_isomorphic(s_str: String, t_str: String) -> bool {
    match s_str.len() == t_str.len() {
        true => {
            let mut dict_s2t = HashMap::new();
            let mut dict_t2s = HashMap::new();
            s_str.chars().zip(t_str.chars()).all(|(s, t)| {
                match (dict_s2t.get(&s), dict_t2s.get(&t)) {
                    (None, None) => {
                        dict_s2t.insert(s, t);
                        dict_t2s.insert(t, s);
                        true
                    }
                    (None, Some(t_)) => {
                        t_ == &t
                    },
                    (Some(s_), None) => s_ == &s,
                    (Some(s_), Some(t_)) => s_ == &s && t_ == &t,
                }
            })
        }
        false => false,
    }
}
