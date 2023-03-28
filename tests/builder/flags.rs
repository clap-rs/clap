use clap::{arg, Arg, ArgAction, Command};

#[cfg(feature = "error-context")]
use super::utils;

#[test]
fn flag_using_short() {
    let m = Command::new("flag")
        .args([
            arg!(-f --flag "some flag").action(ArgAction::SetTrue),
            arg!(-c --color "some other flag").action(ArgAction::SetTrue),
        ])
        .try_get_matches_from(vec!["", "-f", "-c"])
        .unwrap();
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
}

#[test]
fn lots_o_flags_sep() {
    let r = Command::new("opts")
        .args_override_self(true)
        .arg(arg!(o: -o ... "some flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec![
            "", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o", "-o",
            "-o", "-o", "-o",
        ]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert!(*m.get_one::<bool>("o").expect("defaulted by clap"));
}

#[test]
fn lots_o_flags_combined() {
    let r = Command::new("opts")
        .args_override_self(true)
        .arg(arg!(o: -o ... "some flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec![
            "",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
            "-ooooooooooooooooooooooooooooooooooooooooo",
        ]);
    assert!(r.is_ok(), "{:?}", r.unwrap_err().kind());
    let m = r.unwrap();
    assert!(m.contains_id("o"));
    assert!(*m.get_one::<bool>("o").expect("defaulted by clap"));
}

#[test]
fn flag_using_long() {
    let m = Command::new("flag")
        .args([
            arg!(--flag "some flag").action(ArgAction::SetTrue),
            arg!(--color "some other flag").action(ArgAction::SetTrue),
        ])
        .try_get_matches_from(vec!["", "--flag", "--color"])
        .unwrap();
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
}

#[test]
fn flag_using_long_with_literals() {
    use clap::error::ErrorKind;

    let m = Command::new("flag")
        .arg(
            Arg::new("rainbow")
                .long("rainbow")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["", "--rainbow=false"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::TooManyValues);
}

#[test]
fn flag_using_mixed() {
    let m = Command::new("flag")
        .args([
            arg!(-f --flag "some flag").action(ArgAction::SetTrue),
            arg!(-c --color "some other flag").action(ArgAction::SetTrue),
        ])
        .try_get_matches_from(vec!["", "-f", "--color"])
        .unwrap();
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));

    let m = Command::new("flag")
        .args([
            arg!(-f --flag "some flag").action(ArgAction::SetTrue),
            arg!(-c --color "some other flag").action(ArgAction::SetTrue),
        ])
        .try_get_matches_from(vec!["", "--flag", "-c"])
        .unwrap();
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
}

#[test]
fn multiple_flags_in_single() {
    let m = Command::new("multe_flags")
        .args([
            arg!(-f --flag "some flag").action(ArgAction::SetTrue),
            arg!(-c --color "some other flag").action(ArgAction::SetTrue),
            arg!(-d --debug "another other flag").action(ArgAction::SetTrue),
        ])
        .try_get_matches_from(vec!["", "-fcd"])
        .unwrap();
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("debug").expect("defaulted by clap"));
}

#[test]
#[cfg(feature = "error-context")]
fn unexpected_value_error() {
    const USE_FLAG_AS_ARGUMENT: &str = "\
error: unexpected value 'foo' for '--a-flag' found; no more were expected

Usage: mycat --a-flag [filename]

For more information, try '--help'.
";

    let cmd = Command::new("mycat")
        .arg(Arg::new("filename"))
        .arg(Arg::new("a-flag").long("a-flag").action(ArgAction::SetTrue));

    utils::assert_output(cmd, "mycat --a-flag=foo", USE_FLAG_AS_ARGUMENT, true);
}

#[test]
#[cfg(feature = "error-context")]
fn issue_1284_argument_in_flag_style() {
    const USE_FLAG_AS_ARGUMENT: &str = "\
error: unexpected argument '--another-flag' found

  tip: to pass '--another-flag' as a value, use '-- --another-flag'

Usage: mycat [OPTIONS] [filename]

For more information, try '--help'.
";

    let cmd = Command::new("mycat")
        .arg(Arg::new("filename"))
        .arg(Arg::new("a-flag").long("a-flag").action(ArgAction::SetTrue));

    let m = cmd
        .clone()
        .try_get_matches_from(vec!["", "--", "--another-flag"])
        .unwrap();
    assert_eq!(
        m.get_one::<String>("filename").map(|v| v.as_str()),
        Some("--another-flag")
    );

    let m = cmd
        .clone()
        .try_get_matches_from(vec!["", "--a-flag"])
        .unwrap();
    assert!(*m.get_one::<bool>("a-flag").expect("defaulted by clap"));

    let m = cmd
        .clone()
        .try_get_matches_from(vec!["", "--", "--a-flag"])
        .unwrap();
    assert_eq!(
        m.get_one::<String>("filename").map(|v| v.as_str()),
        Some("--a-flag")
    );

    utils::assert_output(cmd, "mycat --another-flag", USE_FLAG_AS_ARGUMENT, true);
}

#[test]
#[cfg(feature = "error-context")]
fn issue_2308_multiple_dashes() {
    static MULTIPLE_DASHES: &str = "\
error: unexpected argument '-----' found

  tip: to pass '-----' as a value, use '-- -----'

Usage: test <arg>

For more information, try '--help'.
";
    let cmd = Command::new("test").arg(Arg::new("arg").action(ArgAction::Set).required(true));

    utils::assert_output(cmd, "test -----", MULTIPLE_DASHES, true);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "Argument filename: long \"--filename\" must not start with a `-`, that will be handled by the parser"]
fn leading_dash_stripped() {
    let cmd = Command::new("mycat").arg(Arg::new("filename").long("--filename"));
    cmd.debug_assert();
}
