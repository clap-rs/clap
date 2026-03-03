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
mod test {
    use super::*;
    #[test]
    fn true_lit() {
        for lit in TRUE_LITERALS {
            assert_eq!(Some(true), str_to_bool(lit));
        }
    }

    #[test]
    fn false_lit() {
        for lit in FALSE_LITERALS {
            assert_eq!(Some(false), str_to_bool(lit));
        }
    }

    #[test]
    fn empty_str() {
        assert_eq!(None, str_to_bool(""));
        assert_eq!(None, str_to_bool("  "));
    }

    #[test]
    fn case_sensitivity() {
        assert_eq!(Some(true), str_to_bool("TRUE"));
        assert_eq!(Some(false), str_to_bool("fAlSe"));
    }

    #[test]
    fn unrecognized() {
        assert_eq!(None, str_to_bool("affirmative"));
        assert_eq!(None, str_to_bool("2"));
    }

    #[test]
    fn whitespace() {
        assert_eq!(None, str_to_bool("  true "));
        assert_eq!(None, str_to_bool("false  "));
    }
}
