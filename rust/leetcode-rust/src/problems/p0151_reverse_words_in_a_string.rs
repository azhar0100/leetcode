pub fn reverse_words(s: String) -> String {
    s.split(" ")
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
