mod utils;

use clap::{clap_app, App, AppSettings, Arg, ArgGroup, ArgSettings, ErrorKind};

static REQUIRE_DELIM_HELP: &str = "test 1.3
Kevin K.
tests stuff

USAGE:
    test --fake <some>:<val>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --fake <some>:<val>    some help";

static HELP: &str = "clap-test v1.4.8
Kevin K. <kbknapp@gmail.com>
tests clap library

USAGE:
    clap-test [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

ARGS:
    <positional>        tests positionals
    <positional2>       tests positionals with exclusions
    <positional3>...    tests specific values [possible values: vi, emacs]

FLAGS:
    -f, --flag       tests flags
    -F               tests flags with exclusions
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --long-option-2 <option2>    tests long options with exclusions
        --maxvals3 <maxvals>...      Tests 3 max vals
        --minvals2 <minvals>...      Tests 2 min vals
        --multvals <one> <two>       Tests multiple values, not mult occs
        --multvalsmo <one> <two>     Tests multiple values, and mult occs
    -o, --option <opt>...            tests options
    -O, --Option <option3>           specific vals [possible values: fast, slow]

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    subcmd    tests subcommands";

static SC_NEGATES_REQS: &str = "prog 1.0

USAGE:
    prog --opt <FILE> [PATH]
    prog [PATH] <SUBCOMMAND>

ARGS:
    <PATH>    help

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <FILE>    tests options

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test";

static ARGS_NEGATE_SC: &str = "prog 1.0

USAGE:
    prog [FLAGS] [OPTIONS] [PATH]
    prog <SUBCOMMAND>

ARGS:
    <PATH>    help

FLAGS:
    -f, --flag       testing flags
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <FILE>    tests options

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test";

static AFTER_HELP: &str = "some text that comes before the help

clap-test v1.4.8
tests clap library

USAGE:
    clap-test

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

some text that comes after the help";

static AFTER_LONG_HELP: &str = "some longer text that comes before the help

clap-test v1.4.8
tests clap library

USAGE:
    clap-test

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information

some longer text that comes after the help";

static HIDDEN_ARGS: &str = "prog 1.0

USAGE:
    prog [FLAGS] [OPTIONS]

FLAGS:
    -f, --flag       testing flags
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <FILE>    tests options";

static SC_HELP: &str = "clap-test-subcmd 0.1
Kevin K. <kbknapp@gmail.com>
tests subcommands

USAGE:
    clap-test subcmd [FLAGS] [OPTIONS] [--] [scpositional]

ARGS:
    <scpositional>    tests positionals

FLAGS:
    -f, --flag       tests flags
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --option <scoption>...     tests options
    -s, --subcmdarg <subcmdarg>    tests other args";

static ISSUE_1046_HIDDEN_SCS: &str = "prog 1.0

USAGE:
    prog [FLAGS] [OPTIONS] [PATH]

ARGS:
    <PATH>    some

FLAGS:
    -f, --flag       testing flags
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <FILE>    tests options";

// Using number_of_values(1) with multiple(true) misaligns help message
static ISSUE_760: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --option <option>...    tests options
    -O, --opt <opt>             tests options";

static RIPGREP_USAGE: &str = "ripgrep 0.5

USAGE:
    rg [OPTIONS] <pattern> [<path> ...]
    rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
    rg [OPTIONS] --files [<path> ...]
    rg [OPTIONS] --type-list

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static MULTI_SC_HELP: &str = "ctest-subcmd-multi 0.1
Kevin K. <kbknapp@gmail.com>
tests subcommands

USAGE:
    ctest subcmd multi [FLAGS] [OPTIONS]

FLAGS:
    -f, --flag       tests flags
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --option <scoption>...    tests options";

static ISSUE_626_CUTOFF: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café is an
                         establishment which primarily serves hot
                         coffee, related coffee beverages (e.g., café
                         latte, cappuccino, espresso), tea, and other
                         hot beverages. Some coffeehouses also serve
                         cold beverages such as iced coffee and iced
                         tea. Many cafés also serve some type of food,
                         such as light snacks, muffins, or pastries.";

static ISSUE_626_PANIC: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cafe <FILE>
            La culture du café est très développée
            dans de nombreux pays à climat chaud
            d\'Amérique, d\'Afrique et d\'Asie, dans
            des plantations qui sont cultivées pour
            les marchés d\'exportation. Le café est
            souvent une contribution majeure aux
            exportations des régions productrices.";

static HIDE_POS_VALS: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café.
    -p, --pos <VAL>      Some vals [possible values: fast, slow]";

static FINAL_WORD_WRAPPING: &str = "ctest 0.1

USAGE:
    ctest

FLAGS:
    -h, --help
            Prints help
            information

    -V, --version
            Prints
            version
            information";

static OLD_NEWLINE_CHARS: &str = "ctest 0.1

USAGE:
    ctest [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -m               Some help with some wrapping
                     (Defaults to something)
    -V, --version    Prints version information";

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

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static ISSUE_688: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --filter <filter>    Sets the filter, or sampling method, to use for interpolation when resizing the particle
                             images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic,
                             Gaussian, Lanczos3]";

static ISSUE_702: &str = "myapp 1.0
foo
bar

USAGE:
    myapp [OPTIONS] [--] [ARGS]

ARGS:
    <arg1>       some option
    <arg2>...    some option

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --label <label>...    a label
    -o, --other <other>       some other option
    -s, --some <some>         some option";

static ISSUE_777: &str = "A app with a crazy very long long
long name hahaha 1.0
Some Very Long Name and crazy long
email <email@server.com>
Show how the about text is not
wrapped

USAGE:
    ctest

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version
            information";

static ISSUE_1642: &str = "prog 

USAGE:
    prog [FLAGS]

FLAGS:
        --config
            The config file used by the myprog must be in JSON format
            with only valid keys and may not contain other nonsense
            that cannot be read by this program. Obviously I'm going on
            and on, so I'll stop now.

    -h, --help
            Prints help information

    -V, --version
            Prints version information";

static HELP_CONFLICT: &str = "conflict 

USAGE:
    conflict [FLAGS]

FLAGS:
    -h               
        --help       Prints help information
    -V, --version    Prints version information";

static LAST_ARG: &str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] [-- <ARGS>...]

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static LAST_ARG_SC: &str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] [-- <ARGS>...]
    last <SUBCOMMAND>

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    some";

static LAST_ARG_REQ: &str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] -- <ARGS>...

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static LAST_ARG_REQ_SC: &str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] -- <ARGS>...
    last <SUBCOMMAND>

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    some";

static HIDE_DEFAULT_VAL: &str = "default 0.1

USAGE:
    default [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --arg <argument>    Pass an argument to the program. [default: default-argument]";

static ESCAPED_DEFAULT_VAL: &str = "default 0.1

USAGE:
    default [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --arg <argument>    Pass an argument to the program. [default: \"\\n\"] [possible values: normal, \" \", \"\\n\", \"\\t\",
                            other]";

static LAST_ARG_USAGE: &str = "flamegraph 0.1

USAGE:
    flamegraph [FLAGS] [OPTIONS] [BINFILE] [-- <ARGS>...]

ARGS:
    <BINFILE>    The path of the binary to be profiled. for a binary.
    <ARGS>...    Any arguments you wish to pass to the being profiled.

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    Prints out more stuff.
    -V, --version    Prints version information

OPTIONS:
    -f, --frequency <HERTZ>    The sampling frequency.
    -t, --timeout <SECONDS>    Timeout in seconds.";

static LAST_ARG_REQ_MULT: &str = "example 1.0

USAGE:
    example <FIRST>... [--] <SECOND>...

ARGS:
    <FIRST>...     First
    <SECOND>...    Second

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static DEFAULT_HELP: &str = "ctest 1.0

USAGE:
    ctest

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

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

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information";

static HIDE_ENV: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café.";

static SHOW_ENV: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]";

static HIDE_ENV_VALS: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café. [env: ENVVAR]
    -p, --pos <VAL>      Some vals [possible values: fast, slow]";

static SHOW_ENV_VALS: &str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]
    -p, --pos <VAL>      Some vals [possible values: fast, slow]";

static CUSTOM_HELP_SECTION: &str = "blorp 1.4
Will M.
does stuff

USAGE:
    test --fake <some>:<val>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --fake <some>:<val>    some help

NETWORKING:
    -n, --no-proxy    Do not use system proxy settings";

static ISSUE_1487: &str = "test 

USAGE:
    ctest <arg1|arg2>

ARGS:
    <arg1>    
    <arg2>    

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static ISSUE_1364: &str = "demo 

USAGE:
    demo [FLAGS] [OPTIONS] [FILES]...

ARGS:
    <FILES>...    

FLAGS:
    -f               
    -h, --help       Prints help information
    -V, --version    Prints version information";

static OPTION_USAGE_ORDER: &str = "order 

USAGE:
    order [FLAGS]

FLAGS:
    -a                     
    -b                     
    -B                     
    -h, --help             Prints help information
    -s                     
        --select_file      
        --select_folder    
    -V, --version          Prints version information
    -x";

fn setup() -> App<'static> {
    App::new("test")
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
    assert_eq!(m.unwrap_err().kind, ErrorKind::DisplayHelp);
}

#[test]
fn help_long() {
    let m = setup().try_get_matches_from(vec!["myprog", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::DisplayHelp);
}

#[test]
fn help_no_subcommand() {
    let m = setup().try_get_matches_from(vec!["myprog", "help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn help_subcommand() {
    let m = setup()
        .subcommand(
            App::new("test")
                .about("tests things")
                .arg("-v --verbose 'with verbosity'"),
        )
        .try_get_matches_from(vec!["myprog", "help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::DisplayHelp);
}

#[test]
fn req_last_arg_usage() {
    let app = clap_app!(example =>
        (version: "1.0")
        (@arg FIRST: ... * "First")
        (@arg SECOND: ... * +last "Second")
    );
    assert!(utils::compare_output(
        app,
        "example --help",
        LAST_ARG_REQ_MULT,
        false
    ));
}

#[test]
fn args_with_last_usage() {
    let app = App::new("flamegraph")
        .version("0.1")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::new("verbose")
                .about("Prints out more stuff.")
                .short('v')
                .long("verbose")
                .setting(ArgSettings::MultipleOccurrences),
        )
        .arg(
            Arg::new("timeout")
                .about("Timeout in seconds.")
                .short('t')
                .long("timeout")
                .value_name("SECONDS"),
        )
        .arg(
            Arg::new("frequency")
                .about("The sampling frequency.")
                .short('f')
                .long("frequency")
                .value_name("HERTZ"),
        )
        .arg(
            Arg::new("binary path")
                .about("The path of the binary to be profiled. for a binary.")
                .value_name("BINFILE"),
        )
        .arg(
            Arg::new("pass through args")
                .about("Any arguments you wish to pass to the being profiled.")
                .settings(&[
                    ArgSettings::MultipleValues,
                    ArgSettings::MultipleOccurrences,
                    ArgSettings::Last,
                ])
                .value_name("ARGS"),
        );
    assert!(utils::compare_output(
        app,
        "flamegraph --help",
        LAST_ARG_USAGE,
        false
    ));
}

#[test]
fn subcommand_short_help() {
    let m = utils::complex_app().try_get_matches_from(vec!["clap-test", "subcmd", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::DisplayHelp);
}

#[test]
fn subcommand_long_help() {
    let m = utils::complex_app().try_get_matches_from(vec!["clap-test", "subcmd", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::DisplayHelp);
}

#[test]
fn subcommand_help_rev() {
    let m = utils::complex_app().try_get_matches_from(vec!["clap-test", "help", "subcmd"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::DisplayHelp);
}

#[test]
fn complex_help_output() {
    assert!(utils::compare_output(
        utils::complex_app(),
        "clap-test --help",
        HELP,
        false
    ));
}

#[test]
fn after_and_before_help_output() {
    let app = App::new("clap-test")
        .version("v1.4.8")
        .about("tests clap library")
        .before_help("some text that comes before the help")
        .after_help("some text that comes after the help");
    assert!(utils::compare_output(
        app.clone(),
        "clap-test -h",
        AFTER_HELP,
        false
    ));
    assert!(utils::compare_output(
        app,
        "clap-test --help",
        AFTER_HELP,
        false
    ));
}

#[test]
fn after_and_before_long_help_output() {
    let app = App::new("clap-test")
        .version("v1.4.8")
        .about("tests clap library")
        .before_help("some text that comes before the help")
        .after_help("some text that comes after the help")
        .before_long_help("some longer text that comes before the help")
        .after_long_help("some longer text that comes after the help");
    assert!(utils::compare_output(
        app.clone(),
        "clap-test --help",
        AFTER_LONG_HELP,
        false
    ));
    assert!(utils::compare_output(
        app,
        "clap-test -h",
        AFTER_HELP,
        false
    ));
}

#[test]
fn multi_level_sc_help() {
    let app = App::new("ctest").subcommand(
        App::new("subcmd").subcommand(
            App::new("multi")
                .about("tests subcommands")
                .author("Kevin K. <kbknapp@gmail.com>")
                .version("0.1")
                .arg("-f, --flag                    'tests flags'")
                .arg("-o, --option [scoption]...    'tests options'"),
        ),
    );
    assert!(utils::compare_output(
        app,
        "ctest help subcmd multi",
        MULTI_SC_HELP,
        false
    ));
}

#[test]
fn no_wrap_help() {
    let app = App::new("ctest").term_width(0).override_help(MULTI_SC_HELP);
    assert!(utils::compare_output(
        app,
        "ctest --help",
        MULTI_SC_HELP,
        false
    ));
}

#[test]
fn no_wrap_default_help() {
    let app = App::new("ctest").version("1.0").term_width(0);
    assert!(utils::compare_output(
        app,
        "ctest --help",
        DEFAULT_HELP,
        false
    ));
}

#[test]
#[cfg(feature = "wrap_help")]
fn wrapped_help() {
    static WRAPPED_HELP: &str = "test 

USAGE:
    test [FLAGS]

FLAGS:
    -a, --all
            Also do versioning for private crates (will not be
            published)

        --exact
            Specify inter dependency version numbers exactly with
            `=`

    -h, --help
            Prints help information

        --no-git-commit
            Do not commit version changes

        --no-git-push
            Do not push generated commit and tags to git remote

    -V, --version
            Prints version information";
    let app = App::new("test")
        .term_width(67)
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .about("Also do versioning for private crates (will not be published)"),
        )
        .arg(
            Arg::new("exact")
                .long("exact")
                .about("Specify inter dependency version numbers exactly with `=`"),
        )
        .arg(
            Arg::new("no_git_commit")
                .long("no-git-commit")
                .about("Do not commit version changes"),
        )
        .arg(
            Arg::new("no_git_push")
                .long("no-git-push")
                .about("Do not push generated commit and tags to git remote"),
        );
    assert!(utils::compare_output(
        app,
        "test --help",
        WRAPPED_HELP,
        false
    ));
}

#[test]
#[cfg(feature = "wrap_help")]
fn unwrapped_help() {
    static UNWRAPPED_HELP: &str = "test 

USAGE:
    test [FLAGS]

FLAGS:
    -a, --all              Also do versioning for private crates
                           (will not be published)
        --exact            Specify inter dependency version numbers
                           exactly with `=`
    -h, --help             Prints help information
        --no-git-commit    Do not commit version changes
        --no-git-push      Do not push generated commit and tags to
                           git remote
    -V, --version          Prints version information";
    let app = App::new("test")
        .term_width(68)
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .about("Also do versioning for private crates (will not be published)"),
        )
        .arg(
            Arg::new("exact")
                .long("exact")
                .about("Specify inter dependency version numbers exactly with `=`"),
        )
        .arg(
            Arg::new("no_git_commit")
                .long("no-git-commit")
                .about("Do not commit version changes"),
        )
        .arg(
            Arg::new("no_git_push")
                .long("no-git-push")
                .about("Do not push generated commit and tags to git remote"),
        );
    assert!(utils::compare_output(
        app,
        "test --help",
        UNWRAPPED_HELP,
        false
    ));
}

#[test]
fn complex_subcommand_help_output() {
    let a = utils::complex_app();
    assert!(utils::compare_output(
        a,
        "clap-test subcmd --help",
        SC_HELP,
        false
    ));
}

#[test]
fn issue_626_unicode_cutoff() {
    let app = App::new("ctest").version("0.1").term_width(70).arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .about(
                "A coffeehouse, coffee shop, or café is an establishment \
                 which primarily serves hot coffee, related coffee beverages \
                 (e.g., café latte, cappuccino, espresso), tea, and other hot \
                 beverages. Some coffeehouses also serve cold beverages such as \
                 iced coffee and iced tea. Many cafés also serve some type of \
                 food, such as light snacks, muffins, or pastries.",
            )
            .takes_value(true),
    );
    assert!(utils::compare_output(
        app,
        "ctest --help",
        ISSUE_626_CUTOFF,
        false
    ));
}

#[test]
fn hide_possible_vals() {
    let app = App::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("pos")
                .short('p')
                .long("pos")
                .value_name("VAL")
                .possible_values(&["fast", "slow"])
                .about("Some vals")
                .takes_value(true),
        )
        .arg(
            Arg::new("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .hide_possible_values(true)
                .possible_values(&["fast", "slow"])
                .about("A coffeehouse, coffee shop, or café.")
                .takes_value(true),
        );
    assert!(utils::compare_output(
        app,
        "ctest --help",
        HIDE_POS_VALS,
        false
    ));
}

#[test]
fn issue_626_panic() {
    let app = App::new("ctest")
        .version("0.1")
        .term_width(52)
        .arg(Arg::new("cafe")
           .short('c')
           .long("cafe")
           .value_name("FILE")
           .about("La culture du café est très développée dans de nombreux pays à climat chaud d'Amérique, \
           d'Afrique et d'Asie, dans des plantations qui sont cultivées pour les marchés d'exportation. \
           Le café est souvent une contribution majeure aux exportations des régions productrices.")
           .takes_value(true));
    assert!(utils::compare_output(
        app,
        "ctest --help",
        ISSUE_626_PANIC,
        false
    ));
}

#[test]
fn issue_626_variable_panic() {
    for i in 10..320 {
        let _ = App::new("ctest")
            .version("0.1")
            .term_width(i)
            .arg(Arg::new("cafe")
               .short('c')
               .long("cafe")
               .value_name("FILE")
               .about("La culture du café est très développée dans de nombreux pays à climat chaud d'Amérique, \
               d'Afrique et d'Asie, dans des plantations qui sont cultivées pour les marchés d'exportation. \
               Le café est souvent une contribution majeure aux exportations des régions productrices.")
               .takes_value(true))
            .try_get_matches_from(vec!["ctest", "--help"]);
    }
}

#[test]
fn final_word_wrapping() {
    let app = App::new("ctest").version("0.1").term_width(24);
    assert!(utils::compare_output(
        app,
        "ctest --help",
        FINAL_WORD_WRAPPING,
        false
    ));
}

#[test]
fn wrapping_newline_chars() {
    let app = App::new("ctest")
        .version("0.1")
        .term_width(60)
        .arg(Arg::new("mode").about(
            "x, max, maximum   20 characters, contains symbols.\n\
             l, long           Copy-friendly, 14 characters, contains symbols.\n\
             m, med, medium    Copy-friendly, 8 characters, contains symbols.\n",
        ));
    assert!(utils::compare_output(
        app,
        "ctest --help",
        WRAPPING_NEWLINE_CHARS,
        false
    ));
}

#[test]
fn old_newline_chars() {
    let app = App::new("ctest").version("0.1").arg(
        Arg::new("mode")
            .short('m')
            .about("Some help with some wrapping\n(Defaults to something)"),
    );
    assert!(utils::compare_output(
        app,
        "ctest --help",
        OLD_NEWLINE_CHARS,
        false
    ));
}

#[test]
fn issue_688_hidden_pos_vals() {
    let filter_values = ["Nearest", "Linear", "Cubic", "Gaussian", "Lanczos3"];

    let app1 = App::new("ctest")
            .version("0.1")
			.term_width(120)
			.setting(AppSettings::HidePossibleValuesInHelp)
			.arg(Arg::new("filter")
				.about("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
                images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
				.long("filter")
				.possible_values(&filter_values)
				.takes_value(true));
    assert!(utils::compare_output(
        app1,
        "ctest --help",
        ISSUE_688,
        false
    ));

    let app2 = App::new("ctest")
            .version("0.1")
			.term_width(120)
			.arg(Arg::new("filter")
				.about("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
                images. The default is Linear (Bilinear).")
				.long("filter")
				.possible_values(&filter_values)
				.takes_value(true));
    assert!(utils::compare_output(
        app2,
        "ctest --help",
        ISSUE_688,
        false
    ));

    let app3 = App::new("ctest")
            .version("0.1")
			.term_width(120)
			.arg(Arg::new("filter")
				.about("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
                images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
				.long("filter")
				.takes_value(true));
    assert!(utils::compare_output(
        app3,
        "ctest --help",
        ISSUE_688,
        false
    ));
}

#[test]
fn issue_702_multiple_values() {
    let app = App::new("myapp")
        .version("1.0")
        .author("foo")
        .about("bar")
        .arg(Arg::new("arg1").about("some option"))
        .arg(Arg::new("arg2").multiple(true).about("some option"))
        .arg(
            Arg::new("some")
                .about("some option")
                .short('s')
                .long("some")
                .takes_value(true),
        )
        .arg(
            Arg::new("other")
                .about("some other option")
                .short('o')
                .long("other")
                .takes_value(true),
        )
        .arg(
            Arg::new("label")
                .about("a label")
                .short('l')
                .long("label")
                .multiple(true)
                .takes_value(true),
        );
    assert!(utils::compare_output(app, "myapp --help", ISSUE_702, false));
}

#[test]
fn long_about() {
    let app = App::new("myapp")
        .version("1.0")
        .author("foo")
        .about("bar")
        .long_about(
            "something really really long, with\nmultiple lines of text\nthat should be displayed",
        )
        .arg(Arg::new("arg1").about("some option"));
    assert!(utils::compare_output(
        app,
        "myapp --help",
        LONG_ABOUT,
        false
    ));
}

#[test]
fn issue_760() {
    let app = App::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("option")
                .about("tests options")
                .short('o')
                .long("option")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
        .arg(
            Arg::new("opt")
                .about("tests options")
                .short('O')
                .long("opt")
                .takes_value(true),
        );
    assert!(utils::compare_output(app, "ctest --help", ISSUE_760, false));
}

#[test]
fn ripgrep_usage() {
    let app = App::new("ripgrep").version("0.5").override_usage(
        "rg [OPTIONS] <pattern> [<path> ...]
    rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
    rg [OPTIONS] --files [<path> ...]
    rg [OPTIONS] --type-list",
    );

    assert!(utils::compare_output(
        app,
        "rg --help",
        RIPGREP_USAGE,
        false
    ));
}

#[test]
fn ripgrep_usage_using_templates() {
    let app = App::new("ripgrep")
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

FLAGS:
{flags}",
        );

    assert!(utils::compare_output(
        app,
        "rg --help",
        RIPGREP_USAGE,
        false
    ));
}

#[test]
fn sc_negates_reqs() {
    let app = App::new("prog")
        .version("1.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg("-o, --opt <FILE> 'tests options'")
        .arg(Arg::new("PATH").about("help"))
        .subcommand(App::new("test"));
    assert!(utils::compare_output(
        app,
        "prog --help",
        SC_NEGATES_REQS,
        false
    ));
}

#[test]
fn hidden_args() {
    let app = App::new("prog")
        .version("1.0")
        .arg("-f, --flag 'testing flags'")
        .arg("-o, --opt [FILE] 'tests options'")
        .arg(Arg::new("pos").hidden(true));
    assert!(utils::compare_output(
        app,
        "prog --help",
        HIDDEN_ARGS,
        false
    ));
}

#[test]
fn args_negate_sc() {
    let app = App::new("prog")
        .version("1.0")
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg("-f, --flag 'testing flags'")
        .arg("-o, --opt [FILE] 'tests options'")
        .arg(Arg::new("PATH").about("help"))
        .subcommand(App::new("test"));
    assert!(utils::compare_output(
        app,
        "prog --help",
        ARGS_NEGATE_SC,
        false
    ));
}

#[test]
fn issue_1046_hidden_scs() {
    let app = App::new("prog")
        .version("1.0")
        .arg("-f, --flag 'testing flags'")
        .arg("-o, --opt [FILE] 'tests options'")
        .arg(Arg::new("PATH").about("some"))
        .subcommand(App::new("test").setting(AppSettings::Hidden));
    assert!(utils::compare_output(
        app,
        "prog --help",
        ISSUE_1046_HIDDEN_SCS,
        false
    ));
}

#[test]
fn issue_777_wrap_all_things() {
    let app = App::new("A app with a crazy very long long long name hahaha")
        .version("1.0")
        .author("Some Very Long Name and crazy long email <email@server.com>")
        .about("Show how the about text is not wrapped")
        .term_width(35);
    assert!(utils::compare_output(app, "ctest --help", ISSUE_777, false));
}

static OVERRIDE_HELP_SHORT: &str = "test 0.1

USAGE:
    test

FLAGS:
    -H, --help       Prints help information
    -V, --version    Prints version information";

#[test]
fn override_help_short() {
    let app = App::new("test")
        .version("0.1")
        .mut_arg("help", |h| h.short('H'));

    assert!(utils::compare_output(
        app.clone(),
        "test --help",
        OVERRIDE_HELP_SHORT,
        false
    ));
    assert!(utils::compare_output(
        app,
        "test -H",
        OVERRIDE_HELP_SHORT,
        false
    ));
}

static OVERRIDE_HELP_LONG: &str = "test 0.1

USAGE:
    test [FLAGS]

FLAGS:
    -h, --hell       Prints help information
    -V, --version    Prints version information";

#[test]
fn override_help_long() {
    let app = App::new("test")
        .version("0.1")
        .mut_arg("help", |h| h.long("hell"));

    assert!(utils::compare_output(
        app.clone(),
        "test --hell",
        OVERRIDE_HELP_LONG,
        false
    ));
    assert!(utils::compare_output(
        app,
        "test -h",
        OVERRIDE_HELP_LONG,
        false
    ));
}

static OVERRIDE_HELP_ABOUT: &str = "test 0.1

USAGE:
    test

FLAGS:
    -h, --help       Print help information
    -V, --version    Prints version information";

#[test]
fn override_help_about() {
    let app = App::new("test")
        .version("0.1")
        .mut_arg("help", |h| h.about("Print help information"));

    assert!(utils::compare_output(
        app.clone(),
        "test --help",
        OVERRIDE_HELP_ABOUT,
        false
    ));
    assert!(utils::compare_output(
        app,
        "test -h",
        OVERRIDE_HELP_ABOUT,
        false
    ));
}

#[test]
fn arg_short_conflict_with_help() {
    let app = App::new("conflict").arg(Arg::new("home").short('h'));

    assert!(utils::compare_output(
        app,
        "conflict --help",
        HELP_CONFLICT,
        false
    ));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Short option names must be unique for each argument, but '-h' is in use by both 'home' and 'help'"]
fn arg_short_conflict_with_help_mut_arg() {
    let _ = App::new("conflict")
        .arg(Arg::new("home").short('h'))
        .mut_arg("help", |h| h.short('h'))
        .try_get_matches_from(vec![""]);
}

#[test]
fn last_arg_mult_usage() {
    let app = App::new("last")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).about("some"))
        .arg(Arg::new("CORPUS").about("some"))
        .arg(Arg::new("ARGS").multiple(true).last(true).about("some"));
    assert!(utils::compare_output(app, "last --help", LAST_ARG, false));
}

#[test]
fn last_arg_mult_usage_req() {
    let app = App::new("last")
        .version("0.1")
        .arg(Arg::new("TARGET").required(true).about("some"))
        .arg(Arg::new("CORPUS").about("some"))
        .arg(
            Arg::new("ARGS")
                .multiple(true)
                .last(true)
                .required(true)
                .about("some"),
        );
    assert!(utils::compare_output(
        app,
        "last --help",
        LAST_ARG_REQ,
        false
    ));
}

#[test]
fn last_arg_mult_usage_req_with_sc() {
    let app = App::new("last")
        .version("0.1")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::new("TARGET").required(true).about("some"))
        .arg(Arg::new("CORPUS").about("some"))
        .arg(
            Arg::new("ARGS")
                .multiple(true)
                .last(true)
                .required(true)
                .about("some"),
        )
        .subcommand(App::new("test").about("some"));
    assert!(utils::compare_output(
        app,
        "last --help",
        LAST_ARG_REQ_SC,
        false
    ));
}

#[test]
fn last_arg_mult_usage_with_sc() {
    let app = App::new("last")
        .version("0.1")
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(Arg::new("TARGET").required(true).about("some"))
        .arg(Arg::new("CORPUS").about("some"))
        .arg(Arg::new("ARGS").multiple(true).last(true).about("some"))
        .subcommand(App::new("test").about("some"));
    assert!(utils::compare_output(
        app,
        "last --help",
        LAST_ARG_SC,
        false
    ));
}

#[test]
fn hidden_default_val() {
    let app1 = App::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .about("Pass an argument to the program. [default: default-argument]")
            .long("arg")
            .default_value("default-argument")
            .hide_default_value(true),
    );
    assert!(utils::compare_output(
        app1,
        "default --help",
        HIDE_DEFAULT_VAL,
        false
    ));

    let app2 = App::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .about("Pass an argument to the program.")
            .long("arg")
            .default_value("default-argument"),
    );
    assert!(utils::compare_output(
        app2,
        "default --help",
        HIDE_DEFAULT_VAL,
        false
    ));
}

#[test]
fn escaped_whitespace_values() {
    let app1 = App::new("default").version("0.1").term_width(120).arg(
        Arg::new("argument")
            .about("Pass an argument to the program.")
            .long("arg")
            .default_value("\n")
            .possible_values(&["normal", " ", "\n", "\t", "other"]),
    );
    assert!(utils::compare_output(
        app1,
        "default --help",
        ESCAPED_DEFAULT_VAL,
        false
    ));
}

fn issue_1112_setup() -> App<'static> {
    App::new("test")
        .version("1.3")
        .arg(Arg::new("help1").long("help").short('h').about("some help"))
        .subcommand(
            App::new("foo").arg(Arg::new("help1").long("help").short('h').about("some help")),
        )
}

#[test]
fn prefer_user_help_long_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "--help"]);

    assert!(m.is_ok());
    assert!(m.unwrap().is_present("help1"));
}

#[test]
fn prefer_user_help_short_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "-h"]);

    assert!(m.is_ok());
    assert!(m.unwrap().is_present("help1"));
}

#[test]
fn prefer_user_subcmd_help_long_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "foo", "--help"]);

    assert!(m.is_ok());
    assert!(m
        .unwrap()
        .subcommand_matches("foo")
        .unwrap()
        .is_present("help1"));
}

#[test]
fn prefer_user_subcmd_help_short_1112() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "foo", "-h"]);

    assert!(m.is_ok());
    assert!(m
        .unwrap()
        .subcommand_matches("foo")
        .unwrap()
        .is_present("help1"));
}

#[test]
fn issue_1052_require_delim_help() {
    let app = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .arg(
            Arg::from("-f, --fake <some> <val> 'some help'")
                .require_delimiter(true)
                .value_delimiter(":"),
        );

    assert!(utils::compare_output(
        app,
        "test --help",
        REQUIRE_DELIM_HELP,
        false
    ));
}

#[test]
fn hide_env() {
    use std::env;

    env::set_var("ENVVAR", "MYVAL");
    let app = App::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .hide_env(true)
            .env("ENVVAR")
            .about("A coffeehouse, coffee shop, or café.")
            .takes_value(true),
    );
    assert!(utils::compare_output(app, "ctest --help", HIDE_ENV, false));
}

#[test]
fn show_env() {
    use std::env;

    env::set_var("ENVVAR", "MYVAL");
    let app = App::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .hide_env(false)
            .env("ENVVAR")
            .about("A coffeehouse, coffee shop, or café.")
            .takes_value(true),
    );
    assert!(utils::compare_output(app, "ctest --help", SHOW_ENV, false));
}

#[test]
fn hide_env_vals() {
    use std::env;

    env::set_var("ENVVAR", "MYVAL");
    let app = App::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("pos")
                .short('p')
                .long("pos")
                .value_name("VAL")
                .possible_values(&["fast", "slow"])
                .about("Some vals")
                .takes_value(true),
        )
        .arg(
            Arg::new("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .hide_env_values(true)
                .env("ENVVAR")
                .about("A coffeehouse, coffee shop, or café.")
                .takes_value(true),
        );
    assert!(utils::compare_output(
        app,
        "ctest --help",
        HIDE_ENV_VALS,
        false
    ));
}

#[test]
fn show_env_vals() {
    use std::env;

    env::set_var("ENVVAR", "MYVAL");
    let app = App::new("ctest")
        .version("0.1")
        .arg(
            Arg::new("pos")
                .short('p')
                .long("pos")
                .value_name("VAL")
                .possible_values(&["fast", "slow"])
                .about("Some vals")
                .takes_value(true),
        )
        .arg(
            Arg::new("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .hide_possible_values(true)
                .env("ENVVAR")
                .about("A coffeehouse, coffee shop, or café.")
                .takes_value(true),
        );
    assert!(utils::compare_output(
        app,
        "ctest --help",
        SHOW_ENV_VALS,
        false
    ));
}

#[test]
fn custom_headers_headers() {
    let app = App::new("blorp")
        .author("Will M.")
        .about("does stuff")
        .version("1.4")
        .arg(
            Arg::from("-f, --fake <some> <val> 'some help'")
                .require_delimiter(true)
                .value_delimiter(":"),
        )
        .help_heading("NETWORKING")
        .arg(
            Arg::new("no-proxy")
                .short('n')
                .long("no-proxy")
                .about("Do not use system proxy settings"),
        );

    assert!(utils::compare_output(
        app,
        "test --help",
        CUSTOM_HELP_SECTION,
        false
    ));
}

static MULTIPLE_CUSTOM_HELP_SECTIONS: &str = "blorp 1.4
Will M.
does stuff

USAGE:
    test [OPTIONS] --fake <some>:<val> --birthday-song <song> --birthday-song-volume <volume>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --fake <some>:<val>    some help
    -s, --speed <SPEED>        How fast? [possible values: fast, slow]

NETWORKING:
    -a, --server-addr    Set server address
    -n, --no-proxy       Do not use system proxy settings

SPECIAL:
    -b, --birthday-song <song>             Change which song is played for birthdays
    -v, --birthday-song-volume <volume>    Change the volume of the birthday song";

#[test]
fn multiple_custom_help_headers() {
    let app = App::new("blorp")
        .author("Will M.")
        .about("does stuff")
        .version("1.4")
        .arg(
            Arg::from("-f, --fake <some> <val> 'some help'")
                .require_delimiter(true)
                .value_delimiter(":"),
        )
        .help_heading("NETWORKING")
        .arg(
            Arg::new("no-proxy")
                .short('n')
                .long("no-proxy")
                .about("Do not use system proxy settings"),
        )
        .help_heading("SPECIAL")
        .arg(
            Arg::from("-b, --birthday-song <song> 'Change which song is played for birthdays'")
                .help_heading(Some("IGNORE THIS")),
        )
        .stop_custom_headings()
        .arg(
            Arg::from(
                "-v --birthday-song-volume <volume> 'Change the volume of the birthday song'",
            )
            .help_heading(Some("SPECIAL")),
        )
        .arg(
            Arg::new("server-addr")
                .short('a')
                .long("server-addr")
                .about("Set server address")
                .help_heading(Some("NETWORKING")),
        )
        .arg(
            Arg::new("speed")
                .long("speed")
                .short('s')
                .value_name("SPEED")
                .possible_values(&["fast", "slow"])
                .about("How fast?")
                .takes_value(true),
        );

    assert!(utils::compare_output(
        app,
        "test --help",
        MULTIPLE_CUSTOM_HELP_SECTIONS,
        false
    ));
}

static ISSUE_897: &str = "ctest-foo 0.1
Long about foo

USAGE:
    ctest foo

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information";

#[test]
fn show_long_about_issue_897() {
    let app = App::new("ctest").version("0.1").subcommand(
        App::new("foo")
            .version("0.1")
            .about("About foo")
            .long_about("Long about foo"),
    );
    assert!(utils::compare_output(
        app,
        "ctest foo --help",
        ISSUE_897,
        false
    ));
}

static ISSUE_897_SHORT: &str = "ctest-foo 0.1
About foo

USAGE:
    ctest foo

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

#[test]
fn show_short_about_issue_897() {
    let app = App::new("ctest").version("0.1").subcommand(
        App::new("foo")
            .version("0.1")
            .about("About foo")
            .long_about("Long about foo"),
    );
    assert!(utils::compare_output(
        app,
        "ctest foo -h",
        ISSUE_897_SHORT,
        false
    ));
}

#[test]
fn issue_1364_no_short_options() {
    let app = App::new("demo")
        .arg(Arg::new("foo").short('f'))
        .arg(
            Arg::new("baz")
                .short('z')
                .value_name("BAZ")
                .hidden_short_help(true),
        )
        .arg(Arg::new("files").value_name("FILES").multiple(true));

    assert!(utils::compare_output(app, "demo -h", ISSUE_1364, false));
}

#[rustfmt::skip]
#[test]
fn issue_1487() {
    let app = App::new("test")
        .arg(Arg::new("arg1")
            .group("group1"))
        .arg(Arg::new("arg2")
            .group("group1"))
        .group(ArgGroup::new("group1")
            .args(&["arg1", "arg2"])
            .required(true));
    assert!(utils::compare_output(app, "ctest -h", ISSUE_1487, false));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "AppSettings::HelpRequired is enabled for the App"]
fn help_required_but_not_given() {
    App::new("myapp")
        .setting(AppSettings::HelpRequired)
        .arg(Arg::new("foo"))
        .get_matches_from(empty_args());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "AppSettings::HelpRequired is enabled for the App"]
fn help_required_but_not_given_settings_after_args() {
    App::new("myapp")
        .arg(Arg::new("foo"))
        .setting(AppSettings::HelpRequired)
        .get_matches_from(empty_args());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "AppSettings::HelpRequired is enabled for the App"]
fn help_required_but_not_given_for_one_of_two_arguments() {
    App::new("myapp")
        .setting(AppSettings::HelpRequired)
        .arg(Arg::new("foo"))
        .arg(Arg::new("bar").about("It does bar stuff"))
        .get_matches_from(empty_args());
}

#[test]
fn help_required_locally_but_not_given_for_subcommand() {
    App::new("myapp")
        .setting(AppSettings::HelpRequired)
        .arg(Arg::new("foo").about("It does foo stuff"))
        .subcommand(
            App::new("bar")
                .arg(Arg::new("create").about("creates bar"))
                .arg(Arg::new("delete")),
        )
        .get_matches_from(empty_args());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "AppSettings::HelpRequired is enabled for the App"]
fn help_required_globally_but_not_given_for_subcommand() {
    App::new("myapp")
        .global_setting(AppSettings::HelpRequired)
        .arg(Arg::new("foo").about("It does foo stuff"))
        .subcommand(
            App::new("bar")
                .arg(Arg::new("create").about("creates bar"))
                .arg(Arg::new("delete")),
        )
        .get_matches_from(empty_args());
}

#[test]
fn help_required_and_given_for_subcommand() {
    App::new("myapp")
        .setting(AppSettings::HelpRequired)
        .arg(Arg::new("foo").about("It does foo stuff"))
        .subcommand(
            App::new("bar")
                .arg(Arg::new("create").about("creates bar"))
                .arg(Arg::new("delete").about("deletes bar")),
        )
        .get_matches_from(empty_args());
}

#[test]
fn help_required_and_given() {
    App::new("myapp")
        .setting(AppSettings::HelpRequired)
        .arg(Arg::new("foo").about("It does foo stuff"))
        .get_matches_from(empty_args());
}

#[test]
fn help_required_and_no_args() {
    App::new("myapp")
        .setting(AppSettings::HelpRequired)
        .get_matches_from(empty_args());
}

#[test]
fn issue_1642_long_help_spacing() {
    let app = App::new("prog").arg(Arg::new("cfg").long("config").long_about(
        "The config file used by the myprog must be in JSON format
with only valid keys and may not contain other nonsense
that cannot be read by this program. Obviously I'm going on
and on, so I'll stop now.",
    ));
    assert!(utils::compare_output(app, "prog --help", ISSUE_1642, false));
}

const AFTER_HELP_NO_ARGS: &str = "myapp 1.0

USAGE:
    myapp

This is after help.
";

#[test]
fn after_help_no_args() {
    let mut app = App::new("myapp")
        .version("1.0")
        .setting(AppSettings::DisableHelpFlag)
        .setting(AppSettings::DisableVersionFlag)
        .after_help("This is after help.");

    let help = {
        let mut output = Vec::new();
        app.write_help(&mut output).unwrap();
        String::from_utf8(output).unwrap()
    };

    assert_eq!(help, AFTER_HELP_NO_ARGS);
}

static HELP_ABOUT_MULTI_SC: &str = "myapp-subcmd-multi 1.0

USAGE:
    myapp subcmd multi

FLAGS:
    -h, --help       Print custom help about text
    -V, --version    Prints version information";

static HELP_ABOUT_MULTI_SC_OVERRIDE: &str = "myapp-subcmd-multi 1.0

USAGE:
    myapp subcmd multi

FLAGS:
    -h, --help       Print custom help about text from multi
    -V, --version    Prints version information";

#[test]
fn help_about_multi_subcmd() {
    let app = App::new("myapp")
        .mut_arg("help", |h| h.about("Print custom help about text"))
        .subcommand(App::new("subcmd").subcommand(App::new("multi").version("1.0")));

    assert!(utils::compare_output(
        app.clone(),
        "myapp help subcmd multi",
        HELP_ABOUT_MULTI_SC,
        false
    ));
    assert!(utils::compare_output(
        app.clone(),
        "myapp subcmd multi -h",
        HELP_ABOUT_MULTI_SC,
        false
    ));
    assert!(utils::compare_output(
        app,
        "myapp subcmd multi --help",
        HELP_ABOUT_MULTI_SC,
        false
    ));
}

#[test]
fn help_about_multi_subcmd_override() {
    let app = App::new("myapp")
        .mut_arg("help", |h| h.about("Print custom help about text"))
        .subcommand(App::new("subcmd").subcommand(
            App::new("multi").version("1.0").mut_arg("help", |h| {
                h.about("Print custom help about text from multi")
            }),
        ));

    assert!(utils::compare_output(
        app.clone(),
        "myapp help subcmd multi",
        HELP_ABOUT_MULTI_SC_OVERRIDE,
        false
    ));
    assert!(utils::compare_output(
        app.clone(),
        "myapp subcmd multi -h",
        HELP_ABOUT_MULTI_SC_OVERRIDE,
        false
    ));
    assert!(utils::compare_output(
        app,
        "myapp subcmd multi --help",
        HELP_ABOUT_MULTI_SC_OVERRIDE,
        false
    ));
}

#[test]
fn option_usage_order() {
    let app = App::new("order").args(&[
        Arg::new("a").short('a'),
        Arg::new("B").short('B'),
        Arg::new("b").short('b'),
        Arg::new("save").short('s'),
        Arg::new("select_file").long("select_file"),
        Arg::new("select_folder").long("select_folder"),
        Arg::new("x").short('x'),
    ]);

    assert!(utils::compare_output(
        app,
        "order --help",
        OPTION_USAGE_ORDER,
        false
    ));
}

#[test]
fn issue_1794_usage() {
    static USAGE_WITH_GROUP: &'static str = "hello 

USAGE:
    deno <pos1|--option1> [pos2]

ARGS:
    <pos1>    
    <pos2>    

FLAGS:
    -h, --help       Prints help information
        --option1    
    -V, --version    Prints version information";

    let app = clap::App::new("hello")
        .bin_name("deno")
        .arg(Arg::new("option1").long("option1").takes_value(false))
        .arg(Arg::new("pos1").takes_value(true))
        .group(
            ArgGroup::new("arg1")
                .args(&["pos1", "option1"])
                .required(true),
        )
        .arg(Arg::new("pos2").takes_value(true));

    assert!(utils::compare_output(
        app,
        "deno --help",
        USAGE_WITH_GROUP,
        false
    ));
}

static ONLY_CUSTOM_HEADING_FLAGS: &'static str = "test 1.4

USAGE:
    test [OPTIONS]

OPTIONS:
        --speed <speed>    How fast

NETWORKING:
        --flag    Some flag";

#[test]
fn only_custom_heading_flags() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .mut_arg("help", |a| a.hidden(true))
        .arg(
            Arg::new("speed")
                .long("speed")
                .takes_value(true)
                .about("How fast"),
        )
        .help_heading("NETWORKING")
        .arg(Arg::new("flag").long("flag").about("Some flag"));

    assert!(utils::compare_output(
        app,
        "test --help",
        ONLY_CUSTOM_HEADING_FLAGS,
        false
    ));
}

static ONLY_CUSTOM_HEADING_OPTS: &'static str = "test 1.4

USAGE:
    test

FLAGS:
    -h, --help    Prints help information

NETWORKING:
    -s, --speed <SPEED>    How fast";

#[test]
fn only_custom_heading_opts() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .help_heading("NETWORKING")
        .arg(Arg::from("-s, --speed [SPEED] 'How fast'"));

    assert!(utils::compare_output(
        app,
        "test --help",
        ONLY_CUSTOM_HEADING_OPTS,
        false
    ));
}

static CUSTOM_HEADING_POS: &'static str = "test 1.4

USAGE:
    test [ARGS]

ARGS:
    <gear>    Which gear

FLAGS:
    -h, --help    Prints help information

NETWORKING:
    <speed>    How fast";

#[test]
fn custom_heading_pos() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .arg(Arg::new("gear").about("Which gear"))
        .help_heading("NETWORKING")
        .arg(Arg::new("speed").about("How fast"));

    assert!(utils::compare_output(
        app,
        "test --help",
        CUSTOM_HEADING_POS,
        false
    ));
}

static ONLY_CUSTOM_HEADING_POS: &'static str = "test 1.4

USAGE:
    test [speed]

FLAGS:
    -h, --help    Prints help information

NETWORKING:
    <speed>    How fast";

#[test]
fn only_custom_heading_pos() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .help_heading("NETWORKING")
        .arg(Arg::new("speed").about("How fast"));

    assert!(utils::compare_output(
        app,
        "test --help",
        ONLY_CUSTOM_HEADING_POS,
        false
    ));
}

static ONLY_CUSTOM_HEADING_FLAGS_NO_ARGS: &'static str = "test 1.4

USAGE:
    test

NETWORKING:
        --flag    Some flag";

#[test]
fn only_custom_heading_flags_no_args() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .mut_arg("help", |a| a.hidden(true))
        .help_heading("NETWORKING")
        .arg(Arg::new("flag").long("flag").about("Some flag"));

    assert!(utils::compare_output(
        app,
        "test --help",
        ONLY_CUSTOM_HEADING_FLAGS_NO_ARGS,
        false
    ));
}

static ONLY_CUSTOM_HEADING_OPTS_NO_ARGS: &'static str = "test 1.4

USAGE:
    test

NETWORKING:
    -s, --speed <SPEED>    How fast";

#[test]
fn only_custom_heading_opts_no_args() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .mut_arg("help", |a| a.hidden(true))
        .help_heading("NETWORKING")
        .arg(Arg::from("-s, --speed [SPEED] 'How fast'"));

    assert!(utils::compare_output(
        app,
        "test --help",
        ONLY_CUSTOM_HEADING_OPTS_NO_ARGS,
        false
    ));
}

static ONLY_CUSTOM_HEADING_POS_NO_ARGS: &'static str = "test 1.4

USAGE:
    test [speed]

NETWORKING:
    <speed>    How fast";

#[test]
fn only_custom_heading_pos_no_args() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .mut_arg("help", |a| a.hidden(true))
        .help_heading("NETWORKING")
        .arg(Arg::new("speed").about("How fast"));

    assert!(utils::compare_output(
        app,
        "test --help",
        ONLY_CUSTOM_HEADING_POS_NO_ARGS,
        false
    ));
}
