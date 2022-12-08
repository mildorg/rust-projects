#[allow(dead_code)]
pub(crate) fn to_first_upper(word: &str) -> String {
    let mut chars = word.chars();

    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
