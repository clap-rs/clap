/// True values are `y`, `yes`, `t`, `true`, `on`, and `1`.
pub(crate) const TRUE_LITERALS: [&str; 6] = ["y", "yes", "t", "true", "on", "1"];

/// False values are `n`, `no`, `f`, `false`, `off`, and `0`.
pub(crate) const FALSE_LITERALS: [&str; 6] = ["n", "no", "f", "false", "off", "0"];

/// Converts a string literal representation of truth to true or false.
///
/// Translated from the Python function [`strtobool`].
///
/// [`strtobool`]: https://docs.python.org/3/distutils/apiref.html#distutils.util.strtobool
///
/// # Errors
/// Returns Err(val) if `val` is anything else.
pub(crate) fn str_to_bool<S: AsRef<str>>(val: S) -> Result<bool, S> {
    let pat: &str = &val.as_ref().to_lowercase();
    if TRUE_LITERALS.contains(&pat) {
        Ok(true)
    } else if FALSE_LITERALS.contains(&pat) {
        Ok(false)
    } else {
        Err(val)
    }
}
