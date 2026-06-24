use clap::{arg, error::ErrorKind, parser::ValueSource, Arg, ArgAction, Command};
use snapbox::str;

use super::utils;

#[test]
fn single_short_arg_without_value() {
    let cmd = Command::new("cmd")
        .ignore_errors(true)
        .arg(arg!(
            -c --config <FILE> "Sets a custom config file"
        ))
        .arg(arg!(--"unset-flag"));

    let r = cmd.try_get_matches_from(vec!["cmd", "-c" /* missing: , "config file" */]);

    assert!(r.is_ok(), "unexpected error: {r:?}");
    let m = r.unwrap();
    assert!(m.contains_id("config"));
    assert_eq!(m.get_one::<String>("config").cloned(), None);
    assert_eq!(m.get_one::<bool>("unset-flag").copied(), Some(false));
}

#[test]
fn single_long_arg_without_value() {
    let cmd = Command::new("cmd")
        .ignore_errors(true)
        .arg(arg!(
            -c --config <FILE> "Sets a custom config file"
        ))
        .arg(arg!(--"unset-flag"));

    let r = cmd.try_get_matches_from(vec!["cmd", "--config" /* missing: , "config file" */]);

    assert!(r.is_ok(), "unexpected error: {r:?}");
    let m = r.unwrap();
    assert!(m.contains_id("config"));
    assert_eq!(m.get_one::<String>("config").cloned(), None);
    assert_eq!(m.get_one::<bool>("unset-flag").copied(), Some(false));
}

#[test]
fn multiple_args_and_final_arg_without_value() {
    let cmd = Command::new("cmd")
        .ignore_errors(true)
        .arg(arg!(
            -c --config <FILE> "Sets a custom config file"
        ))
        .arg(arg!(
            -x --stuff <FILE> "Sets a custom stuff file"
        ))
        .arg(arg!(f: -f "Flag").action(ArgAction::SetTrue))
        .arg(arg!(--"unset-flag"));

    let r = cmd.try_get_matches_from(vec![
        "cmd", "-c", "file", "-f", "-x", /* missing: , "some stuff" */
    ]);

    assert!(r.is_ok(), "unexpected error: {r:?}");
    let m = r.unwrap();
    assert_eq!(
        m.get_one::<String>("config").map(|v| v.as_str()),
        Some("file")
    );
    assert_eq!(m.get_one::<bool>("f").copied(), Some(true));
    assert_eq!(m.get_one::<String>("stuff").map(|v| v.as_str()), None);
    assert_eq!(m.get_one::<bool>("unset-flag").copied(), Some(false));
}

#[test]
fn multiple_args_and_intermittent_arg_without_value() {
    let cmd = Command::new("cmd")
        .ignore_errors(true)
        .arg(arg!(
            -c --config <FILE> "Sets a custom config file"
        ))
        .arg(arg!(
            -x --stuff <FILE> "Sets a custom stuff file"
        ))
        .arg(arg!(f: -f "Flag").action(ArgAction::SetTrue))
        .arg(arg!(--"unset-flag"));

    let r = cmd.try_get_matches_from(vec![
        "cmd", "-x", /* missing: ,"some stuff" */
        "-c", "file", "-f",
    ]);

    assert!(r.is_ok(), "unexpected error: {r:?}");
    let m = r.unwrap();
    assert_eq!(
        m.get_one::<String>("config").map(|v| v.as_str()),
        Some("file")
    );
    assert_eq!(m.get_one::<bool>("f").copied(), Some(true));
    assert_eq!(m.get_one::<String>("stuff").map(|v| v.as_str()), None);
    assert_eq!(m.get_one::<bool>("unset-flag").copied(), Some(false));
}

#[test]
fn unexpected_argument() {
    let cmd = Command::new("cmd")
        .ignore_errors(true)
        .arg(arg!(
            -c --config [FILE] "Sets a custom config file"
        ))
        .arg(arg!(--"unset-flag"));

    let r = cmd.try_get_matches_from(vec!["cmd", "-c", "config file", "unexpected"]);

    assert!(r.is_ok(), "unexpected error: {r:?}");
    let m = r.unwrap();
    assert!(m.contains_id("config"));
    assert_eq!(
        m.get_one::<String>("config").cloned(),
        Some("config file".to_owned())
    );
    assert_eq!(m.get_one::<bool>("unset-flag").copied(), Some(false));
}

#[test]
#[cfg(feature = "error-context")]
fn did_you_mean() {
    let mut cmd = Command::new("cmd").arg(arg!(--"ignore-immutable"));

    // Verify we are in a "did you mean" error
    let r = cmd.try_get_matches_from_mut(vec!["cmd", "--ig"]);
    assert!(r.is_err());
    let err = r.unwrap_err();
    utils::assert_error(
        err,
        ErrorKind::UnknownArgument,
        str![[r#"
error: unexpected argument '--ig' found

  tip: a similar argument exists: '--ignore-immutable'

Usage: cmd --ignore-immutable

For more information, try '--help'.

"#]],
        true,
    );

    let r = cmd
        .ignore_errors(true)
        .try_get_matches_from(vec!["cmd", "--ig"]);
    assert!(r.is_ok(), "unexpected error: {r:?}");
    let m = r.unwrap();
    assert!(m.contains_id("ignore-immutable"), "{m:#?}");
    assert_eq!(
        m.value_source("ignore-immutable"),
        Some(ValueSource::DefaultValue)
    );
}

#[test]
fn subcommand() {
    let cmd = Command::new("test")
        .ignore_errors(true)
        .subcommand(
            Command::new("some")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .action(ArgAction::Set)
                        .help("testing testing"),
                )
                .arg(
                    Arg::new("stuff")
                        .short('x')
                        .long("stuff")
                        .action(ArgAction::Set)
                        .help("stuf value"),
                )
                .arg(arg!(--"unset-flag")),
        )
        .arg(Arg::new("other").long("other"))
        .arg(arg!(--"unset-flag"));

    let m = cmd
        .try_get_matches_from(vec![
            "myprog",
            "some",
            "--test", /* missing: ,"some val" */
            "-x",
            "some other val",
        ])
        .unwrap();

    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(
        sub_m.contains_id("test"),
        "expected subcommand to be present due to partial parsing"
    );
    assert_eq!(sub_m.get_one::<String>("test").map(|v| v.as_str()), None);
    assert_eq!(
        sub_m.get_one::<String>("stuff").map(|v| v.as_str()),
        Some("some other val")
    );
    assert_eq!(sub_m.get_one::<bool>("unset-flag").copied(), Some(false));

    assert_eq!(m.get_one::<bool>("unset-flag").copied(), Some(false));
}

#[test]
fn help_flag() {
    let cmd = Command::new("test").ignore_errors(true);

    utils::assert_output(
        cmd,
        "test --help",
        str![[r#"
Usage: test

Options:
  -h, --help  Print help

"#]],
        false,
    );
}

#[test]
fn help_flag_subcommand() {
    let cmd = Command::new("test")
        .subcommand(Command::new("sub"))
        .ignore_errors(true);

    utils::assert_output(
        cmd,
        "test sub --help",
        str![[r#"
Usage: test sub

Options:
  -h, --help  Print help

"#]],
        false,
    );
}

#[test]
fn version_flag() {
    let cmd = Command::new("test").ignore_errors(true).version("0.1");

    utils::assert_output(cmd, "test --version", "test 0.1\n", false);
}
