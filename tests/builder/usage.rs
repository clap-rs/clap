use clap::{Arg, Command, ErrorKind};

fn cmd() -> Command<'static> {
    Command::new("prog")
        .arg(Arg::new("a").required(true))
        .arg(Arg::new("b").short('b').takes_value(true).group("debug"))
        .arg(Arg::new("c").short('c').takes_value(true).group("debug"))
}

#[test]
fn valid_cases() {
    let res = cmd().try_get_matches_from(vec!["prog", "aaa"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let res = cmd().try_get_matches_from(vec!["prog", "aaa", "-b", "bbb"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let res = cmd().try_get_matches_from(vec!["prog", "aaa", "-c", "ccc"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

/// https://github.com/clap-rs/clap/issues/3665 and https://github.com/clap-rs/clap/issues/3556
#[test]
fn no_duplicate_required_argument_a_when_conflict() {
    let res = cmd().try_get_matches_from(vec!["prog", "aaa", "-b", "bbb", "-c", "ccc"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
    assert_eq!(
        err.to_string(),
        "error: The argument '-b <b>' cannot be used with '-c <c>'

USAGE:
    prog -b <b> <a>

For more information try --help
"
    );
}
