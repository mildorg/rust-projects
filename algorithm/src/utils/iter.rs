#[cfg(test)]
pub fn to_sting<T: ToString>(list: impl IntoIterator<Item = T>) -> String {
    list.into_iter()
        .map(|elem| elem.to_string())
        .collect::<String>()
}
