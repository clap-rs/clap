extern crate clap;

fn assert_display_contains_parent_message(error: std::io::Error) {
    let expected = error.to_string();
    let clap_error = clap::Error::from(error);
    let clap_formatted = clap_error.to_string();
    assert!(
        &clap_formatted.contains(&expected),
        "expected: {:#?}\nto be contained in: {:#?}",
        &expected,
        &clap_formatted,
    );
}

#[test]
fn stringy_io_error_display() {
    assert_display_contains_parent_message(std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "a plain string message",
    ));
}

#[cfg(unix)] // platform-specific because it requires raw error codes not published via stdlib
#[test]
fn raw_io_error_display() {
    assert_display_contains_parent_message(std::io::Error::from_raw_os_error(2))
}

#[test]
fn custom_io_error_display() {
    #[derive(Debug)]
    struct MyCustomError;

    impl std::fmt::Display for MyCustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "my very nice error message for {:p}", self)
        }
    }
    impl std::error::Error for MyCustomError {}

    assert_display_contains_parent_message(std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        MyCustomError {},
    ))
}
