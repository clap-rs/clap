extern crate clap;

use clap::{App, Arg};

#[test]
fn hidden_args() {
    let mut app = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .args(&[Arg::from_usage("-f, --flag 'some flag'").hidden(true),
                    Arg::from_usage("-F, --flag2 'some other flag'"),
                    Arg::from_usage("--option [opt] 'some option'")]);
    // We call a get_matches method to cause --help and --version to be built
    let _ = app.get_matches_from_safe_borrow(vec![""]);

    // Now we check the output of print_help()
    let mut help = vec![];
    app.write_help(&mut help).expect("failed to print help");
    assert_eq!(&*String::from_utf8_lossy(&*help), &*String::from("test 1.3\n\
Kevin K.
tests stuff

USAGE:
    test [FLAGS] [OPTIONS]

FLAGS:
    -F, --flag2      some other flag
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --option <opt>    some option"));
}
