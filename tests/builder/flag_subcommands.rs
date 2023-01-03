use super::utils;

use clap::{arg, error::ErrorKind, Arg, ArgAction, Command};

#[test]
fn flag_subcommand_normal() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some").short_flag('S').long_flag("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .help("testing testing")
                    .action(ArgAction::SetTrue),
            ),
        )
        .try_get_matches_from(vec!["myprog", "some", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(*sub_matches
        .get_one::<bool>("test")
        .expect("defaulted by clap"));
}

#[test]
fn flag_subcommand_normal_with_alias() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some")
                .short_flag('S')
                .long_flag("S")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing")
                        .action(ArgAction::SetTrue),
                )
                .alias("result"),
        )
        .try_get_matches_from(vec!["myprog", "result", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(*sub_matches
        .get_one::<bool>("test")
        .expect("defaulted by clap"));
}

#[test]
fn flag_subcommand_short() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some").short_flag('S').arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .help("testing testing")
                    .action(ArgAction::SetTrue),
            ),
        )
        .try_get_matches_from(vec!["myprog", "-S", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(*sub_matches
        .get_one::<bool>("test")
        .expect("defaulted by clap"));
}

#[test]
fn flag_subcommand_short_with_args() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some").short_flag('S').arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .help("testing testing")
                    .action(ArgAction::SetTrue),
            ),
        )
        .try_get_matches_from(vec!["myprog", "-St"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(*sub_matches
        .get_one::<bool>("test")
        .expect("defaulted by clap"));
}

#[test]
fn flag_subcommand_short_with_alias() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing")
                        .action(ArgAction::SetTrue),
                )
                .short_flag_alias('M')
                .short_flag_alias('B'),
        )
        .try_get_matches_from(vec!["myprog", "-Bt"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(*sub_matches
        .get_one::<bool>("test")
        .expect("defaulted by clap"));
}

#[test]
fn flag_subcommand_short_with_alias_same_as_short_flag() {
    let matches = Command::new("test")
        .subcommand(Command::new("some").short_flag('S').short_flag_alias('S'))
        .try_get_matches_from(vec!["myprog", "-S"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
}

#[test]
fn flag_subcommand_long_with_alias_same_as_long_flag() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some")
                .long_flag("sync")
                .long_flag_alias("sync"),
        )
        .try_get_matches_from(vec!["myprog", "--sync"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
}

#[test]
fn flag_subcommand_short_with_aliases_vis_and_hidden() {
    let cmd = Command::new("test").subcommand(
        Command::new("some")
            .short_flag('S')
            .arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .help("testing testing"),
            )
            .visible_short_flag_aliases(['M', 'B'])
            .short_flag_alias('C'),
    );
    let app1 = cmd.clone();
    let matches1 = app1.try_get_matches_from(vec!["test", "-M"]).unwrap();
    assert_eq!(matches1.subcommand_name().unwrap(), "some");

    let app2 = cmd.clone();
    let matches2 = app2.try_get_matches_from(vec!["test", "-C"]).unwrap();
    assert_eq!(matches2.subcommand_name().unwrap(), "some");

    let app3 = cmd.clone();
    let matches3 = app3.try_get_matches_from(vec!["test", "-B"]).unwrap();
    assert_eq!(matches3.subcommand_name().unwrap(), "some");
}

#[test]
fn flag_subcommand_short_with_aliases() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing")
                        .action(ArgAction::SetTrue),
                )
                .short_flag_aliases(['M', 'B']),
        )
        .try_get_matches_from(vec!["myprog", "-Bt"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(*sub_matches
        .get_one::<bool>("test")
        .expect("defaulted by clap"));
}

#[test]
#[should_panic]
fn flag_subcommand_short_with_alias_hyphen() {
    let _ = Command::new("test")
        .subcommand(
            Command::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing"),
                )
                .short_flag_alias('-'),
        )
        .try_get_matches_from(vec!["myprog", "-Bt"])
        .unwrap();
}

#[test]
#[should_panic]
fn flag_subcommand_short_with_aliases_hyphen() {
    let _ = Command::new("test")
        .subcommand(
            Command::new("some")
                .short_flag('S')
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing"),
                )
                .short_flag_aliases(['-', '-', '-']),
        )
        .try_get_matches_from(vec!["myprog", "-Bt"])
        .unwrap();
}

#[test]
fn flag_subcommand_short_after_long_arg() {
    let m = Command::new("pacman")
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .arg(Arg::new("clean").short('c').action(ArgAction::SetTrue)),
        )
        .arg(Arg::new("arg").long("arg").action(ArgAction::Set))
        .try_get_matches_from(vec!["pacman", "--arg", "foo", "-Sc"])
        .unwrap();
    let subm = m.subcommand_matches("sync");
    assert!(subm.is_some());
    let subm = subm.unwrap();
    assert!(*subm.get_one::<bool>("clean").expect("defaulted by clap"));
}

#[test]
fn flag_subcommand_long() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some").long_flag("some").arg(
                Arg::new("test")
                    .short('t')
                    .long("test")
                    .help("testing testing")
                    .action(ArgAction::SetTrue),
            ),
        )
        .try_get_matches_from(vec!["myprog", "--some", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(*sub_matches
        .get_one::<bool>("test")
        .expect("defaulted by clap"));
}

#[test]
fn flag_subcommand_long_with_alias() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some")
                .long_flag("some")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing")
                        .action(ArgAction::SetTrue),
                )
                .long_flag_alias("result"),
        )
        .try_get_matches_from(vec!["myprog", "--result", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(*sub_matches
        .get_one::<bool>("test")
        .expect("defaulted by clap"));
}

#[test]
fn flag_subcommand_long_with_aliases() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some")
                .long_flag("some")
                .arg(
                    Arg::new("test")
                        .short('t')
                        .long("test")
                        .help("testing testing")
                        .action(ArgAction::SetTrue),
                )
                .long_flag_aliases(["result", "someall"]),
        )
        .try_get_matches_from(vec!["myprog", "--result", "--test"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(*sub_matches
        .get_one::<bool>("test")
        .expect("defaulted by clap"));
}

#[test]
fn flag_subcommand_multiple() {
    let matches = Command::new("test")
        .subcommand(
            Command::new("some")
                .short_flag('S')
                .long_flag("some")
                .arg(arg!(-f --flag "some flag").action(ArgAction::SetTrue))
                .arg(arg!(-p --print "print something").action(ArgAction::SetTrue))
                .subcommand(
                    Command::new("result")
                        .short_flag('R')
                        .long_flag("result")
                        .arg(arg!(-f --flag "some flag").action(ArgAction::SetTrue))
                        .arg(arg!(-p --print "print something").action(ArgAction::SetTrue)),
                ),
        )
        .try_get_matches_from(vec!["myprog", "-SfpRfp"])
        .unwrap();
    assert_eq!(matches.subcommand_name().unwrap(), "some");
    let sub_matches = matches.subcommand_matches("some").unwrap();
    assert!(*sub_matches
        .get_one::<bool>("flag")
        .expect("defaulted by clap"));
    assert!(*sub_matches
        .get_one::<bool>("print")
        .expect("defaulted by clap"));
    assert_eq!(sub_matches.subcommand_name().unwrap(), "result");
    let result_matches = sub_matches.subcommand_matches("result").unwrap();
    assert!(*result_matches
        .get_one::<bool>("flag")
        .expect("defaulted by clap"));
    assert!(*result_matches
        .get_one::<bool>("print")
        .expect("defaulted by clap"));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'-f\' short flag for the \'test\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_short_conflict_with_arg() {
    let _ = Command::new("test")
        .subcommand(Command::new("some").short_flag('f').long_flag("some"))
        .arg(Arg::new("test").short('f'))
        .try_get_matches_from(vec!["myprog", "-f"])
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'-f\' short flag is specified for both \'some\' and \'result\' subcommands"]
fn flag_subcommand_short_conflict_with_alias() {
    let _ = Command::new("test")
        .subcommand(Command::new("some").short_flag('f').long_flag("some"))
        .subcommand(Command::new("result").short_flag('t').short_flag_alias('f'))
        .try_get_matches_from(vec!["myprog", "-f"])
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'--flag\' long flag is specified for both \'some\' and \'result\' subcommands"]
fn flag_subcommand_long_conflict_with_alias() {
    let _ = Command::new("test")
        .subcommand(Command::new("some").long_flag("flag"))
        .subcommand(
            Command::new("result")
                .long_flag("test")
                .long_flag_alias("flag"),
        )
        .try_get_matches_from(vec!["myprog", "--flag"])
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'-f\' short flag for the \'test\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_short_conflict_with_arg_alias() {
    let _ = Command::new("test")
        .subcommand(Command::new("some").short_flag('f').long_flag("some"))
        .arg(Arg::new("test").short('t').short_alias('f'))
        .try_get_matches_from(vec!["myprog", "-f"])
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'--some\' long flag for the \'test\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_long_conflict_with_arg_alias() {
    let _ = Command::new("test")
        .subcommand(Command::new("some").short_flag('f').long_flag("some"))
        .arg(Arg::new("test").long("test").alias("some"))
        .try_get_matches_from(vec!["myprog", "--some"])
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "the \'--flag\' long flag for the \'flag\' argument conflicts with the short flag for \'some\' subcommand"]
fn flag_subcommand_long_conflict_with_arg() {
    let _ = Command::new("test")
        .subcommand(Command::new("some").short_flag('a').long_flag("flag"))
        .arg(Arg::new("flag").long("flag"))
        .try_get_matches_from(vec!["myprog", "--flag"])
        .unwrap();
}

#[test]
#[should_panic = "the '--help' long flag for the 'help' argument conflicts with the short flag for 'help' subcommand"]
fn flag_subcommand_conflict_with_help() {
    let _ = Command::new("test")
        .subcommand(Command::new("help").short_flag('h').long_flag("help"))
        .try_get_matches_from(vec!["myprog", "--help"])
        .unwrap();
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "the '--version' long flag for the 'version' argument conflicts with the short flag for 'ver' subcommand"]
fn flag_subcommand_conflict_with_version() {
    let _ = Command::new("test")
        .version("1.0.0")
        .subcommand(Command::new("ver").short_flag('V').long_flag("version"))
        .try_get_matches_from(vec!["myprog", "--version"])
        .unwrap();
}

#[test]
fn flag_subcommand_long_infer_pass() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test").long_flag("test"))
        .try_get_matches_from(vec!["prog", "--te"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[cfg(not(feature = "suggestions"))]
#[test]
fn flag_subcommand_long_infer_fail() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test").long_flag("test"))
        .subcommand(Command::new("temp").long_flag("temp"))
        .try_get_matches_from(vec!["prog", "--te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[cfg(feature = "suggestions")]
#[test]
fn flag_subcommand_long_infer_fail() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test").long_flag("test"))
        .subcommand(Command::new("temp").long_flag("temp"))
        .try_get_matches_from(vec!["prog", "--te"]);
    assert!(m.is_err(), "{:#?}", m.unwrap());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn flag_subcommand_long_infer_pass_close() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test").long_flag("test"))
        .subcommand(Command::new("temp").long_flag("temp"))
        .try_get_matches_from(vec!["prog", "--tes"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

#[test]
fn flag_subcommand_long_infer_exact_match() {
    let m = Command::new("prog")
        .infer_subcommands(true)
        .subcommand(Command::new("test").long_flag("test"))
        .subcommand(Command::new("testa").long_flag("testa"))
        .subcommand(Command::new("testb").long_flag("testb"))
        .try_get_matches_from(vec!["prog", "--test"])
        .unwrap();
    assert_eq!(m.subcommand_name(), Some("test"));
}

static FLAG_SUBCOMMAND_HELP: &str = "\
Query the package database.

Usage: pacman {query|--query|-Q} [OPTIONS]

Options:
  -s, --search <search>...  search locally installed packages for matching strings
  -i, --info <info>...      view package information
  -h, --help                Print help
";

#[test]
fn flag_subcommand_long_short_normal_usage_string() {
    let cmd = Command::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .subcommand_required(true)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .help("view package information")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        );
    utils::assert_output(cmd, "pacman -Qh", FLAG_SUBCOMMAND_HELP, false);
}

static FLAG_SUBCOMMAND_NO_SHORT_HELP: &str = "\
Query the package database.

Usage: pacman {query|--query} [OPTIONS]

Options:
  -s, --search <search>...  search locally installed packages for matching strings
  -i, --info <info>...      view package information
  -h, --help                Print help
";

#[test]
fn flag_subcommand_long_normal_usage_string() {
    let cmd = Command::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .subcommand_required(true)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("query")
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .help("view package information")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        );
    utils::assert_output(
        cmd,
        "pacman query --help",
        FLAG_SUBCOMMAND_NO_SHORT_HELP,
        false,
    );
}

static FLAG_SUBCOMMAND_NO_LONG_HELP: &str = "\
Query the package database.

Usage: pacman {query|-Q} [OPTIONS]

Options:
  -s, --search <search>...  search locally installed packages for matching strings
  -i, --info <info>...      view package information
  -h, --help                Print help
";

#[test]
fn flag_subcommand_short_normal_usage_string() {
    let cmd = Command::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .subcommand_required(true)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("query")
                .short_flag('Q')
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .help("view package information")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        );
    utils::assert_output(
        cmd,
        "pacman query --help",
        FLAG_SUBCOMMAND_NO_LONG_HELP,
        false,
    );
}
