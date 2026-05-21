/// True values are `y`, `yes`, `t`, `true`, `on`, and `1`.
pub(crate) const TRUE_LITERALS: [&str; 6] = ["y", "yes", "t", "true", "on", "1"];

/// False values are `n`, `no`, `f`, `false`, `off`, and `0`.
pub(crate) const FALSE_LITERALS: [&str; 6] = ["n", "no", "f", "false", "off", "0"];

/// Converts a string literal representation of truth to true or false.
///
/// `false` values are `n`, `no`, `f`, `false`, `off`, and `0` (case insensitive).
///
/// Any other value will be considered as `true`.
pub(crate) fn str_to_bool(val: impl AsRef<str>) -> Option<bool> {
    let pat: &str = &val.as_ref().to_lowercase();
    if TRUE_LITERALS.contains(&pat) {
        Some(true)
    } else if FALSE_LITERALS.contains(&pat) {
        Some(false)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(str_to_bool(""), None);
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(str_to_bool(" "), None);
        assert_eq!(str_to_bool("\t"), None);
        assert_eq!(str_to_bool("\n"), None);
    }

    #[test]
    fn whitespace_padded() {
        assert_eq!(str_to_bool(" true "), None);
        assert_eq!(str_to_bool("false "), None);
        assert_eq!(str_to_bool(" yes"), None);
    }

    #[test]
    fn mixed_case() {
        assert_eq!(str_to_bool("TrUe"), Some(true));
        assert_eq!(str_to_bool("FALSE"), Some(false));
        assert_eq!(str_to_bool("Yes"), Some(true));
        assert_eq!(str_to_bool("nO"), Some(false));
        assert_eq!(str_to_bool("ON"), Some(true));
        assert_eq!(str_to_bool("Off"), Some(false));
    }

    #[test]
    fn unicode_lookalikes() {
        // Fullwidth characters
        assert_eq!(str_to_bool("\u{FF54}\u{FF52}\u{FF55}\u{FF45}"), None);
        // Cyrillic lookalike (U+0435)
        assert_eq!(str_to_bool("tru\u{0435}"), None);
        // Zero-width space
        assert_eq!(str_to_bool("true\u{200B}"), None);
    }
}
