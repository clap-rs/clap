use clap::{app_from_crate, ErrorKind};

static EVERYTHING: &str = "clap {{version}}
Kevin K. <kbknapp@gmail.com>:Clap Maintainers
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    clap

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
";

/// ATTENTION!: If you see this test fails, one of the possibilities is that you
/// enables the wrap_help feature and your terminal width is small.
#[test]
fn app_from_crate() {
    let res = app_from_crate!().try_get_matches_from(vec!["clap", "--help"]);

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayHelp);
    assert_eq!(
        err.to_string(),
        EVERYTHING.replace("{{version}}", env!("CARGO_PKG_VERSION")),
        "\n\n\n\n>>>>>> ATTENTION!!!!!! <<<<<<\
        \nThis test fails often because your terminal width is small.\n\n\n"
    );
}
