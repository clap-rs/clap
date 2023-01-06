use clap::{arg, error::ErrorKind, Arg, ArgAction, Command};

use super::utils;

#[test]
fn subcommand() {
    let m = Command::new("test")
        .subcommand(
            Command::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .action(ArgAction::Set)
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
                    .action(ArgAction::Set)
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
                    .action(ArgAction::Set)
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
        .subcommand(Command::new("test").aliases(["do-stuff", "test-stuff"]))
        .try_get_matches_from(vec!["myprog", "test-stuff"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
#[cfg(feature = "suggestions")]
#[cfg(feature = "error-context")]
fn subcmd_did_you_mean_output() {
    #[cfg(feature = "suggestions")]
    static DYM_SUBCMD: &str = "\
error: unrecognized subcommand 'subcm'

  note: subcommand 'subcmd' exists
  note: to pass 'subcm' as a value, use 'dym -- subcm'

Usage: dym [COMMAND]

For more information, try '--help'.
";

    let cmd = Command::new("dym").subcommand(Command::new("subcmd"));
    utils::assert_output(cmd, "dym subcm", DYM_SUBCMD, true);
}

#[test]
#[cfg(feature = "suggestions")]
#[cfg(feature = "error-context")]
fn subcmd_did_you_mean_output_ambiguous() {
    #[cfg(feature = "suggestions")]
    static DYM_SUBCMD_AMBIGUOUS: &str = "\
error: unrecognized subcommand 'te'

  note: subcommand 'test', 'temp' exist
  note: to pass 'te' as a value, use 'dym -- te'

Usage: dym [COMMAND]

For more information, try '--help'.
";

    let cmd = Command::new("dym")
        .subcommand(Command::new("test"))
        .subcommand(Command::new("temp"));
    utils::assert_output(cmd, "dym te", DYM_SUBCMD_AMBIGUOUS, true);
}

#[test]
#[cfg(feature = "suggestions")]
#[cfg(feature = "error-context")]
fn subcmd_did_you_mean_output_arg() {
    static EXPECTED: &str = "\
error: unexpected argument '--subcmarg'

  note: 'subcmd --subcmdarg' exists

Usage: dym [COMMAND]

For more information, try '--help'.
";

    let cmd = Command::new("dym")
        .subcommand(Command::new("subcmd").arg(arg!(-s --subcmdarg <subcmdarg> "tests")));

    utils::assert_output(cmd, "dym --subcmarg subcmd", EXPECTED, true);
}

#[test]
#[cfg(feature = "suggestions")]
#[cfg(feature = "error-context")]
fn subcmd_did_you_mean_output_arg_false_positives() {
    static EXPECTED: &str = "\
error: unexpected argument '--subcmarg'

Usage: dym [COMMAND]

For more information, try '--help'.
";

    let cmd = Command::new("dym")
        .subcommand(Command::new("subcmd").arg(arg!(-s --subcmdarg <subcmdarg> "tests")));

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
    static VISIBLE_ALIAS_HELP: &str = "\
Usage: clap-test [COMMAND]

Commands:
  test  Some help [aliases: dongle, done]
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

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
    static INVISIBLE_ALIAS_HELP: &str = "\
Usage: clap-test [COMMAND]

Commands:
  test  Some help
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

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
        .replace("install", ["module", "install"])
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
        .arg(arg!(--"ui-path" <PATH>).required(true))
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
        .arg(arg!(--"ui-path" <PATH>).required(true))
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
        .arg(Arg::new("pea").short('p').action(ArgAction::Set))
        .arg(
            Arg::new("slop")
                .action(ArgAction::Set)
                .num_args(1..)
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

    assert_eq!(
        &cmd.render_usage().to_string(),
        "Usage: myprog [TEST_PLACEHOLDER]"
    );

    let help_text = cmd.render_help().to_string();

    assert!(help_text.contains("TEST_HEADER:"));
}

#[test]
#[cfg(feature = "error-context")]
fn subcommand_used_after_double_dash() {
    static SUBCMD_AFTER_DOUBLE_DASH: &str = "\
error: unexpected argument 'subcmd'

  note: subcommand 'subcmd' exists; to use it, remove the '--' before it

Usage: cmd [COMMAND]

For more information, try '--help'.
";

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
        .try_get_matches_from(["opt", "--global", "global"])
        .unwrap();
    assert_eq!(m.subcommand_name().unwrap(), "global");
    assert!(*m.get_one::<bool>("global").expect("defaulted by clap"));

    let m = cmd
        .clone()
        .try_get_matches_from(["opt", "--global"])
        .unwrap();
    assert!(m.subcommand_name().is_none());
    assert!(*m.get_one::<bool>("global").expect("defaulted by clap"));

    let m = cmd.try_get_matches_from(["opt", "global"]).unwrap();
    assert_eq!(m.subcommand_name().unwrap(), "global");
    assert!(!*m.get_one::<bool>("global").expect("defaulted by clap"));
}

#[test]
#[cfg(feature = "error-context")]
fn subcommand_not_recognized() {
    let cmd = Command::new("fake")
        .subcommand(Command::new("sub"))
        .disable_help_subcommand(true)
        .infer_subcommands(true);
    utils::assert_output(
        cmd,
        "fake help",
        "error: unrecognized subcommand 'help'

Usage: fake [COMMAND]

For more information, try '--help'.
",
        true,
    );
}

#[test]
fn busybox_like_multicall() {
    fn applet_commands() -> [Command; 2] {
        [Command::new("true"), Command::new("false")]
    }
    let cmd = Command::new("busybox")
        .multicall(true)
        .subcommand(Command::new("busybox").subcommands(applet_commands()))
        .subcommands(applet_commands());

    let m = cmd
        .clone()
        .try_get_matches_from(["busybox", "true"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("busybox"));
    assert_eq!(m.subcommand().unwrap().1.subcommand_name(), Some("true"));

    let m = cmd.clone().try_get_matches_from(["true"]).unwrap();
    assert_eq!(m.subcommand_name(), Some("true"));

    let m = cmd.clone().try_get_matches_from(["a.out"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidSubcommand);
}

#[test]
fn hostname_like_multicall() {
    let mut cmd = Command::new("hostname")
        .multicall(true)
        .subcommand(Command::new("hostname"))
        .subcommand(Command::new("dnsdomainname"));

    let m = cmd.clone().try_get_matches_from(["hostname"]).unwrap();
    assert_eq!(m.subcommand_name(), Some("hostname"));

    let m = cmd.clone().try_get_matches_from(["dnsdomainname"]).unwrap();
    assert_eq!(m.subcommand_name(), Some("dnsdomainname"));

    let m = cmd.clone().try_get_matches_from(["a.out"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::InvalidSubcommand);

    let m = cmd.try_get_matches_from_mut(["hostname", "hostname"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnknownArgument);

    let m = cmd.try_get_matches_from(["hostname", "dnsdomainname"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
#[cfg(feature = "error-context")]
fn bad_multicall_command_error() {
    let cmd = Command::new("repl")
        .version("1.0.0")
        .propagate_version(true)
        .multicall(true)
        .subcommand(Command::new("foo"))
        .subcommand(Command::new("bar"));

    let err = cmd.clone().try_get_matches_from(["world"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidSubcommand);
    static HELLO_EXPECTED: &str = "\
error: unrecognized subcommand 'world'

Usage: <COMMAND>

For more information, try 'help'.
";
    utils::assert_eq(HELLO_EXPECTED, err.to_string());

    #[cfg(feature = "suggestions")]
    {
        let err = cmd.clone().try_get_matches_from(["baz"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidSubcommand);
        static BAZ_EXPECTED: &str = "\
error: unrecognized subcommand 'baz'

  note: subcommand 'bar' exists
  note: to pass 'baz' as a value, use ' -- baz'

Usage: <COMMAND>

For more information, try 'help'.
";
        utils::assert_eq(BAZ_EXPECTED, err.to_string());
    }

    // Verify whatever we did to get the above to work didn't disable `--help` and `--version`.

    let err = cmd
        .clone()
        .try_get_matches_from(["foo", "--help"])
        .unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);

    let err = cmd
        .clone()
        .try_get_matches_from(["foo", "--version"])
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
Usage: foo bar [value]

Arguments:
  [value]  

Options:
  -h, --help     Print help
  -V, --version  Print version
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
Usage: foo bar [value]

Arguments:
  [value]  

Options:
  -h, --help     Print help
  -V, --version  Print version
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
Usage: foo bar [value]

Arguments:
  [value]  

Options:
  -h, --help     Print help
  -V, --version  Print version
";
    let mut cmd = Command::new("repl")
        .version("1.0.0")
        .propagate_version(true)
        .multicall(true)
        .subcommand(Command::new("foo").subcommand(Command::new("bar").arg(Arg::new("value"))));
    cmd.build();
    let subcmd = cmd.find_subcommand_mut("foo").unwrap();
    let subcmd = subcmd.find_subcommand_mut("bar").unwrap();

    let help = subcmd.render_help().to_string();
    utils::assert_eq(EXPECTED, help);
}

#[test]
#[should_panic = "Command test: command name `repeat` is duplicated"]
fn duplicate_subcommand() {
    Command::new("test")
        .subcommand(Command::new("repeat"))
        .subcommand(Command::new("repeat"))
        .build()
}

#[test]
#[should_panic = "Command test: command `unique` alias `repeat` is duplicated"]
fn duplicate_subcommand_alias() {
    Command::new("test")
        .subcommand(Command::new("repeat"))
        .subcommand(Command::new("unique").alias("repeat"))
        .build()
}
