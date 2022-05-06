use clap::{error::ErrorKind, Arg, Command};

static HELP: &str = "prog 

USAGE:
    prog [OPTIONS]

OPTIONS:
    -a            
    -b            
    -c            
    -h, --help    Print help information
";

static ONLY_B_ERROR: &str = "error: The following required arguments were not provided:
    -c

USAGE:
    prog -b -c

For more information try --help
";

static ONLY_C_ERROR: &str = "error: The following required arguments were not provided:
    -b

USAGE:
    prog -c -b

For more information try --help
";

fn cmd() -> Command<'static> {
    Command::new("prog")
        .arg(
            Arg::new("a")
                .short('a')
                .required_unless_present_any(&["b", "c"])
                .conflicts_with_all(&["b", "c"]),
        )
        .arg(
            Arg::new("b")
                .short('b')
                .required_unless_present("a")
                .requires("c"),
        )
        .arg(
            Arg::new("c")
                .short('c')
                .required_unless_present("a")
                .requires("b"),
        )
}

#[test]
fn valid_cases() {
    let res = cmd().try_get_matches_from(vec!["", "-a"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let res = cmd().clone().try_get_matches_from(vec!["", "-b", "-c"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let res = cmd().try_get_matches_from(vec!["", "-c", "-b"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[test]
fn help_text() {
    let res = cmd().try_get_matches_from(vec!["prog", "--help"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);
    println!("{}", err);
    assert_eq!(err.to_string(), HELP);
}

#[test]
fn no_duplicate_error() {
    let res = cmd().try_get_matches_from(vec!["", "-b"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    assert_eq!(err.to_string(), ONLY_B_ERROR);

    let res = cmd().try_get_matches_from(vec!["", "-c"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    assert_eq!(err.to_string(), ONLY_C_ERROR);
}
