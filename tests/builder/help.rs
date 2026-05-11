#![cfg(feature = "help")]

use clap::{arg, builder::PossibleValue, error::ErrorKind, Arg, ArgAction, ArgGroup, Command};
use snapbox::assert_data_eq;
use snapbox::str;

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
fn help_short() {
    let m = setup().try_get_matches_from(vec!["myprog", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

#[test]
fn help_long() {
    let m = setup().try_get_matches_from(vec!["myprog", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

#[test]
fn help_no_subcommand() {
    let m = setup().try_get_matches_from(vec!["myprog", "help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::UnknownArgument);
}

#[test]
fn help_subcommand() {
    let m = setup()
        .subcommand(
            Command::new("test")
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
fn req_last_arg_usage() {
    let cmd = Command::new("example")
        .version("1.0")
        .arg(Arg::new("FIRST").help("First").num_args(1..).required(true))
        .arg(
            Arg::new("SECOND")
                .help("Second")
                .num_args(1..)
                .required(true)
                .last(true),
        );
    let expected = str![[r#"
Usage: example <FIRST>... -- <SECOND>...

Arguments:
  <FIRST>...   First
  <SECOND>...  Second

Options:
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "example --help", expected, false);
}

#[test]
fn args_with_last_usage() {
    let cmd = Command::new("flamegraph")
        .version("0.1")
        .arg(
            Arg::new("verbose")
                .help("Prints out more stuff.")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("timeout")
                .help("Timeout in seconds.")
                .short('t')
                .long("timeout")
                .value_name("SECONDS"),
        )
        .arg(
            Arg::new("frequency")
                .help("The sampling frequency.")
                .short('f')
                .long("frequency")
                .value_name("HERTZ"),
        )
        .arg(
            Arg::new("binary path")
                .help("The path of the binary to be profiled. for a binary.")
                .value_name("BINFILE"),
        )
        .arg(
            Arg::new("pass through args")
                .help("Any arguments you wish to pass to the being profiled.")
                .action(ArgAction::Set)
                .num_args(1..)
                .last(true)
                .value_name("ARGS"),
        );
    let expected = str![[r#"
Usage: flamegraph [OPTIONS] [BINFILE] [-- <ARGS>...]

Arguments:
  [BINFILE]  The path of the binary to be profiled. for a binary.
  [ARGS]...  Any arguments you wish to pass to the being profiled.

Options:
  -v, --verbose            Prints out more stuff.
  -t, --timeout <SECONDS>  Timeout in seconds.
  -f, --frequency <HERTZ>  The sampling frequency.
  -h, --help               Print help
  -V, --version            Print version

"#]];
    utils::assert_output(cmd, "flamegraph --help", expected, false);
}

#[test]
fn subcommand_short_help() {
    let m = utils::complex_app().try_get_matches_from(vec!["clap-test", "subcmd", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

#[test]
fn subcommand_long_help() {
    let m = utils::complex_app().try_get_matches_from(vec!["clap-test", "subcmd", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

#[test]
fn subcommand_help_rev() {
    let m = utils::complex_app().try_get_matches_from(vec!["clap-test", "help", "subcmd"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

#[test]
fn complex_help_output() {
    let expected = str![[r#"
clap-test v1.4.8
Kevin K. <kbknapp@gmail.com>
tests clap library

Usage: clap-test [OPTIONS] [positional] [positional2] [positional3]... [COMMAND]

Commands:
  subcmd  tests subcommands
  help    Print this message or the help of the given subcommand(s)

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
    utils::assert_output(utils::complex_app(), "clap-test --help", expected, false);
}

#[test]
fn after_and_before_help_output() {
    let cmd = Command::new("clap-test")
        .version("v1.4.8")
        .about("tests clap library")
        .before_help("some text that comes before the help")
        .after_help("some text that comes after the help");

    let expected = str![[r#"
some text that comes before the help

tests clap library

Usage: clap-test

Options:
  -h, --help     Print help
  -V, --version  Print version

some text that comes after the help

"#]];
    utils::assert_output(cmd.clone(), "clap-test -h", expected, false);

    let expected = str![[r#"
some text that comes before the help

tests clap library

Usage: clap-test

Options:
  -h, --help     Print help
  -V, --version  Print version

some text that comes after the help

"#]];
    utils::assert_output(cmd, "clap-test --help", expected, false);
}

#[test]
fn after_and_before_long_help_output() {
    let cmd = Command::new("clap-test")
        .version("v1.4.8")
        .about("tests clap library")
        .before_help("some text that comes before the help")
        .after_help("some text that comes after the help")
        .before_long_help("some longer text that comes before the help")
        .after_long_help("some longer text that comes after the help");

    let expected = str![[r#"
some longer text that comes before the help

tests clap library

Usage: clap-test

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

some longer text that comes after the help

"#]];
    utils::assert_output(cmd.clone(), "clap-test --help", expected, false);

    let expected = str![[r#"
some text that comes before the help

tests clap library

Usage: clap-test

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version

some text that comes after the help

"#]];
    utils::assert_output(cmd, "clap-test -h", expected, false);
}

#[test]
fn multi_level_sc_help() {
    let cmd = Command::new("ctest").subcommand(
        Command::new("subcmd").subcommand(
            Command::new("multi")
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
fn try_help_default() {
    let cmd = Command::new("ctest").version("1.0").term_width(0);

    let expected = str![[r#"
error: unexpected argument 'bar' found

Usage: ctest

For more information, try '--help'.

"#]];
    utils::assert_output(cmd, "ctest bar", expected, true);
}

#[test]
fn try_help_custom_flag() {
    let cmd = Command::new("ctest")
        .version("1.0")
        .disable_help_flag(true)
        .arg(
            Arg::new("help")
                .long("help")
                .short('h')
                .action(ArgAction::Help),
        )
        .term_width(0);

    let expected = str![[r#"
error: unexpected argument 'bar' found

Usage: ctest

For more information, try '--help'.

"#]];
    utils::assert_output(cmd, "ctest bar", expected, true);
}

#[test]
fn try_help_custom_flag_short() {
    let cmd = Command::new("ctest")
        .version("1.0")
        .disable_help_flag(true)
        .arg(Arg::new("help").short('h').action(ArgAction::HelpShort))
        .term_width(0);

    let expected = str![[r#"
error: unexpected argument 'bar' found

Usage: ctest

For more information, try '-h'.

"#]];
    utils::assert_output(cmd, "ctest bar", expected, true);
}

#[test]
fn try_help_custom_flag_long() {
    let cmd = Command::new("ctest")
        .version("1.0")
        .disable_help_flag(true)
        .arg(Arg::new("help").long("help").action(ArgAction::HelpShort))
        .term_width(0);

    let expected = str![[r#"
error: unexpected argument 'bar' found

Usage: ctest

For more information, try '--help'.

"#]];
    utils::assert_output(cmd, "ctest bar", expected, true);
}

#[test]
fn try_help_custom_flag_no_action() {
    let cmd = Command::new("ctest")
        .version("1.0")
        .disable_help_flag(true)
        // Note `ArgAction::Help` is excluded
        .arg(Arg::new("help").long("help").global(true))
        .term_width(0);

    let expected = str![[r#"
error: unexpected argument 'bar' found

Usage: ctest

"#]];
    utils::assert_output(cmd, "ctest bar", expected, true);
}

#[test]
fn try_help_subcommand_default() {
    let cmd = Command::new("ctest")
        .version("1.0")
        .subcommand(Command::new("foo"))
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
        .subcommand(Command::new("foo"))
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
        .subcommand(Command::new("foo"))
        .term_width(0);

    let expected = str![[r#"
error: unrecognized subcommand 'bar'

Usage: ctest [COMMAND]

For more information, try 'help'.

"#]];
    utils::assert_output(cmd, "ctest bar", expected, true);
}

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
                .about("One two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen")
        );

    let expected = str![[r#"
Usage: test [OPTIONS] [COMMAND]

Commands:
  sub1  One two three four five six seven eight nine ten eleven
        twelve thirteen fourteen fifteen
  help  Print this message or the help of the given subcommand(s)

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
                .about("One two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen")
        );

    let expected = str![[r#"
Usage: test [OPTIONS] [COMMAND]

Commands:
  sub1  One two three four five six seven eight nine ten eleven
        twelve thirteen fourteen fifteen
  help  Print this message or the help of the given subcommand(s)

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
#[cfg(feature = "wrap_help")]
fn possible_value_wrapped_help() {
    let cmd = Command::new("test")
        .term_width(67)
        .arg(
            Arg::new("possible_values")
                .long("possible-values")
                .action(ArgAction::Set)
                .value_parser([
                    PossibleValue::new("short_name")
                        .help("Long enough help message, barely warrant wrapping"),
                    PossibleValue::new("second").help("Short help gets handled the same"),
                ]),
        )
        .arg(
            Arg::new("possible_values_with_new_line")
                .long("possible-values-with-new-line")
                .action(ArgAction::Set)
                .value_parser([
                    PossibleValue::new("long enough name to trigger new line").help(
                        "Really long enough help message to clearly warrant wrapping believe me",
                    ),
                    PossibleValue::new("second"),
                ]),
        )
        .arg(
            Arg::new("possible_values_without_new_line")
                .long("possible-values-without-new-line")
                .action(ArgAction::Set)
                .value_parser([
                    PossibleValue::new("name").help("Short enough help message with no wrapping"),
                    PossibleValue::new("second").help("short help"),
                ]),
        );

    let expected = str![[r#"
Usage: test [OPTIONS]

Options:
      --possible-values <possible_values>
          Possible values:
          - short_name: Long enough help message, barely warrant
            wrapping
          - second:     Short help gets handled the same

      --possible-values-with-new-line <possible_values_with_new_line>
          Possible values:
          - long enough name to trigger new line: Really long
            enough help message to clearly warrant wrapping believe
            me
          - second

      --possible-values-without-new-line <possible_values_without_new_line>
          Possible values:
          - name:   Short enough help message with no wrapping
          - second: short help

  -h, --help
          Print help (see a summary with '-h')

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn complex_subcommand_help_output() {
    let a = utils::complex_app();

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

#[test]
#[cfg(feature = "wrap_help")]
fn issue_626_unicode_cutoff() {
    let cmd = Command::new("ctest").version("0.1").term_width(70).arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .help(
                "A coffeehouse, coffee shop, or café is an establishment \
             which primarily serves hot coffee, related coffee beverages \
             (e.g., café latte, cappuccino, espresso), tea, and other hot \
             beverages. Some coffeehouses also serve cold beverages such as \
             iced coffee and iced tea. Many cafés also serve some type of \
             food, such as light snacks, muffins, or pastries.",
            )
            .action(ArgAction::Set),
    );

    let expected = str![[r#"
Usage: ctest [OPTIONS]

Options:
  -c, --cafe <FILE>  A coffeehouse, coffee shop, or café is an
                     establishment which primarily serves hot coffee,
                     related coffee beverages (e.g., café latte,
                     cappuccino, espresso), tea, and other hot
                     beverages. Some coffeehouses also serve cold
                     beverages such as iced coffee and iced tea. Many
                     cafés also serve some type of food, such as light
                     snacks, muffins, or pastries.
  -h, --help         Print help
  -V, --version      Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
fn hide_possible_vals() {
    let cmd = Command::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("pos")
                .short('p')
                .long("pos")
                .value_name("VAL")
                .value_parser(["fast", "slow"])
                .help("Some vals")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .hide_possible_values(true)
                .value_parser(["fast", "slow"])
                .help("A coffeehouse, coffee shop, or café.")
                .action(ArgAction::Set),
        );

    let expected = str![[r#"
Usage: ctest [OPTIONS]

Options:
  -p, --pos <VAL>    Some vals [possible values: fast, slow]
  -c, --cafe <FILE>  A coffeehouse, coffee shop, or café.
  -h, --help         Print help
  -V, --version      Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
fn hide_single_possible_val() {
    let cmd = Command::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("pos")
                .short('p')
                .long("pos")
                .value_name("VAL")
                .value_parser([
                    "fast".into(),
                    "slow".into(),
                    PossibleValue::new("secret speed").hide(true),
                ])
                .help("Some vals")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .help("A coffeehouse, coffee shop, or café.")
                .action(ArgAction::Set),
        );

    let expected = str![[r#"
Usage: ctest [OPTIONS]

Options:
  -p, --pos <VAL>    Some vals [possible values: fast, slow]
  -c, --cafe <FILE>  A coffeehouse, coffee shop, or café.
  -h, --help         Print help
  -V, --version      Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
fn possible_vals_with_help() {
    let app = Command::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("pos")
                .short('p')
                .long("pos")
                .value_name("VAL")
                .value_parser([
                    PossibleValue::new("fast"),
                    PossibleValue::new("slow").help("not as fast"),
                    PossibleValue::new("secret speed").hide(true),
                ])
                .help("Some vals")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .help("A coffeehouse, coffee shop, or café.")
                .action(ArgAction::Set),
        );

    let expected = str![[r#"
Usage: ctest [OPTIONS]

Options:
  -p, --pos <VAL>
          Some vals

          Possible values:
          - fast
          - slow: not as fast

  -c, --cafe <FILE>
          A coffeehouse, coffee shop, or café.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

"#]];
    utils::assert_output(app, "ctest --help", expected, false);
}

#[cfg(feature = "wrap_help")]
fn setup_aliases() -> Command {
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

#[test]
#[cfg(feature = "wrap_help")]
fn visible_aliases_with_short_help() {
    let app = setup_aliases().term_width(80);

    let expected = str![[r#"
Usage: ctest [OPTIONS] [COMMAND]

Commands:
  rev, -r, --inplace  In place [aliases: -s, -f, -g, --origin, --path,
                      --tryfrom, source, from, onsource]
  help                Print this message or the help of the given subcommand(s)

Options:
  -d, --destination <FILE>  File to save into [aliases: -t, -i, -o, --file,
                            --into, --to]
  -h, --help                Print help (see more with '--help')
  -V, --version             Print version

"#]];
    utils::assert_output(app, "ctest -h", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn visible_aliases_with_long_help() {
    let app = setup_aliases().term_width(80);

    let expected = str![[r#"
Usage: ctest [OPTIONS] [COMMAND]

Commands:
  rev, -r, --inplace  In place [aliases: -s, -f, -g, --origin, --path,
                      --tryfrom, source, from, onsource]
  help                Print this message or the help of the given subcommand(s)

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
fn hidden_possible_vals() {
    let app = Command::new("ctest").arg(
        Arg::new("pos")
            .hide_possible_values(true)
            .value_parser([
                PossibleValue::new("fast"),
                PossibleValue::new("slow").help("not as fast"),
            ])
            .action(ArgAction::Set),
    );

    let expected = str![[r#"
Usage: ctest [pos]

Arguments:
  [pos]  

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(app, "ctest --help", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn issue_626_panic() {
    let cmd = Command::new("ctest")
        .version("0.1")
        .term_width(52)
        .arg(Arg::new("cafe")
           .short('c')
           .long("cafe")
           .value_name("FILE")
           .help("La culture du café est très développée dans de nombreux pays à climat chaud d'Amérique, \
           d'Afrique et d'Asie, dans des plantations qui sont cultivées pour les marchés d'exportation. \
           Le café est souvent une contribution majeure aux exportations des régions productrices.")
           .action(ArgAction::Set));

    let expected = str![[r#"
Usage: ctest [OPTIONS]

Options:
  -c, --cafe <FILE>
          La culture du café est très développée
          dans de nombreux pays à climat chaud
          d'Amérique, d'Afrique et d'Asie, dans des
          plantations qui sont cultivées pour les
          marchés d'exportation. Le café est souvent
          une contribution majeure aux exportations
          des régions productrices.
  -h, --help
          Print help
  -V, --version
          Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn issue_626_variable_panic() {
    for i in 10..320 {
        let _ = Command::new("ctest")
            .version("0.1")
            .term_width(i)
            .arg(Arg::new("cafe")
               .short('c')
               .long("cafe")
               .value_name("FILE")
               .help("La culture du café est très développée dans de nombreux pays à climat chaud d'Amérique, \
               d'Afrique et d'Asie, dans des plantations qui sont cultivées pour les marchés d'exportation. \
               Le café est souvent une contribution majeure aux exportations des régions productrices.")
               .action(ArgAction::Set))
            .try_get_matches_from(vec!["ctest", "--help"]);
    }
}

#[test]
#[cfg(feature = "wrap_help")]
fn final_word_wrapping() {
    let cmd = Command::new("ctest").version("0.1").term_width(24);

    let expected = str![[r#"
Usage: ctest

Options:
  -h, --help
          Print help
  -V, --version
          Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn wrapping_newline_chars() {
    let cmd = Command::new("ctest")
        .version("0.1")
        .term_width(60)
        .arg(Arg::new("mode").help(
            "x, max, maximum   20 characters, contains symbols.\n\
             l, long           Copy-friendly, 14 characters, contains symbols.\n\
             m, med, medium    Copy-friendly, 8 characters, contains symbols.\n",
        ));

    let expected = str![[r#"
Usage: ctest [mode]

Arguments:
  [mode]  x, max, maximum   20 characters, contains symbols.
          l, long           Copy-friendly, 14 characters,
          contains symbols.
          m, med, medium    Copy-friendly, 8 characters,
          contains symbols.

Options:
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn wrapped_indentation() {
    let cmd = Command::new("ctest")
        .version("0.1")
        .term_width(60)
        .arg(Arg::new("mode").help(
            "Some values:
  - l, long           Copy-friendly, 14 characters, contains symbols.
  - m, med, medium    Copy-friendly, 8 characters, contains symbols.",
        ));

    let expected = str![[r#"
Usage: ctest [mode]

Arguments:
  [mode]  Some values:
            - l, long           Copy-friendly, 14
            characters, contains symbols.
            - m, med, medium    Copy-friendly, 8 characters,
            contains symbols.

Options:
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn wrapping_newline_variables() {
    let cmd = Command::new("ctest")
        .version("0.1")
        .term_width(60)
        .arg(Arg::new("mode").help(
            "x, max, maximum   20 characters, contains symbols.{n}\
             l, long           Copy-friendly, 14 characters, contains symbols.{n}\
             m, med, medium    Copy-friendly, 8 characters, contains symbols.{n}",
        ));

    let expected = str![[r#"
Usage: ctest [mode]

Arguments:
  [mode]  x, max, maximum   20 characters, contains symbols.
          l, long           Copy-friendly, 14 characters,
          contains symbols.
          m, med, medium    Copy-friendly, 8 characters,
          contains symbols.

Options:
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn dont_wrap_urls() {
    let cmd = Command::new("Example")
        .term_width(30)
        .subcommand(Command::new("update").arg(
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

#[test]
fn old_newline_chars() {
    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("mode")
            .short('m')
            .action(ArgAction::SetTrue)
            .help("Some help with some wrapping\n(Defaults to something)"),
    );

    let expected = str![[r#"
Usage: ctest [OPTIONS]

Options:
  -m             Some help with some wrapping
                 (Defaults to something)
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
fn old_newline_variables() {
    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("mode")
            .short('m')
            .action(ArgAction::SetTrue)
            .help("Some help with some wrapping{n}(Defaults to something)"),
    );

    let expected = str![[r#"
Usage: ctest [OPTIONS]

Options:
  -m             Some help with some wrapping
                 (Defaults to something)
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn issue_688_hide_pos_vals() {
    #[cfg(not(feature = "unstable-v5"))]
    let expected = str![[r#"
Usage: ctest [OPTIONS]

Options:
      --filter <filter>  Sets the filter, or sampling method, to use for interpolation when resizing the particle
                         images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian,
                         Lanczos3]
  -h, --help             Print help
  -V, --version          Print version

"#]];

    #[cfg(feature = "unstable-v5")]
    let expected = str![[r#"
Usage: ctest [OPTIONS]

Options:
      --filter <filter>  Sets the filter, or sampling method, to use for interpolation when resizing
                         the particle images. The default is Linear (Bilinear). [possible values:
                         Nearest, Linear, Cubic, Gaussian, Lanczos3]
  -h, --help             Print help
  -V, --version          Print version

"#]];

    let filter_values = ["Nearest", "Linear", "Cubic", "Gaussian", "Lanczos3"];

    let app1 = Command::new("ctest")
        .version("0.1")
			.term_width(120)
			.hide_possible_values(true)
			.arg(Arg::new("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
            images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
				.long("filter")
				.value_parser(filter_values)
				.action(ArgAction::Set));

    utils::assert_output(app1, "ctest --help", expected.clone(), false);

    let app2 = Command::new("ctest")
        .version("0.1")
			.term_width(120)
			.arg(Arg::new("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
            images. The default is Linear (Bilinear).")
				.long("filter")
				.value_parser(filter_values)
				.action(ArgAction::Set));

    utils::assert_output(app2, "ctest --help", expected.clone(), false);

    let app3 = Command::new("ctest")
        .version("0.1")
			.term_width(120)
			.arg(Arg::new("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
            images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
				.long("filter")
				.action(ArgAction::Set));

    utils::assert_output(app3, "ctest --help", expected.clone(), false);
}

#[test]
fn issue_702_multiple_values() {
    let cmd = Command::new("myapp")
        .version("1.0")
        .author("foo")
        .about("bar")
        .arg(Arg::new("arg1").help("some option"))
        .arg(
            Arg::new("arg2")
                .action(ArgAction::Set)
                .num_args(1..)
                .help("some option"),
        )
        .arg(
            Arg::new("some")
                .help("some option")
                .short('s')
                .long("some")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("other")
                .help("some other option")
                .short('o')
                .long("other")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("label")
                .help("a label")
                .short('l')
                .long("label")
                .num_args(1..)
                .action(ArgAction::Set),
        );

    let expected = str![[r#"
bar

Usage: myapp [OPTIONS] [arg1] [arg2]...

Arguments:
  [arg1]     some option
  [arg2]...  some option

Options:
  -s, --some <some>       some option
  -o, --other <other>     some other option
  -l, --label <label>...  a label
  -h, --help              Print help
  -V, --version           Print version

"#]];
    utils::assert_output(cmd, "myapp --help", expected, false);
}

#[test]
fn long_about() {
    let cmd = Command::new("myapp")
        .version("1.0")
        .author("foo")
        .about("bar")
        .long_about(
            "something really really long, with\nmultiple lines of text\nthat should be displayed",
        )
        .arg(Arg::new("arg1").help("some option"));

    let expected = str![[r#"
something really really long, with
multiple lines of text
that should be displayed

Usage: myapp [arg1]

Arguments:
  [arg1]
          some option

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

"#]];
    utils::assert_output(cmd, "myapp --help", expected, false);
}

#[test]
fn explicit_short_long_help() {
    let cmd = Command::new("myapp")
        .disable_help_flag(true)
        .version("1.0")
        .author("foo")
        .about("bar")
        .long_about(
            "something really really long, with\nmultiple lines of text\nthat should be displayed",
        )
        .arg(Arg::new("arg1").help("some option"))
        .arg(Arg::new("short").short('?').action(ArgAction::HelpShort))
        .arg(
            Arg::new("long")
                .short('h')
                .long("help")
                .action(ArgAction::HelpLong),
        );

    let expected = str![[r#"
bar

Usage: myapp [arg1]

Arguments:
  [arg1]  some option

Options:
  -?             
  -h, --help     
  -V, --version  Print version

"#]];
    utils::assert_output(cmd.clone(), "myapp -?", expected, false);

    let expected = str![[r#"
something really really long, with
multiple lines of text
that should be displayed

Usage: myapp [arg1]

Arguments:
  [arg1]
          some option

Options:
  -?
          

  -h, --help
          

  -V, --version
          Print version

"#]];
    utils::assert_output(cmd.clone(), "myapp -h", expected, false);

    let expected = str![[r#"
something really really long, with
multiple lines of text
that should be displayed

Usage: myapp [arg1]

Arguments:
  [arg1]
          some option

Options:
  -?
          

  -h, --help
          

  -V, --version
          Print version

"#]];
    utils::assert_output(cmd, "myapp --help", expected, false);
}

#[test]
fn ripgrep_usage() {
    let cmd = Command::new("ripgrep").version("0.5").override_usage(
        "rg [OPTIONS] <pattern> [<path> ...]
       rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
       rg [OPTIONS] --files [<path> ...]
       rg [OPTIONS] --type-list",
    );

    let expected = str![[r#"
Usage: rg [OPTIONS] <pattern> [<path> ...]
       rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
       rg [OPTIONS] --files [<path> ...]
       rg [OPTIONS] --type-list

Options:
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "rg --help", expected, false);
}

#[test]
fn ripgrep_usage_using_templates() {
    #[cfg(not(feature = "unstable-v5"))]
    let cmd = Command::new("ripgrep")
        .version("0.5")
        .override_usage(
            "\
       rg [OPTIONS] <pattern> [<path> ...]
       rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
       rg [OPTIONS] --files [<path> ...]
       rg [OPTIONS] --type-list",
        )
        .help_template(
            "\
{bin} {version}

Usage: {usage}

Options:
{options}",
        );

    #[cfg(feature = "unstable-v5")]
    let cmd = Command::new("ripgrep")
        .version("0.5")
        .override_usage(
            "\
       rg [OPTIONS] <pattern> [<path> ...]
       rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
       rg [OPTIONS] --files [<path> ...]
       rg [OPTIONS] --type-list",
        )
        .help_template(
            "\
{name} {version}

Usage: {usage}

Options:
{options}",
        );

    let expected = str![[r#"
ripgrep 0.5

Usage: rg [OPTIONS] <pattern> [<path> ...]
       rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
       rg [OPTIONS] --files [<path> ...]
       rg [OPTIONS] --type-list

Options:
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "rg --help", expected, false);
}

#[test]
fn sc_negates_reqs() {
    let cmd = Command::new("prog")
        .version("1.0")
        .subcommand_negates_reqs(true)
        .arg(arg!(-o --opt <FILE> "tests options").required(true))
        .arg(Arg::new("PATH").help("help"))
        .subcommand(Command::new("test"));

    let expected = str![[r#"
Usage: prog --opt <FILE> [PATH]
       prog [PATH] <COMMAND>

Commands:
  test  
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  help

Options:
  -o, --opt <FILE>  tests options
  -h, --help        Print help
  -V, --version     Print version

"#]];
    utils::assert_output(cmd, "prog --help", expected, false);
}

#[test]
fn hide_args() {
    let cmd = Command::new("prog")
        .version("1.0")
        .arg(arg!(-f --flag "testing flags"))
        .arg(arg!(-o --opt <FILE> "tests options"))
        .arg(Arg::new("pos").hide(true));

    let expected = str![[r#"
Usage: prog [OPTIONS]

Options:
  -f, --flag        testing flags
  -o, --opt <FILE>  tests options
  -h, --help        Print help
  -V, --version     Print version

"#]];
    utils::assert_output(cmd, "prog --help", expected, false);
}

#[test]
fn args_negate_sc() {
    let cmd = Command::new("prog")
        .version("1.0")
        .args_conflicts_with_subcommands(true)
        .arg(arg!(-f --flag "testing flags"))
        .arg(arg!(-o --opt <FILE> "tests options"))
        .arg(Arg::new("PATH").help("help"))
        .subcommand(Command::new("test"));

    let expected = str![[r#"
Usage: prog [OPTIONS] [PATH]
       prog <COMMAND>

Commands:
  test  
  help  Print this message or the help of the given subcommand(s)

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

#[test]
fn issue_1046_hide_scs() {
    let cmd = Command::new("prog")
        .version("1.0")
        .arg(arg!(-f --flag "testing flags"))
        .arg(arg!(-o --opt <FILE> "tests options"))
        .arg(Arg::new("PATH").help("some"))
        .subcommand(Command::new("test").hide(true));

    let expected = str![[r#"
Usage: prog [OPTIONS] [PATH]

Arguments:
  [PATH]  some

Options:
  -f, --flag        testing flags
  -o, --opt <FILE>  tests options
  -h, --help        Print help
  -V, --version     Print version

"#]];
    utils::assert_output(cmd, "prog --help", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn issue_777_wrap_all_things() {
    let cmd = Command::new("A cmd with a crazy very long long long name hahaha")
        .version("1.0")
        .author("Some Very Long Name and crazy long email <email@server.com>")
        .about("Show how the about text is not wrapped")
        .help_template(utils::FULL_TEMPLATE)
        .term_width(35);

    let expected = str![[r#"
A cmd with a crazy very long long
long name hahaha 1.0
Some Very Long Name and crazy long
email <email@server.com>
Show how the about text is not
wrapped

Usage: ctest

Options:
  -h, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "ctest --help", expected, false);
}

#[test]
fn dont_strip_padding_issue_5083() {
    let cmd = Command::new("test")
        .help_template("{subcommands}")
        .subcommands([
            Command::new("one"),
            Command::new("two"),
            Command::new("three"),
        ]);

    let expected = str![[r#"
  one    
  two    
  three  
  help   Print this message or the help of the given subcommand(s)

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn override_help_short() {
    let cmd = Command::new("test")
        .version("0.1")
        .arg(arg!(-H --help "Print help").action(ArgAction::Help))
        .disable_help_flag(true);

    let expected = str![[r#"
Usage: test

Options:
  -H, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd.clone(), "test --help", expected, false);

    let expected = str![[r#"
Usage: test

Options:
  -H, --help     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "test -H", expected, false);
}

#[test]
fn override_help_long() {
    let cmd = Command::new("test")
        .version("0.1")
        .arg(arg!(-h --hell "Print help").action(ArgAction::Help))
        .disable_help_flag(true);

    let expected = str![[r#"
Usage: test

Options:
  -h, --hell     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd.clone(), "test --hell", expected, false);

    let expected = str![[r#"
Usage: test

Options:
  -h, --hell     Print help
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "test -h", expected, false);
}

#[test]
fn override_help_about() {
    let cmd = Command::new("test")
        .version("0.1")
        .arg(arg!(-h --help "Print custom help information").action(ArgAction::Help))
        .disable_help_flag(true);

    let expected = str![[r#"
Usage: test

Options:
  -h, --help     Print custom help information
  -V, --version  Print version

"#]];
    utils::assert_output(cmd.clone(), "test --help", expected, false);

    let expected = str![[r#"
Usage: test

Options:
  -h, --help     Print custom help information
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "test -h", expected, false);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "Command conflict: Argument names must be unique, but 'help' is in use by more than one argument or group (call `cmd.disable_help_flag(true)` to remove the auto-generated `--help`)"]
fn arg_id_conflict_with_help() {
    Command::new("conflict")
        .arg(Arg::new("help").short('?').action(ArgAction::SetTrue))
        .build();
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "Command conflict: Short option names must be unique for each argument, but '-h' is in use by both 'home' and 'help' (call `cmd.disable_help_flag(true)` to remove the auto-generated `--help`)"]
fn arg_short_conflict_with_help() {
    Command::new("conflict")
        .arg(Arg::new("home").short('h').action(ArgAction::SetTrue))
        .build();
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "Command conflict: Long option names must be unique for each argument, but '--help' is in use by both 'custom-help' and 'help' (call `cmd.disable_help_flag(true)` to remove the auto-generated `--help`)"]
fn arg_long_conflict_with_help() {
    Command::new("conflict")
        .arg(
            Arg::new("custom-help")
                .long("help")
                .action(ArgAction::SetTrue),
        )
        .build();
}

#[test]
fn last_arg_mult_usage() {
    let cmd = Command::new("last")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(Arg::new("CORPUS").help("some"))
        .arg(
            Arg::new("ARGS")
                .action(ArgAction::Set)
                .num_args(1..)
                .last(true)
                .help("some"),
        );

    let expected = str![[r#"
Usage: last <TARGET> [CORPUS] [-- <ARGS>...]

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

#[test]
fn last_arg_mult_usage_req() {
    let cmd = Command::new("last")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(Arg::new("CORPUS").help("some"))
        .arg(
            Arg::new("ARGS")
                .action(ArgAction::Set)
                .num_args(1..)
                .last(true)
                .required(true)
                .help("some"),
        );

    let expected = str![[r#"
Usage: last <TARGET> [CORPUS] -- <ARGS>...

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
        .subcommand(Command::new("test").about("some"));

    let expected = str![[r#"
Usage: last <TARGET> [CORPUS] -- <ARGS>...
       last [TARGET] [CORPUS] <COMMAND>

Commands:
  test  some
  help  Print this message or the help of the given subcommand(s)

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
        .subcommand(Command::new("test").about("some"));

    let expected = str![[r#"
Usage: last <TARGET> [CORPUS] [-- <ARGS>...]
       last <COMMAND>

Commands:
  test  some
  help  Print this message or the help of the given subcommand(s)

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

#[test]
fn hide_default_val() {
    let app1 = Command::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .help("Pass an argument to the program. [default: default-argument]")
            .long("arg")
            .default_value("default-argument")
            .hide_default_value(true),
    );

    let expected = str![[r#"
Usage: default [OPTIONS]

Options:
      --arg <argument>  Pass an argument to the program. [default: default-argument]
  -h, --help            Print help
  -V, --version         Print version

"#]];
    utils::assert_output(app1, "default --help", expected, false);

    let app2 = Command::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .help("Pass an argument to the program.")
            .long("arg")
            .default_value("default-argument"),
    );

    let expected = str![[r#"
Usage: default [OPTIONS]

Options:
      --arg <argument>  Pass an argument to the program. [default: default-argument]
  -h, --help            Print help
  -V, --version         Print version

"#]];
    utils::assert_output(app2, "default --help", expected, false);
}

#[test]
fn empty_default_value() {
    let app = Command::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .help("Pass an argument to the program.")
            .long("arg")
            .default_value(""),
    );

    let expected = str![[r#"
Usage: default [OPTIONS]

Options:
      --arg <argument>  Pass an argument to the program. [default: ""]
  -h, --help            Print help
  -V, --version         Print version

"#]];
    utils::assert_output(app, "default --help", expected, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn escaped_whitespace_values() {
    let app1 = Command::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .help("Pass an argument to the program.")
            .long("arg")
            .default_value("\n")
            .value_parser(["normal", " ", "\n", "\t", "other"]),
    );

    #[cfg(not(feature = "unstable-v5"))]
    let expected = str![[r#"
Usage: default [OPTIONS]

Options:
      --arg <argument>  Pass an argument to the program. [default: "\n"] [possible values: normal, " ", "\n", "\t",
                        other]
  -h, --help            Print help
  -V, --version         Print version

"#]];

    #[cfg(feature = "unstable-v5")]
    let expected = str![[r#"
Usage: default [OPTIONS]

Options:
      --arg <argument>  Pass an argument to the program. [default: "\n"] [possible values: normal, "
                        ", "\n", "\t", other]
  -h, --help            Print help
  -V, --version         Print version

"#]];

    utils::assert_output(app1, "default --help", expected, false);
}

fn issue_1112_setup() -> Command {
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
                    .long("help")
                    .short('h')
                    .help("some help")
                    .action(ArgAction::SetTrue),
            ),
        )
}

#[test]
fn prefer_user_help_long_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "--help"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert!(*m.get_one::<bool>("help1").expect("defaulted by clap"));
}

#[test]
fn prefer_user_help_short_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "-h"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert!(*m.get_one::<bool>("help1").expect("defaulted by clap"));
}

#[test]
fn prefer_user_subcmd_help_long_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "foo", "--help"]);

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
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "foo", "-h"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();
    assert!(m
        .subcommand_matches("foo")
        .unwrap()
        .get_one::<bool>("help1")
        .expect("defaulted by clap"));
}

#[test]
fn issue_1052_require_delim_help() {
    let cmd = Command::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .arg(
            arg!(-f --fake <s> "some help")
                .required(true)
                .value_names(["some", "val"])
                .action(ArgAction::Set)
                .value_delimiter(':'),
        );

    let expected = str![[r#"
tests stuff

Usage: test --fake <some> <val>

Options:
  -f, --fake <some> <val>  some help
  -h, --help               Print help
  -V, --version            Print version

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn custom_headers_headers() {
    let cmd = Command::new("blorp")
        .author("Will M.")
        .about("does stuff")
        .version("1.4")
        .arg(
            arg!(-f --fake <s> "some help")
                .required(true)
                .value_names(["some", "val"])
                .action(ArgAction::Set)
                .value_delimiter(':'),
        )
        .next_help_heading(Some("NETWORKING"))
        .arg(
            Arg::new("no-proxy")
                .short('n')
                .long("no-proxy")
                .action(ArgAction::SetTrue)
                .help("Do not use system proxy settings"),
        )
        .args([Arg::new("port").long("port").action(ArgAction::SetTrue)]);

    let expected = str![[r#"
does stuff

Usage: test [OPTIONS] --fake <some> <val>

Options:
  -f, --fake <some> <val>  some help
  -h, --help               Print help
  -V, --version            Print version

NETWORKING:
  -n, --no-proxy  Do not use system proxy settings
      --port

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn multiple_custom_help_headers() {
    let cmd = Command::new("blorp")
        .author("Will M.")
        .about("does stuff")
        .version("1.4")
        .arg(
            arg!(-f --fake <s> "some help")
                .required(true)
                .value_names(["some", "val"])
                .action(ArgAction::Set)
                .value_delimiter(':'),
        )
        .next_help_heading(Some("NETWORKING"))
        .arg(
            Arg::new("no-proxy")
                .short('n')
                .long("no-proxy")
                .action(ArgAction::SetTrue)
                .help("Do not use system proxy settings"),
        )
        .next_help_heading(Some("SPECIAL"))
        .arg(
            arg!(-b --"birthday-song" <song> "Change which song is played for birthdays")
                .required(true)
                .help_heading(Some("OVERRIDE SPECIAL")),
        )
        .arg(arg!(--style <style> "Choose musical style to play the song").help_heading(None))
        .arg(
            arg!(
                -v --"birthday-song-volume" <volume> "Change the volume of the birthday song"
            )
            .required(true),
        )
        .next_help_heading(None)
        .arg(
            Arg::new("server-addr")
                .short('a')
                .long("server-addr")
                .action(ArgAction::SetTrue)
                .help("Set server address")
                .help_heading(Some("NETWORKING")),
        )
        .arg(
            Arg::new("speed")
                .long("speed")
                .short('s')
                .value_name("SPEED")
                .value_parser(["fast", "slow"])
                .help("How fast?")
                .action(ArgAction::Set),
        );

    let expected = str![[r#"
does stuff

Usage: test [OPTIONS] --fake <some> <val> --birthday-song <song> --birthday-song-volume <volume>

Options:
  -f, --fake <some> <val>  some help
      --style <style>      Choose musical style to play the song
  -s, --speed <SPEED>      How fast? [possible values: fast, slow]
  -h, --help               Print help
  -V, --version            Print version

NETWORKING:
  -n, --no-proxy     Do not use system proxy settings
  -a, --server-addr  Set server address

OVERRIDE SPECIAL:
  -b, --birthday-song <song>  Change which song is played for birthdays

SPECIAL:
  -v, --birthday-song-volume <volume>  Change the volume of the birthday song

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn custom_help_headers_hide_args() {
    let cmd = Command::new("blorp")
        .author("Will M.")
        .about("does stuff")
        .version("1.4")
        .next_help_heading(Some("NETWORKING"))
        .arg(
            Arg::new("no-proxy")
                .short('n')
                .long("no-proxy")
                .help("Do not use system proxy settings")
                .hide_short_help(true),
        )
        .next_help_heading(Some("SPECIAL"))
        .arg(
            arg!(-b --song <song> "Change which song is played for birthdays")
                .required(true)
                .help_heading(Some("OVERRIDE SPECIAL")),
        )
        .arg(
            arg!(
                -v --"song-volume" <volume> "Change the volume of the birthday song"
            )
            .required(true),
        )
        .next_help_heading(None)
        .arg(
            Arg::new("server-addr")
                .short('a')
                .long("server-addr")
                .help("Set server address")
                .help_heading(Some("NETWORKING"))
                .hide_short_help(true),
        );

    let expected = str![[r#"
does stuff

Usage: test [OPTIONS] --song <song> --song-volume <volume>

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version

OVERRIDE SPECIAL:
  -b, --song <song>  Change which song is played for birthdays

SPECIAL:
  -v, --song-volume <volume>  Change the volume of the birthday song

"#]];
    utils::assert_output(cmd, "test -h", expected, false);
}

#[test]
fn show_long_about_issue_897() {
    let cmd = Command::new("ctest").version("0.1").subcommand(
        Command::new("foo")
            .version("0.1")
            .about("About foo")
            .long_about("Long about foo"),
    );

    let expected = str![[r#"
Long about foo

Usage: ctest foo

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

"#]];
    utils::assert_output(cmd, "ctest foo --help", expected, false);
}

#[test]
fn show_short_about_issue_897() {
    let cmd = Command::new("ctest").version("0.1").subcommand(
        Command::new("foo")
            .version("0.1")
            .about("About foo")
            .long_about("Long about foo"),
    );

    let expected = str![[r#"
About foo

Usage: ctest foo

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version

"#]];
    utils::assert_output(cmd, "ctest foo -h", expected, false);
}

#[test]
fn issue_1364_no_short_options() {
    let cmd = Command::new("demo")
        .arg(Arg::new("foo").short('f').action(ArgAction::SetTrue))
        .arg(
            Arg::new("baz")
                .short('z')
                .value_name("BAZ")
                .hide_short_help(true),
        )
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .action(ArgAction::Set)
                .num_args(1..),
        );

    let expected = str![[r#"
Usage: demo [OPTIONS] [FILES]...

Arguments:
  [FILES]...  

Options:
  -f          
  -h, --help  Print help (see more with '--help')

"#]];
    utils::assert_output(cmd, "demo -h", expected, false);
}

#[test]
fn short_with_value() {
    let cmd = Command::new("demo").arg(
        Arg::new("baz")
            .short('z')
            .value_name("BAZ")
            .help("Short only")
            .help_heading("Baz"),
    );

    let expected = str![[r#"
Usage: demo [OPTIONS]

Options:
  -h, --help  Print help

Baz:
  -z <BAZ>  Short only

"#]];
    utils::assert_output(cmd, "demo -h", expected, false);
}

#[test]
fn short_with_count() {
    let cmd = Command::new("demo").arg(
        Arg::new("baz")
            .short('z')
            .action(ArgAction::Count)
            .help("Short only")
            .help_heading("Baz"),
    );

    let expected = str![[r#"
Usage: demo [OPTIONS]

Options:
  -h, --help  Print help

Baz:
  -z...  Short only

"#]];
    utils::assert_output(cmd, "demo -h", expected, false);
}

#[test]
fn issue_1487() {
    let cmd = Command::new("test")
        .arg(Arg::new("arg1").group("group1"))
        .arg(Arg::new("arg2").group("group1"))
        .group(
            ArgGroup::new("group1")
                .args(["arg1", "arg2"])
                .required(true),
        );

    let expected = str![[r#"
Usage: ctest <arg1|arg2>

Arguments:
  [arg1]  
  [arg2]  

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "ctest -h", expected, false);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command::help_expected is enabled for the Command"]
fn help_required_but_not_given() {
    Command::new("myapp")
        .help_expected(true)
        .arg(Arg::new("foo"))
        .try_get_matches_from(empty_args())
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command::help_expected is enabled for the Command"]
fn help_required_but_not_given_settings_after_args() {
    Command::new("myapp")
        .arg(Arg::new("foo"))
        .help_expected(true)
        .try_get_matches_from(empty_args())
        .unwrap();
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command::help_expected is enabled for the Command"]
fn help_required_but_not_given_for_one_of_two_arguments() {
    Command::new("myapp")
        .help_expected(true)
        .arg(Arg::new("foo"))
        .arg(Arg::new("bar").help("It does bar stuff"))
        .try_get_matches_from(empty_args())
        .unwrap();
}

#[test]
#[should_panic = "List of such arguments: delete"]
fn help_required_globally() {
    Command::new("myapp")
        .help_expected(true)
        .arg(Arg::new("foo").help("It does foo stuff"))
        .subcommand(
            Command::new("bar")
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
            Command::new("bar")
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
            Command::new("bar")
                .arg(Arg::new("create").help("creates bar"))
                .arg(Arg::new("delete").help("deletes bar")),
        )
        .try_get_matches_from(empty_args())
        .unwrap();
}

#[test]
fn help_required_and_given() {
    Command::new("myapp")
        .help_expected(true)
        .arg(Arg::new("foo").help("It does foo stuff"))
        .try_get_matches_from(empty_args())
        .unwrap();
}

#[test]
fn help_required_and_no_args() {
    Command::new("myapp")
        .help_expected(true)
        .try_get_matches_from(empty_args())
        .unwrap();
}

#[test]
fn issue_1642_long_help_spacing() {
    let cmd = Command::new("prog").arg(
        Arg::new("cfg")
            .long("config")
            .action(ArgAction::SetTrue)
            .long_help(
                "The config file used by the myprog must be in JSON format
with only valid keys and may not contain other nonsense
that cannot be read by this program. Obviously I'm going on
and on, so I'll stop now.",
            ),
    );

    let expected = str![[r#"
Usage: prog [OPTIONS]

Options:
      --config
          The config file used by the myprog must be in JSON format
          with only valid keys and may not contain other nonsense
          that cannot be read by this program. Obviously I'm going on
          and on, so I'll stop now.

  -h, --help
          Print help (see a summary with '-h')

"#]];
    utils::assert_output(cmd, "prog --help", expected, false);
}

const AFTER_HELP_NO_ARGS: &str = "\
Usage: myapp

This is after help.
";

#[test]
fn after_help_no_args() {
    let mut cmd = Command::new("myapp")
        .version("1.0")
        .disable_help_flag(true)
        .disable_version_flag(true)
        .after_help("This is after help.");

    let help = cmd.render_help().to_string();

    assert_eq!(help, AFTER_HELP_NO_ARGS);
}

#[test]
fn help_subcmd_help() {
    let cmd = Command::new("myapp")
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

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
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    let expected = str![[r#"
Print this message or the help of the given subcommand(s)

Usage: myapp subcmd help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd.clone(), "myapp subcmd help help", expected, false);
}

#[test]
fn global_args_should_show_on_toplevel_help_message() {
    let cmd = Command::new("myapp")
        .arg(
            Arg::new("someglobal")
                .short('g')
                .long("some-global")
                .global(true),
        )
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    let expected = str![[r#"
Usage: myapp [OPTIONS] [COMMAND]

Commands:
  subcmd  
  help    Print this message or the help of the given subcommand(s)

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
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    let expected = str![[r#"
Print this message or the help of the given subcommand(s)

Usage: myapp help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)

"#]];
    utils::assert_output(cmd, "myapp help help", expected, false);
}

#[test]
fn global_args_should_show_on_help_message_for_subcommand() {
    let cmd = Command::new("myapp")
        .arg(
            Arg::new("someglobal")
                .short('g')
                .long("some-global")
                .global(true),
        )
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    let expected = str![[r#"
Usage: myapp subcmd [OPTIONS] [COMMAND]

Commands:
  multi  
  help   Print this message or the help of the given subcommand(s)

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
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    let expected = str![[r#"
Usage: myapp subcmd multi [OPTIONS]

Options:
  -g, --some-global <someglobal>  
  -h, --help                      Print help
  -V, --version                   Print version

"#]];
    utils::assert_output(cmd, "myapp help subcmd multi", expected, false);
}

#[test]
fn option_usage_order() {
    let cmd = Command::new("order").args([
        Arg::new("a").short('a').action(ArgAction::SetTrue),
        Arg::new("B").short('B').action(ArgAction::SetTrue),
        Arg::new("b").short('b').action(ArgAction::SetTrue),
        Arg::new("save").short('s').action(ArgAction::SetTrue),
        Arg::new("select_file")
            .long("select_file")
            .action(ArgAction::SetTrue),
        Arg::new("select_folder")
            .long("select_folder")
            .action(ArgAction::SetTrue),
        Arg::new("x").short('x').action(ArgAction::SetTrue),
    ]);

    let expected = str![[r#"
Usage: order [OPTIONS]

Options:
  -a                   
  -B                   
  -b                   
  -s                   
      --select_file    
      --select_folder  
  -x                   
  -h, --help           Print help

"#]];
    utils::assert_output(cmd, "order --help", expected, false);
}

#[test]
fn prefer_about_over_long_about_in_subcommands_list() {
    let cmd = Command::new("about-in-subcommands-list").subcommand(
        Command::new("sub")
            .long_about("long about sub")
            .about("short about sub"),
    );

    let expected = str![[r#"
Usage: about-in-subcommands-list [COMMAND]

Commands:
  sub   short about sub
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "about-in-subcommands-list --help", expected, false);
}

#[test]
fn issue_1794_usage() {
    let cmd = Command::new("hello")
        .bin_name("deno")
        .arg(
            Arg::new("option1")
                .long("option1")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("pos1").action(ArgAction::Set))
        .group(
            ArgGroup::new("arg1")
                .args(["pos1", "option1"])
                .required(true),
        )
        .arg(Arg::new("pos2").action(ArgAction::Set));

    let expected = str![[r#"
Usage: deno <pos1|--option1> [pos2]

Arguments:
  [pos1]  
  [pos2]  

Options:
      --option1  
  -h, --help     Print help

"#]];
    utils::assert_output(cmd, "deno --help", expected, false);
}

#[test]
fn custom_heading_pos() {
    let cmd = Command::new("test")
        .version("1.4")
        .arg(Arg::new("gear").help("Which gear"))
        .next_help_heading(Some("NETWORKING"))
        .arg(Arg::new("speed").help("How fast"));

    let expected = str![[r#"
Usage: test [gear] [speed]

Arguments:
  [gear]  Which gear

Options:
  -h, --help     Print help
  -V, --version  Print version

NETWORKING:
  [speed]  How fast

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn only_custom_heading_opts_no_args() {
    let cmd = Command::new("test")
        .version("1.4")
        .disable_version_flag(true)
        .disable_help_flag(true)
        .arg(arg!(--help).action(ArgAction::Help).hide(true))
        .next_help_heading(Some("NETWORKING"))
        .arg(arg!(-s --speed <SPEED> "How fast"));

    let expected = str![[r#"
Usage: test [OPTIONS]

NETWORKING:
  -s, --speed <SPEED>  How fast

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn only_custom_heading_pos_no_args() {
    let cmd = Command::new("test")
        .version("1.4")
        .disable_version_flag(true)
        .disable_help_flag(true)
        .arg(arg!(--help).action(ArgAction::Help).hide(true))
        .next_help_heading(Some("NETWORKING"))
        .arg(Arg::new("speed").help("How fast"));

    let expected = str![[r#"
Usage: test [speed]

NETWORKING:
  [speed]  How fast

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn issue_2508_number_of_values_with_single_value_name() {
    let cmd = Command::new("my_app")
        .arg(Arg::new("some_arg").long("some_arg").num_args(2))
        .arg(
            Arg::new("some_arg_issue")
                .long("some_arg_issue")
                .num_args(2)
                .value_name("ARG"),
        );

    let expected = str![[r#"
Usage: my_app [OPTIONS]

Options:
      --some_arg <some_arg> <some_arg>  
      --some_arg_issue <ARG> <ARG>      
  -h, --help                            Print help

"#]];
    utils::assert_output(cmd, "my_app --help", expected, false);
}

#[test]
fn missing_positional_final_required() {
    let cmd = Command::new("test")
        .allow_missing_positional(true)
        .arg(Arg::new("arg1"))
        .arg(Arg::new("arg2").required(true));

    let expected = str![[r#"
Usage: test [arg1] <arg2>

Arguments:
  [arg1]  
  <arg2>  

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn missing_positional_final_multiple() {
    let cmd = Command::new("test")
        .allow_missing_positional(true)
        .arg(Arg::new("foo"))
        .arg(Arg::new("bar"))
        .arg(Arg::new("baz").action(ArgAction::Set).num_args(1..));

    let expected = str![[r#"
Usage: test [foo] [bar] [baz]...

Arguments:
  [foo]     
  [bar]     
  [baz]...  

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn positional_multiple_values_is_dotted() {
    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .action(ArgAction::Set)
            .num_args(1..),
    );

    let expected = str![[r#"
Usage: test <foo>...

Arguments:
  <foo>...  

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "test --help", expected, false);

    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .action(ArgAction::Set)
            .value_name("BAR")
            .num_args(1..),
    );

    let expected = str![[r#"
Usage: test <BAR>...

Arguments:
  <BAR>...  

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn positional_multiple_occurrences_is_dotted() {
    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .action(ArgAction::Set)
            .num_args(1..)
            .action(ArgAction::Append),
    );

    let expected = str![[r#"
Usage: test <foo>...

Arguments:
  <foo>...  

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "test --help", expected, false);

    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .action(ArgAction::Set)
            .value_name("BAR")
            .num_args(1..)
            .action(ArgAction::Append),
    );

    let expected = str![[r#"
Usage: test <BAR>...

Arguments:
  <BAR>...  

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn too_few_value_names_is_dotted() {
    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .long("foo")
            .required(true)
            .action(ArgAction::Set)
            .num_args(3)
            .value_names(["one", "two"]),
    );

    let expected = str![[r#"
Usage: test --foo <one> <two>...

Options:
      --foo <one> <two>...  
  -h, --help                Print help

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
#[should_panic = "Argument foo: Too many value names (2) compared to `num_args` (1)"]
fn too_many_value_names_panics() {
    Command::new("test")
        .arg(
            Arg::new("foo")
                .long("foo")
                .required(true)
                .action(ArgAction::Set)
                .num_args(1)
                .value_names(["one", "two"]),
        )
        .debug_assert();
}

#[test]
fn help_enum_arg_with_no_description() {
    let cmd = Command::new("test").arg(
        Arg::new("config")
            .action(ArgAction::Set)
            // .help("No help description for this argument")
            .short('c')
            .long("config")
            .value_name("MODE")
            .value_parser([
                PossibleValue::new("fast"),
                PossibleValue::new("slow").help("slower than fast"),
                PossibleValue::new("secret speed").hide(true),
            ])
            .default_value("fast"),
    );

    let expected = str![[r#"
Usage: test [OPTIONS]

Options:
  -c, --config <MODE>
          Possible values:
          - fast
          - slow: slower than fast
          
          [default: fast]

  -h, --help
          Print help (see a summary with '-h')

"#]];
    utils::assert_output(cmd, "test --help", expected, false);
}

#[test]
fn disabled_help_flag() {
    let res = Command::new("foo")
        .subcommand(Command::new("sub"))
        .disable_help_flag(true)
        .try_get_matches_from("foo a".split(' '));
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidSubcommand);
}

#[test]
fn disabled_help_flag_and_subcommand() {
    let res = Command::new("foo")
        .subcommand(Command::new("sub"))
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
        .subcommand(Command::new("help").arg(Arg::new("arg").action(ArgAction::Set)))
        .subcommand(Command::new("not_help").arg(Arg::new("arg").action(ArgAction::Set)))
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
        .subcommand(Command::new("help").long_flag("help"))
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
        .subcommand(Command::new("help").short_flag('h'));
    let matches = cmd.try_get_matches_from(["foo", "-h"]).unwrap();
    assert!(matches.subcommand_matches("help").is_some());
}

#[test]
fn subcommand_help_doesnt_have_useless_help_flag() {
    // The main care-about is that the docs and behavior match.  Since the `help` subcommand
    // currently ignores the `--help` flag, the output shouldn't have it.
    let cmd = Command::new("example").subcommand(Command::new("test").about("Subcommand"));

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
        .subcommand(Command::new("test").about("Subcommand"));
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
        .subcommand(Command::new("subcommand"));

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
fn help_without_short() {
    let mut cmd = Command::new("test")
        .arg(arg!(-h --hex <NUM>).required(true))
        .arg(arg!(--help).action(ArgAction::Help))
        .disable_help_flag(true);

    cmd.build();
    let help = cmd.get_arguments().find(|a| a.get_id() == "help").unwrap();
    assert_eq!(help.get_short(), None);

    let m = cmd.try_get_matches_from(["test", "-h", "0x100"]).unwrap();
    assert_eq!(
        m.get_one::<String>("hex").map(|v| v.as_str()),
        Some("0x100")
    );
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
        .subcommand(Command::new("test").about("some"));

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
        .subcommand(Command::new("test").about("some"));

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
        .subcommand(Command::new("test").about("some"));
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
        .subcommand(Command::new("subcmd"));

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
        .subcommand(Command::new("subcmd"));

    let expected = str![[r#"
Usage: ctest subcmd

Options:
  -h, --help  Print help

"#]];
    utils::assert_output(cmd, "ctest subcmd --help", expected, false);
}

#[test]
fn no_wrap_help() {
    static MULTI_SC_HELP: &str = "\
tests subcommands

Usage: ctest subcmd multi [OPTIONS]

Options:
  -f, --flag                  tests flags
  -o, --option <scoption>...  tests options
  -h, --help                  Print help
  -V, --version               Print version
";
    let cmd = Command::new("ctest")
        .term_width(0)
        .override_help(MULTI_SC_HELP);

    utils::assert_output(cmd, "ctest --help", MULTI_SC_HELP, false);
}

#[test]
fn display_name_default() {
    let mut cmd = Command::new("app").bin_name("app.exe");
    cmd.build();
    assert_eq!(cmd.get_display_name(), None);
}

#[test]
fn display_name_explicit() {
    let mut cmd = Command::new("app")
        .bin_name("app.exe")
        .display_name("app.display");
    cmd.build();
    assert_eq!(cmd.get_display_name(), Some("app.display"));
}

#[test]
fn display_name_subcommand_default() {
    let mut cmd = Command::new("parent").subcommand(Command::new("child").bin_name("child.exe"));
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
            .bin_name("child.exe")
            .display_name("child.display"),
    );
    cmd.build();
    assert_eq!(
        cmd.find_subcommand("child").unwrap().get_display_name(),
        Some("child.display")
    );
}

#[test]
fn flatten_basic() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
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

#[test]
fn flatten_with_global() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent").global(true))
        .subcommand(
            Command::new("test")
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
fn flatten_arg_required() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent").required(true))
        .subcommand(
            Command::new("test")
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

#[test]
fn flatten_with_external_subcommand() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .allow_external_subcommands(true)
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
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
fn flatten_without_subcommands() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"));

    let expected = str![[r#"
parent command

Usage: parent [OPTIONS]

Options:
      --parent <parent>  
  -h, --help             Print help

"#]];
    utils::assert_output(cmd, "parent -h", expected, false);
}

#[test]
fn flatten_with_subcommand_required() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .subcommand_required(true)
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("test")
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

#[test]
fn flatten_hidden_command() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .about("child1 command")
                .arg(Arg::new("child").long("child1")),
        )
        .subcommand(
            Command::new("child2")
                .about("child2 command")
                .arg(Arg::new("child").long("child2")),
        )
        .subcommand(
            Command::new("child3")
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

#[test]
fn flatten_recursive() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .flatten_help(true)
                .about("child1 command")
                .arg(Arg::new("child").long("child1"))
                .subcommand(
                    Command::new("grandchild1")
                        .flatten_help(true)
                        .about("grandchild1 command")
                        .arg(Arg::new("grandchild").long("grandchild1"))
                        .subcommand(
                            Command::new("greatgrandchild1")
                                .about("greatgrandchild1 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild1")),
                        )
                        .subcommand(
                            Command::new("greatgrandchild2")
                                .about("greatgrandchild2 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild2")),
                        )
                        .subcommand(
                            Command::new("greatgrandchild3")
                                .about("greatgrandchild3 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild3")),
                        ),
                )
                .subcommand(
                    Command::new("grandchild2")
                        .about("grandchild2 command")
                        .arg(Arg::new("grandchild").long("grandchild2")),
                )
                .subcommand(
                    Command::new("grandchild3")
                        .about("grandchild3 command")
                        .arg(Arg::new("grandchild").long("grandchild3")),
                ),
        )
        .subcommand(
            Command::new("child2")
                .about("child2 command")
                .arg(Arg::new("child").long("child2")),
        )
        .subcommand(
            Command::new("child3")
                .hide(true)
                .about("child3 command")
                .arg(Arg::new("child").long("child3"))
                .subcommand(
                    Command::new("grandchild1")
                        .flatten_help(true)
                        .about("grandchild1 command")
                        .arg(Arg::new("grandchild").long("grandchild1"))
                        .subcommand(
                            Command::new("greatgrandchild1")
                                .about("greatgrandchild1 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild1")),
                        )
                        .subcommand(
                            Command::new("greatgrandchild2")
                                .about("greatgrandchild2 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild2")),
                        )
                        .subcommand(
                            Command::new("greatgrandchild3")
                                .about("greatgrandchild3 command")
                                .arg(Arg::new("greatgrandchild").long("greatgrandchild3")),
                        ),
                )
                .subcommand(
                    Command::new("grandchild2")
                        .about("grandchild2 command")
                        .arg(Arg::new("grandchild").long("grandchild2")),
                )
                .subcommand(
                    Command::new("grandchild3")
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

#[test]
fn flatten_not_recursive() {
    let cmd = Command::new("parent")
        .flatten_help(true)
        .about("parent command")
        .arg(Arg::new("parent").long("parent"))
        .subcommand(
            Command::new("child1")
                .about("child1 command")
                .arg(Arg::new("child").long("child1"))
                .subcommand(
                    Command::new("grandchild1")
                        .about("grandchild1 command")
                        .arg(Arg::new("grandchild").long("grandchild1")),
                )
                .subcommand(
                    Command::new("grandchild2")
                        .about("grandchild2 command")
                        .arg(Arg::new("grandchild").long("grandchild2")),
                )
                .subcommand(
                    Command::new("grandchild3")
                        .about("grandchild3 command")
                        .arg(Arg::new("grandchild").long("grandchild3")),
                ),
        )
        .subcommand(
            Command::new("child2")
                .about("child2 command")
                .arg(Arg::new("child").long("child2")),
        )
        .subcommand(
            Command::new("child3")
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

#[test]
fn mixed_argument_types() {
    let cmd = Command::new("myprog")
        .about("mixed arguments")
        .next_help_heading("Mixed")
        .arg(arg!(-b --both "Both long and short"))
        .arg(arg!(--long "Long only"))
        .arg(arg!(<POSITIONAL> "Positional"));

    let expected = str![[r#"
mixed arguments

Usage: myprog [OPTIONS] <POSITIONAL>

Options:
  -h, --help  Print help

Mixed:
  -b, --both    Both long and short
      --long    Long only
  <POSITIONAL>  Positional

"#]];
    utils::assert_output(cmd, "myprog --help", expected, false);
}

#[test]
fn mixed_argument_types_short_positional() {
    let cmd = Command::new("myprog")
        .about("mixed arguments")
        .next_help_heading("Mixed")
        .arg(arg!(-b --both "Both long and short"))
        .arg(arg!(--long "Long only"))
        .arg(arg!(<S> "Short positional"));

    let expected = str![[r#"
mixed arguments

Usage: myprog [OPTIONS] <S>

Options:
  -h, --help  Print help

Mixed:
  -b, --both  Both long and short
      --long  Long only
  <S>         Short positional

"#]];
    utils::assert_output(cmd, "myprog --help", expected, false);
}

#[test]
fn mixed_argument_types_no_short() {
    let cmd = Command::new("myprog")
        .about("mixed arguments")
        .next_help_heading("Mixed")
        .arg(arg!(--long "Long only"))
        .arg(arg!(<POSITIONAL> "Positional"));

    let expected = str![[r#"
mixed arguments

Usage: myprog [OPTIONS] <POSITIONAL>

Options:
  -h, --help  Print help

Mixed:
      --long    Long only
  <POSITIONAL>  Positional

"#]];
    utils::assert_output(cmd, "myprog --help", expected, false);
}

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
                .next_line_help(false)
                .about(text)
                .long_about(text),
            Command::new("next_line_help_true")
                .next_line_help(true)
                .about(text)
                .long_about(text),
        ]);

    let expected = str![[r#"
Usage: myprog [OPTIONS] [COMMAND]

Commands:
  default
          Hello
  next_line_help_false
          Hello
  next_line_help_true
          Hello
  help
          Print this message or the help of the given subcommand(s)

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
  next_line_help_false
          Hello
  next_line_help_true
          Hello
  help
          Print this message or the help of the given subcommand(s)

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
                .next_line_help(false)
                .about(text)
                .long_about(text),
            Command::new("next_line_help_true")
                .next_line_help(true)
                .about(text)
                .long_about(text),
        ]);

    let expected = str![[r#"
Usage: myprog [OPTIONS] [COMMAND]

Commands:
  default
          Hello
  next_line_help_false
          Hello
  next_line_help_true
          Hello
  help
          Print this message or the help of the given subcommand(s)

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
  next_line_help_false
          Hello
  next_line_help_true
          Hello
  help
          Print this message or the help of the given subcommand(s)

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
                .next_line_help(false)
                .about(text)
                .long_about(text),
            Command::new("next_line_help_true")
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
  help
          Print this message or the help of the given subcommand(s)

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
  help
          Print this message or the help of the given subcommand(s)

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
                .next_line_help(false)
                .about(text)
                .long_about(text),
            Command::new("next_line_help_true")
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
  help
          Print this message or the help of the given subcommand(s)

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
  help
          Print this message or the help of the given subcommand(s)

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
