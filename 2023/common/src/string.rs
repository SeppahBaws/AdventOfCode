/// Test if the `needle` parameter matches the beginning characters of the `haystack` parameter.
pub fn matches_begin(needle: &String, haystack: &String) -> bool {
    needle.len() <= haystack.len() && &haystack[..needle.len()] == needle.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_matches_begin() {
        assert_eq!(matches_begin(&"al".into(), &"alfred".into()), true);
        assert_eq!(matches_begin(&"test".into(), &"test".into()), true);
        assert_eq!(matches_begin(&"".into(), &"".into()), true);

        assert_eq!(matches_begin(&"fred".into(), &"alfred".into()), false);
        assert_eq!(matches_begin(&"much longer".into(), &"short".into()), false);
    }
}
