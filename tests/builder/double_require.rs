use clap::{App, Arg, ErrorKind};

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
    prog [OPTIONS] -b -c

For more information try --help
";

static ONLY_C_ERROR: &str = "error: The following required arguments were not provided:
    -b

USAGE:
    prog [OPTIONS] -c -b

For more information try --help
";

fn app() -> App<'static> {
    App::new("prog")
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
    let res = app().try_get_matches_from(vec!["", "-a"]);
    assert!(res.is_ok());
    let res = app().clone().try_get_matches_from(vec!["", "-b", "-c"]);
    assert!(res.is_ok());
    let res = app().try_get_matches_from(vec!["", "-c", "-b"]);
    assert!(res.is_ok());
}

#[test]
fn help_text() {
    let res = app().try_get_matches_from(vec!["prog", "--help"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayHelp);
    println!("{}", err.to_string());
    assert_eq!(err.to_string(), HELP);
}

#[test]
fn no_duplicate_error() {
    let res = app().try_get_matches_from(vec!["", "-b"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
    assert_eq!(err.to_string(), ONLY_B_ERROR);

    let res = app().try_get_matches_from(vec!["", "-c"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
    assert_eq!(err.to_string(), ONLY_C_ERROR);
}
