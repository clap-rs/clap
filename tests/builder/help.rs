#![cfg(feature = "help")]

use clap::{arg, builder::PossibleValue, error::ErrorKind, Arg, ArgAction, ArgGroup, Command};

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

    static EXPECTED: &str = "error: unrecognized subcommand 'foo'

Usage: ctest subcmd multi [OPTIONS]

For more information, try '--help'.
";
    utils::assert_eq(EXPECTED, err.to_string());
}

#[test]
fn req_last_arg_usage() {
    static LAST_ARG_REQ_MULT: &str = "\
Usage: example <FIRST>... -- <SECOND>...

Arguments:
  <FIRST>...   First
  <SECOND>...  Second

Options:
  -h, --help     Print help
  -V, --version  Print version
";

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
    utils::assert_output(cmd, "example --help", LAST_ARG_REQ_MULT, false);
}

#[test]
fn args_with_last_usage() {
    static LAST_ARG_USAGE: &str = "\
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
";

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
    utils::assert_output(cmd, "flamegraph --help", LAST_ARG_USAGE, false);
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
    static HELP: &str = "\
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
";

    utils::assert_output(utils::complex_app(), "clap-test --help", HELP, false);
}

#[test]
fn after_and_before_help_output() {
    static AFTER_HELP: &str = "some text that comes before the help

tests clap library

Usage: clap-test

Options:
  -h, --help     Print help
  -V, --version  Print version

some text that comes after the help
";

    let cmd = Command::new("clap-test")
        .version("v1.4.8")
        .about("tests clap library")
        .before_help("some text that comes before the help")
        .after_help("some text that comes after the help");
    utils::assert_output(cmd.clone(), "clap-test -h", AFTER_HELP, false);
    utils::assert_output(cmd, "clap-test --help", AFTER_HELP, false);
}

#[test]
fn after_and_before_long_help_output() {
    static AFTER_HELP: &str = "some text that comes before the help

tests clap library

Usage: clap-test

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version

some text that comes after the help
";

    static AFTER_LONG_HELP: &str = "some longer text that comes before the help

tests clap library

Usage: clap-test

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

some longer text that comes after the help
";

    let cmd = Command::new("clap-test")
        .version("v1.4.8")
        .about("tests clap library")
        .before_help("some text that comes before the help")
        .after_help("some text that comes after the help")
        .before_long_help("some longer text that comes before the help")
        .after_long_help("some longer text that comes after the help");
    utils::assert_output(cmd.clone(), "clap-test --help", AFTER_LONG_HELP, false);
    utils::assert_output(cmd, "clap-test -h", AFTER_HELP, false);
}

static MULTI_SC_HELP: &str = "\
tests subcommands

Usage: ctest subcmd multi [OPTIONS]

Options:
  -f, --flag                  tests flags
  -o, --option <scoption>...  tests options
  -h, --help                  Print help
  -V, --version               Print version
";

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
    utils::assert_output(cmd, "ctest help subcmd multi", MULTI_SC_HELP, false);
}

#[test]
fn no_wrap_default_help() {
    static DEFAULT_HELP: &str = "\
Usage: ctest

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let cmd = Command::new("ctest").version("1.0").term_width(0);
    utils::assert_output(cmd, "ctest --help", DEFAULT_HELP, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn wrapped_help() {
    static WRAPPED_HELP: &str = "\
Usage: test [OPTIONS]

Options:
  -a, --all            Also do versioning for private crates (will
                       not be published)
      --exact          Specify inter dependency version numbers
                       exactly with `=`
      --no-git-commit  Do not commit version changes
      --no-git-push    Do not push generated commit and tags to git
                       remote
  -h, --help           Print help
";
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
        );
    utils::assert_output(cmd, "test --help", WRAPPED_HELP, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn unwrapped_help() {
    static UNWRAPPED_HELP: &str = "\
Usage: test [OPTIONS]

Options:
  -a, --all            Also do versioning for private crates (will
                       not be published)
      --exact          Specify inter dependency version numbers
                       exactly with `=`
      --no-git-commit  Do not commit version changes
      --no-git-push    Do not push generated commit and tags to git
                       remote
  -h, --help           Print help
";
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
        );
    utils::assert_output(cmd, "test --help", UNWRAPPED_HELP, false);
}

#[test]
#[cfg(all(feature = "wrap_help"))]
fn possible_value_wrapped_help() {
    static WRAPPED_HELP: &str = "\
Usage: test [OPTIONS]

Options:
      --possible-values <possible_values>
          Possible values:
          - short_name:
            Long enough help message, barely warrant wrapping
          - second:
            Short help gets handled the same

      --possible-values-with-new-line <possible_values_with_new_line>
          Possible values:
          - long enough name to trigger new line:
            Really long enough help message to clearly warrant
            wrapping believe me
          - second

      --possible-values-without-new-line <possible_values_without_new_line>
          Possible values:
          - name:   Short enough help message with no wrapping
          - second: short help

  -h, --help
          Print help (see a summary with '-h')
";
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
    utils::assert_output(cmd, "test --help", WRAPPED_HELP, false);
}

#[test]
fn complex_subcommand_help_output() {
    static SC_HELP: &str = "clap-test-subcmd 0.1
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
";

    let a = utils::complex_app();
    utils::assert_output(a, "clap-test subcmd --help", SC_HELP, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn issue_626_unicode_cutoff() {
    static ISSUE_626_CUTOFF: &str = "\
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
";

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
    utils::assert_output(cmd, "ctest --help", ISSUE_626_CUTOFF, false);
}

static HIDE_POS_VALS: &str = "\
Usage: ctest [OPTIONS]

Options:
  -p, --pos <VAL>    Some vals [possible values: fast, slow]
  -c, --cafe <FILE>  A coffeehouse, coffee shop, or café.
  -h, --help         Print help
  -V, --version      Print version
";

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
    utils::assert_output(cmd, "ctest --help", HIDE_POS_VALS, false);
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
    utils::assert_output(cmd, "ctest --help", HIDE_POS_VALS, false);
}

#[test]
fn possible_vals_with_help() {
    static POS_VALS_HELP: &str = "\
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
";
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
    utils::assert_output(app, "ctest --help", POS_VALS_HELP, false);
}

#[test]
fn hidden_possible_vals() {
    static POS_VALS_HELP: &str = "\
Usage: ctest [pos]

Arguments:
  [pos]  

Options:
  -h, --help  Print help
";
    let app = Command::new("ctest").arg(
        Arg::new("pos")
            .hide_possible_values(true)
            .value_parser([
                PossibleValue::new("fast"),
                PossibleValue::new("slow").help("not as fast"),
            ])
            .action(ArgAction::Set),
    );
    utils::assert_output(app, "ctest --help", POS_VALS_HELP, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn issue_626_panic() {
    static ISSUE_626_PANIC: &str = "\
Usage: ctest [OPTIONS]

Options:
  -c, --cafe <FILE>
          La culture du café est très développée
          dans de nombreux pays à climat chaud
          d\'Amérique, d\'Afrique et d\'Asie, dans des
          plantations qui sont cultivées pour les
          marchés d\'exportation. Le café est souvent
          une contribution majeure aux exportations
          des régions productrices.
  -h, --help
          Print help
  -V, --version
          Print version
";

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
    utils::assert_output(cmd, "ctest --help", ISSUE_626_PANIC, false);
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
    static FINAL_WORD_WRAPPING: &str = "\
Usage: ctest

Options:
  -h, --help
          Print help
  -V, --version
          Print version
";

    let cmd = Command::new("ctest").version("0.1").term_width(24);
    utils::assert_output(cmd, "ctest --help", FINAL_WORD_WRAPPING, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn wrapping_newline_chars() {
    static WRAPPING_NEWLINE_CHARS: &str = "\
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
";

    let cmd = Command::new("ctest")
        .version("0.1")
        .term_width(60)
        .arg(Arg::new("mode").help(
            "x, max, maximum   20 characters, contains symbols.\n\
             l, long           Copy-friendly, 14 characters, contains symbols.\n\
             m, med, medium    Copy-friendly, 8 characters, contains symbols.\n",
        ));
    utils::assert_output(cmd, "ctest --help", WRAPPING_NEWLINE_CHARS, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn wrapping_newline_variables() {
    static WRAPPING_NEWLINE_CHARS: &str = "\
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
";

    let cmd = Command::new("ctest")
        .version("0.1")
        .term_width(60)
        .arg(Arg::new("mode").help(
            "x, max, maximum   20 characters, contains symbols.{n}\
             l, long           Copy-friendly, 14 characters, contains symbols.{n}\
             m, med, medium    Copy-friendly, 8 characters, contains symbols.{n}",
        ));
    utils::assert_output(cmd, "ctest --help", WRAPPING_NEWLINE_CHARS, false);
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

    const EXPECTED: &str = "\
Usage: Example update [OPTIONS]

Options:
      --force-non-host
          Install toolchains
          that require an
          emulator. See
          https://github.com/rust-lang/rustup/wiki/Non-host-toolchains
  -h, --help
          Print help
";
    utils::assert_output(cmd, "Example update --help", EXPECTED, false);
}

static OLD_NEWLINE_CHARS: &str = "\
Usage: ctest [OPTIONS]

Options:
  -m             Some help with some wrapping
                 (Defaults to something)
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn old_newline_chars() {
    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("mode")
            .short('m')
            .action(ArgAction::SetTrue)
            .help("Some help with some wrapping\n(Defaults to something)"),
    );
    utils::assert_output(cmd, "ctest --help", OLD_NEWLINE_CHARS, false);
}

#[test]
fn old_newline_variables() {
    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("mode")
            .short('m')
            .action(ArgAction::SetTrue)
            .help("Some help with some wrapping{n}(Defaults to something)"),
    );
    utils::assert_output(cmd, "ctest --help", OLD_NEWLINE_CHARS, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn issue_688_hide_pos_vals() {
    static ISSUE_688: &str = "\
Usage: ctest [OPTIONS]

Options:
      --filter <filter>  Sets the filter, or sampling method, to use for interpolation when resizing the particle
                         images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian,
                         Lanczos3]
  -h, --help             Print help
  -V, --version          Print version
";

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
    utils::assert_output(app1, "ctest --help", ISSUE_688, false);

    let app2 = Command::new("ctest")
        .version("0.1")
			.term_width(120)
			.arg(Arg::new("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
            images. The default is Linear (Bilinear).")
				.long("filter")
				.value_parser(filter_values)
				.action(ArgAction::Set));
    utils::assert_output(app2, "ctest --help", ISSUE_688, false);

    let app3 = Command::new("ctest")
        .version("0.1")
			.term_width(120)
			.arg(Arg::new("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
            images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
				.long("filter")
				.action(ArgAction::Set));
    utils::assert_output(app3, "ctest --help", ISSUE_688, false);
}

#[test]
fn issue_702_multiple_values() {
    static ISSUE_702: &str = "\
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
";

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
    utils::assert_output(cmd, "myapp --help", ISSUE_702, false);
}

#[test]
fn long_about() {
    static LONG_ABOUT: &str = "\
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
";

    let cmd = Command::new("myapp")
        .version("1.0")
        .author("foo")
        .about("bar")
        .long_about(
            "something really really long, with\nmultiple lines of text\nthat should be displayed",
        )
        .arg(Arg::new("arg1").help("some option"));
    utils::assert_output(cmd, "myapp --help", LONG_ABOUT, false);
}

#[test]
fn ripgrep_usage() {
    static RIPGREP_USAGE: &str = "\
Usage: rg [OPTIONS] <pattern> [<path> ...]
       rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
       rg [OPTIONS] --files [<path> ...]
       rg [OPTIONS] --type-list

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let cmd = Command::new("ripgrep").version("0.5").override_usage(
        "rg [OPTIONS] <pattern> [<path> ...]
       rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
       rg [OPTIONS] --files [<path> ...]
       rg [OPTIONS] --type-list",
    );

    utils::assert_output(cmd, "rg --help", RIPGREP_USAGE, false);
}

#[test]
fn ripgrep_usage_using_templates() {
    static RIPGREP_USAGE: &str = "\
ripgrep 0.5

Usage: rg [OPTIONS] <pattern> [<path> ...]
       rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
       rg [OPTIONS] --files [<path> ...]
       rg [OPTIONS] --type-list

Options:
  -h, --help     Print help
  -V, --version  Print version
";

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

    utils::assert_output(cmd, "rg --help", RIPGREP_USAGE, false);
}

#[test]
fn sc_negates_reqs() {
    static SC_NEGATES_REQS: &str = "\
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
";

    let cmd = Command::new("prog")
        .version("1.0")
        .subcommand_negates_reqs(true)
        .arg(arg!(-o --opt <FILE> "tests options").required(true))
        .arg(Arg::new("PATH").help("help"))
        .subcommand(Command::new("test"));
    utils::assert_output(cmd, "prog --help", SC_NEGATES_REQS, false);
}

#[test]
fn hide_args() {
    static HIDDEN_ARGS: &str = "\
Usage: prog [OPTIONS]

Options:
  -f, --flag        testing flags
  -o, --opt <FILE>  tests options
  -h, --help        Print help
  -V, --version     Print version
";

    let cmd = Command::new("prog")
        .version("1.0")
        .arg(arg!(-f --flag "testing flags"))
        .arg(arg!(-o --opt <FILE> "tests options"))
        .arg(Arg::new("pos").hide(true));
    utils::assert_output(cmd, "prog --help", HIDDEN_ARGS, false);
}

#[test]
fn args_negate_sc() {
    static ARGS_NEGATE_SC: &str = "\
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
";

    let cmd = Command::new("prog")
        .version("1.0")
        .args_conflicts_with_subcommands(true)
        .arg(arg!(-f --flag "testing flags"))
        .arg(arg!(-o --opt <FILE> "tests options"))
        .arg(Arg::new("PATH").help("help"))
        .subcommand(Command::new("test"));
    utils::assert_output(cmd, "prog --help", ARGS_NEGATE_SC, false);
}

#[test]
fn issue_1046_hide_scs() {
    static ISSUE_1046_HIDDEN_SCS: &str = "\
Usage: prog [OPTIONS] [PATH]

Arguments:
  [PATH]  some

Options:
  -f, --flag        testing flags
  -o, --opt <FILE>  tests options
  -h, --help        Print help
  -V, --version     Print version
";

    let cmd = Command::new("prog")
        .version("1.0")
        .arg(arg!(-f --flag "testing flags"))
        .arg(arg!(-o --opt <FILE> "tests options"))
        .arg(Arg::new("PATH").help("some"))
        .subcommand(Command::new("test").hide(true));
    utils::assert_output(cmd, "prog --help", ISSUE_1046_HIDDEN_SCS, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn issue_777_wrap_all_things() {
    static ISSUE_777: &str = "A cmd with a crazy very long long
long name hahaha 1.0
Some Very Long Name and crazy long
email <email@server.com>
Show how the about text is not
wrapped

Usage: ctest

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let cmd = Command::new("A cmd with a crazy very long long long name hahaha")
        .version("1.0")
        .author("Some Very Long Name and crazy long email <email@server.com>")
        .about("Show how the about text is not wrapped")
        .help_template(utils::FULL_TEMPLATE)
        .term_width(35);
    utils::assert_output(cmd, "ctest --help", ISSUE_777, false);
}

static OVERRIDE_HELP_SHORT: &str = "\
Usage: test

Options:
  -H, --help     Print help
  -V, --version  Print version
";

#[test]
fn override_help_short() {
    let cmd = Command::new("test")
        .version("0.1")
        .arg(arg!(-H --help "Print help").action(ArgAction::Help))
        .disable_help_flag(true);

    utils::assert_output(cmd.clone(), "test --help", OVERRIDE_HELP_SHORT, false);
    utils::assert_output(cmd, "test -H", OVERRIDE_HELP_SHORT, false);
}

static OVERRIDE_HELP_LONG: &str = "\
Usage: test [OPTIONS]

Options:
  -h, --hell     Print help
  -V, --version  Print version
";

#[test]
fn override_help_long() {
    let cmd = Command::new("test")
        .version("0.1")
        .arg(arg!(-h --hell "Print help").action(ArgAction::Help))
        .disable_help_flag(true);

    utils::assert_output(cmd.clone(), "test --hell", OVERRIDE_HELP_LONG, false);
    utils::assert_output(cmd, "test -h", OVERRIDE_HELP_LONG, false);
}

static OVERRIDE_HELP_ABOUT: &str = "\
Usage: test

Options:
  -h, --help     Print custom help information
  -V, --version  Print version
";

#[test]
fn override_help_about() {
    let cmd = Command::new("test")
        .version("0.1")
        .arg(arg!(-h --help "Print custom help information").action(ArgAction::Help))
        .disable_help_flag(true);

    utils::assert_output(cmd.clone(), "test --help", OVERRIDE_HELP_ABOUT, false);
    utils::assert_output(cmd, "test -h", OVERRIDE_HELP_ABOUT, false);
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
    static LAST_ARG: &str = "\
Usage: last <TARGET> [CORPUS] [-- <ARGS>...]

Arguments:
  <TARGET>   some
  [CORPUS]   some
  [ARGS]...  some

Options:
  -h, --help     Print help
  -V, --version  Print version
";

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
    utils::assert_output(cmd, "last --help", LAST_ARG, false);
}

#[test]
fn last_arg_mult_usage_req() {
    static LAST_ARG_REQ: &str = "\
Usage: last <TARGET> [CORPUS] -- <ARGS>...

Arguments:
  <TARGET>   some
  [CORPUS]   some
  <ARGS>...  some

Options:
  -h, --help     Print help
  -V, --version  Print version
";

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
    utils::assert_output(cmd, "last --help", LAST_ARG_REQ, false);
}

#[test]
fn last_arg_mult_usage_req_with_sc() {
    static LAST_ARG_REQ_SC: &str = "\
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
";

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
    utils::assert_output(cmd, "last --help", LAST_ARG_REQ_SC, false);
}

#[test]
fn last_arg_mult_usage_with_sc() {
    static LAST_ARG_SC: &str = "\
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
";

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
    utils::assert_output(cmd, "last --help", LAST_ARG_SC, false);
}

static HIDE_DEFAULT_VAL: &str = "\
Usage: default [OPTIONS]

Options:
      --arg <argument>  Pass an argument to the program. [default: default-argument]
  -h, --help            Print help
  -V, --version         Print version
";

#[test]
fn hide_default_val() {
    let app1 = Command::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .help("Pass an argument to the program. [default: default-argument]")
            .long("arg")
            .default_value("default-argument")
            .hide_default_value(true),
    );
    utils::assert_output(app1, "default --help", HIDE_DEFAULT_VAL, false);

    let app2 = Command::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .help("Pass an argument to the program.")
            .long("arg")
            .default_value("default-argument"),
    );
    utils::assert_output(app2, "default --help", HIDE_DEFAULT_VAL, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn escaped_whitespace_values() {
    static ESCAPED_DEFAULT_VAL: &str = "\
Usage: default [OPTIONS]

Options:
      --arg <argument>  Pass an argument to the program. [default: \"\\n\"] [possible values: normal, \" \", \"\\n\", \"\\t\",
                        other]
  -h, --help            Print help
  -V, --version         Print version
";

    let app1 = Command::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .help("Pass an argument to the program.")
            .long("arg")
            .default_value("\n")
            .value_parser(["normal", " ", "\n", "\t", "other"]),
    );
    utils::assert_output(app1, "default --help", ESCAPED_DEFAULT_VAL, false);
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
    static REQUIRE_DELIM_HELP: &str = "\
tests stuff

Usage: test --fake <some> <val>

Options:
  -f, --fake <some> <val>  some help
  -h, --help               Print help
  -V, --version            Print version
";

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

    utils::assert_output(cmd, "test --help", REQUIRE_DELIM_HELP, false);
}

#[test]
fn custom_headers_headers() {
    static CUSTOM_HELP_SECTION: &str = "\
does stuff

Usage: test [OPTIONS] --fake <some> <val>

Options:
  -f, --fake <some> <val>  some help
  -h, --help               Print help
  -V, --version            Print version

NETWORKING:
  -n, --no-proxy  Do not use system proxy settings
      --port
";

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

    utils::assert_output(cmd, "test --help", CUSTOM_HELP_SECTION, false);
}

static MULTIPLE_CUSTOM_HELP_SECTIONS: &str = "\
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
";

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

    utils::assert_output(cmd, "test --help", MULTIPLE_CUSTOM_HELP_SECTIONS, false);
}

static CUSTOM_HELP_SECTION_HIDDEN_ARGS: &str = "\
does stuff

Usage: test [OPTIONS] --song <song> --song-volume <volume>

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version

OVERRIDE SPECIAL:
  -b, --song <song>  Change which song is played for birthdays

SPECIAL:
  -v, --song-volume <volume>  Change the volume of the birthday song
";

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

    utils::assert_output(cmd, "test -h", CUSTOM_HELP_SECTION_HIDDEN_ARGS, false);
}

static ISSUE_897: &str = "\
Long about foo

Usage: ctest foo

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
";

#[test]
fn show_long_about_issue_897() {
    let cmd = Command::new("ctest").version("0.1").subcommand(
        Command::new("foo")
            .version("0.1")
            .about("About foo")
            .long_about("Long about foo"),
    );
    utils::assert_output(cmd, "ctest foo --help", ISSUE_897, false);
}

static ISSUE_897_SHORT: &str = "\
About foo

Usage: ctest foo

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
";

#[test]
fn show_short_about_issue_897() {
    let cmd = Command::new("ctest").version("0.1").subcommand(
        Command::new("foo")
            .version("0.1")
            .about("About foo")
            .long_about("Long about foo"),
    );
    utils::assert_output(cmd, "ctest foo -h", ISSUE_897_SHORT, false);
}

#[test]
fn issue_1364_no_short_options() {
    static ISSUE_1364: &str = "\
Usage: demo [OPTIONS] [FILES]...

Arguments:
  [FILES]...  

Options:
  -f          
  -h, --help  Print help (see more with '--help')
";

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

    utils::assert_output(cmd, "demo -h", ISSUE_1364, false);
}

#[rustfmt::skip]
#[test]
fn issue_1487() {
static ISSUE_1487: &str = "\
Usage: ctest <arg1|arg2>

Arguments:
  [arg1]  
  [arg2]  

Options:
  -h, --help  Print help
";

    let cmd = Command::new("test")
    .arg(Arg::new("arg1")
        .group("group1"))
    .arg(Arg::new("arg2")
        .group("group1"))
    .group(ArgGroup::new("group1")
        .args(["arg1", "arg2"])
        .required(true));
    utils::assert_output(cmd, "ctest -h", ISSUE_1487, false);
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
    static ISSUE_1642: &str = "\
Usage: prog [OPTIONS]

Options:
      --config
          The config file used by the myprog must be in JSON format
          with only valid keys and may not contain other nonsense
          that cannot be read by this program. Obviously I'm going on
          and on, so I'll stop now.

  -h, --help
          Print help (see a summary with '-h')
";

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
    utils::assert_output(cmd, "prog --help", ISSUE_1642, false);
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
    static HELP_SUBCMD_HELP: &str = "\
Print this message or the help of the given subcommand(s)

Usage: myapp help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)
";

    let cmd = Command::new("myapp")
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    utils::assert_output(cmd.clone(), "myapp help help", HELP_SUBCMD_HELP, false);
}

#[test]
fn subcmd_help_subcmd_help() {
    static SUBCMD_HELP_SUBCMD_HELP: &str = "\
Print this message or the help of the given subcommand(s)

Usage: myapp subcmd help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)
";

    let cmd = Command::new("myapp")
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    utils::assert_output(
        cmd.clone(),
        "myapp subcmd help help",
        SUBCMD_HELP_SUBCMD_HELP,
        false,
    );
}

#[test]
fn global_args_should_show_on_toplevel_help_message() {
    static HELP: &str = "\
Usage: myapp [OPTIONS] [COMMAND]

Commands:
  subcmd\x20\x20
  help    Print this message or the help of the given subcommand(s)

Options:
  -g, --some-global <someglobal>\x20\x20
  -h, --help                      Print help
";

    let cmd = Command::new("myapp")
        .arg(
            Arg::new("someglobal")
                .short('g')
                .long("some-global")
                .global(true),
        )
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    utils::assert_output(cmd, "myapp help", HELP, false);
}

#[test]
fn global_args_should_not_show_on_help_message_for_help_help() {
    static HELP_HELP: &str = "\
Print this message or the help of the given subcommand(s)

Usage: myapp help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)
";

    let cmd = Command::new("myapp")
        .arg(
            Arg::new("someglobal")
                .short('g')
                .long("some-global")
                .global(true),
        )
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    utils::assert_output(cmd, "myapp help help", HELP_HELP, false);
}

#[test]
fn global_args_should_show_on_help_message_for_subcommand() {
    static HELP_SUBCMD: &str = "\
Usage: myapp subcmd [OPTIONS] [COMMAND]

Commands:
  multi\x20\x20
  help   Print this message or the help of the given subcommand(s)

Options:
  -g, --some-global <someglobal>\x20\x20
  -h, --help                      Print help
";

    let cmd = Command::new("myapp")
        .arg(
            Arg::new("someglobal")
                .short('g')
                .long("some-global")
                .global(true),
        )
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    utils::assert_output(cmd, "myapp help subcmd", HELP_SUBCMD, false);
}

#[test]
fn global_args_should_show_on_help_message_for_nested_subcommand() {
    static HELP_SUB_SUBCMD: &str = "\
Usage: myapp subcmd multi [OPTIONS]

Options:
  -g, --some-global <someglobal>\x20\x20
  -h, --help                      Print help
  -V, --version                   Print version
";

    let cmd = Command::new("myapp")
        .arg(
            Arg::new("someglobal")
                .short('g')
                .long("some-global")
                .global(true),
        )
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    utils::assert_output(cmd, "myapp help subcmd multi", HELP_SUB_SUBCMD, false);
}

#[test]
fn option_usage_order() {
    static OPTION_USAGE_ORDER: &str = "\
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
";

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

    utils::assert_output(cmd, "order --help", OPTION_USAGE_ORDER, false);
}

#[test]
fn prefer_about_over_long_about_in_subcommands_list() {
    static ABOUT_IN_COMMANDS_LIST: &str = "\
Usage: about-in-subcommands-list [COMMAND]

Commands:
  sub   short about sub
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
";

    let cmd = Command::new("about-in-subcommands-list").subcommand(
        Command::new("sub")
            .long_about("long about sub")
            .about("short about sub"),
    );

    utils::assert_output(
        cmd,
        "about-in-subcommands-list --help",
        ABOUT_IN_COMMANDS_LIST,
        false,
    );
}

#[test]
fn issue_1794_usage() {
    static USAGE_WITH_GROUP: &str = "\
Usage: deno <pos1|--option1> [pos2]

Arguments:
  [pos1]  
  [pos2]  

Options:
      --option1  
  -h, --help     Print help
";

    let cmd = clap::Command::new("hello")
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

    utils::assert_output(cmd, "deno --help", USAGE_WITH_GROUP, false);
}

static CUSTOM_HEADING_POS: &str = "\
Usage: test [gear] [speed]

Arguments:
  [gear]  Which gear

Options:
  -h, --help     Print help
  -V, --version  Print version

NETWORKING:
  [speed]  How fast
";

#[test]
fn custom_heading_pos() {
    let cmd = Command::new("test")
        .version("1.4")
        .arg(Arg::new("gear").help("Which gear"))
        .next_help_heading(Some("NETWORKING"))
        .arg(Arg::new("speed").help("How fast"));

    utils::assert_output(cmd, "test --help", CUSTOM_HEADING_POS, false);
}

static ONLY_CUSTOM_HEADING_OPTS_NO_ARGS: &str = "\
Usage: test [OPTIONS]

NETWORKING:
  -s, --speed <SPEED>  How fast
";

#[test]
fn only_custom_heading_opts_no_args() {
    let cmd = Command::new("test")
        .version("1.4")
        .disable_version_flag(true)
        .disable_help_flag(true)
        .arg(arg!(--help).action(ArgAction::Help).hide(true))
        .next_help_heading(Some("NETWORKING"))
        .arg(arg!(-s --speed <SPEED> "How fast"));

    utils::assert_output(cmd, "test --help", ONLY_CUSTOM_HEADING_OPTS_NO_ARGS, false);
}

static ONLY_CUSTOM_HEADING_POS_NO_ARGS: &str = "\
Usage: test [speed]

NETWORKING:
  [speed]  How fast
";

#[test]
fn only_custom_heading_pos_no_args() {
    let cmd = Command::new("test")
        .version("1.4")
        .disable_version_flag(true)
        .disable_help_flag(true)
        .arg(arg!(--help).action(ArgAction::Help).hide(true))
        .next_help_heading(Some("NETWORKING"))
        .arg(Arg::new("speed").help("How fast"));

    utils::assert_output(cmd, "test --help", ONLY_CUSTOM_HEADING_POS_NO_ARGS, false);
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
    utils::assert_output(
        cmd,
        "my_app --help",
        "\
Usage: my_app [OPTIONS]

Options:
      --some_arg <some_arg> <some_arg>  
      --some_arg_issue <ARG> <ARG>      
  -h, --help                            Print help
",
        false,
    );
}

#[test]
fn missing_positional_final_required() {
    let cmd = Command::new("test")
        .allow_missing_positional(true)
        .arg(Arg::new("arg1"))
        .arg(Arg::new("arg2").required(true));
    utils::assert_output(
        cmd,
        "test --help",
        "\
Usage: test [arg1] <arg2>

Arguments:
  [arg1]  
  <arg2>  

Options:
  -h, --help  Print help
",
        false,
    );
}

#[test]
fn missing_positional_final_multiple() {
    let cmd = Command::new("test")
        .allow_missing_positional(true)
        .arg(Arg::new("foo"))
        .arg(Arg::new("bar"))
        .arg(Arg::new("baz").action(ArgAction::Set).num_args(1..));
    utils::assert_output(
        cmd,
        "test --help",
        "\
Usage: test [foo] [bar] [baz]...

Arguments:
  [foo]     
  [bar]     
  [baz]...  

Options:
  -h, --help  Print help
",
        false,
    );
}

#[test]
fn positional_multiple_values_is_dotted() {
    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .action(ArgAction::Set)
            .num_args(1..),
    );
    utils::assert_output(
        cmd,
        "test --help",
        "\
Usage: test <foo>...

Arguments:
  <foo>...  

Options:
  -h, --help  Print help
",
        false,
    );

    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .action(ArgAction::Set)
            .value_name("BAR")
            .num_args(1..),
    );
    utils::assert_output(
        cmd,
        "test --help",
        "\
Usage: test <BAR>...

Arguments:
  <BAR>...  

Options:
  -h, --help  Print help
",
        false,
    );
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
    utils::assert_output(
        cmd,
        "test --help",
        "\
Usage: test <foo>...

Arguments:
  <foo>...  

Options:
  -h, --help  Print help
",
        false,
    );

    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .action(ArgAction::Set)
            .value_name("BAR")
            .num_args(1..)
            .action(ArgAction::Append),
    );
    utils::assert_output(
        cmd,
        "test --help",
        "\
Usage: test <BAR>...

Arguments:
  <BAR>...  

Options:
  -h, --help  Print help
",
        false,
    );
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
    utils::assert_output(
        cmd,
        "test --help",
        "\
Usage: test --foo <one> <two>...

Options:
      --foo <one> <two>...  
  -h, --help                Print help
",
        false,
    );
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
        .debug_assert()
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

    utils::assert_output(
        cmd,
        "example help help",
        "\
Print this message or the help of the given subcommand(s)

Usage: example help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)
",
        false,
    );
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
        "`help` should not be present: {:?}",
        args
    );
}

#[test]
fn dont_propagate_version_to_help_subcommand() {
    let cmd = clap::Command::new("example")
        .version("1.0")
        .propagate_version(true)
        .subcommand(clap::Command::new("subcommand"));

    utils::assert_output(
        cmd.clone(),
        "example help help",
        "\
Print this message or the help of the given subcommand(s)

Usage: example help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)
",
        false,
    );

    cmd.debug_assert();
}

#[test]
fn help_without_short() {
    let mut cmd = clap::Command::new("test")
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
    static EXPECTED: &str = "\
some

Usage: parent <TARGET> <ARGS> test

Options:
  -h, --help  Print help
";
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
    utils::assert_output(cmd, "parent test --help", EXPECTED, false);
}

#[test]
fn parent_cmd_req_in_usage_with_help_subcommand() {
    static EXPECTED: &str = "\
some

Usage: parent <TARGET> <ARGS> test

Options:
  -h, --help  Print help
";
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
    utils::assert_output(cmd, "parent help test", EXPECTED, false);
}

#[test]
fn parent_cmd_req_in_usage_with_render_help() {
    static EXPECTED: &str = "\
some

Usage: parent <TARGET> <ARGS> test

Options:
  -h, --help  Print help
";
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
    utils::assert_eq(EXPECTED, help);
}

#[test]
fn parent_cmd_req_ignored_when_negates_reqs() {
    static MULTI_SC_HELP: &str = "\
Usage: ctest subcmd

Options:
  -h, --help  Print help
";

    let cmd = Command::new("ctest")
        .arg(arg!(<input>))
        .subcommand_negates_reqs(true)
        .subcommand(Command::new("subcmd"));
    utils::assert_output(cmd, "ctest subcmd --help", MULTI_SC_HELP, false);
}

#[test]
fn parent_cmd_req_ignored_when_conflicts() {
    static MULTI_SC_HELP: &str = "\
Usage: ctest subcmd

Options:
  -h, --help  Print help
";

    let cmd = Command::new("ctest")
        .arg(arg!(<input>))
        .args_conflicts_with_subcommands(true)
        .subcommand(Command::new("subcmd"));
    utils::assert_output(cmd, "ctest subcmd --help", MULTI_SC_HELP, false);
}

#[test]
fn no_wrap_help() {
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
