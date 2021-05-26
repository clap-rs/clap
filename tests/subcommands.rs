mod utils;

use clap::{App, Arg, ErrorKind};

static VISIBLE_ALIAS_HELP: &str = "clap-test 2.6

USAGE:
    clap-test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    Some help [aliases: dongle, done]";

static INVISIBLE_ALIAS_HELP: &str = "clap-test 2.6

USAGE:
    clap-test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    Some help";

static SUBCMD_ALPHA_ORDER: &str = "test 1

USAGE:
    test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    a1      blah a1
    b1      blah b1
    help    Prints this message or the help of the given subcommand(s)";

static SUBCMD_DECL_ORDER: &str = "test 1

USAGE:
    test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    b1      blah b1
    a1      blah a1
    help    Prints this message or the help of the given subcommand(s)";

#[cfg(feature = "suggestions")]
static DYM_SUBCMD: &str = "error: The subcommand 'subcm' wasn't recognized

	Did you mean 'subcmd'?

If you believe you received this message in error, try re-running with 'dym -- subcm'

USAGE:
    dym [SUBCOMMAND]

For more information try --help";

#[cfg(feature = "suggestions")]
static DYM_SUBCMD_AMBIGUOUS: &str = "error: The subcommand 'te' wasn't recognized

	Did you mean 'test' or 'temp'?

If you believe you received this message in error, try re-running with 'dym -- te'

USAGE:
    dym [SUBCOMMAND]

For more information try --help";

static SUBCMD_AFTER_DOUBLE_DASH: &str =
    "error: Found argument 'subcmd' which wasn't expected, or isn't valid in this context

\tIf you tried to supply `subcmd` as a subcommand, remove the '--' before it.

USAGE:
    app [SUBCOMMAND]

For more information try --help";

#[test]
fn subcommand() {
    let m = App::new("test")
        .subcommand(
            App::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .takes_value(true)
                    .about("testing testing"),
            ),
        )
        .arg(Arg::new("other").long("other"))
        .get_matches_from(vec!["myprog", "some", "--test", "testing"]);

    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "testing");
}

#[test]
fn subcommand_none_given() {
    let m = App::new("test")
        .subcommand(
            App::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .takes_value(true)
                    .about("testing testing"),
            ),
        )
        .arg(Arg::new("other").long("other"))
        .get_matches_from(vec![""]);

    assert!(m.subcommand_name().is_none());
}

#[test]
fn subcommand_multiple() {
    let m = App::new("test")
        .subcommands(vec![
            App::new("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .takes_value(true)
                    .about("testing testing"),
            ),
            App::new("add").arg(Arg::new("roster").short('r')),
        ])
        .arg(Arg::new("other").long("other"))
        .get_matches_from(vec!["myprog", "some", "--test", "testing"]);

    assert!(m.subcommand_matches("some").is_some());
    assert!(m.subcommand_matches("add").is_none());
    assert_eq!(m.subcommand_name().unwrap(), "some");
    let sub_m = m.subcommand_matches("some").unwrap();
    assert!(sub_m.is_present("test"));
    assert_eq!(sub_m.value_of("test").unwrap(), "testing");
}

#[test]
fn subcommand_display_order() {
    let app_subcmd_alpha_order = App::new("test").version("1").subcommands(vec![
        App::new("b1")
            .about("blah b1")
            .arg(Arg::new("test").short('t')),
        App::new("a1")
            .about("blah a1")
            .arg(Arg::new("roster").short('r')),
    ]);

    assert!(utils::compare_output(
        app_subcmd_alpha_order,
        "test --help",
        SUBCMD_ALPHA_ORDER,
        false,
    ));

    let app_subcmd_decl_order = App::new("test")
        .version("1")
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .subcommands(vec![
            App::new("b1")
                .about("blah b1")
                .arg(Arg::new("test").short('t')),
            App::new("a1")
                .about("blah a1")
                .arg(Arg::new("roster").short('r')),
        ]);

    assert!(utils::compare_output(
        app_subcmd_decl_order,
        "test --help",
        SUBCMD_DECL_ORDER,
        false,
    ));
}

#[test]
fn single_alias() {
    let m = App::new("myprog")
        .subcommand(App::new("test").alias("do-stuff"))
        .get_matches_from(vec!["myprog", "do-stuff"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn multiple_aliases() {
    let m = App::new("myprog")
        .subcommand(App::new("test").aliases(&["do-stuff", "test-stuff"]))
        .get_matches_from(vec!["myprog", "test-stuff"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
#[cfg(feature = "suggestions")]
fn subcmd_did_you_mean_output() {
    let app = App::new("dym").subcommand(App::new("subcmd"));
    assert!(utils::compare_output(app, "dym subcm", DYM_SUBCMD, true));
}

#[test]
#[cfg(feature = "suggestions")]
fn subcmd_did_you_mean_output_ambiguous() {
    let app = App::new("dym")
        .subcommand(App::new("test"))
        .subcommand(App::new("temp"));
    assert!(utils::compare_output(
        app,
        "dym te",
        DYM_SUBCMD_AMBIGUOUS,
        true
    ));
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

For more information try --help";

    let app = App::new("dym")
        .subcommand(App::new("subcmd").arg(Arg::from("-s --subcmdarg [subcmdarg] 'tests'")));

    assert!(utils::compare_output(
        app,
        "dym --subcmarg subcmd",
        EXPECTED,
        true
    ));
}

#[test]
#[cfg(feature = "suggestions")]
fn subcmd_did_you_mean_output_arg_false_positives() {
    static EXPECTED: &str =
        "error: Found argument '--subcmarg' which wasn't expected, or isn't valid in this context

\tIf you tried to supply `--subcmarg` as a value rather than a flag, use `-- --subcmarg`

USAGE:
    dym [SUBCOMMAND]

For more information try --help";

    let app = App::new("dym")
        .subcommand(App::new("subcmd").arg(Arg::from("-s --subcmdarg [subcmdarg] 'tests'")));

    assert!(utils::compare_output(
        app,
        "dym --subcmarg foo",
        EXPECTED,
        true
    ));
}

#[test]
fn alias_help() {
    let m = App::new("myprog")
        .subcommand(App::new("test").alias("do-stuff"))
        .try_get_matches_from(vec!["myprog", "help", "do-stuff"]);
    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::DisplayHelp);
}

#[test]
fn visible_aliases_help_output() {
    let app = App::new("clap-test").version("2.6").subcommand(
        App::new("test")
            .about("Some help")
            .alias("invisible")
            .visible_alias("dongle")
            .visible_alias("done"),
    );
    assert!(utils::compare_output(
        app,
        "clap-test --help",
        VISIBLE_ALIAS_HELP,
        false
    ));
}

#[test]
fn invisible_aliases_help_output() {
    let app = App::new("clap-test")
        .version("2.6")
        .subcommand(App::new("test").about("Some help").alias("invisible"));
    assert!(utils::compare_output(
        app,
        "clap-test --help",
        INVISIBLE_ALIAS_HELP,
        false
    ));
}

#[test]
fn replace() {
    let m = App::new("prog")
        .subcommand(App::new("module").subcommand(App::new("install").about("Install module")))
        .replace("install", &["module", "install"])
        .get_matches_from(vec!["prog", "install"]);

    assert_eq!(m.subcommand_name(), Some("module"));
    assert_eq!(
        m.subcommand_matches("module").unwrap().subcommand_name(),
        Some("install")
    );
}

#[test]
fn issue_1031_args_with_same_name() {
    let res = App::new("prog")
        .arg(Arg::from("--ui-path=<PATH>"))
        .subcommand(App::new("signer"))
        .try_get_matches_from(vec!["prog", "--ui-path", "signer"]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.value_of("ui-path"), Some("signer"));
}

#[test]
fn issue_1031_args_with_same_name_no_more_vals() {
    let res = App::new("prog")
        .arg(Arg::from("--ui-path=<PATH>"))
        .subcommand(App::new("signer"))
        .try_get_matches_from(vec!["prog", "--ui-path", "value", "signer"]);

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);
    let m = res.unwrap();
    assert_eq!(m.value_of("ui-path"), Some("value"));
    assert_eq!(m.subcommand_name(), Some("signer"));
}

#[test]
fn issue_1161_multiple_hyphen_hyphen() {
    // from example 22
    let res = App::new("myprog")
        .arg(Arg::new("eff").short('f'))
        .arg(Arg::new("pea").short('p').takes_value(true))
        .arg(Arg::new("slop").takes_value(true).multiple(true).last(true))
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

    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);
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
    let actual = m.values_of("slop").map(|vals| vals.collect::<Vec<_>>());

    assert_eq!(expected, actual);
}

#[test]
fn issue_1722_not_emit_error_when_arg_follows_similar_to_a_subcommand() {
    let m = App::new("myprog")
        .subcommand(App::new("subcommand"))
        .arg(Arg::new("argument"))
        .try_get_matches_from(vec!["myprog", "--", "subcommand"]);
    assert_eq!(m.unwrap().value_of("argument"), Some("subcommand"));
}

#[test]
fn subcommand_placeholder_test() {
    let mut app = App::new("myprog")
        .subcommand(App::new("subcommand"))
        .subcommand_placeholder("TEST_PLACEHOLDER", "TEST_HEADER");

    assert_eq!(
        &app.generate_usage(),
        "USAGE:\n    myprog [TEST_PLACEHOLDER]"
    );

    let mut help_text = Vec::new();
    app.write_help(&mut help_text)
        .expect("Failed to write to internal buffer");

    assert!(String::from_utf8(help_text)
        .unwrap()
        .contains("TEST_HEADER:"));
}

#[test]
fn subcommand_used_after_double_dash() {
    let app = App::new("app").subcommand(App::new("subcmd"));

    assert!(utils::compare_output(
        app,
        "app -- subcmd",
        SUBCMD_AFTER_DOUBLE_DASH,
        true
    ));
}

#[test]
fn subcommand_after_argument() {
    let m = App::new("myprog")
        .arg(Arg::new("some_text"))
        .subcommand(App::new("test"))
        .get_matches_from(vec!["myprog", "teat", "test"]);
    assert_eq!(m.value_of("some_text"), Some("teat"));
    assert_eq!(m.subcommand().unwrap().0, "test");
}

#[test]
fn subcommand_after_argument_looks_like_help() {
    let m = App::new("myprog")
        .arg(Arg::new("some_text"))
        .subcommand(App::new("test"))
        .get_matches_from(vec!["myprog", "helt", "test"]);
    assert_eq!(m.value_of("some_text"), Some("helt"));
    assert_eq!(m.subcommand().unwrap().0, "test");
}

#[test]
fn issue_2494_subcommand_is_present() {
    let app = App::new("opt")
        .arg(Arg::new("global").long("global"))
        .subcommand(App::new("global"));

    let m = app.clone().get_matches_from(&["opt", "--global", "global"]);
    assert_eq!(m.subcommand_name().unwrap(), "global");
    assert!(m.is_present("global"));

    let m = app.clone().get_matches_from(&["opt", "--global"]);
    assert!(m.subcommand_name().is_none());
    assert!(m.is_present("global"));

    let m = app.get_matches_from(&["opt", "global"]);
    assert_eq!(m.subcommand_name().unwrap(), "global");
    assert!(!m.is_present("global"));
}
