use super::p0383_ransom_note::string_dictionary;

pub fn is_anagram(s: String, t: String) -> bool {
    string_dictionary(&s) == string_dictionary(&t)
}