use std::collections::HashMap;
pub fn string_dictionary(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut acc, c| {
        acc.entry(c).and_modify(|x| *x += 1).or_insert(1);
        acc
    })
}

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let ransom_note_dict = string_dictionary(&ransom_note);
    let magazine_dict = string_dictionary(&magazine);
    ransom_note_dict.iter().all(|(note_k, note_v)| {
        let magazine_v = magazine_dict
            .get(&note_k)
            .unwrap_or(&0);
        note_v <= magazine_v
    })
}
