use super::utils;

use clap::{arg, error::ErrorKind, Arg, ArgAction, Command};

static VISIBLE_ALIAS_HELP: &str = "clap-test 2.6

USAGE:
    clap-test [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    test    Some help [aliases: dongle, done]
";

static INVISIBLE_ALIAS_HELP: &str = "clap-test 2.6

USAGE:
    clap-test [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    test    Some help
";

static SUBCMD_ALPHA_ORDER: &str = "test 1

USAGE:
    test [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    a1      blah a1
    b1      blah b1
    help    Print this message or the help of the given subcommand(s)
";

static SUBCMD_DECL_ORDER: &str = "test 1

USAGE:
    test [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    b1      blah b1
    a1      blah a1
    help    Print this message or the help of the given subcommand(s)
";

#[cfg(feature = "suggestions")]
static DYM_SUBCMD: &str = "error: The subcommand 'subcm' wasn't recognized

	Did you mean 'subcmd'?

If you believe you received this message in error, try re-running with 'dym -- subcm'

USAGE:
    dym [SUBCOMMAND]

For more information try --help
";

#[cfg(feature = "suggestions")]
static DYM_SUBCMD_AMBIGUOUS: &str = "error: The subcommand 'te' wasn't recognized

	Did you mean 'test' or 'temp'?

If you believe you received this message in error, try re-running with 'dym -- te'

USAGE:
    dym [SUBCOMMAND]

For more information try --help
";

static SUBCMD_AFTER_DOUBLE_DASH: &str =
    "error: Found argument 'subcmd' which wasn't expected, or isn't valid in this context

\tIf you tried to supply `subcmd` as a subcommand, remove the '--' before it.

USAGE:
    cmd [SUBCOMMAND]

For more information try --help
";

#[test]
fn subcommand() {
    let m = Command::new("test")
        .subcommand(
            Command::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .takes_value(true)
                    .help("testing testing"),
            ),
        )
        .arg(Arg::new("other").long("other"))
        .try_get_matches_from(vec!["myprog", "some", "--test", "testing"])
        .unwrap();

    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.contains_id("test"));
    assert_eq!(
        sub_m.get_one::<String>("test").map(|v| v.as_str()).unwrap(),
        "testing"
    );
}

#[test]
fn subcommand_none_given() {
    let m = Command::new("test")
        .subcommand(
            Command::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .takes_value(true)
                    .help("testing testing"),
            ),
        )
        .arg(Arg::new("other").long("other"))
        .try_get_matches_from(vec![""])
        .unwrap();

    assert!(m.subcommand_name().is_none());
}

#[test]
fn subcommand_multiple() {
    let m = Command::new("test")
        .subcommands(vec![
            Command::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .takes_value(true)
                    .help("testing testing"),
            ),
            Command::new("add").arg(Arg::new("roster").short('r')),
        ])
        .arg(Arg::new("other").long("other"))
        .try_get_matches_from(vec!["myprog", "some", "--test", "testing"])
        .unwrap();

    assert!(m.subcommand_matches("some").is_some());
    assert!(m.subcommand_matches("add").is_none());
    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.contains_id("test"));
    assert_eq!(
        sub_m.get_one::<String>("test").map(|v| v.as_str()).unwrap(),
        "testing"
    );
}

#[test]
fn subcommand_display_order() {
    let app_subcmd_alpha_order = Command::new("test").version("1").subcommands(vec![
        Command::new("b1")
            .about("blah b1")
            .arg(Arg::new("test").short('t')),
        Command::new("a1")
            .about("blah a1")
            .arg(Arg::new("roster").short('r')),
    ]);

    utils::assert_output(
        app_subcmd_alpha_order,
        "test --help",
        SUBCMD_ALPHA_ORDER,
        false,
    );

    let app_subcmd_decl_order = Command::new("test")
        .version("1")
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .subcommands(vec![
            Command::new("b1")
                .about("blah b1")
                .arg(Arg::new("test").short('t')),
            Command::new("a1")
                .about("blah a1")
                .arg(Arg::new("roster").short('r')),
        ]);

    utils::assert_output(
        app_subcmd_decl_order,
        "test --help",
        SUBCMD_DECL_ORDER,
        false,
    );
}

#[test]
fn single_alias() {
    let m = Command::new("myprog")
        .subcommand(Command::new("test").alias("do-stuff"))
        .try_get_matches_from(vec!["myprog", "do-stuff"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn multiple_aliases() {
    let m = Command::new("myprog")
        .subcommand(Command::new("test").aliases(&["do-stuff", "test-stuff"]))
        .try_get_matches_from(vec!["myprog", "test-stuff"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
#[cfg(feature = "suggestions")]
fn subcmd_did_you_mean_output() {
    let cmd = Command::new("dym").subcommand(Command::new("subcmd"));
    utils::assert_output(cmd, "dym subcm", DYM_SUBCMD, true);
}

#[test]
#[cfg(feature = "suggestions")]
fn subcmd_did_you_mean_output_ambiguous() {
    let cmd = Command::new("dym")
        .subcommand(Command::new("test"))
        .subcommand(Command::new("temp"));
    utils::assert_output(cmd, "dym te", DYM_SUBCMD_AMBIGUOUS, true);
}

#[test]
#[cfg(feature = "suggestions")]
fn subcmd_did_you_mean_output_arg() {
    static EXPECTED: &str =
        "error: Found argument '--subcmarg' which wasn't expected, or isn't valid in this context

\tDid you mean to put '--subcmdarg' after the subcommand 'subcmd'?

\tIf you tried to supply `--subcmarg` as a value rather than a flag, use `-- --subcmarg`

USAGE:
    dym [SUBCOMMAND]

For more information try --help
";

    let cmd = Command::new("dym").subcommand(
        Command::new("subcmd").arg(arg!(-s --subcmdarg <subcmdarg> "tests").required(false)),
    );

    utils::assert_output(cmd, "dym --subcmarg subcmd", EXPECTED, true);
}

#[test]
#[cfg(feature = "suggestions")]
fn subcmd_did_you_mean_output_arg_false_positives() {
    static EXPECTED: &str =
        "error: Found argument '--subcmarg' which wasn't expected, or isn't valid in this context

\tIf you tried to supply `--subcmarg` as a value rather than a flag, use `-- --subcmarg`

USAGE:
    dym [SUBCOMMAND]

For more information try --help
";

    let cmd = Command::new("dym").subcommand(
        Command::new("subcmd").arg(arg!(-s --subcmdarg <subcmdarg> "tests").required(false)),
    );

    utils::assert_output(cmd, "dym --subcmarg foo", EXPECTED, true);
}

#[test]
fn alias_help() {
    let m = Command::new("myprog")
        .subcommand(Command::new("test").alias("do-stuff"))
        .try_get_matches_from(vec!["myprog", "help", "do-stuff"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

#[test]
fn visible_aliases_help_output() {
    let cmd = Command::new("clap-test").version("2.6").subcommand(
        Command::new("test")
            .about("Some help")
            .alias("invisible")
            .visible_alias("dongle")
            .visible_alias("done"),
    );
    utils::assert_output(cmd, "clap-test --help", VISIBLE_ALIAS_HELP, false);
}

#[test]
fn invisible_aliases_help_output() {
    let cmd = Command::new("clap-test")
        .version("2.6")
        .subcommand(Command::new("test").about("Some help").alias("invisible"));
    utils::assert_output(cmd, "clap-test --help", INVISIBLE_ALIAS_HELP, false);
}

#[test]
#[cfg(feature = "unstable-replace")]
fn replace() {
    let m = Command::new("prog")
        .subcommand(
            Command::new("module").subcommand(Command::new("install").about("Install module")),
        )
        .replace("install", &["module", "install"])
        .try_get_matches_from(vec!["prog", "install"])
        .unwrap();

    assert_eq!(m.subcommand_name(), Some("module"));
    assert_eq!(
        m.subcommand_matches("module").unwrap().subcommand_name(),
        Some("install")
    );
}

#[test]
fn issue_1031_args_with_same_name() {
    let res = Command::new("prog")
        .arg(arg!(--"ui-path" <PATH>))
        .subcommand(Command::new("signer"))
        .try_get_matches_from(vec!["prog", "--ui-path", "signer"]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert_eq!(
        m.get_one::<String>("ui-path").map(|v| v.as_str()),
        Some("signer")
    );
}

#[test]
fn issue_1031_args_with_same_name_no_more_vals() {
    let res = Command::new("prog")
        .arg(arg!(--"ui-path" <PATH>))
        .subcommand(Command::new("signer"))
        .try_get_matches_from(vec!["prog", "--ui-path", "value", "signer"]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert_eq!(
        m.get_one::<String>("ui-path").map(|v| v.as_str()),
        Some("value")
    );
    assert_eq!(m.subcommand_name(), Some("signer"));
}

#[test]
fn issue_1161_multiple_hyphen_hyphen() {
    // from example 22
    let res = Command::new("myprog")
        .arg(Arg::new("eff").short('f'))
        .arg(Arg::new("pea").short('p').takes_value(true))
        .arg(
            Arg::new("slop")
                .takes_value(true)
                .multiple_values(true)
                .last(true),
        )
        .try_get_matches_from(vec![
            "-f",
            "-p=bob",
            "--",
            "sloppy",
            "slop",
            "-a",
            "--",
            "subprogram",
            "position",
            "args",
        ]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());
    let m = res.unwrap();

    let expected = Some(vec![
        "sloppy",
        "slop",
        "-a",
        "--",
        "subprogram",
        "position",
        "args",
    ]);
    let actual = m
        .get_many::<String>("slop")
        .map(|vals| vals.map(|s| s.as_str()).collect::<Vec<_>>());

    assert_eq!(expected, actual);
}

#[test]
fn issue_1722_not_emit_error_when_arg_follows_similar_to_a_subcommand() {
    let m = Command::new("myprog")
        .subcommand(Command::new("subcommand"))
        .arg(Arg::new("argument"))
        .try_get_matches_from(vec!["myprog", "--", "subcommand"]);
    assert_eq!(
        m.unwrap().get_one::<String>("argument").map(|v| v.as_str()),
        Some("subcommand")
    );
}

#[test]
fn subcommand_placeholder_test() {
    let mut cmd = Command::new("myprog")
        .subcommand(Command::new("subcommand"))
        .subcommand_value_name("TEST_PLACEHOLDER")
        .subcommand_help_heading("TEST_HEADER");

    assert_eq!(&cmd.render_usage(), "USAGE:\n    myprog [TEST_PLACEHOLDER]");

    let mut help_text = Vec::new();
    cmd.write_help(&mut help_text)
        .expect("Failed to write to internal buffer");

    assert!(String::from_utf8(help_text)
        .unwrap()
        .contains("TEST_HEADER:"));
}

#[test]
fn subcommand_used_after_double_dash() {
    let cmd = Command::new("cmd").subcommand(Command::new("subcmd"));

    utils::assert_output(cmd, "cmd -- subcmd", SUBCMD_AFTER_DOUBLE_DASH, true);
}

#[test]
fn subcommand_after_argument() {
    let m = Command::new("myprog")
        .arg(Arg::new("some_text"))
        .subcommand(Command::new("test"))
        .try_get_matches_from(vec!["myprog", "teat", "test"])
        .unwrap();
    assert_eq!(
        m.get_one::<String>("some_text").map(|v| v.as_str()),
        Some("teat")
    );
    assert_eq!(m.subcommand().unwrap().0, "test");
}

#[test]
fn subcommand_after_argument_looks_like_help() {
    let m = Command::new("myprog")
        .arg(Arg::new("some_text"))
        .subcommand(Command::new("test"))
        .try_get_matches_from(vec!["myprog", "helt", "test"])
        .unwrap();
    assert_eq!(
        m.get_one::<String>("some_text").map(|v| v.as_str()),
        Some("helt")
    );
    assert_eq!(m.subcommand().unwrap().0, "test");
}

#[test]
fn issue_2494_subcommand_is_present() {
    let cmd = Command::new("opt")
        .arg(Arg::new("global").long("global").action(ArgAction::SetTrue))
        .subcommand(Command::new("global"));

    let m = cmd
        .clone()
        .try_get_matches_from(&["opt", "--global", "global"])
        .unwrap();
    assert_eq!(m.subcommand_name().unwrap(), "global");
    assert!(*m.get_one::<bool>("global").expect("defaulted by clap"));

    let m = cmd
        .clone()
        .try_get_matches_from(&["opt", "--global"])
        .unwrap();
    assert!(m.subcommand_name().is_none());
    assert!(*m.get_one::<bool>("global").expect("defaulted by clap"));

    let m = cmd.try_get_matches_from(&["opt", "global"]).unwrap();
    assert_eq!(m.subcommand_name().unwrap(), "global");
    assert!(!*m.get_one::<bool>("global").expect("defaulted by clap"));
}

#[test]
fn subcommand_not_recognized() {
    let cmd = Command::new("fake")
        .subcommand(Command::new("sub"))
        .disable_help_subcommand(true)
        .infer_subcommands(true);
    utils::assert_output(
        cmd,
        "fake help",
        "error: The subcommand 'help' wasn't recognized

USAGE:
    fake [SUBCOMMAND]

For more information try --help
",
        true,
    );
}

#[test]
fn busybox_like_multicall() {
    fn applet_commands() -> [Command<'static>; 2] {
        [Command::new("true"), Command::new("false")]
    }
    let cmd = Command::new("busybox")
        .multicall(true)
        .subcommand(Command::new("busybox").subcommands(applet_commands()))
        .subcommands(applet_commands());

    let m = cmd
        .clone()
        .try_get_matches_from(&["busybox", "true"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("busybox"));
    assert_eq!(m.subcommand().unwrap().1.subcommand_name(), Some("true"));

    let m = cmd.clone().try_get_matches_from(&["true"]).unwrap();
    assert_eq!(m.subcommand_name(), Some("true"));

    let m = cmd.clone().try_get_matches_from(&["a.out"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnrecognizedSubcommand);
}

#[test]
fn hostname_like_multicall() {
    let mut cmd = Command::new("hostname")
        .multicall(true)
        .subcommand(Command::new("hostname"))
        .subcommand(Command::new("dnsdomainname"));

    let m = cmd.clone().try_get_matches_from(&["hostname"]).unwrap();
    assert_eq!(m.subcommand_name(), Some("hostname"));

    let m = cmd
        .clone()
        .try_get_matches_from(&["dnsdomainname"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("dnsdomainname"));

    let m = cmd.clone().try_get_matches_from(&["a.out"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnrecognizedSubcommand);

    let m = cmd.try_get_matches_from_mut(&["hostname", "hostname"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnknownArgument);

    let m = cmd.try_get_matches_from(&["hostname", "dnsdomainname"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn bad_multicall_command_error() {
    let cmd = Command::new("repl")
        .version("1.0.0")
        .propagate_version(true)
        .multicall(true)
        .subcommand(Command::new("foo"))
        .subcommand(Command::new("bar"));

    let err = cmd.clone().try_get_matches_from(&["world"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnrecognizedSubcommand);
    static HELLO_EXPECTED: &str = "\
error: The subcommand 'world' wasn't recognized

USAGE:
    <SUBCOMMAND>

For more information try help
";
    utils::assert_eq(HELLO_EXPECTED, err.to_string());

    #[cfg(feature = "suggestions")]
    {
        let err = cmd.clone().try_get_matches_from(&["baz"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidSubcommand);
        static BAZ_EXPECTED: &str = "\
error: The subcommand 'baz' wasn't recognized

\tDid you mean 'bar'?

If you believe you received this message in error, try re-running with ' -- baz'

USAGE:
    <SUBCOMMAND>

For more information try help
";
        utils::assert_eq(BAZ_EXPECTED, err.to_string());
    }

    // Verify whatever we did to get the above to work didn't disable `--help` and `--version`.

    let err = cmd
        .clone()
        .try_get_matches_from(&["foo", "--help"])
        .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);

    let err = cmd
        .clone()
        .try_get_matches_from(&["foo", "--version"])
        .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
}

#[test]
#[should_panic = "Command repl: Arguments like oh-no cannot be set on a multicall command"]
fn cant_have_args_with_multicall() {
    let mut cmd = Command::new("repl")
        .version("1.0.0")
        .propagate_version(true)
        .multicall(true)
        .subcommand(Command::new("foo"))
        .subcommand(Command::new("bar"))
        .arg(Arg::new("oh-no"));
    cmd.build();
}

#[test]
fn multicall_help_flag() {
    static EXPECTED: &str = "\
foo-bar 1.0.0

USAGE:
    foo bar [value]

ARGS:
    <value>    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";
    let cmd = Command::new("repl")
        .version("1.0.0")
        .propagate_version(true)
        .multicall(true)
        .subcommand(Command::new("foo").subcommand(Command::new("bar").arg(Arg::new("value"))));
    utils::assert_output(cmd, "foo bar --help", EXPECTED, false);
}

#[test]
fn multicall_help_subcommand() {
    static EXPECTED: &str = "\
foo-bar 1.0.0

USAGE:
    foo bar [value]

ARGS:
    <value>    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";
    let cmd = Command::new("repl")
        .version("1.0.0")
        .propagate_version(true)
        .multicall(true)
        .subcommand(Command::new("foo").subcommand(Command::new("bar").arg(Arg::new("value"))));
    utils::assert_output(cmd, "help foo bar", EXPECTED, false);
}

#[test]
fn multicall_render_help() {
    static EXPECTED: &str = "\
foo-bar 1.0.0

USAGE:
    foo bar [value]

ARGS:
    <value>    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";
    let mut cmd = Command::new("repl")
        .version("1.0.0")
        .propagate_version(true)
        .multicall(true)
        .subcommand(Command::new("foo").subcommand(Command::new("bar").arg(Arg::new("value"))));
    cmd.build();
    let subcmd = cmd.find_subcommand_mut("foo").unwrap();
    let subcmd = subcmd.find_subcommand_mut("bar").unwrap();

    let mut buf = Vec::new();
    subcmd.write_help(&mut buf).unwrap();
    utils::assert_eq(EXPECTED, String::from_utf8(buf).unwrap());
}
