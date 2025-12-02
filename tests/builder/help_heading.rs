#![cfg(feature = "help")]

use clap::{Arg, Command};

use super::utils;

fn setup() -> Command {
    Command::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
}

fn empty_args() -> impl IntoIterator<Item = String> {
    std::iter::empty()
}

#[test]
fn test_multiple_commands_mixed_standard_and_custom_headers() {
    static VISIBLE_ALIAS_HELP: &str = "\
Usage: clap-test [COMMAND]

Help Section:
  help      Print this message or the help of the given subcommand(s)

Commands:
  def_cmd1  First command under default command heading
  def_cmd4  Fourth command under default command heading
  def_cmd3  Third command under default command heading
  def_cmd2  Second command under default command heading

First Custom:
  ch1_cmd1  First command under first custom command heading
  ch1_cmd2  Second command under first custom command heading
  ch1_cmd4  Fourth command under first custom command heading
  ch1_cmd3  Third command under first custom command heading

Second Custom:
  ch2_cmd3  Third command under second custom command heading
  ch2_cmd4  Fourth command under second custom command heading
  ch2_cmd2  Second command under second custom command heading
  ch2_cmd1  First command under second custom command heading

Options:
  -h, --help     Print help
  -V, --version  Print version
";
    
        let cmd = Command::new("clap-test")
            .version("2.6")
            .subcommand_help_heading("Help Section")
            .subcommand(
                Command::new("def_cmd1")
                    .about("First command under default command heading")
                    .help_heading("Commands"),
            )
            .subcommand(
                Command::new("def_cmd4")
                    .about("Fourth command under default command heading")
                    .help_heading("Commands"),
            )
            .subcommand(
                Command::new("def_cmd3")
                .about("Third command under default command heading")
                .help_heading("Commands"),
            )
            .subcommand(
                Command::new("def_cmd2")
                    .about("Second command under default command heading")
                    .help_heading("Commands"),
            )
            .subcommand(
                Command::new("ch1_cmd1")
                    .about("First command under first custom command heading")
                    .help_heading("First Custom"),
            )
            .subcommand(
                Command::new("ch2_cmd3")
                    .about("Third command under second custom command heading")
                    .help_heading("Second Custom"),
            )
            .subcommand(
                Command::new("ch1_cmd2")
                    .about("Second command under first custom command heading")
                    .help_heading("First Custom"),
            )
            .subcommand(
                Command::new("ch1_cmd4")
                .about("Fourth command under first custom command heading")
                .help_heading("First Custom"),
            )
            .subcommand(
                Command::new("ch2_cmd4")
                .about("Fourth command under second custom command heading")
                .help_heading("Second Custom"),
            )
            .subcommand(
                Command::new("ch2_cmd2")
                .about("Second command under second custom command heading")
                .help_heading("Second Custom"),
            )
            .subcommand(
                Command::new("ch2_cmd1")
                .about("First command under second custom command heading")
                .help_heading("Second Custom"),
            )
            .subcommand(
                Command::new("ch1_cmd3")
                    .about("Third command under first custom command heading")
                    .help_heading("First Custom"),
            );
    
        utils::assert_output(cmd, "clap-test --help", VISIBLE_ALIAS_HELP, false);
    }

#[test]
fn test_multiple_commands_mixed_headings_flatten() {
    static VISIBLE_ALIAS_HELP: &str = "\
Usage: clap-test
       clap-test def_cmd1
       clap-test def_cmd2
       clap-test cust_cmd1
       clap-test cust_cmd2 --child <child>
       clap-test other_cmd1
       clap-test other_cmd2 --child <child>
       clap-test help [COMMAND]...

Options:
  -h, --help     Print help
  -V, --version  Print version

clap-test def_cmd1:
Def_cmd1 under default command heading
  -h, --help  Print help

clap-test def_cmd2:
Def_cmd2 under default command heading
  -h, --help  Print help

clap-test cust_cmd1:
Cust_cmd1 under custom command heading
  -h, --help  Print help

clap-test cust_cmd2:
Cust_cmd2 under custom command heading
      --child <child>  child help
  -h, --help           Print help

clap-test other_cmd1:
Other_cmd1 under other command heading
  -h, --help  Print help

clap-test other_cmd2:
Other_cmd2 under other command heading
      --child <child>  
  -h, --help           Print help

clap-test help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)
";
    
    let cmd = Command::new("clap-test")
        .version("2.6")
        .flatten_help(true)
        .subcommand(
            Command::new("def_cmd1")
                .about("Def_cmd1 under default command heading")
        )
        .subcommand(
            Command::new("cust_cmd1")
            .about("Cust_cmd1 under custom command heading")
            .help_heading("Custom Heading"),
        )
        .subcommand(
            Command::new("other_cmd1")
            .about("Other_cmd1 under other command heading")
            .help_heading("Other Heading"),
        )
        .subcommand(
            Command::new("other_cmd2")
            .about("Other_cmd2 under other command heading")
            .help_heading("Other Heading")
            .arg(Arg::new("child").long("child").required(true)),
        )
        .subcommand(
            Command::new("def_cmd2")
                .about("Def_cmd2 under default command heading")
        )
        .subcommand(
            Command::new("cust_cmd2")
                .about("Cust_cmd2 under custom command heading")
                .help_heading("Custom Heading")
                .arg(Arg::new("child").long("child").required(true).help("child help")),
        );

    utils::assert_output(cmd, "clap-test --help", VISIBLE_ALIAS_HELP, false);
}


#[test]
fn test_help_header_hide_commands_header() {
    static VISIBLE_ALIAS_HELP: &str = "\
Usage: clap-test [COMMAND]

Test commands:
  test  Some help

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let cmd = Command::new("clap-test")
        .version("2.6")
        .disable_help_subcommand(true)
        .subcommand_help_heading("Test commands")
        .subcommand(Command::new("test").about("Some help"));
    utils::assert_output(cmd, "clap-test --help", VISIBLE_ALIAS_HELP, false);
}



mod adjusted_help{


// This file is created from a copy of help.rs
// Any test not containing a subcommand is discarded
// Any test with subcommand has help_heading added

use clap::{arg, error::ErrorKind, Arg, ArgAction, Command};
use snapbox::assert_data_eq;
use snapbox::str;

use super::utils;
use super::setup;
use super::empty_args;


#[test]
fn help_subcommand() {
    let m = setup()
        .subcommand(
            Command::new("test")
                .help_heading("Custom Heading")
                .about("tests things")
                .arg(arg!(-v --verbose "with verbosity")),
        )
        .try_get_matches_from(vec!["myprog", "help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

#[test]
#[cfg(feature = "error-context")]
fn help_multi_subcommand_error() {
    let cmd = Command::new("ctest").subcommand(
        Command::new("subcmd").subcommand(
            Command::new("multi")
                .help_heading("Custom Heading")
                .about("tests subcommands")
                .author("Kevin K. <kbknapp@gmail.com>")
                .version("0.1")
                .arg(arg!(
                    -f --flag                    "tests flags"
                ))
                .arg(
                    arg!(
                        -o --option <scoption>    "tests options"
                    )
                    .required(false)
                    .num_args(1..)
                    .action(ArgAction::Append),
                ),
        ),
    );
    let err = cmd
        .try_get_matches_from(["ctest", "help", "subcmd", "multi", "foo"])
        .unwrap_err();

    assert_data_eq!(
        err.to_string(),
        str![[r#"
error: unrecognized subcommand 'foo'

Usage: ctest subcmd multi [OPTIONS]

For more information, try '--help'.

"#]]
    );
}

#[test]
fn subcommand_short_help() {
    let m = utils::complex_app_with_help_heading().try_get_matches_from(vec!["clap-test", "subcmd", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

#[test]
fn subcommand_long_help() {
    let m = utils::complex_app_with_help_heading().try_get_matches_from(vec!["clap-test", "subcmd", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

#[test]
fn subcommand_help_rev() {
    let m = utils::complex_app_with_help_heading().try_get_matches_from(vec!["clap-test", "help", "subcmd"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

// Updated to include the custom heading in the expected output
#[test]
fn complex_help_output() {
    let expected = str![[r#"
clap-test v1.4.8
Kevin K. <kbknapp@gmail.com>
tests clap library

Usage: clap-test [OPTIONS] [positional] [positional2] [positional3]... [COMMAND]

Commands:
  help    Print this message or the help of the given subcommand(s)

Custom Heading:
  subcmd  tests subcommands

Arguments:
  [positional]      tests positionals
  [positional2]     tests positionals with exclusions
  [positional3]...  tests specific values [possible values: vi, emacs]

Options:
  -o, --option <opt>...                  tests options
  -f, --flag...                          tests flags
  -F                                     tests flags with exclusions
      --long-option-2 <option2>          tests long options with exclusions
  -O, --option3 <option3>                specific vals [possible values: fast, slow]
      --multvals <one> <two>             Tests multiple values, not mult occs
      --multvalsmo <one> <two>           Tests multiple values, and mult occs
      --minvals2 <minvals> <minvals>...  Tests 2 min vals
      --maxvals3 <maxvals>...            Tests 3 max vals
      --optvaleq[=<optval>]              Tests optional value, require = sign
      --optvalnoeq [<optval>]            Tests optional value
  -h, --help                             Print help
  -V, --version                          Print version

"#]];
    utils::assert_output(utils::complex_app_with_help_heading(), "clap-test --help", expected, false);
}

#[test]
fn multi_level_sc_help() {
    let cmd = Command::new("ctest").subcommand(
        Command::new("subcmd")
            .help_heading("First Heading")
            .subcommand(
                Command::new("multi")
                    .about("tests subcommands")
                    .help_heading("Second Heading")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .version("0.1")
                    .arg(arg!(
                        -f --flag                    "tests flags"
                    ))
                    .arg(
                        arg!(
                            -o --option <scoption>    "tests options"
                        )
                        .required(false)
                        .num_args(1..)
                        .action(ArgAction::Append),
                    ),
            ),
    );

    let expected = str![[r#"
tests subcommands

Usage: ctest subcmd multi [OPTIONS]

Options:
  -f, --flag                  tests flags
  -o, --option <scoption>...  tests options
  -h, --help                  Print help
  -V, --version               Print version

"#]];
    utils::assert_output(cmd, "ctest help subcmd multi", expected, false);
}

#[test]
fn no_wrap_default_help() {
    let cmd = Command::new("ctest").version("1.0").term_width(0);

    let expected = str![[r#"
Usage: ctest

Options:
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}


#[test]
fn try_help_subcommand_default() {
    let cmd = Command::new("ctest")
        .version("1.0")
        .subcommand(Command::new("foo").help_heading("Custom Heading"))
        .term_width(0);
    
    let expected = str![[r#"
error: unrecognized subcommand 'bar'

Usage: ctest [COMMAND]

For more information, try '--help'.

"#]];
    utils::assert_output(cmd, "ctest bar", expected, true);
}

#[test]
fn try_help_subcommand_custom_flag() {
    let cmd = Command::new("ctest")
    .version("1.0")
    .disable_help_flag(true)
    .arg(
        Arg::new("help")
        .long("help")
        .short('h')
        .action(ArgAction::Help)
        .global(true),
    )
    .subcommand(Command::new("foo").help_heading("Custom Heading"))
        .term_width(0);

    let expected = str![[r#"
error: unrecognized subcommand 'bar'

Usage: ctest [COMMAND]

For more information, try '--help'.

"#]];
    utils::assert_output(cmd, "ctest bar", expected, true);
}

#[test]
fn try_help_subcommand_custom_flag_no_action() {
    let cmd = Command::new("ctest")
        .version("1.0")
        .disable_help_flag(true)
        // Note `ArgAction::Help` is excluded
        .arg(Arg::new("help").long("help").global(true))
        .subcommand(Command::new("foo").help_heading("Custom Heading"))
        .term_width(0);

    let expected = str![[r#"
error: unrecognized subcommand 'bar'

Usage: ctest [COMMAND]

For more information, try 'help'.

"#]];
    utils::assert_output(cmd, "ctest bar", expected, true);
}

// Updated to include the custom heading in the expected output
#[test]
#[cfg(feature = "wrap_help")]
fn wrapped_help() {
    let cmd = Command::new("test")
        .term_width(67)
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Also do versioning for private crates (will not be published)"),
        )
        .arg(
            Arg::new("exact")
                .long("exact")
                .action(ArgAction::SetTrue)
                .help("Specify inter dependency version numbers exactly with `=`"),
        )
        .arg(
            Arg::new("no_git_commit")
                .long("no-git-commit")
                .action(ArgAction::SetTrue)
                .help("Do not commit version changes"),
        )
        .arg(
            Arg::new("no_git_push")
                .long("no-git-push")
                .action(ArgAction::SetTrue)
                .help("Do not push generated commit and tags to git remote"),
        )
        .subcommand(
            Command::new("sub1")
                .help_heading("Custom Heading")
                .about("One two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen")
        );

    let expected = str![[r#"
Usage: test [OPTIONS] [COMMAND]

Commands:
  help  Print this message or the help of the given subcommand(s)

Custom Heading:
  sub1  One two three four five six seven eight nine ten eleven
        twelve thirteen fourteen fifteen

Options:
  -a, --all            Also do versioning for private crates (will
                       not be published)
      --exact          Specify inter dependency version numbers
                       exactly with `=`
      --no-git-commit  Do not commit version changes
      --no-git-push    Do not push generated commit and tags to git
                       remote
  -h, --help           Print help

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
#[cfg(feature = "wrap_help")]
fn unwrapped_help() {
    let cmd = Command::new("test")
        .term_width(68)
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Also do versioning for private crates (will not be published)"),
        )
        .arg(
            Arg::new("exact")
                .long("exact")
                .action(ArgAction::SetTrue)
                .help("Specify inter dependency version numbers exactly with `=`"),
        )
        .arg(
            Arg::new("no_git_commit")
                .long("no-git-commit")
                .action(ArgAction::SetTrue)
                .help("Do not commit version changes"),
        )
        .arg(
            Arg::new("no_git_push")
                .long("no-git-push")
                .action(ArgAction::SetTrue)
                .help("Do not push generated commit and tags to git remote"),
        )
        .subcommand(
            Command::new("sub1")
                .help_heading("Custom Heading")
                .about("One two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen")
        );

    let expected = str![[r#"
Usage: test [OPTIONS] [COMMAND]

Commands:
  help  Print this message or the help of the given subcommand(s)

Custom Heading:
  sub1  One two three four five six seven eight nine ten eleven
        twelve thirteen fourteen fifteen

Options:
  -a, --all            Also do versioning for private crates (will
                       not be published)
      --exact          Specify inter dependency version numbers
                       exactly with `=`
      --no-git-commit  Do not commit version changes
      --no-git-push    Do not push generated commit and tags to git
                       remote
  -h, --help           Print help

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn complex_subcommand_help_output() {
    let a = utils::complex_app_with_help_heading();

    let expected = str![[r#"
clap-test-subcmd 0.1
Kevin K. <kbknapp@gmail.com>
tests subcommands

Usage: clap-test subcmd [OPTIONS] [scpositional]

Arguments:
  [scpositional]  tests positionals

Options:
  -o, --option <scoption>...   tests options
  -f, --flag...                tests flags
  -s, --subcmdarg <subcmdarg>  tests other args
  -h, --help                   Print help
  -V, --version                Print version

"#]];
    utils::assert_output(a, "clap-test subcmd --help", expected, false);
}

// Updated to include the custom heading in the expected output
#[cfg(feature = "wrap_help")]
fn setup_aliases_with_help_heading() -> Command {
    Command::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("dest")
                .short('d')
                .long("destination")
                .value_name("FILE")
                .help("File to save into")
                .long_help("The Filepath to save into the result")
                .short_alias('q')
                .short_aliases(['w', 'e'])
                .alias("arg-alias")
                .aliases(["do-stuff", "do-tests"])
                .visible_short_alias('t')
                .visible_short_aliases(['i', 'o'])
                .visible_alias("file")
                .visible_aliases(["into", "to"])
                .action(ArgAction::Set),
        )
        .subcommand(
            Command::new("rev")
                .help_heading("Custom Heading")
                .short_flag('r')
                .long_flag("inplace")
                .about("In place")
                .long_about("Change mode to work in place on source")
                .alias("subc-alias")
                .aliases(["subc-do-stuff", "subc-do-tests"])
                .short_flag_alias('j')
                .short_flag_aliases(['k', 'l'])
                .long_flag_alias("subc-long-flag-alias")
                .long_flag_aliases(["subc-long-do-stuff", "subc-long-do-tests"])
                .visible_alias("source")
                .visible_aliases(["from", "onsource"])
                .visible_short_flag_alias('s')
                .visible_short_flag_aliases(['f', 'g'])
                .visible_long_flag_alias("origin")
                .visible_long_flag_aliases(["path", "tryfrom"])
                .arg(
                    Arg::new("input")
                        .value_name("INPUT")
                        .help("The source file"),
                ),
        )
}

// Updated to include the custom heading in the expected output
#[test]
#[cfg(feature = "wrap_help")]
fn visible_aliases_with_short_help() {
    let app = setup_aliases_with_help_heading().term_width(80);

    let expected = str![[r#"
Usage: ctest [OPTIONS] [COMMAND]

Commands:
  help                Print this message or the help of the given subcommand(s)

Custom Heading:
  rev, -r, --inplace  In place [aliases: -s, -f, -g, --origin, --path,
                      --tryfrom, source, from, onsource]

Options:
  -d, --destination <FILE>  File to save into [aliases: -t, -i, -o, --file,
                            --into, --to]
  -h, --help                Print help (see more with '--help')
  -V, --version             Print version

"#]];
    utils::assert_output(app, "ctest -h", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
#[cfg(feature = "wrap_help")]
fn visible_aliases_with_long_help() {
    let app = setup_aliases_with_help_heading().term_width(80);

    let expected = str![[r#"
Usage: ctest [OPTIONS] [COMMAND]

Commands:
  help                Print this message or the help of the given subcommand(s)

Custom Heading:
  rev, -r, --inplace  In place [aliases: -s, -f, -g, --origin, --path,
                      --tryfrom, source, from, onsource]

Options:
  -d, --destination <FILE>
          The Filepath to save into the result
          
          [aliases: -t, -i, -o, --file, --into, --to]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

"#]];
    utils::assert_output(app, "ctest --help", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn dont_wrap_urls() {
    let cmd = Command::new("Example")
        .term_width(30)
        .subcommand(Command::new("update")
            .help_heading("Custom Heading")
            .arg(
            Arg::new("force-non-host")
                .help("Install toolchains that require an emulator. See https://github.com/rust-lang/rustup/wiki/Non-host-toolchains")
                .long("force-non-host")
                .action(ArgAction::SetTrue))
    );

    let expected = str![[r#"
Usage: Example update [OPTIONS]

Options:
      --force-non-host
          Install toolchains
          that require an
          emulator. See
          https://github.com/rust-lang/rustup/wiki/Non-host-toolchains
  -h, --help
          Print help

"#]];
    utils::assert_output(cmd, "Example update --help", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
fn sc_negates_reqs() {
    let cmd = Command::new("prog")
        .version("1.0")
        .subcommand_negates_reqs(true)
        .arg(arg!(-o --opt <FILE> "tests options").required(true))
        .arg(Arg::new("PATH").help("help"))
        .subcommand(Command::new("test").help_heading("Custom Heading"));

    let expected = str![[r#"
Usage: prog --opt <FILE> [PATH]
       prog [PATH] <COMMAND>

Commands:
  help  Print this message or the help of the given subcommand(s)

Custom Heading:
  test  

Arguments:
  [PATH]  help

Options:
  -o, --opt <FILE>  tests options
  -h, --help        Print help
  -V, --version     Print version

"#]];
    utils::assert_output(cmd, "prog --help", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
fn args_negate_sc() {
    let cmd = Command::new("prog")
        .version("1.0")
        .args_conflicts_with_subcommands(true)
        .arg(arg!(-f --flag "testing flags"))
        .arg(arg!(-o --opt <FILE> "tests options"))
        .arg(Arg::new("PATH").help("help"))
        .subcommand(Command::new("test").help_heading("Custom Heading"));

    let expected = str![[r#"
Usage: prog [OPTIONS] [PATH]
       prog <COMMAND>

Commands:
  help  Print this message or the help of the given subcommand(s)

Custom Heading:
  test  

Arguments:
  [PATH]  help

Options:
  -f, --flag        testing flags
  -o, --opt <FILE>  tests options
  -h, --help        Print help
  -V, --version     Print version

"#]];
    utils::assert_output(cmd, "prog --help", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
fn dont_strip_padding_issue_5083() {
    let cmd = Command::new("test")
        .help_template("{subcommands}")
        .subcommands([
            Command::new("one"),
            Command::new("two").help_heading("Custom Heading"),
            Command::new("three"),
        ]);

    let expected = str![[r#"
  one    
  three  
  help   Print this message or the help of the given subcommand(s)

Custom Heading:
  two

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}


// Updated to include the custom heading in the expected output
#[test]
fn last_arg_mult_usage_req_with_sc() {
    let cmd = Command::new("last")
        .version("0.1")
        .subcommand_negates_reqs(true)
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(Arg::new("CORPUS").help("some"))
        .arg(
            Arg::new("ARGS")
                .action(ArgAction::Set)
                .num_args(1..)
                .last(true)
                .required(true)
                .help("some"),
        )
        .subcommand(Command::new("test").help_heading("Custom Heading").about("some"));

    let expected = str![[r#"
Usage: last <TARGET> [CORPUS] -- <ARGS>...
       last [TARGET] [CORPUS] <COMMAND>

Commands:
  help  Print this message or the help of the given subcommand(s)

Custom Heading:
  test  some

Arguments:
  <TARGET>   some
  [CORPUS]   some
  <ARGS>...  some

Options:
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "last --help", expected, false);
}

#[test]
fn last_arg_mult_usage_with_sc() {
    let cmd = Command::new("last")
        .version("0.1")
        .args_conflicts_with_subcommands(true)
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(Arg::new("CORPUS").help("some"))
        .arg(
            Arg::new("ARGS")
                .action(ArgAction::Set)
                .num_args(1..)
                .last(true)
                .help("some"),
        )
        .subcommand(Command::new("test").help_heading("Custom Heading").about("some"));

    let expected = str![[r#"
Usage: last <TARGET> [CORPUS] [-- <ARGS>...]
       last <COMMAND>

Commands:
  help  Print this message or the help of the given subcommand(s)

Custom Heading:
  test  some

Arguments:
  <TARGET>   some
  [CORPUS]   some
  [ARGS]...  some

Options:
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "last --help", expected, false);
}

fn issue_1112_setup_help_heading() -> Command {
    Command::new("test")
        .version("1.3")
        .disable_help_flag(true)
        .arg(
            Arg::new("help1")
                .long("help")
                .short('h')
                .help("some help")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("foo").arg(
                Arg::new("help1")
                    .help_heading("Custom Heading")
                    .long("help")
                    .short('h')
                    .help("some help")
                    .action(ArgAction::SetTrue),
            ),
        )
}

#[test]
fn prefer_user_help_long_1112() {
    let m = issue_1112_setup_help_heading().try_get_matches_from(vec!["test", "--help"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert!(*m.get_one::<bool>("help1").expect("defaulted by clap"));
}

#[test]
fn prefer_user_help_short_1112() {
    let m = issue_1112_setup_help_heading().try_get_matches_from(vec!["test", "-h"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert!(*m.get_one::<bool>("help1").expect("defaulted by clap"));
}

#[test]
fn prefer_user_subcmd_help_long_1112() {
    let m = issue_1112_setup_help_heading().try_get_matches_from(vec!["test", "foo", "--help"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert!(*m
        .subcommand_matches("foo")
        .unwrap()
        .get_one::<bool>("help1")
        .expect("defaulted by clap"));
}

#[test]
fn prefer_user_subcmd_help_short_1112() {
    let m = issue_1112_setup_help_heading().try_get_matches_from(vec!["test", "foo", "-h"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert!(m
        .subcommand_matches("foo")
        .unwrap()
        .get_one::<bool>("help1")
        .expect("defaulted by clap"));
}


#[test]
#[should_panic = "List of such arguments: delete"]
fn help_required_globally() {
    Command::new("myapp")
        .help_expected(true)
        .arg(Arg::new("foo").help("It does foo stuff"))
        .subcommand(
            Command::new("bar").help_heading("Custom Heading")
                .arg(Arg::new("create").help("creates bar"))
                .arg(Arg::new("delete")),
        )
        .try_get_matches_from(empty_args())
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command::help_expected is enabled for the Command"]
fn help_required_globally_but_not_given_for_subcommand() {
    Command::new("myapp")
        .help_expected(true)
        .arg(Arg::new("foo").help("It does foo stuff"))
        .subcommand(
            Command::new("bar").help_heading("Custom Heading")
                .arg(Arg::new("create").help("creates bar"))
                .arg(Arg::new("delete")),
        )
        .try_get_matches_from(empty_args())
        .unwrap();
}

#[test]
fn help_required_and_given_for_subcommand() {
    Command::new("myapp")
        .help_expected(true)
        .arg(Arg::new("foo").help("It does foo stuff"))
        .subcommand(
            Command::new("bar").help_heading("Custom Heading")
                .arg(Arg::new("create").help("creates bar"))
                .arg(Arg::new("delete").help("deletes bar")),
        )
        .try_get_matches_from(empty_args())
        .unwrap();
}

#[test]
fn help_subcmd_help() {
    let cmd = Command::new("myapp")
        .subcommand(Command::new("subcmd")
            .help_heading("First Heading")
            .subcommand(Command::new("multi")
                .help_heading("Second Heading")
                .version("1.0")));

    let expected = str![[r#"
Print this message or the help of the given subcommand(s)

Usage: myapp help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd.clone(), "myapp help help", expected, false);
}

#[test]
fn subcmd_help_subcmd_help() {
    let cmd = Command::new("myapp")
        .subcommand(Command::new("subcmd")
            .help_heading("First Heading")
            .subcommand(
                Command::new("multi")
                    .help_heading("Second Heading")
                    .version("1.0")));

    let expected = str![[r#"
Print this message or the help of the given subcommand(s)

Usage: myapp subcmd help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd.clone(), "myapp subcmd help help", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
fn global_args_should_show_on_toplevel_help_message() {
    let cmd = Command::new("myapp")
        .arg(
            Arg::new("someglobal")
                .short('g')
                .long("some-global")
                .global(true),
        )
        .subcommand(Command::new("subcmd")
            .help_heading("First Heading")
            .subcommand(Command::new("multi")
                .help_heading("Second Heading")
                .version("1.0")));

    let expected = str![[r#"
Usage: myapp [OPTIONS] [COMMAND]

Commands:
  help    Print this message or the help of the given subcommand(s)

First Heading:
  subcmd  

Options:
  -g, --some-global <someglobal>  
  -h, --help                      Print help

"#]];
    utils::assert_output(cmd, "myapp help", expected, false);
}

#[test]
fn global_args_should_not_show_on_help_message_for_help_help() {
    let cmd = Command::new("myapp")
        .arg(
            Arg::new("someglobal")
                .short('g')
                .long("some-global")
                .global(true),
        )
        .subcommand(Command::new("subcmd")
            .help_heading("First Heading")
            .subcommand(Command::new("multi")
                .help_heading("Second Heading")
                .version("1.0")));

    let expected = str![[r#"
Print this message or the help of the given subcommand(s)

Usage: myapp help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "myapp help help", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
fn global_args_should_show_on_help_message_for_subcommand() {
    let cmd = Command::new("myapp")
        .arg(
            Arg::new("someglobal")
                .short('g')
                .long("some-global")
                .global(true),
        )
        .subcommand(Command::new("subcmd")
            .help_heading("First Heading")
            .subcommand(Command::new("multi")
                .help_heading("Second Heading")
                .version("1.0")));

    let expected = str![[r#"
Usage: myapp subcmd [OPTIONS] [COMMAND]

Commands:
  help   Print this message or the help of the given subcommand(s)

Second Heading:
  multi  

Options:
  -g, --some-global <someglobal>  
  -h, --help                      Print help

"#]];
    utils::assert_output(cmd, "myapp help subcmd", expected, false);
}

#[test]
fn global_args_should_show_on_help_message_for_nested_subcommand() {
    let cmd = Command::new("myapp")
        .arg(
            Arg::new("someglobal")
                .short('g')
                .long("some-global")
                .global(true),
        )
        .subcommand(Command::new("subcmd")
            .help_heading("First Heading")
            .subcommand(Command::new("multi")
                .help_heading("Second Heading")
                .version("1.0")));

    let expected = str![[r#"
Usage: myapp subcmd multi [OPTIONS]

Options:
  -g, --some-global <someglobal>  
  -h, --help                      Print help
  -V, --version                   Print version

"#]];
    utils::assert_output(cmd, "myapp help subcmd multi", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
fn prefer_about_over_long_about_in_subcommands_list() {
    let cmd = Command::new("about-in-subcommands-list").subcommand(
        Command::new("sub")
            .help_heading("Custom Heading")
            .long_about("long about sub")
            .about("short about sub"),
    );

    let expected = str![[r#"
Usage: about-in-subcommands-list [COMMAND]

Commands:
  help  Print this message or the help of the given subcommand(s)

Custom Heading:
  sub   short about sub

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "about-in-subcommands-list --help", expected, false);
}

#[test]
fn disabled_help_flag() {
    let res = Command::new("foo")
        .subcommand(Command::new("sub")
            .help_heading("Custom Heading"))
        .disable_help_flag(true)
        .try_get_matches_from("foo a".split(' '));
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidSubcommand);
}

#[test]
fn disabled_help_flag_and_subcommand() {
    let res = Command::new("foo")
        .subcommand(Command::new("sub")
            .help_heading("Custom Heading"))
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .try_get_matches_from("foo help".split(' '));
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidSubcommand);
    assert!(
        err.to_string().ends_with('\n'),
        "Errors should have a trailing newline, got {:?}",
        err.to_string()
    );
}

#[test]
fn override_help_subcommand() {
    let cmd = Command::new("bar")
        .subcommand(Command::new("help")
            .help_heading("Help Heading")
            .arg(Arg::new("arg").action(ArgAction::Set)))
        .subcommand(Command::new("not_help")
            .help_heading("Not Help")
            .arg(Arg::new("arg").action(ArgAction::Set)))
        .disable_help_subcommand(true);
    let matches = cmd.try_get_matches_from(["bar", "help", "foo"]).unwrap();
    assert_eq!(
        matches
            .subcommand_matches("help")
            .unwrap()
            .get_one::<String>("arg")
            .map(|v| v.as_str()),
        Some("foo")
    );
}

#[test]
fn override_help_flag_using_long() {
    let cmd = Command::new("foo")
        .subcommand(Command::new("help")
            .help_heading("Custom Heading")
            .long_flag("help"))
        .disable_help_flag(true)
        .disable_help_subcommand(true);
    let matches = cmd.try_get_matches_from(["foo", "--help"]).unwrap();
    assert!(matches.subcommand_matches("help").is_some());
}

#[test]
fn override_help_flag_using_short() {
    let cmd = Command::new("foo")
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .subcommand(Command::new("help")
            .help_heading("Custom Heading")
            .short_flag('h'));
    let matches = cmd.try_get_matches_from(["foo", "-h"]).unwrap();
    assert!(matches.subcommand_matches("help").is_some());
}

#[test]
fn subcommand_help_doesnt_have_useless_help_flag() {
    // The main care-about is that the docs and behavior match.  Since the `help` subcommand
    // currently ignores the `--help` flag, the output shouldn't have it.
    let cmd = Command::new("example")
                .subcommand(Command::new("test")
                    .help_heading("Custom Heading")
                    .about("Subcommand"));

    let expected = str![[r#"
Print this message or the help of the given subcommand(s)

Usage: example help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "example help help", expected, false);
}

#[test]
fn disable_help_flag_affects_help_subcommand() {
    let mut cmd = Command::new("test_app")
        .disable_help_flag(true)
        .subcommand(Command::new("test")
            .help_heading("Custom Heading")
            .about("Subcommand"));
    cmd.build();

    let args = cmd
        .find_subcommand("help")
        .unwrap()
        .get_arguments()
        .map(|a| a.get_id().as_str())
        .collect::<Vec<_>>();
    assert!(
        !args.contains(&"help"),
        "`help` should not be present: {args:?}"
    );
}

#[test]
fn dont_propagate_version_to_help_subcommand() {
    let cmd = Command::new("example")
        .version("1.0")
        .propagate_version(true)
        .subcommand(Command::new("subcommand")
                        .help_heading("Custom Heading"));

    let expected = str![[r#"
Print this message or the help of the given subcommand(s)

Usage: example help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd.clone(), "example help help", expected, false);

    cmd.debug_assert();
}


#[test]
fn parent_cmd_req_in_usage_with_help_flag() {
    let cmd = Command::new("parent")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(
            Arg::new("ARGS")
                .action(ArgAction::Set)
                .required(true)
                .help("some"),
        )
        .subcommand(Command::new("test")
                        .help_heading("Custom Heading")
                        .about("some"));

    let expected = str![[r#"
some

Usage: parent <TARGET> <ARGS> test

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "parent test --help", expected, false);
}

#[test]
fn parent_cmd_req_in_usage_with_help_subcommand() {
    let cmd = Command::new("parent")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(
            Arg::new("ARGS")
                .action(ArgAction::Set)
                .required(true)
                .help("some"),
        )
        .subcommand(Command::new("test")
                        .help_heading("Custom Heading")
                        .about("some"));

    let expected = str![[r#"
some

Usage: parent <TARGET> <ARGS> test

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "parent help test", expected, false);
}

#[test]
fn parent_cmd_req_in_usage_with_render_help() {
    let mut cmd = Command::new("parent")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(
            Arg::new("ARGS")
                .action(ArgAction::Set)
                .required(true)
                .help("some"),
        )
        .subcommand(Command::new("test")
                        .help_heading("Custom Heading")
                        .about("some"));
    cmd.build();
    let subcmd = cmd.find_subcommand_mut("test").unwrap();

    let help = subcmd.render_help().to_string();
    assert_data_eq!(
        help,
        str![[r#"
some

Usage: parent <TARGET> <ARGS> test

Options:
  -h, --help  Print help

"#]]
    );
}

#[test]
fn parent_cmd_req_ignored_when_negates_reqs() {
    let cmd = Command::new("ctest")
        .arg(arg!(<input>))
        .subcommand_negates_reqs(true)
        .subcommand(Command::new("subcmd")
                        .help_heading("Custom Heading"));

    let expected = str![[r#"
Usage: ctest subcmd

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "ctest subcmd --help", expected, false);
}

#[test]
fn parent_cmd_req_ignored_when_conflicts() {
    let cmd = Command::new("ctest")
        .arg(arg!(<input>))
        .args_conflicts_with_subcommands(true)
        .subcommand(Command::new("subcmd")
                        .help_heading("Custom Heading"));

    let expected = str![[r#"
Usage: ctest subcmd

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "ctest subcmd --help", expected, false);
}

#[test]
fn display_name_subcommand_default() {
    let mut cmd = Command::new("parent")
        .subcommand(Command::new("child")
                        .help_heading("Custom Heading")
                        .bin_name("child.exe"));
    cmd.build();
    assert_eq!(
        cmd.find_subcommand("child").unwrap().get_display_name(),
        Some("parent-child")
    );
}

#[test]
fn display_name_subcommand_explicit() {
    let mut cmd = Command::new("parent").subcommand(
        Command::new("child")
            .help_heading("Custom Heading")
            .bin_name("child.exe")
            .display_name("child.display"),
    );
    cmd.build();
    assert_eq!(
        cmd.find_subcommand("child").unwrap().get_display_name(),
        Some("child.display")
    );
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_basic() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
                .help_heading("Custom Heading")
                .about("test command")
                .arg(Arg::new("child").long("child")),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]
       parent test [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>  
  -h, --help             Print help

parent test:
test command
      --child <child>  
  -h, --help           Print help

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_short_help() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(
            Arg::new("parent")
                .long("parent")
                .help("foo")
                .long_help("bar"),
        )
        .subcommand(
            Command::new("test")
                .about("test command")
                .help_heading("Custom Heading")
                .long_about("long some")
                .arg(Arg::new("child").long("child").help("foo").long_help("bar")),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]
       parent test [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>  foo
  -h, --help             Print help (see more with '--help')

parent test:
test command
      --child <child>  foo
  -h, --help           Print help (see more with '--help')

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_long_help() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(
            Arg::new("parent")
                .long("parent")
                .help("foo")
                .long_help("bar"),
        )
        .subcommand(
            Command::new("test")
                .help_heading("Custom Heading")
                .about("test command")
                .long_about("long some")
                .arg(Arg::new("child").long("child").help("foo").long_help("bar")),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]
       parent test [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>
          bar

  -h, --help
          Print help (see a summary with '-h')

parent test:
test command
      --child <child>
          bar

  -h, --help
          Print help (see a summary with '-h')

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...
          Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent --help", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_help_cmd() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(
            Arg::new("parent")
                .long("parent")
                .help("foo")
                .long_help("bar"),
        )
        .subcommand(
            Command::new("test")
                .help_heading("Custom Heading")
                .about("test command")
                .long_about("long some")
                .arg(Arg::new("child").long("child").help("foo").long_help("bar")),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]
       parent test [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>
          bar

  -h, --help
          Print help (see a summary with '-h')

parent test:
test command
      --child <child>
          bar

  -h, --help
          Print help (see a summary with '-h')

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...
          Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent help", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_with_global() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent").global(true))
        .subcommand(
            Command::new("test")
                .help_heading("Custom Heading")
                .about("test command")
                .arg(Arg::new("child").long("child")),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]
       parent test [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>  
  -h, --help             Print help

parent test:
test command
      --child <child>  
  -h, --help           Print help

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_arg_required() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent").required(true))
        .subcommand(
            Command::new("test")
                .help_heading("Custom Heading")
                .about("test command")
                .arg(Arg::new("child").long("child").required(true)),
        );

    let expected = str![[r#"
parent command

Usage: parent --parent <parent>
       parent --parent <parent> test --child <child>
       parent --parent <parent> help [COMMAND]...

Options:
      --parent <parent>  
  -h, --help             Print help

parent --parent <parent> test:
test command
      --child <child>  
  -h, --help           Print help

parent --parent <parent> help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_with_external_subcommand() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .allow_external_subcommands(true)
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
                .help_heading("Custom Heading")
                .about("test command")
                .arg(Arg::new("child").long("child")),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]
       parent test [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>  
  -h, --help             Print help

parent test:
test command
      --child <child>  
  -h, --help           Print help

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_with_subcommand_required() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .subcommand_required(true)
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
                .help_heading("Custom Heading")
                .about("test command")
                .arg(Arg::new("child").long("child")),
        );

    let expected = str![[r#"
parent command

Usage: parent test [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>  
  -h, --help             Print help

parent test:
test command
      --child <child>  
  -h, --help           Print help

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_with_args_conflicts_with_subcommands() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .subcommand_required(true)
        .args_conflicts_with_subcommands(true)
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
                .help_heading("Custom Heading")
                .about("test command")
                .arg(Arg::new("child").long("child")),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]
       parent test [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>  
  -h, --help             Print help

parent test:
test command
      --child <child>  
  -h, --help           Print help

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

#[test]
fn flatten_single_hidden_command() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .help_heading("Custom Heading")
                .hide(true)
                .about("child1 command")
                .arg(Arg::new("child").long("child1")),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]

Options:
      --parent <parent>  
  -h, --help             Print help

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_hidden_command() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .help_heading("Custom Heading")
                .about("child1 command")
                .arg(Arg::new("child").long("child1")),
        )
        .subcommand(
            Command::new("child2")
                .help_heading("Custom Heading")
                .about("child2 command")
                .arg(Arg::new("child").long("child2")),
        )
        .subcommand(
            Command::new("child3")
                .help_heading("Custom Heading")
                .hide(true)
                .about("child3 command")
                .arg(Arg::new("child").long("child3")),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]
       parent child1 [OPTIONS]
       parent child2 [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>  
  -h, --help             Print help

parent child1:
child1 command
      --child1 <child>  
  -h, --help            Print help

parent child2:
child2 command
      --child2 <child>  
  -h, --help            Print help

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_recursive() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .flatten_help(true)
                .help_heading("Custom Heading")
                .about("child1 command")
                .arg(Arg::new("child").long("child1"))
                .subcommand(
                    Command::new("grandchild1")
                    .flatten_help(true)
                        .help_heading("Custom Heading")
                        .about("grandchild1 command")
                        .arg(Arg::new("grandchild").long("grandchild1"))
                        .subcommand(
                            Command::new("greatgrandchild1")
                                .help_heading("Custom Heading")
                                .about("greatgrandchild1 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild1")),
                            )
                            .subcommand(
                                Command::new("greatgrandchild2")
                                .help_heading("Custom Heading")
                                .about("greatgrandchild2 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild2")),
                            )
                            .subcommand(
                                Command::new("greatgrandchild3")
                                .help_heading("Custom Heading")
                                .about("greatgrandchild3 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild3")),
                            ),
                        )
                .subcommand(
                     Command::new("grandchild2")
                        .help_heading("Custom Heading")
                        .about("grandchild2 command")
                        .arg(Arg::new("grandchild").long("grandchild2")),
                    )
                    .subcommand(
                        Command::new("grandchild3")
                        .help_heading("Custom Heading")
                        .about("grandchild3 command")
                        .arg(Arg::new("grandchild").long("grandchild3")),
                ),
        )
        .subcommand(
            Command::new("child2")
                .help_heading("Custom Heading")
                .about("child2 command")
                .arg(Arg::new("child").long("child2")),
        )
        .subcommand(
            Command::new("child3")
                .hide(true)
                .help_heading("Custom Heading")
                .about("child3 command")
                .arg(Arg::new("child").long("child3"))
                .subcommand(
                    Command::new("grandchild1")
                        .flatten_help(true)
                        .help_heading("Custom Heading")
                        .about("grandchild1 command")
                        .arg(Arg::new("grandchild").long("grandchild1"))
                        .subcommand(
                            Command::new("greatgrandchild1")
                                .help_heading("Custom Heading")
                                .about("greatgrandchild1 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild1")),
                        )
                        .subcommand(
                            Command::new("greatgrandchild2")
                                .help_heading("Custom Heading")
                                .about("greatgrandchild2 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild2")),
                        )
                        .subcommand(
                            Command::new("greatgrandchild3")
                                .help_heading("Custom Heading")
                                .about("greatgrandchild3 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild3")),
                        ),
                )
                .subcommand(
                    Command::new("grandchild2")
                        .help_heading("Custom Heading")
                        .about("grandchild2 command")
                        .arg(Arg::new("grandchild").long("grandchild2")),
                )
                .subcommand(
                    Command::new("grandchild3")
                        .help_heading("Custom Heading")
                        .about("grandchild3 command")
                        .arg(Arg::new("grandchild").long("grandchild3")),
                ),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]
       parent child1 [OPTIONS]
       parent child1 grandchild1 [OPTIONS]
       parent child1 grandchild1 greatgrandchild1 [OPTIONS]
       parent child1 grandchild1 greatgrandchild2 [OPTIONS]
       parent child1 grandchild1 greatgrandchild3 [OPTIONS]
       parent child1 grandchild1 help [COMMAND]
       parent child1 grandchild2 [OPTIONS]
       parent child1 grandchild3 [OPTIONS]
       parent child1 help [COMMAND]
       parent child2 [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>  
  -h, --help             Print help

parent child1:
child1 command
      --child1 <child>  
  -h, --help            Print help

parent child1 grandchild1:
grandchild1 command
      --grandchild1 <grandchild>  
  -h, --help                      Print help

parent child1 grandchild1 greatgrandchild1:
greatgrandchild1 command
      --greatgrandchild1 <greatgrandchild>  
  -h, --help                                Print help

parent child1 grandchild1 greatgrandchild2:
greatgrandchild2 command
      --greatgrandchild2 <greatgrandchild>  
  -h, --help                                Print help

parent child1 grandchild1 greatgrandchild3:
greatgrandchild3 command
      --greatgrandchild3 <greatgrandchild>  
  -h, --help                                Print help

parent child1 grandchild1 help:
Print this message or the help of the given subcommand(s)

parent child1 grandchild2:
grandchild2 command
      --grandchild2 <grandchild>  
  -h, --help                      Print help

parent child1 grandchild3:
grandchild3 command
      --grandchild3 <grandchild>  
  -h, --help                      Print help

parent child1 help:
Print this message or the help of the given subcommand(s)

parent child2:
child2 command
      --child2 <child>  
  -h, --help            Print help

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

// Parent Help - should it be the first (as normally under Command 
// and not custom heading) or should it be the last as is expected here?
#[test]
fn flatten_not_recursive() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .help_heading("Custom Heading")
                .about("child1 command")
                .arg(Arg::new("child").long("child1"))
                .subcommand(
                    Command::new("grandchild1")
                        .help_heading("Custom Heading")
                        .about("grandchild1 command")
                        .arg(Arg::new("grandchild").long("grandchild1")),
                )
                .subcommand(
                    Command::new("grandchild2")
                        .help_heading("Custom Heading")
                        .about("grandchild2 command")
                        .arg(Arg::new("grandchild").long("grandchild2")),
                )
                .subcommand(
                    Command::new("grandchild3")
                        .help_heading("Custom Heading")
                        .about("grandchild3 command")
                        .arg(Arg::new("grandchild").long("grandchild3")),
                ),
        )
        .subcommand(
            Command::new("child2")
                .help_heading("Custom Heading")
                .about("child2 command")
                .arg(Arg::new("child").long("child2")),
        )
        .subcommand(
            Command::new("child3")
                .help_heading("Custom Heading")
                .about("child3 command")
                .arg(Arg::new("child").long("child3")),
        );

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]
       parent child1 [OPTIONS] [COMMAND]
       parent child2 [OPTIONS]
       parent child3 [OPTIONS]
       parent help [COMMAND]...

Options:
      --parent <parent>  
  -h, --help             Print help

parent child1:
child1 command
      --child1 <child>  
  -h, --help            Print help

parent child2:
child2 command
      --child2 <child>  
  -h, --help            Print help

parent child3:
child3 command
      --child3 <child>  
  -h, --help            Print help

parent help:
Print this message or the help of the given subcommand(s)
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
#[cfg(feature = "wrap_help")]
fn next_line_command_short() {
    let value_name = "V";
    let text = "Hello";

    let cmd = Command::new("test")
        .term_width(120)
        .next_line_help(true)
        .args([
            Arg::new("default")
                .long("default")
                .value_name(value_name)
                .help(text)
                .long_help(text),
            Arg::new("next_line_help_false")
                .long("next_line_help_false")
                .next_line_help(false)
                .value_name(value_name)
                .help(text)
                .long_help(text),
            Arg::new("next_line_help_true")
                .long("next_line_help_true")
                .next_line_help(true)
                .value_name(value_name)
                .help(text)
                .long_help(text),
        ])
        .subcommands([
            Command::new("default").about(text).long_about(text),
            Command::new("next_line_help_false")
                .help_heading("Custom Heading")
                .next_line_help(false)
                .about(text)
                .long_about(text),
            Command::new("next_line_help_true")
                .help_heading("Custom Heading")
                .next_line_help(true)
                .about(text)
                .long_about(text),
        ]);

    let expected = str![[r#"
Usage: myprog [OPTIONS] [COMMAND]

Commands:
  default
          Hello
  help
          Print this message or the help of the given subcommand(s)

Custom Heading:
  next_line_help_false
          Hello
  next_line_help_true
          Hello

Options:
      --default <V>
          Hello
      --next_line_help_false <V>
          Hello
      --next_line_help_true <V>
          Hello
  -h, --help
          Print help (see more with '--help')

"#]];
    utils::assert_output(cmd.clone(), "myprog -h", expected, false);

    let expected = str![[r#"
Usage: myprog [OPTIONS] [COMMAND]

Commands:
  default
          Hello
  help
          Print this message or the help of the given subcommand(s)

Custom Heading:
  next_line_help_false
          Hello
  next_line_help_true
          Hello

Options:
      --default <V>
          Hello

      --next_line_help_false <V>
          Hello

      --next_line_help_true <V>
          Hello

  -h, --help
          Print help (see a summary with '-h')

"#]];
    utils::assert_output(cmd, "myprog --help", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
#[cfg(feature = "wrap_help")]
fn next_line_arg_short() {
    let value_name = "V";
    let text = "Hello";

    let cmd = Command::new("test")
        .term_width(120)
        .next_line_help(true)
        .args([
            Arg::new("default")
                .long("default")
                .value_name(value_name)
                .help(text)
                .long_help(text),
            Arg::new("next_line_help_false")
                .long("next_line_help_false")
                .next_line_help(false)
                .value_name(value_name)
                .help(text)
                .long_help(text),
            Arg::new("next_line_help_true")
                .long("next_line_help_true")
                .next_line_help(true)
                .value_name(value_name)
                .help(text)
                .long_help(text),
        ])
        .subcommands([
            Command::new("default").about(text).long_about(text),
            Command::new("next_line_help_false")
                .help_heading("Custom Heading")
                .next_line_help(false)
                .about(text)
                .long_about(text),
            Command::new("next_line_help_true")
                .help_heading("Custom Heading")
                .next_line_help(true)
                .about(text)
                .long_about(text),
        ]);

    let expected = str![[r#"
Usage: myprog [OPTIONS] [COMMAND]

Commands:
  default
          Hello
  help
          Print this message or the help of the given subcommand(s)

Custom Heading:
  next_line_help_false
          Hello
  next_line_help_true
          Hello

Options:
      --default <V>
          Hello
      --next_line_help_false <V>
          Hello
      --next_line_help_true <V>
          Hello
  -h, --help
          Print help (see more with '--help')

"#]];
    utils::assert_output(cmd.clone(), "myprog -h", expected, false);

    let expected = str![[r#"
Usage: myprog [OPTIONS] [COMMAND]

Commands:
  default
          Hello
  help
          Print this message or the help of the given subcommand(s)

Custom Heading:
  next_line_help_false
          Hello
  next_line_help_true
          Hello

Options:
      --default <V>
          Hello

      --next_line_help_false <V>
          Hello

      --next_line_help_true <V>
          Hello

  -h, --help
          Print help (see a summary with '-h')

"#]];
    utils::assert_output(cmd, "myprog --help", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
#[cfg(feature = "wrap_help")]
fn next_line_command_wrapped() {
    let value_name = "SOME_LONG_VALUE";
    let text = "Also do versioning for private crates (will not be published)

Specify inter dependency version numbers exactly with `=`

Do not commit version changes

Do not push generated commit and tags to git remote
";

    let cmd = Command::new("test")
        .term_width(67)
        .next_line_help(true)
        .args([
            Arg::new("default")
                .long("default")
                .value_name(value_name)
                .help(text)
                .long_help(text),
            Arg::new("next_line_help_false")
                .long("next_line_help_false")
                .next_line_help(false)
                .value_name(value_name)
                .help(text)
                .long_help(text),
            Arg::new("next_line_help_true")
                .long("next_line_help_true")
                .next_line_help(true)
                .value_name(value_name)
                .help(text)
                .long_help(text),
        ])
        .subcommands([
            Command::new("default").about(text).long_about(text),
            Command::new("next_line_help_false")
                .help_heading("Custom Heading")
                .next_line_help(false)
                .about(text)
                .long_about(text),
            Command::new("next_line_help_true")
                .help_heading("Custom Heading")
                .next_line_help(true)
                .about(text)
                .long_about(text),
        ]);

    let expected = str![[r#"
Usage: myprog [OPTIONS] [COMMAND]

Commands:
  default
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
  help
          Print this message or the help of the given subcommand(s)

Custom Heading:
  next_line_help_false
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
  next_line_help_true
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote

Options:
      --default <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
      --next_line_help_false <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
      --next_line_help_true <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
  -h, --help
          Print help (see more with '--help')

"#]];
    utils::assert_output(cmd.clone(), "myprog -h", expected, false);

    let expected = str![[r#"
Usage: myprog [OPTIONS] [COMMAND]

Commands:
  default
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
  help
          Print this message or the help of the given subcommand(s)

Custom Heading:
  next_line_help_false
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
  next_line_help_true
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote

Options:
      --default <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote

      --next_line_help_false <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote

      --next_line_help_true <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote

  -h, --help
          Print help (see a summary with '-h')

"#]];
    utils::assert_output(cmd, "myprog --help", expected, false);
}

// Updated to include the custom heading in the expected output
#[test]
#[cfg(feature = "wrap_help")]
fn next_line_arg_wrapped() {
    let value_name = "SOME_LONG_VALUE";
    let text = "Also do versioning for private crates (will not be published)

Specify inter dependency version numbers exactly with `=`

Do not commit version changes

Do not push generated commit and tags to git remote
";

    let cmd = Command::new("test")
        .term_width(67)
        .next_line_help(true)
        .args([
            Arg::new("default")
                .long("default")
                .value_name(value_name)
                .help(text)
                .long_help(text),
            Arg::new("next_line_help_false")
                .long("next_line_help_false")
                .next_line_help(false)
                .value_name(value_name)
                .help(text)
                .long_help(text),
            Arg::new("next_line_help_true")
                .long("next_line_help_true")
                .next_line_help(true)
                .value_name(value_name)
                .help(text)
                .long_help(text),
        ])
        .subcommands([
            Command::new("default").about(text).long_about(text),
            Command::new("next_line_help_false")
                .help_heading("Custom Heading")
                .next_line_help(false)
                .about(text)
                .long_about(text),
            Command::new("next_line_help_true")
                .help_heading("Custom Heading")
                .next_line_help(true)
                .about(text)
                .long_about(text),
        ]);

    let expected = str![[r#"
Usage: myprog [OPTIONS] [COMMAND]

Commands:
  default
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
  help
          Print this message or the help of the given subcommand(s)

Custom Heading:
  next_line_help_false
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
  next_line_help_true
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote

Options:
      --default <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
      --next_line_help_false <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
      --next_line_help_true <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
  -h, --help
          Print help (see more with '--help')

"#]];
    utils::assert_output(cmd.clone(), "myprog -h", expected, false);

    let expected = str![[r#"
Usage: myprog [OPTIONS] [COMMAND]

Commands:
  default
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
  help
          Print this message or the help of the given subcommand(s)

Custom Heading:
  next_line_help_false
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote
  next_line_help_true
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote

Options:
      --default <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote

      --next_line_help_false <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote

      --next_line_help_true <SOME_LONG_VALUE>
          Also do versioning for private crates (will not be
          published)
          
          Specify inter dependency version numbers exactly with `=`
          
          Do not commit version changes
          
          Do not push generated commit and tags to git remote

  -h, --help
          Print help (see a summary with '-h')

"#]];
    utils::assert_output(cmd, "myprog --help", expected, false);
}

}