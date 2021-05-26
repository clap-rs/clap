mod utils;

use clap::{App, AppSettings, Arg, ErrorKind};

#[test]
fn flag_subcommand_normal() {
    let matches = App::new("test")
        .subcommand(
            App::new("some").short_flag('S').long_flag("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .about("testing testing"),
            ),
        )
        .get_matches_from(vec!["myprog", "some", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_normal_with_alias() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .long_flag("S")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .alias("result"),
        )
        .get_matches_from(vec!["myprog", "result", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short() {
    let matches = App::new("test")
        .subcommand(
            App::new("some").short_flag('S').arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .about("testing testing"),
            ),
        )
        .get_matches_from(vec!["myprog", "-S", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short_with_args() {
    let matches = App::new("test")
        .subcommand(
            App::new("some").short_flag('S').arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .about("testing testing"),
            ),
        )
        .get_matches_from(vec!["myprog", "-St"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short_with_alias() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .short_flag_alias('M')
                .short_flag_alias('B'),
        )
        .get_matches_from(vec!["myprog", "-Bt"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_short_with_alias_same_as_short_flag() {
    let matches = App::new("test")
        .subcommand(App::new("some").short_flag('S').short_flag_alias('S'))
        .get_matches_from(vec!["myprog", "-S"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
}

#[test]
fn flag_subcommand_long_with_alias_same_as_long_flag() {
    let matches = App::new("test")
        .subcommand(App::new("some").long_flag("sync").long_flag_alias("sync"))
        .get_matches_from(vec!["myprog", "--sync"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
}

#[test]
fn flag_subcommand_short_with_aliases_vis_and_hidden() {
    let app = App::new("test").subcommand(
        App::new("some")
            .short_flag('S')
            .arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .about("testing testing"),
            )
            .visible_short_flag_aliases(&['M', 'B'])
            .short_flag_alias('C'),
    );
    let app1 = app.clone();
    let matches1 = app1.get_matches_from(vec!["test", "-M"]);
    assert_eq!(matches1.subcommand_name().unwrap(), "some");

    let app2 = app.clone();
    let matches2 = app2.get_matches_from(vec!["test", "-C"]);
    assert_eq!(matches2.subcommand_name().unwrap(), "some");

    let app3 = app.clone();
    let matches3 = app3.get_matches_from(vec!["test", "-B"]);
    assert_eq!(matches3.subcommand_name().unwrap(), "some");
}

#[test]
fn flag_subcommand_short_with_aliases() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .short_flag_aliases(&['M', 'B']),
        )
        .get_matches_from(vec!["myprog", "-Bt"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
#[should_panic]
fn flag_subcommand_short_with_alias_hyphen() {
    let _ = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .short_flag_alias('-'),
        )
        .get_matches_from(vec!["myprog", "-Bt"]);
}

#[test]
#[should_panic]
fn flag_subcommand_short_with_aliases_hyphen() {
    let _ = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .short_flag_aliases(&['-', '-', '-']),
        )
        .get_matches_from(vec!["myprog", "-Bt"]);
}

#[test]
fn flag_subcommand_short_after_long_arg_with_val() {
    let m = App::new("pacman")
        .subcommand(
            App::new("sync")
                .short_flag('S')
                .arg(Arg::new("info").short('i'))
                .arg(Arg::new("package").multiple(true).takes_value(true)),
        )
        .arg(
            Arg::new("arg")
                .long("arg")
                .takes_value(true)
                .multiple(false)
                .global(true),
        )
        .get_matches_from(vec!["pacman", "--arg", "foo", "-Si"]);
    let subm = m.subcommand_matches("sync");
    assert!(subm.is_some());
    let subm = subm.unwrap();
    assert!(subm.is_present("info"));
}

#[test]
fn flag_subcommand_long() {
    let matches = App::new("test")
        .subcommand(
            App::new("some").long_flag("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .about("testing testing"),
            ),
        )
        .get_matches_from(vec!["myprog", "--some", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_long_with_alias() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .long_flag("some")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .long_flag_alias("result"),
        )
        .get_matches_from(vec!["myprog", "--result", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_long_with_aliases() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .long_flag("some")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .about("testing testing"),
                )
                .long_flag_aliases(&["result", "someall"]),
        )
        .get_matches_from(vec!["myprog", "--result", "--test"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("test"));
}

#[test]
fn flag_subcommand_multiple() {
    let matches = App::new("test")
        .subcommand(
            App::new("some")
                .short_flag('S')
                .long_flag("some")
                .arg(Arg::from("-f, --flag 'some flag'"))
                .arg(Arg::from("-p, --print 'print something'"))
                .subcommand(
                    App::new("result")
                        .short_flag('R')
                        .long_flag("result")
                        .arg(Arg::from("-f, --flag 'some flag'"))
                        .arg(Arg::from("-p, --print 'print something'")),
                ),
        )
        .get_matches_from(vec!["myprog", "-SfpRfp"]);
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(sub_matches.is_present("flag"));
    assert!(sub_matches.is_present("print"));
    assert_eq!(sub_matches.subcommand_name().unwrap(), "result");
    let result_matches = sub_matches.subcommand_matches("result").unwrap();
    assert!(result_matches.is_present("flag"));
    assert!(result_matches.is_present("print"));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'-f\' short flag for the \'test\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_short_conflict_with_arg() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('f').long_flag("some"))
        .arg(Arg::new("test").short('f'))
        .get_matches_from(vec!["myprog", "-f"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'-f\' short flag is specified for both \'some\' and \'result\' subcommands"]
fn flag_subcommand_short_conflict_with_alias() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('f').long_flag("some"))
        .subcommand(App::new("result").short_flag('t').short_flag_alias('f'))
        .get_matches_from(vec!["myprog", "-f"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'--flag\' long flag is specified for both \'some\' and \'result\' subcommands"]
fn flag_subcommand_long_conflict_with_alias() {
    let _ = App::new("test")
        .subcommand(App::new("some").long_flag("flag"))
        .subcommand(App::new("result").long_flag("test").long_flag_alias("flag"))
        .get_matches_from(vec!["myprog", "--flag"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'-f\' short flag for the \'test\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_short_conflict_with_arg_alias() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('f').long_flag("some"))
        .arg(Arg::new("test").short('t').short_alias('f'))
        .get_matches_from(vec!["myprog", "-f"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'--some\' long flag for the \'test\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_long_conflict_with_arg_alias() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('f').long_flag("some"))
        .arg(Arg::new("test").long("test").alias("some"))
        .get_matches_from(vec!["myprog", "--some"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'--flag\' long flag for the \'flag\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_long_conflict_with_arg() {
    let _ = App::new("test")
        .subcommand(App::new("some").short_flag('a').long_flag("flag"))
        .arg(Arg::new("flag").long("flag"))
        .get_matches_from(vec!["myprog", "--flag"]);
}

#[test]
fn flag_subcommand_conflict_with_help() {
    let _ = App::new("test")
        .subcommand(App::new("help").short_flag('h').long_flag("help"))
        .get_matches_from(vec!["myprog", "--help"]);
}

#[test]
fn flag_subcommand_conflict_with_version() {
    let _ = App::new("test")
        .subcommand(App::new("ver").short_flag('V').long_flag("version"))
        .get_matches_from(vec!["myprog", "--version"]);
}

#[test]
fn flag_subcommand_long_infer_pass() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test").long_flag("test"))
        .get_matches_from(vec!["prog", "--te"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn flag_subcommand_long_infer_fail() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test").long_flag("test"))
        .subcommand(App::new("temp").long_flag("temp"))
        .try_get_matches_from(vec!["prog", "--te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[cfg(feature = "suggestions")]
#[test]
fn flag_subcommand_long_infer_fail() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test").long_flag("test"))
        .subcommand(App::new("temp").long_flag("temp"))
        .try_get_matches_from(vec!["prog", "--te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn flag_subcommand_long_infer_pass_close() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test").long_flag("test"))
        .subcommand(App::new("temp").long_flag("temp"))
        .get_matches_from(vec!["prog", "--tes"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn flag_subcommand_long_infer_exact_match() {
    let m = App::new("prog")
        .setting(AppSettings::InferSubcommands)
        .subcommand(App::new("test").long_flag("test"))
        .subcommand(App::new("testa").long_flag("testa"))
        .subcommand(App::new("testb").long_flag("testb"))
        .get_matches_from(vec!["prog", "--test"]);
    assert_eq!(m.subcommand_name(), Some("test"));
}

static FLAG_SUBCOMMAND_HELP: &str = "pacman-query 

Query the package database.

USAGE:
    pacman {query, --query, -Q} [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --info <info>...        view package information
    -s, --search <search>...    search locally installed packages for matching strings";

#[test]
fn flag_subcommand_long_short_normal_usage_string() {
    let app = App::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            App::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .about("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .about("view package information")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        );
    assert!(utils::compare_output(
        app,
        "pacman -Qh",
        FLAG_SUBCOMMAND_HELP,
        false
    ));
}

static FLAG_SUBCOMMAND_NO_SHORT_HELP: &str = "pacman-query 

Query the package database.

USAGE:
    pacman {query, --query} [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --info <info>...        view package information
    -s, --search <search>...    search locally installed packages for matching strings";

#[test]
fn flag_subcommand_long_normal_usage_string() {
    let app = App::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            App::new("query")
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .about("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .about("view package information")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        );
    assert!(utils::compare_output(
        app,
        "pacman query --help",
        FLAG_SUBCOMMAND_NO_SHORT_HELP,
        false
    ));
}

static FLAG_SUBCOMMAND_NO_LONG_HELP: &str = "pacman-query 

Query the package database.

USAGE:
    pacman {query, -Q} [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --info <info>...        view package information
    -s, --search <search>...    search locally installed packages for matching strings";

#[test]
fn flag_subcommand_short_normal_usage_string() {
    let app = App::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            App::new("query")
                .short_flag('Q')
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .about("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .about("view package information")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        );
    assert!(utils::compare_output(
        app,
        "pacman query --help",
        FLAG_SUBCOMMAND_NO_LONG_HELP,
        false
    ));
}
