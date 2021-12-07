use crate::utils;

use clap::App;

#[test]
fn very_large_display_order() {
    let app = App::new("test").subcommand(App::new("sub").display_order(usize::MAX));

    assert!(utils::compare_output(
        app,
        "test --help",
        "test 

USAGE:
    test [SUBCOMMAND]

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    sub     
",
        false
    ));
}
