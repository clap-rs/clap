use super::utils;

use clap::{arg, error::ErrorKind, Arg, ArgGroup, Command, PossibleValue};

static REQUIRE_DELIM_HELP: &str = "test 1.3
Kevin K.
tests stuff

USAGE:
    test --fake <some>:<val>

OPTIONS:
    -f, --fake <some>:<val>    some help
    -h, --help                 Print help information
    -V, --version              Print version information
";

static HELP: &str = "clap-test v1.4.8
Kevin K. <kbknapp@gmail.com>
tests clap library

USAGE:
    clap-test [OPTIONS] [ARGS] [SUBCOMMAND]

ARGS:
    <positional>        tests positionals
    <positional2>       tests positionals with exclusions
    <positional3>...    tests specific values [possible values: vi, emacs]

OPTIONS:
    -f, --flag                       tests flags
    -F                               tests flags with exclusions
    -h, --help                       Print help information
        --long-option-2 <option2>    tests long options with exclusions
        --maxvals3 <maxvals>...      Tests 3 max vals
        --minvals2 <minvals>...      Tests 2 min vals
        --multvals <one> <two>       Tests multiple values, not mult occs
        --multvalsmo <one> <two>     Tests multiple values, and mult occs
    -o, --option <opt>...            tests options
    -O, --option3 <option3>          specific vals [possible values: fast, slow]
        --optvaleq[=<optval>]        Tests optional value, require = sign
        --optvalnoeq [<optval>]      Tests optional value
    -V, --version                    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    subcmd    tests subcommands
";

static SC_NEGATES_REQS: &str = "prog 1.0

USAGE:
    prog --opt <FILE> [PATH]
    prog [PATH] <SUBCOMMAND>

ARGS:
    <PATH>    help

OPTIONS:
    -h, --help          Print help information
    -o, --opt <FILE>    tests options
    -V, --version       Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    test    
";

static ARGS_NEGATE_SC: &str = "prog 1.0

USAGE:
    prog [OPTIONS] [PATH]
    prog <SUBCOMMAND>

ARGS:
    <PATH>    help

OPTIONS:
    -f, --flag          testing flags
    -h, --help          Print help information
    -o, --opt <FILE>    tests options
    -V, --version       Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    test    
";

static AFTER_HELP: &str = "some text that comes before the help

clap-test v1.4.8
tests clap library

USAGE:
    clap-test

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

some text that comes after the help
";

static AFTER_LONG_HELP: &str = "some longer text that comes before the help

clap-test v1.4.8
tests clap library

USAGE:
    clap-test

OPTIONS:
    -h, --help
            Print help information

    -V, --version
            Print version information

some longer text that comes after the help
";

static HIDDEN_ARGS: &str = "prog 1.0

USAGE:
    prog [OPTIONS]

OPTIONS:
    -f, --flag          testing flags
    -h, --help          Print help information
    -o, --opt <FILE>    tests options
    -V, --version       Print version information
";

static SC_HELP: &str = "clap-test-subcmd 0.1
Kevin K. <kbknapp@gmail.com>
tests subcommands

USAGE:
    clap-test subcmd [OPTIONS] [--] [scpositional]

ARGS:
    <scpositional>    tests positionals

OPTIONS:
    -f, --flag                     tests flags
    -h, --help                     Print help information
    -o, --option <scoption>...     tests options
    -s, --subcmdarg <subcmdarg>    tests other args
    -V, --version                  Print version information
";

static ISSUE_1046_HIDDEN_SCS: &str = "prog 1.0

USAGE:
    prog [OPTIONS] [PATH]

ARGS:
    <PATH>    some

OPTIONS:
    -f, --flag          testing flags
    -h, --help          Print help information
    -o, --opt <FILE>    tests options
    -V, --version       Print version information
";

// Using number_of_values(1) with multiple_values(true) misaligns help message
static ISSUE_760: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -o, --option <option>    tests options
    -O, --opt <opt>          tests options
    -V, --version            Print version information
";

static RIPGREP_USAGE: &str = "ripgrep 0.5

USAGE:
    rg [OPTIONS] <pattern> [<path> ...]
    rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
    rg [OPTIONS] --files [<path> ...]
    rg [OPTIONS] --type-list

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

static MULTI_SC_HELP: &str = "ctest-subcmd-multi 0.1
Kevin K. <kbknapp@gmail.com>
tests subcommands

USAGE:
    ctest subcmd multi [OPTIONS]

OPTIONS:
    -f, --flag                    tests flags
    -h, --help                    Print help information
    -o, --option <scoption>...    tests options
    -V, --version                 Print version information
";

static ISSUE_626_CUTOFF: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café is an
                         establishment which primarily serves hot
                         coffee, related coffee beverages (e.g., café
                         latte, cappuccino, espresso), tea, and other
                         hot beverages. Some coffeehouses also serve
                         cold beverages such as iced coffee and iced
                         tea. Many cafés also serve some type of food,
                         such as light snacks, muffins, or pastries.
    -h, --help           Print help information
    -V, --version        Print version information
";

static ISSUE_626_PANIC: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe <FILE>
            La culture du café est très développée
            dans de nombreux pays à climat chaud
            d\'Amérique, d\'Afrique et d\'Asie, dans
            des plantations qui sont cultivées pour
            les marchés d\'exportation. Le café est
            souvent une contribution majeure aux
            exportations des régions productrices.

    -h, --help
            Print help information

    -V, --version
            Print version information
";

static HIDE_POS_VALS: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café.
    -h, --help           Print help information
    -p, --pos <VAL>      Some vals [possible values: fast, slow]
    -V, --version        Print version information
";

static FINAL_WORD_WRAPPING: &str = "ctest 0.1

USAGE:
    ctest

OPTIONS:
    -h, --help
            Print help
            information

    -V, --version
            Print
            version
            information
";

static OLD_NEWLINE_CHARS: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -h, --help       Print help information
    -m               Some help with some wrapping
                     (Defaults to something)
    -V, --version    Print version information
";

static WRAPPING_NEWLINE_CHARS: &str = "ctest 0.1

USAGE:
    ctest [mode]

ARGS:
    <mode>    x, max, maximum   20 characters, contains
              symbols.
              l, long           Copy-friendly, 14
              characters, contains symbols.
              m, med, medium    Copy-friendly, 8
              characters, contains symbols.

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

static ISSUE_688: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
        --filter <filter>    Sets the filter, or sampling method, to use for interpolation when resizing the particle
                             images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic,
                             Gaussian, Lanczos3]
    -h, --help               Print help information
    -V, --version            Print version information
";

static ISSUE_702: &str = "myapp 1.0
foo
bar

USAGE:
    myapp [OPTIONS] [--] [ARGS]

ARGS:
    <arg1>       some option
    <arg2>...    some option

OPTIONS:
    -h, --help                Print help information
    -l, --label <label>...    a label
    -o, --other <other>       some other option
    -s, --some <some>         some option
    -V, --version             Print version information
";

static ISSUE_777: &str = "A cmd with a crazy very long long
long name hahaha 1.0
Some Very Long Name and crazy long
email <email@server.com>
Show how the about text is not
wrapped

USAGE:
    ctest

OPTIONS:
    -h, --help
            Print help information

    -V, --version
            Print version
            information
";

static ISSUE_1642: &str = "prog 

USAGE:
    prog [OPTIONS]

OPTIONS:
        --config
            The config file used by the myprog must be in JSON format
            with only valid keys and may not contain other nonsense
            that cannot be read by this program. Obviously I'm going on
            and on, so I'll stop now.

    -h, --help
            Print help information
";

static HELP_CONFLICT: &str = "conflict 

USAGE:
    conflict [OPTIONS]

OPTIONS:
    -h            
        --help    Print help information
";

static LAST_ARG: &str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] [-- <ARGS>...]

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

static LAST_ARG_SC: &str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] [-- <ARGS>...]
    last <SUBCOMMAND>

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    test    some
";

static LAST_ARG_REQ: &str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] -- <ARGS>...

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

static LAST_ARG_REQ_SC: &str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] -- <ARGS>...
    last <SUBCOMMAND>

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    test    some
";

static HIDE_DEFAULT_VAL: &str = "default 0.1

USAGE:
    default [OPTIONS]

OPTIONS:
        --arg <argument>    Pass an argument to the program. [default: default-argument]
    -h, --help              Print help information
    -V, --version           Print version information
";

static ESCAPED_DEFAULT_VAL: &str = "default 0.1

USAGE:
    default [OPTIONS]

OPTIONS:
        --arg <argument>    Pass an argument to the program. [default: \"\\n\"] [possible values: normal, \" \", \"\\n\", \"\\t\",
                            other]
    -h, --help              Print help information
    -V, --version           Print version information
";

static LAST_ARG_USAGE: &str = "flamegraph 0.1

USAGE:
    flamegraph [OPTIONS] [BINFILE] [-- <ARGS>...]

ARGS:
    <BINFILE>    The path of the binary to be profiled. for a binary.
    <ARGS>...    Any arguments you wish to pass to the being profiled.

OPTIONS:
    -f, --frequency <HERTZ>    The sampling frequency.
    -h, --help                 Print help information
    -t, --timeout <SECONDS>    Timeout in seconds.
    -v, --verbose              Prints out more stuff.
    -V, --version              Print version information
";

static LAST_ARG_REQ_MULT: &str = "example 1.0

USAGE:
    example <FIRST>... [--] <SECOND>...

ARGS:
    <FIRST>...     First
    <SECOND>...    Second

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

static DEFAULT_HELP: &str = "ctest 1.0

USAGE:
    ctest

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

static LONG_ABOUT: &str = "myapp 1.0
foo
something really really long, with
multiple lines of text
that should be displayed

USAGE:
    myapp [arg1]

ARGS:
    <arg1>
            some option

OPTIONS:
    -h, --help
            Print help information

    -V, --version
            Print version information
";

static CUSTOM_HELP_SECTION: &str = "blorp 1.4
Will M.
does stuff

USAGE:
    test [OPTIONS] --fake <some>:<val>

OPTIONS:
    -f, --fake <some>:<val>    some help
    -h, --help                 Print help information
    -V, --version              Print version information

NETWORKING:
    -n, --no-proxy    Do not use system proxy settings
        --port        
";

static ISSUE_1487: &str = "test 

USAGE:
    ctest <arg1|arg2>

ARGS:
    <arg1>    
    <arg2>    

OPTIONS:
    -h, --help    Print help information
";

static ISSUE_1364: &str = "demo 

USAGE:
    demo [OPTIONS] [FILES]...

ARGS:
    <FILES>...    

OPTIONS:
    -f            
    -h, --help    Print help information
";

static OPTION_USAGE_ORDER: &str = "order 

USAGE:
    order [OPTIONS]

OPTIONS:
    -a                     
    -b                     
    -B                     
    -h, --help             Print help information
    -s                     
        --select_file      
        --select_folder    
    -x                     
";

static ABOUT_IN_SUBCOMMANDS_LIST: &str = "about-in-subcommands-list 

USAGE:
    about-in-subcommands-list [SUBCOMMAND]

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    sub     short about sub
";

fn setup() -> Command<'static> {
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
                    .multiple_values(true)
                    .multiple_occurrences(true),
                ),
        ),
    );
    let err = cmd
        .try_get_matches_from(["ctest", "help", "subcmd", "multi", "foo"])
        .unwrap_err();

    static EXPECTED: &str = "error: The subcommand 'foo' wasn't recognized

USAGE:
    ctest subcmd multi [OPTIONS]

For more information try --help
";
    utils::assert_eq(EXPECTED, err.to_string());
}

#[test]
fn req_last_arg_usage() {
    let cmd = Command::new("example")
        .version("1.0")
        .arg(
            Arg::new("FIRST")
                .help("First")
                .multiple_values(true)
                .required(true),
        )
        .arg(
            Arg::new("SECOND")
                .help("Second")
                .multiple_values(true)
                .required(true)
                .last(true),
        );
    utils::assert_output(cmd, "example --help", LAST_ARG_REQ_MULT, false);
}

#[test]
fn args_with_last_usage() {
    let cmd = Command::new("flamegraph")
        .version("0.1")
        .trailing_var_arg(true)
        .arg(
            Arg::new("verbose")
                .help("Prints out more stuff.")
                .short('v')
                .long("verbose")
                .multiple_occurrences(true),
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
                .takes_value(true)
                .multiple_values(true)
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
    utils::assert_output(utils::complex_app(), "clap-test --help", HELP, false);
}

#[test]
fn after_and_before_help_output() {
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
                    .multiple_values(true)
                    .multiple_occurrences(true),
                ),
        ),
    );
    utils::assert_output(cmd, "ctest help subcmd multi", MULTI_SC_HELP, false);
}

#[test]
fn no_wrap_help() {
    let cmd = Command::new("ctest")
        .term_width(0)
        .override_help(MULTI_SC_HELP);
    utils::assert_output(cmd, "ctest --help", &format!("{}\n", MULTI_SC_HELP), false);
}

#[test]
fn no_wrap_default_help() {
    let cmd = Command::new("ctest").version("1.0").term_width(0);
    utils::assert_output(cmd, "ctest --help", DEFAULT_HELP, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn wrapped_help() {
    static WRAPPED_HELP: &str = "test 

USAGE:
    test [OPTIONS]

OPTIONS:
    -a, --all
            Also do versioning for private crates (will not be
            published)

        --exact
            Specify inter dependency version numbers exactly with
            `=`

    -h, --help
            Print help information

        --no-git-commit
            Do not commit version changes

        --no-git-push
            Do not push generated commit and tags to git remote
";
    let cmd = Command::new("test")
        .term_width(67)
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Also do versioning for private crates (will not be published)"),
        )
        .arg(
            Arg::new("exact")
                .long("exact")
                .help("Specify inter dependency version numbers exactly with `=`"),
        )
        .arg(
            Arg::new("no_git_commit")
                .long("no-git-commit")
                .help("Do not commit version changes"),
        )
        .arg(
            Arg::new("no_git_push")
                .long("no-git-push")
                .help("Do not push generated commit and tags to git remote"),
        );
    utils::assert_output(cmd, "test --help", WRAPPED_HELP, false);
}

#[test]
#[cfg(feature = "wrap_help")]
fn unwrapped_help() {
    static UNWRAPPED_HELP: &str = "test 

USAGE:
    test [OPTIONS]

OPTIONS:
    -a, --all              Also do versioning for private crates
                           (will not be published)
        --exact            Specify inter dependency version numbers
                           exactly with `=`
    -h, --help             Print help information
        --no-git-commit    Do not commit version changes
        --no-git-push      Do not push generated commit and tags to
                           git remote
";
    let cmd = Command::new("test")
        .term_width(68)
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Also do versioning for private crates (will not be published)"),
        )
        .arg(
            Arg::new("exact")
                .long("exact")
                .help("Specify inter dependency version numbers exactly with `=`"),
        )
        .arg(
            Arg::new("no_git_commit")
                .long("no-git-commit")
                .help("Do not commit version changes"),
        )
        .arg(
            Arg::new("no_git_push")
                .long("no-git-push")
                .help("Do not push generated commit and tags to git remote"),
        );
    utils::assert_output(cmd, "test --help", UNWRAPPED_HELP, false);
}

#[test]
#[cfg(all(feature = "wrap_help"))]
fn possible_value_wrapped_help() {
    #[cfg(feature = "unstable-v4")]
    static WRAPPED_HELP: &str = "test 

USAGE:
    test [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

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
";
    #[cfg(not(feature = "unstable-v4"))]
    static WRAPPED_HELP: &str = r#"test 

USAGE:
    test [OPTIONS]

OPTIONS:
    -h, --help                                                                   Print help information
        --possible-values <possible_values>                                      [possible values: short_name, second]
        --possible-values-with-new-line <possible_values_with_new_line>          [possible values: "long enough name to trigger new line", second]
        --possible-values-without-new-line <possible_values_without_new_line>    [possible values: name, second]
"#;
    let cmd = Command::new("test")
        .term_width(67)
        .arg(
            Arg::new("possible_values")
                .long("possible-values")
                .possible_value(
                    PossibleValue::new("short_name")
                        .help("Long enough help message, barely warrant wrapping"),
                )
                .possible_value(
                    PossibleValue::new("second").help("Short help gets handled the same"),
                ),
        )
        .arg(
            Arg::new("possible_values_with_new_line")
                .long("possible-values-with-new-line")
                .possible_value(
                    PossibleValue::new("long enough name to trigger new line").help(
                        "Really long enough help message to clearly warrant wrapping believe me",
                    ),
                )
                .possible_value("second"),
        )
        .arg(
            Arg::new("possible_values_without_new_line")
                .long("possible-values-without-new-line")
                .possible_value(
                    PossibleValue::new("name").help("Short enough help message with no wrapping"),
                )
                .possible_value(PossibleValue::new("second").help("short help")),
        );
    utils::assert_output(cmd, "test --help", WRAPPED_HELP, false);
}

#[test]
fn complex_subcommand_help_output() {
    let a = utils::complex_app();
    utils::assert_output(a, "clap-test subcmd --help", SC_HELP, false);
}

#[test]
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
            .takes_value(true),
    );
    utils::assert_output(cmd, "ctest --help", ISSUE_626_CUTOFF, false);
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
                .possible_values(["fast", "slow"])
                .help("Some vals")
                .takes_value(true),
        )
        .arg(
            Arg::new("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .hide_possible_values(true)
                .possible_values(["fast", "slow"])
                .help("A coffeehouse, coffee shop, or café.")
                .takes_value(true),
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
                .possible_values(["fast", "slow"])
                .possible_value(PossibleValue::new("secret speed").hide(true))
                .help("Some vals")
                .takes_value(true),
        )
        .arg(
            Arg::new("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .help("A coffeehouse, coffee shop, or café.")
                .takes_value(true),
        );
    utils::assert_output(cmd, "ctest --help", HIDE_POS_VALS, false);
}

#[test]
fn possible_vals_with_help() {
    #[cfg(feature = "unstable-v4")]
    static POS_VALS_HELP: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe <FILE>
            A coffeehouse, coffee shop, or café.

    -h, --help
            Print help information

    -p, --pos <VAL>
            Some vals

            Possible values:
              - fast
              - slow: not as fast

    -V, --version
            Print version information
";
    #[cfg(not(feature = "unstable-v4"))]
    static POS_VALS_HELP: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café.
    -h, --help           Print help information
    -p, --pos <VAL>      Some vals [possible values: fast, slow]
    -V, --version        Print version information
";
    let app = Command::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("pos")
                .short('p')
                .long("pos")
                .value_name("VAL")
                .possible_value("fast")
                .possible_value(PossibleValue::new("slow").help("not as fast"))
                .possible_value(PossibleValue::new("secret speed").hide(true))
                .help("Some vals")
                .takes_value(true),
        )
        .arg(
            Arg::new("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .help("A coffeehouse, coffee shop, or café.")
                .takes_value(true),
        );
    utils::assert_output(app, "ctest --help", POS_VALS_HELP, false);
}

#[test]
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
           .takes_value(true));
    utils::assert_output(cmd, "ctest --help", ISSUE_626_PANIC, false);
}

#[test]
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
               .takes_value(true))
            .try_get_matches_from(vec!["ctest", "--help"]);
    }
}

#[test]
fn final_word_wrapping() {
    let cmd = Command::new("ctest").version("0.1").term_width(24);
    utils::assert_output(cmd, "ctest --help", FINAL_WORD_WRAPPING, false);
}

#[test]
fn wrapping_newline_chars() {
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
fn wrapping_newline_variables() {
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
fn dont_wrap_urls() {
    let cmd = Command::new("Example")
        .term_width(30)
        .subcommand(Command::new("update").arg(
            Arg::new("force-non-host")
                .help("Install toolchains that require an emulator. See https://github.com/rust-lang/rustup/wiki/Non-host-toolchains")
                .long("force-non-host")
                .takes_value(false))
    );

    const EXPECTED: &str = "\
Example-update 

USAGE:
    Example update [OPTIONS]

OPTIONS:
        --force-non-host
            Install toolchains
            that require an
            emulator. See
            https://github.com/rust-lang/rustup/wiki/Non-host-toolchains

    -h, --help
            Print help
            information
";
    utils::assert_output(cmd, "Example update --help", EXPECTED, false);
}

#[test]
fn old_newline_chars() {
    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("mode")
            .short('m')
            .help("Some help with some wrapping\n(Defaults to something)"),
    );
    utils::assert_output(cmd, "ctest --help", OLD_NEWLINE_CHARS, false);
}

#[test]
fn old_newline_variables() {
    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("mode")
            .short('m')
            .help("Some help with some wrapping{n}(Defaults to something)"),
    );
    utils::assert_output(cmd, "ctest --help", OLD_NEWLINE_CHARS, false);
}

#[test]
fn issue_688_hide_pos_vals() {
    let filter_values = ["Nearest", "Linear", "Cubic", "Gaussian", "Lanczos3"];

    let app1 = Command::new("ctest")
            .version("0.1")
			.term_width(120)
			.hide_possible_values(true)
			.arg(Arg::new("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
                images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
				.long("filter")
				.possible_values(filter_values)
				.takes_value(true));
    utils::assert_output(app1, "ctest --help", ISSUE_688, false);

    let app2 = Command::new("ctest")
            .version("0.1")
			.term_width(120)
			.arg(Arg::new("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
                images. The default is Linear (Bilinear).")
				.long("filter")
				.possible_values(filter_values)
				.takes_value(true));
    utils::assert_output(app2, "ctest --help", ISSUE_688, false);

    let app3 = Command::new("ctest")
            .version("0.1")
			.term_width(120)
			.arg(Arg::new("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
                images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
				.long("filter")
				.takes_value(true));
    utils::assert_output(app3, "ctest --help", ISSUE_688, false);
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
                .takes_value(true)
                .multiple_values(true)
                .help("some option"),
        )
        .arg(
            Arg::new("some")
                .help("some option")
                .short('s')
                .long("some")
                .takes_value(true),
        )
        .arg(
            Arg::new("other")
                .help("some other option")
                .short('o')
                .long("other")
                .takes_value(true),
        )
        .arg(
            Arg::new("label")
                .help("a label")
                .short('l')
                .long("label")
                .multiple_values(true)
                .takes_value(true),
        );
    utils::assert_output(cmd, "myapp --help", ISSUE_702, false);
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
    utils::assert_output(cmd, "myapp --help", LONG_ABOUT, false);
}

#[test]
fn issue_760() {
    let cmd = Command::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("option")
                .help("tests options")
                .short('o')
                .long("option")
                .takes_value(true)
                .multiple_values(true)
                .number_of_values(1),
        )
        .arg(
            Arg::new("opt")
                .help("tests options")
                .short('O')
                .long("opt")
                .takes_value(true),
        );
    utils::assert_output(cmd, "ctest --help", ISSUE_760, false);
}

#[test]
fn issue_1571() {
    let cmd = Command::new("hello").arg(
        Arg::new("name")
            .long("package")
            .short('p')
            .number_of_values(1)
            .takes_value(true)
            .multiple_values(true),
    );
    utils::assert_output(
        cmd,
        "hello --help",
        "hello 

USAGE:
    hello [OPTIONS]

OPTIONS:
    -h, --help              Print help information
    -p, --package <name>    
",
        false,
    );
}

#[test]
fn ripgrep_usage() {
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
    let cmd = Command::new("ripgrep")
        .version("0.5")
        .override_usage(
            "
    rg [OPTIONS] <pattern> [<path> ...]
    rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
    rg [OPTIONS] --files [<path> ...]
    rg [OPTIONS] --type-list",
        )
        .help_template(
            "\
{bin} {version}

USAGE:{usage}

OPTIONS:
{options}",
        );

    utils::assert_output(cmd, "rg --help", RIPGREP_USAGE, false);
}

#[test]
fn sc_negates_reqs() {
    let cmd = Command::new("prog")
        .version("1.0")
        .subcommand_negates_reqs(true)
        .arg(arg!(-o --opt <FILE> "tests options"))
        .arg(Arg::new("PATH").help("help"))
        .subcommand(Command::new("test"));
    utils::assert_output(cmd, "prog --help", SC_NEGATES_REQS, false);
}

#[test]
fn hide_args() {
    let cmd = Command::new("prog")
        .version("1.0")
        .arg(arg!(-f --flag "testing flags"))
        .arg(arg!(-o --opt <FILE> "tests options").required(false))
        .arg(Arg::new("pos").hide(true));
    utils::assert_output(cmd, "prog --help", HIDDEN_ARGS, false);
}

#[test]
fn args_negate_sc() {
    let cmd = Command::new("prog")
        .version("1.0")
        .args_conflicts_with_subcommands(true)
        .arg(arg!(-f --flag "testing flags"))
        .arg(arg!(-o --opt <FILE> "tests options").required(false))
        .arg(Arg::new("PATH").help("help"))
        .subcommand(Command::new("test"));
    utils::assert_output(cmd, "prog --help", ARGS_NEGATE_SC, false);
}

#[test]
fn issue_1046_hide_scs() {
    let cmd = Command::new("prog")
        .version("1.0")
        .arg(arg!(-f --flag "testing flags"))
        .arg(arg!(-o --opt <FILE> "tests options").required(false))
        .arg(Arg::new("PATH").help("some"))
        .subcommand(Command::new("test").hide(true));
    utils::assert_output(cmd, "prog --help", ISSUE_1046_HIDDEN_SCS, false);
}

#[test]
fn issue_777_wrap_all_things() {
    let cmd = Command::new("A cmd with a crazy very long long long name hahaha")
        .version("1.0")
        .author("Some Very Long Name and crazy long email <email@server.com>")
        .about("Show how the about text is not wrapped")
        .term_width(35);
    utils::assert_output(cmd, "ctest --help", ISSUE_777, false);
}

static OVERRIDE_HELP_SHORT: &str = "test 0.1

USAGE:
    test

OPTIONS:
    -H, --help       Print help information
    -V, --version    Print version information
";

#[test]
fn override_help_short() {
    let cmd = Command::new("test")
        .version("0.1")
        .mut_arg("help", |h| h.short('H'));

    utils::assert_output(cmd.clone(), "test --help", OVERRIDE_HELP_SHORT, false);
    utils::assert_output(cmd, "test -H", OVERRIDE_HELP_SHORT, false);
}

static OVERRIDE_HELP_LONG: &str = "test 0.1

USAGE:
    test [OPTIONS]

OPTIONS:
    -h, --hell       Print help information
    -V, --version    Print version information
";

#[test]
fn override_help_long() {
    let cmd = Command::new("test")
        .version("0.1")
        .mut_arg("help", |h| h.long("hell"));

    utils::assert_output(cmd.clone(), "test --hell", OVERRIDE_HELP_LONG, false);
    utils::assert_output(cmd, "test -h", OVERRIDE_HELP_LONG, false);
}

static OVERRIDE_HELP_ABOUT: &str = "test 0.1

USAGE:
    test

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

#[test]
fn override_help_about() {
    let cmd = Command::new("test")
        .version("0.1")
        .mut_arg("help", |h| h.help("Print help information"));

    utils::assert_output(cmd.clone(), "test --help", OVERRIDE_HELP_ABOUT, false);
    utils::assert_output(cmd, "test -h", OVERRIDE_HELP_ABOUT, false);
}

#[test]
fn arg_short_conflict_with_help() {
    let cmd = Command::new("conflict").arg(Arg::new("home").short('h'));

    utils::assert_output(cmd, "conflict --help", HELP_CONFLICT, false);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "`help`s `-h` conflicts with `home`."]
fn arg_short_conflict_with_help_mut_arg() {
    let _ = Command::new("conflict")
        .arg(Arg::new("home").short('h'))
        .mut_arg("help", |h| h.short('h'))
        .try_get_matches_from(vec![""]);
}

#[test]
fn last_arg_mult_usage() {
    let cmd = Command::new("last")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(Arg::new("CORPUS").help("some"))
        .arg(
            Arg::new("ARGS")
                .takes_value(true)
                .multiple_values(true)
                .last(true)
                .help("some"),
        );
    utils::assert_output(cmd, "last --help", LAST_ARG, false);
}

#[test]
fn last_arg_mult_usage_req() {
    let cmd = Command::new("last")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(Arg::new("CORPUS").help("some"))
        .arg(
            Arg::new("ARGS")
                .takes_value(true)
                .multiple_values(true)
                .last(true)
                .required(true)
                .help("some"),
        );
    utils::assert_output(cmd, "last --help", LAST_ARG_REQ, false);
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
                .takes_value(true)
                .multiple_values(true)
                .last(true)
                .required(true)
                .help("some"),
        )
        .subcommand(Command::new("test").about("some"));
    utils::assert_output(cmd, "last --help", LAST_ARG_REQ_SC, false);
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
                .takes_value(true)
                .multiple_values(true)
                .last(true)
                .help("some"),
        )
        .subcommand(Command::new("test").about("some"));
    utils::assert_output(cmd, "last --help", LAST_ARG_SC, false);
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
fn escaped_whitespace_values() {
    let app1 = Command::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .help("Pass an argument to the program.")
            .long("arg")
            .default_value("\n")
            .possible_values(["normal", " ", "\n", "\t", "other"]),
    );
    utils::assert_output(app1, "default --help", ESCAPED_DEFAULT_VAL, false);
}

fn issue_1112_setup() -> Command<'static> {
    Command::new("test")
        .version("1.3")
        .arg(Arg::new("help1").long("help").short('h').help("some help"))
        .subcommand(
            Command::new("foo").arg(Arg::new("help1").long("help").short('h').help("some help")),
        )
}

#[test]
fn prefer_user_help_long_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "--help"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert!(m.unwrap().is_present("help1"));
}

#[test]
fn prefer_user_help_short_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "-h"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert!(m.unwrap().is_present("help1"));
}

#[test]
fn prefer_user_subcmd_help_long_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "foo", "--help"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert!(m
        .unwrap()
        .subcommand_matches("foo")
        .unwrap()
        .is_present("help1"));
}

#[test]
fn prefer_user_subcmd_help_short_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "foo", "-h"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    assert!(m
        .unwrap()
        .subcommand_matches("foo")
        .unwrap()
        .is_present("help1"));
}

#[test]
fn issue_1052_require_delim_help() {
    let cmd = Command::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .arg(
            arg!(-f --fake "some help")
                .required(true)
                .value_names(&["some", "val"])
                .takes_value(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true)
                .value_delimiter(':'),
        );

    utils::assert_output(cmd, "test --help", REQUIRE_DELIM_HELP, false);
}

#[test]
fn custom_headers_headers() {
    let cmd = Command::new("blorp")
        .author("Will M.")
        .about("does stuff")
        .version("1.4")
        .arg(
            arg!(-f --fake "some help")
                .required(true)
                .value_names(&["some", "val"])
                .takes_value(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true)
                .value_delimiter(':'),
        )
        .next_help_heading(Some("NETWORKING"))
        .arg(
            Arg::new("no-proxy")
                .short('n')
                .long("no-proxy")
                .help("Do not use system proxy settings"),
        )
        .args(&[Arg::new("port").long("port")]);

    utils::assert_output(cmd, "test --help", CUSTOM_HELP_SECTION, false);
}

static MULTIPLE_CUSTOM_HELP_SECTIONS: &str = "blorp 1.4
Will M.
does stuff

USAGE:
    test [OPTIONS] --fake <some>:<val> --birthday-song <song> --birthday-song-volume <volume>

OPTIONS:
    -f, --fake <some>:<val>    some help
    -h, --help                 Print help information
    -s, --speed <SPEED>        How fast? [possible values: fast, slow]
        --style <style>        Choose musical style to play the song
    -V, --version              Print version information

NETWORKING:
    -a, --server-addr    Set server address
    -n, --no-proxy       Do not use system proxy settings

OVERRIDE SPECIAL:
    -b, --birthday-song <song>    Change which song is played for birthdays

SPECIAL:
    -v, --birthday-song-volume <volume>    Change the volume of the birthday song
";

#[test]
fn multiple_custom_help_headers() {
    let cmd = Command::new("blorp")
        .author("Will M.")
        .about("does stuff")
        .version("1.4")
        .arg(
            arg!(-f --fake "some help")
                .required(true)
                .value_names(&["some", "val"])
                .takes_value(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true)
                .value_delimiter(':'),
        )
        .next_help_heading(Some("NETWORKING"))
        .arg(
            Arg::new("no-proxy")
                .short('n')
                .long("no-proxy")
                .help("Do not use system proxy settings"),
        )
        .next_help_heading(Some("SPECIAL"))
        .arg(
            arg!(-b --"birthday-song" <song> "Change which song is played for birthdays")
                .help_heading(Some("OVERRIDE SPECIAL")),
        )
        .arg(
            arg!(--style <style> "Choose musical style to play the song")
                .required(false)
                .help_heading(None),
        )
        .arg(arg!(
            -v --"birthday-song-volume" <volume> "Change the volume of the birthday song"
        ))
        .next_help_heading(None)
        .arg(
            Arg::new("server-addr")
                .short('a')
                .long("server-addr")
                .help("Set server address")
                .help_heading(Some("NETWORKING")),
        )
        .arg(
            Arg::new("speed")
                .long("speed")
                .short('s')
                .value_name("SPEED")
                .possible_values(["fast", "slow"])
                .help("How fast?")
                .takes_value(true),
        );

    utils::assert_output(cmd, "test --help", MULTIPLE_CUSTOM_HELP_SECTIONS, false);
}

static CUSTOM_HELP_SECTION_HIDDEN_ARGS: &str = "blorp 1.4
Will M.
does stuff

USAGE:
    test [OPTIONS] --song <song> --song-volume <volume>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

OVERRIDE SPECIAL:
    -b, --song <song>    Change which song is played for birthdays

SPECIAL:
    -v, --song-volume <volume>    Change the volume of the birthday song
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
                .help_heading(Some("OVERRIDE SPECIAL")),
        )
        .arg(arg!(
            -v --"song-volume" <volume> "Change the volume of the birthday song"
        ))
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

static ISSUE_897: &str = "ctest-foo 0.1
Long about foo

USAGE:
    ctest foo

OPTIONS:
    -h, --help
            Print help information

    -V, --version
            Print version information
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

static ISSUE_897_SHORT: &str = "ctest-foo 0.1
About foo

USAGE:
    ctest foo

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
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
    let cmd = Command::new("demo")
        .arg(Arg::new("foo").short('f'))
        .arg(
            Arg::new("baz")
                .short('z')
                .value_name("BAZ")
                .hide_short_help(true),
        )
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .takes_value(true)
                .multiple_values(true),
        );

    utils::assert_output(cmd, "demo -h", ISSUE_1364, false);
}

#[rustfmt::skip]
#[test]
fn issue_1487() {
    let cmd = Command::new("test")
        .arg(Arg::new("arg1")
            .group("group1"))
        .arg(Arg::new("arg2")
            .group("group1"))
        .group(ArgGroup::new("group1")
            .args(&["arg1", "arg2"])
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
    let cmd = Command::new("prog").arg(Arg::new("cfg").long("config").long_help(
        "The config file used by the myprog must be in JSON format
with only valid keys and may not contain other nonsense
that cannot be read by this program. Obviously I'm going on
and on, so I'll stop now.",
    ));
    utils::assert_output(cmd, "prog --help", ISSUE_1642, false);
}

const AFTER_HELP_NO_ARGS: &str = "myapp 1.0

USAGE:
    myapp

This is after help.
";

#[test]
fn after_help_no_args() {
    let mut cmd = Command::new("myapp")
        .version("1.0")
        .disable_help_flag(true)
        .disable_version_flag(true)
        .after_help("This is after help.");

    let help = {
        let mut output = Vec::new();
        cmd.write_help(&mut output).unwrap();
        String::from_utf8(output).unwrap()
    };

    assert_eq!(help, AFTER_HELP_NO_ARGS);
}

static HELP_SUBCMD_HELP: &str = "myapp-help 
Print this message or the help of the given subcommand(s)

USAGE:
    myapp help [SUBCOMMAND]...

ARGS:
    <SUBCOMMAND>...    The subcommand whose help message to display

OPTIONS:
    -h, --help    Print custom help text
";

#[test]
fn help_subcmd_help() {
    let cmd = Command::new("myapp")
        .mut_arg("help", |h| h.help("Print custom help text"))
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    utils::assert_output(cmd.clone(), "myapp help help", HELP_SUBCMD_HELP, false);
}

static SUBCMD_HELP_SUBCMD_HELP: &str = "myapp-subcmd-help 
Print this message or the help of the given subcommand(s)

USAGE:
    myapp subcmd help [SUBCOMMAND]...

ARGS:
    <SUBCOMMAND>...    The subcommand whose help message to display

OPTIONS:
    -h, --help    Print custom help text
";

#[test]
fn subcmd_help_subcmd_help() {
    let cmd = Command::new("myapp")
        .mut_arg("help", |h| h.help("Print custom help text"))
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    utils::assert_output(
        cmd.clone(),
        "myapp subcmd help help",
        SUBCMD_HELP_SUBCMD_HELP,
        false,
    );
}

static HELP_ABOUT_MULTI_SC: &str = "myapp-subcmd-multi 1.0

USAGE:
    myapp subcmd multi

OPTIONS:
    -h, --help       Print custom help text
    -V, --version    Print version information
";

static HELP_ABOUT_MULTI_SC_OVERRIDE: &str = "myapp-subcmd-multi 1.0

USAGE:
    myapp subcmd multi

OPTIONS:
    -h, --help       Print custom help text from multi
    -V, --version    Print version information
";

#[test]
fn help_about_multi_subcmd() {
    let cmd = Command::new("myapp")
        .mut_arg("help", |h| h.help("Print custom help text"))
        .subcommand(Command::new("subcmd").subcommand(Command::new("multi").version("1.0")));

    utils::assert_output(
        cmd.clone(),
        "myapp help subcmd multi",
        HELP_ABOUT_MULTI_SC,
        false,
    );
    utils::assert_output(
        cmd.clone(),
        "myapp subcmd multi -h",
        HELP_ABOUT_MULTI_SC,
        false,
    );
    utils::assert_output(cmd, "myapp subcmd multi --help", HELP_ABOUT_MULTI_SC, false);
}

#[test]
fn help_about_multi_subcmd_override() {
    let cmd = Command::new("myapp")
        .mut_arg("help", |h| h.help("Print custom help text"))
        .subcommand(
            Command::new("subcmd").subcommand(
                Command::new("multi")
                    .version("1.0")
                    .mut_arg("help", |h| h.help("Print custom help text from multi")),
            ),
        );

    utils::assert_output(
        cmd.clone(),
        "myapp help subcmd multi",
        HELP_ABOUT_MULTI_SC_OVERRIDE,
        false,
    );
    utils::assert_output(
        cmd.clone(),
        "myapp subcmd multi -h",
        HELP_ABOUT_MULTI_SC_OVERRIDE,
        false,
    );
    utils::assert_output(
        cmd,
        "myapp subcmd multi --help",
        HELP_ABOUT_MULTI_SC_OVERRIDE,
        false,
    );
}

#[test]
fn option_usage_order() {
    let cmd = Command::new("order").args(&[
        Arg::new("a").short('a'),
        Arg::new("B").short('B'),
        Arg::new("b").short('b'),
        Arg::new("save").short('s'),
        Arg::new("select_file").long("select_file"),
        Arg::new("select_folder").long("select_folder"),
        Arg::new("x").short('x'),
    ]);

    utils::assert_output(cmd, "order --help", OPTION_USAGE_ORDER, false);
}

#[test]
fn prefer_about_over_long_about_in_subcommands_list() {
    let cmd = Command::new("about-in-subcommands-list").subcommand(
        Command::new("sub")
            .long_about("long about sub")
            .about("short about sub"),
    );

    utils::assert_output(
        cmd,
        "about-in-subcommands-list --help",
        ABOUT_IN_SUBCOMMANDS_LIST,
        false,
    );
}

#[test]
fn issue_1794_usage() {
    static USAGE_WITH_GROUP: &str = "hello 

USAGE:
    deno <pos1|--option1> [pos2]

ARGS:
    <pos1>    
    <pos2>    

OPTIONS:
    -h, --help       Print help information
        --option1    
";

    let cmd = clap::Command::new("hello")
        .bin_name("deno")
        .arg(Arg::new("option1").long("option1").takes_value(false))
        .arg(Arg::new("pos1").takes_value(true))
        .group(
            ArgGroup::new("arg1")
                .args(&["pos1", "option1"])
                .required(true),
        )
        .arg(Arg::new("pos2").takes_value(true));

    utils::assert_output(cmd, "deno --help", USAGE_WITH_GROUP, false);
}

static CUSTOM_HEADING_POS: &str = "test 1.4

USAGE:
    test [ARGS]

ARGS:
    <gear>    Which gear

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

NETWORKING:
    <speed>    How fast
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

static ONLY_CUSTOM_HEADING_OPTS_NO_ARGS: &str = "test 1.4

USAGE:
    test [OPTIONS]

NETWORKING:
    -s, --speed <SPEED>    How fast
";

#[test]
fn only_custom_heading_opts_no_args() {
    let cmd = Command::new("test")
        .version("1.4")
        .disable_version_flag(true)
        .mut_arg("help", |a| a.hide(true))
        .next_help_heading(Some("NETWORKING"))
        .arg(arg!(-s --speed <SPEED> "How fast").required(false));

    utils::assert_output(cmd, "test --help", ONLY_CUSTOM_HEADING_OPTS_NO_ARGS, false);
}

static ONLY_CUSTOM_HEADING_POS_NO_ARGS: &str = "test 1.4

USAGE:
    test [speed]

NETWORKING:
    <speed>    How fast
";

#[test]
fn only_custom_heading_pos_no_args() {
    let cmd = Command::new("test")
        .version("1.4")
        .disable_version_flag(true)
        .mut_arg("help", |a| a.hide(true))
        .next_help_heading(Some("NETWORKING"))
        .arg(Arg::new("speed").help("How fast"));

    utils::assert_output(cmd, "test --help", ONLY_CUSTOM_HEADING_POS_NO_ARGS, false);
}

#[test]
fn issue_2508_number_of_values_with_single_value_name() {
    let cmd = Command::new("my_app")
        .arg(Arg::new("some_arg").long("some_arg").number_of_values(2))
        .arg(
            Arg::new("some_arg_issue")
                .long("some_arg_issue")
                .number_of_values(2)
                .value_name("ARG"),
        );
    utils::assert_output(
        cmd,
        "my_app --help",
        "my_app 

USAGE:
    my_app [OPTIONS]

OPTIONS:
    -h, --help                              Print help information
        --some_arg <some_arg> <some_arg>    
        --some_arg_issue <ARG> <ARG>        
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
        "test 

USAGE:
    test [arg1] <arg2>

ARGS:
    <arg1>    
    <arg2>    

OPTIONS:
    -h, --help    Print help information
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
        .arg(Arg::new("baz").takes_value(true).multiple_values(true));
    utils::assert_output(
        cmd,
        "test --help",
        "test 

USAGE:
    test [ARGS]

ARGS:
    <foo>       
    <bar>       
    <baz>...    

OPTIONS:
    -h, --help    Print help information
",
        false,
    );
}

#[test]
fn positional_multiple_values_is_dotted() {
    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .takes_value(true)
            .multiple_values(true),
    );
    utils::assert_output(
        cmd,
        "test --help",
        "test 

USAGE:
    test <foo>...

ARGS:
    <foo>...    

OPTIONS:
    -h, --help    Print help information
",
        false,
    );

    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .takes_value(true)
            .value_name("BAR")
            .multiple_values(true),
    );
    utils::assert_output(
        cmd,
        "test --help",
        "test 

USAGE:
    test <BAR>...

ARGS:
    <BAR>...    

OPTIONS:
    -h, --help    Print help information
",
        false,
    );
}

#[test]
fn positional_multiple_occurrences_is_dotted() {
    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .takes_value(true)
            .multiple_occurrences(true),
    );
    utils::assert_output(
        cmd,
        "test --help",
        "test 

USAGE:
    test <foo>...

ARGS:
    <foo>...    

OPTIONS:
    -h, --help    Print help information
",
        false,
    );

    let cmd = Command::new("test").arg(
        Arg::new("foo")
            .required(true)
            .takes_value(true)
            .value_name("BAR")
            .multiple_occurrences(true),
    );
    utils::assert_output(
        cmd,
        "test --help",
        "test 

USAGE:
    test <BAR>...

ARGS:
    <BAR>...    

OPTIONS:
    -h, --help    Print help information
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
            .takes_value(true)
            .number_of_values(3)
            .value_names(&["one", "two"]),
    );
    utils::assert_output(
        cmd,
        "test --help",
        "test 

USAGE:
    test --foo <one> <two>...

OPTIONS:
        --foo <one> <two>...    
    -h, --help                  Print help information
",
        false,
    );
}

#[test]
#[cfg(not(feature = "unstable-v4"))]
fn too_many_value_names_panics() {
    Command::new("test")
        .arg(
            Arg::new("foo")
                .long("foo")
                .required(true)
                .takes_value(true)
                .number_of_values(1)
                .value_names(&["one", "two"]),
        )
        .debug_assert()
}

#[test]
#[cfg(feature = "unstable-v4")]
#[should_panic = "Argument foo: Too many value names (2) compared to number_of_values (1)"]
fn too_many_value_names_panics() {
    Command::new("test")
        .arg(
            Arg::new("foo")
                .long("foo")
                .required(true)
                .takes_value(true)
                .number_of_values(1)
                .value_names(&["one", "two"]),
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
    assert_eq!(err.kind(), ErrorKind::UnrecognizedSubcommand);
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
    assert_eq!(err.kind(), ErrorKind::UnrecognizedSubcommand);
    assert!(
        err.to_string().ends_with('\n'),
        "Errors should have a trailing newline, got {:?}",
        err.to_string()
    );
}

#[test]
fn override_help_subcommand() {
    let cmd = Command::new("bar")
        .subcommand(Command::new("help").arg(Arg::new("arg").takes_value(true)))
        .subcommand(Command::new("not_help").arg(Arg::new("arg").takes_value(true)))
        .disable_help_subcommand(true);
    let matches = cmd.try_get_matches_from(&["bar", "help", "foo"]).unwrap();
    assert_eq!(
        matches.subcommand_matches("help").unwrap().value_of("arg"),
        Some("foo")
    );
}

#[test]
fn override_help_flag_using_long() {
    let cmd = Command::new("foo")
        .subcommand(Command::new("help").long_flag("help"))
        .disable_help_flag(true);
    let matches = cmd.try_get_matches_from(&["foo", "--help"]).unwrap();
    assert!(matches.subcommand_matches("help").is_some());
}

#[test]
fn override_help_flag_using_short() {
    let cmd = Command::new("foo")
        .disable_help_flag(true)
        .subcommand(Command::new("help").short_flag('h'));
    let matches = cmd.try_get_matches_from(&["foo", "-h"]).unwrap();
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
        "example-help 
Print this message or the help of the given subcommand(s)

USAGE:
    example help [SUBCOMMAND]...

ARGS:
    <SUBCOMMAND>...    The subcommand whose help message to display
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
        .map(|a| a.get_id())
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
        "example-help 
Print this message or the help of the given subcommand(s)

USAGE:
    example help [SUBCOMMAND]...

ARGS:
    <SUBCOMMAND>...    The subcommand whose help message to display
",
        false,
    );

    cmd.debug_assert();
}

#[test]
fn help_without_short() {
    let mut cmd = clap::Command::new("test")
        .arg(arg!(-h --hex <NUM>))
        .arg(arg!(--help));

    cmd.build();
    let help = cmd.get_arguments().find(|a| a.get_id() == "help").unwrap();
    assert_eq!(help.get_short(), None);

    let m = cmd.try_get_matches_from(["test", "-h", "0x100"]).unwrap();
    assert_eq!(m.value_of("hex"), Some("0x100"));
}

#[test]
fn parent_cmd_req_in_usage_with_help_flag() {
    static EXPECTED: &str = "parent-test 
some

USAGE:
    parent <TARGET> <ARGS> test

OPTIONS:
    -h, --help    Print help information
";
    let cmd = Command::new("parent")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(
            Arg::new("ARGS")
                .takes_value(true)
                .required(true)
                .help("some"),
        )
        .subcommand(Command::new("test").about("some"));
    utils::assert_output(cmd, "parent test --help", EXPECTED, false);
}

#[test]
fn parent_cmd_req_in_usage_with_help_subcommand() {
    static EXPECTED: &str = "parent-test 
some

USAGE:
    parent <TARGET> <ARGS> test

OPTIONS:
    -h, --help    Print help information
";
    let cmd = Command::new("parent")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(
            Arg::new("ARGS")
                .takes_value(true)
                .required(true)
                .help("some"),
        )
        .subcommand(Command::new("test").about("some"));
    utils::assert_output(cmd, "parent help test", EXPECTED, false);
}

#[test]
fn parent_cmd_req_in_usage_with_render_help() {
    static EXPECTED: &str = "parent-test 
some

USAGE:
    parent <TARGET> <ARGS> test

OPTIONS:
    -h, --help    Print help information
";
    let mut cmd = Command::new("parent")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).help("some"))
        .arg(
            Arg::new("ARGS")
                .takes_value(true)
                .required(true)
                .help("some"),
        )
        .subcommand(Command::new("test").about("some"));
    cmd.build();
    let subcmd = cmd.find_subcommand_mut("test").unwrap();

    let mut buf = Vec::new();
    subcmd.write_help(&mut buf).unwrap();
    utils::assert_eq(EXPECTED, String::from_utf8(buf).unwrap());
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
