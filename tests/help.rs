#[macro_use]
extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, AppSettings, Arg, ArgGroup, ArgSettings, ErrorKind};

static REQUIRE_DELIM_HELP: &'static str = "test 1.3
Kevin K.
tests stuff

USAGE:
    test --fake <some>:<val>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --fake <some>:<val>    some help";

static HELP: &'static str = "clap-test v1.4.8
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
    -O, --Option <option3>           specific vals [possible values: fast, slow]
        --long-option-2 <option2>    tests long options with exclusions
        --maxvals3 <maxvals>...      Tests 3 max vals
        --minvals2 <minvals>...      Tests 2 min vals
        --multvals <one> <two>       Tests mutliple values, not mult occs
        --multvalsmo <one> <two>     Tests mutliple values, and mult occs
    -o, --option <opt>...            tests options

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    subcmd    tests subcommands";

static SC_NEGATES_REQS: &'static str = "prog 1.0

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

static ARGS_NEGATE_SC: &'static str = "prog 1.0

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

static AFTER_HELP: &'static str = "some text that comes before the help

clap-test v1.4.8
tests clap library

USAGE:
    clap-test

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

some text that comes after the help";

static HIDDEN_ARGS: &'static str = "prog 1.0

USAGE:
    prog [FLAGS] [OPTIONS]

FLAGS:
    -f, --flag       testing flags
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <FILE>    tests options";

static SC_HELP: &'static str = "clap-test-subcmd 0.1
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

static ISSUE_1046_HIDDEN_SCS: &'static str = "prog 1.0

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
static ISSUE_760: &'static str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -O, --opt <opt>             tests options
    -o, --option <option>...    tests options";

static RIPGREP_USAGE: &'static str = "ripgrep 0.5

USAGE:
    rg [OPTIONS] <pattern> [<path> ...]
    rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
    rg [OPTIONS] --files [<path> ...]
    rg [OPTIONS] --type-list

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static MULTI_SC_HELP: &'static str = "ctest-subcmd-multi 0.1
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

static ISSUE_626_CUTOFF: &'static str = "ctest 0.1

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

static ISSUE_626_PANIC: &'static str = "ctest 0.1

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

static HIDE_POS_VALS: &'static str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café.
    -p, --pos <VAL>      Some vals [possible values: fast, slow]";

static FINAL_WORD_WRAPPING: &'static str = "ctest 0.1

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

static OLD_NEWLINE_CHARS: &'static str = "ctest 0.1

USAGE:
    ctest [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -m               Some help with some wrapping
                     (Defaults to something)
    -V, --version    Prints version information";

static WRAPPING_NEWLINE_CHARS: &'static str = "ctest 0.1

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

static ISSUE_688: &'static str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --filter <filter>    Sets the filter, or sampling method, to use for interpolation when resizing the particle
                             images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic,
                             Gaussian, Lanczos3]";

static ISSUE_702: &'static str = "myapp 1.0
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

static ISSUE_777: &'static str = "A app with a crazy very long long
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

static CUSTOM_VERSION_AND_HELP: &'static str = "customize 0.1
Nobody <odysseus@example.com>
You can customize the version and help text

USAGE:
    customize

FLAGS:
    -H, --help       Print help information
    -v, --version    Print version information";

static LAST_ARG: &'static str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] [-- <ARGS>...]

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static LAST_ARG_SC: &'static str = "last 0.1

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

static LAST_ARG_REQ: &'static str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] -- <ARGS>...

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static LAST_ARG_REQ_SC: &'static str = "last 0.1

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

static HIDE_DEFAULT_VAL: &'static str = "default 0.1

USAGE:
    default [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --arg <argument>    Pass an argument to the program. [default: default-argument]";

static LAST_ARG_USAGE: &'static str = "flamegraph 0.1

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

static LAST_ARG_REQ_MULT: &'static str = "example 1.0

USAGE:
    example <FIRST>... [--] <SECOND>...

ARGS:
    <FIRST>...     First
    <SECOND>...    Second

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static DEFAULT_HELP: &'static str = "ctest 1.0

USAGE:
    ctest

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";

static LONG_ABOUT: &'static str = "myapp 1.0
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

static HIDE_ENV_VALS: &'static str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café. [env: ENVVAR]
    -p, --pos <VAL>      Some vals [possible values: fast, slow]";

static SHOW_ENV_VALS: &'static str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cafe <FILE>    A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]
    -p, --pos <VAL>      Some vals [possible values: fast, slow]";

static CUSTOM_HELP_SECTION: &'static str = "blorp 1.4
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

static ISSUE_1487: &'static str = "test 

USAGE:
    ctest <arg1|arg2>

ARGS:
    <arg1>    
    <arg2>    

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information";



fn setup() -> App<'static> {
    App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
}

#[test]
fn help_short() {
    let m = setup().try_get_matches_from(vec!["myprog", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn help_long() {
    let m = setup().try_get_matches_from(vec!["myprog", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
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
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn req_last_arg_usage() {
    let app = clap_app!(example =>
        (version: "1.0")
        (@arg FIRST: ... * "First")
        (@arg SECOND: ... * +last "Second")
    );
    assert!(test::compare_output(
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
            Arg::with_name("verbose")
                .help("Prints out more stuff.")
                .short('v')
                .long("verbose")
                .setting(ArgSettings::MultipleOccurrences),
        )
        .arg(
            Arg::with_name("timeout")
                .help("Timeout in seconds.")
                .short('t')
                .long("timeout")
                .value_name("SECONDS"),
        )
        .arg(
            Arg::with_name("frequency")
                .help("The sampling frequency.")
                .short('f')
                .long("frequency")
                .value_name("HERTZ"),
        )
        .arg(
            Arg::with_name("binary path")
                .help("The path of the binary to be profiled. for a binary.")
                .value_name("BINFILE"),
        )
        .arg(
            Arg::with_name("pass through args")
                .help("Any arguments you wish to pass to the being profiled.")
                .settings(&[
                    ArgSettings::MultipleValues,
                    ArgSettings::MultipleOccurrences,
                    ArgSettings::Last,
                ])
                .value_name("ARGS"),
        );
    assert!(test::compare_output(
        app,
        "flamegraph --help",
        LAST_ARG_USAGE,
        false
    ));
}

#[test]
fn subcommand_short_help() {
    let m = test::complex_app().try_get_matches_from(vec!["clap-test", "subcmd", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn subcommand_long_help() {
    let m = test::complex_app().try_get_matches_from(vec!["clap-test", "subcmd", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn subcommand_help_rev() {
    let m = test::complex_app().try_get_matches_from(vec!["clap-test", "help", "subcmd"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn complex_help_output() {
    assert!(test::compare_output(
        test::complex_app(),
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
    assert!(test::compare_output(
        app,
        "clap-test --help",
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
    assert!(test::compare_output(
        app,
        "ctest help subcmd multi",
        MULTI_SC_HELP,
        false
    ));
}

#[test]
fn no_wrap_help() {
    let app = App::new("ctest")
        .set_term_width(0)
        .override_help(MULTI_SC_HELP);
    assert!(test::compare_output(
        app,
        "ctest --help",
        MULTI_SC_HELP,
        false
    ));
}

#[test]
fn no_wrap_default_help() {
    let app = App::new("ctest").version("1.0").set_term_width(0);
    assert!(test::compare_output(
        app,
        "ctest --help",
        DEFAULT_HELP,
        false
    ));
}

#[test]
fn complex_subcommand_help_output() {
    let a = test::complex_app();
    assert!(test::compare_output(
        a,
        "clap-test subcmd --help",
        SC_HELP,
        false
    ));
}

#[test]
fn issue_626_unicode_cutoff() {
    let app = App::new("ctest").version("0.1").set_term_width(70).arg(
        Arg::with_name("cafe")
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
    assert!(test::compare_output(
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
            Arg::with_name("pos")
                .short('p')
                .long("pos")
                .value_name("VAL")
                .possible_values(&["fast", "slow"])
                .help("Some vals")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .hide_possible_values(true)
                .possible_values(&["fast", "slow"])
                .help("A coffeehouse, coffee shop, or café.")
                .takes_value(true),
        );
    assert!(test::compare_output(
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
        .set_term_width(52)
        .arg(Arg::with_name("cafe")
           .short('c')
           .long("cafe")
           .value_name("FILE")
           .help("La culture du café est très développée dans de nombreux pays à climat chaud d'Amérique, \
           d'Afrique et d'Asie, dans des plantations qui sont cultivées pour les marchés d'exportation. \
           Le café est souvent une contribution majeure aux exportations des régions productrices.")
           .takes_value(true));
    assert!(test::compare_output(
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
            .set_term_width(i)
            .arg(Arg::with_name("cafe")
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
    let app = App::new("ctest").version("0.1").set_term_width(24);
    assert!(test::compare_output(
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
        .set_term_width(60)
        .arg(Arg::with_name("mode").help(
            "x, max, maximum   20 characters, contains symbols.{n}\
             l, long           Copy-friendly, 14 characters, contains symbols.{n}\
             m, med, medium    Copy-friendly, 8 characters, contains symbols.{n}",
        ));
    assert!(test::compare_output(
        app,
        "ctest --help",
        WRAPPING_NEWLINE_CHARS,
        false
    ));
}

#[test]
fn old_newline_chars() {
    let app = App::new("ctest").version("0.1").arg(
        Arg::with_name("mode")
            .short('m')
            .help("Some help with some wrapping{n}(Defaults to something)"),
    );
    assert!(test::compare_output(
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
			.set_term_width(120)
			.setting(AppSettings::HidePossibleValuesInHelp)
			.arg(Arg::with_name("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
                images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
				.long("filter")
				.possible_values(&filter_values)
				.takes_value(true));
    assert!(test::compare_output(app1, "ctest --help", ISSUE_688, false));

    let app2 = App::new("ctest")
            .version("0.1")
			.set_term_width(120)
			.arg(Arg::with_name("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
                images. The default is Linear (Bilinear).")
				.long("filter")
				.possible_values(&filter_values)
				.takes_value(true));
    assert!(test::compare_output(app2, "ctest --help", ISSUE_688, false));

    let app3 = App::new("ctest")
            .version("0.1")
			.set_term_width(120)
			.arg(Arg::with_name("filter")
				.help("Sets the filter, or sampling method, to use for interpolation when resizing the particle \
                images. The default is Linear (Bilinear). [possible values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
				.long("filter")
				.takes_value(true));
    assert!(test::compare_output(app3, "ctest --help", ISSUE_688, false));
}

#[test]
fn issue_702_multiple_values() {
    let app = App::new("myapp")
        .version("1.0")
        .author("foo")
        .about("bar")
        .arg(Arg::with_name("arg1").help("some option"))
        .arg(Arg::with_name("arg2").multiple(true).help("some option"))
        .arg(
            Arg::with_name("some")
                .help("some option")
                .short('s')
                .long("some")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("other")
                .help("some other option")
                .short('o')
                .long("other")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("label")
                .help("a label")
                .short('l')
                .long("label")
                .multiple(true)
                .takes_value(true),
        );
    assert!(test::compare_output(app, "myapp --help", ISSUE_702, false));
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
        .arg(Arg::with_name("arg1").help("some option"));
    assert!(test::compare_output(app, "myapp --help", LONG_ABOUT, false));
}

#[test]
fn issue_760() {
    let app = App::new("ctest")
        .version("0.1")
        .arg(
            Arg::with_name("option")
                .help("tests options")
                .short('o')
                .long("option")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
        .arg(
            Arg::with_name("opt")
                .help("tests options")
                .short('O')
                .long("opt")
                .takes_value(true),
        );
    assert!(test::compare_output(app, "ctest --help", ISSUE_760, false));
}

#[test]
fn ripgrep_usage() {
    let app = App::new("ripgrep").version("0.5").override_usage(
        "rg [OPTIONS] <pattern> [<path> ...]
    rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
    rg [OPTIONS] --files [<path> ...]
    rg [OPTIONS] --type-list",
    );

    assert!(test::compare_output(app, "rg --help", RIPGREP_USAGE, false));
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

    assert!(test::compare_output(app, "rg --help", RIPGREP_USAGE, false));
}

#[test]
fn sc_negates_reqs() {
    let app = App::new("prog")
        .version("1.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg("-o, --opt <FILE> 'tests options'")
        .arg(Arg::with_name("PATH").help("help"))
        .subcommand(App::new("test"));
    assert!(test::compare_output(
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
        .arg(Arg::with_name("pos").hidden(true));
    assert!(test::compare_output(app, "prog --help", HIDDEN_ARGS, false));
}

#[test]
fn args_negate_sc() {
    let app = App::new("prog")
        .version("1.0")
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg("-f, --flag 'testing flags'")
        .arg("-o, --opt [FILE] 'tests options'")
        .arg(Arg::with_name("PATH").help("help"))
        .subcommand(App::new("test"));
    assert!(test::compare_output(
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
        .arg(Arg::with_name("PATH").help("some"))
        .subcommand(App::new("test").setting(AppSettings::Hidden));
    assert!(test::compare_output(
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
        .set_term_width(35);
    assert!(test::compare_output(app, "ctest --help", ISSUE_777, false));
}

#[test]
fn customize_version_and_help() {
    let app = App::new("customize")
        .version("0.1")
        .author("Nobody <odysseus@example.com>")
        .about("You can customize the version and help text")
        .mut_arg("help", |h| {
            h.short('H').long("help").help("Print help information")
        })
        .mut_arg("version", |v| {
            v.short('v')
                .long("version")
                .help("Print version information")
        });
    assert!(test::compare_output(
        app,
        "customize --help",
        CUSTOM_VERSION_AND_HELP,
        false
    ));
}

#[test]
fn last_arg_mult_usage() {
    let app = App::new("last")
        .version("0.1")
        .arg(Arg::with_name("TARGET").required(true).help("some"))
        .arg(Arg::with_name("CORPUS").help("some"))
        .arg(
            Arg::with_name("ARGS")
                .multiple(true)
                .last(true)
                .help("some"),
        );
    assert!(test::compare_output(app, "last --help", LAST_ARG, false));
}

#[test]
fn last_arg_mult_usage_req() {
    let app = App::new("last")
        .version("0.1")
        .arg(Arg::with_name("TARGET").required(true).help("some"))
        .arg(Arg::with_name("CORPUS").help("some"))
        .arg(
            Arg::with_name("ARGS")
                .multiple(true)
                .last(true)
                .required(true)
                .help("some"),
        );
    assert!(test::compare_output(
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
        .arg(Arg::with_name("TARGET").required(true).help("some"))
        .arg(Arg::with_name("CORPUS").help("some"))
        .arg(
            Arg::with_name("ARGS")
                .multiple(true)
                .last(true)
                .required(true)
                .help("some"),
        )
        .subcommand(App::new("test").about("some"));
    assert!(test::compare_output(
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
        .arg(Arg::with_name("TARGET").required(true).help("some"))
        .arg(Arg::with_name("CORPUS").help("some"))
        .arg(
            Arg::with_name("ARGS")
                .multiple(true)
                .last(true)
                .help("some"),
        )
        .subcommand(App::new("test").about("some"));
    assert!(test::compare_output(app, "last --help", LAST_ARG_SC, false));
}

#[test]
fn hidden_default_val() {
    let app1 = App::new("default").version("0.1").set_term_width(120).arg(
        Arg::with_name("argument")
            .help("Pass an argument to the program. [default: default-argument]")
            .long("arg")
            .default_value("default-argument")
            .hide_default_value(true),
    );
    assert!(test::compare_output(
        app1,
        "default --help",
        HIDE_DEFAULT_VAL,
        false
    ));

    let app2 = App::new("default").version("0.1").set_term_width(120).arg(
        Arg::with_name("argument")
            .help("Pass an argument to the program.")
            .long("arg")
            .default_value("default-argument"),
    );
    assert!(test::compare_output(
        app2,
        "default --help",
        HIDE_DEFAULT_VAL,
        false
    ));
}

fn issue_1112_setup() -> App<'static> {
    App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .global_setting(AppSettings::NoAutoHelp)
        .arg(Arg::from("-h, --help 'some help'"))
        .subcommand(App::new("foo").arg(Arg::from("-h, --help 'some help'")))
}

#[test]
fn issue_1112_override_help_long() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "--help"]);

    assert!(m.is_ok());
    assert!(m.unwrap().is_present("help"));
}

#[test]
fn issue_1112_override_help_short() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "-h"]);

    assert!(m.is_ok());
    assert!(m.unwrap().is_present("help"));
}

#[test]
fn issue_1112_override_help_subcmd_long() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "foo", "--help"]);

    assert!(m.is_ok());
    assert!(m
        .unwrap()
        .subcommand_matches("foo")
        .unwrap()
        .is_present("help"));
}

#[test]
fn issue_1112_override_help_subcmd_short() {
    let m = issue_1112_setup().try_get_matches_from(vec!["test", "foo", "-h"]);

    assert!(m.is_ok());
    assert!(m
        .unwrap()
        .subcommand_matches("foo")
        .unwrap()
        .is_present("help"));
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

    assert!(test::compare_output(
        app,
        "test --help",
        REQUIRE_DELIM_HELP,
        false
    ));
}

#[test]
fn hide_env_vals() {
    use std::env;

    env::set_var("ENVVAR", "MYVAL");
    let app = App::new("ctest")
        .version("0.1")
        .arg(
            Arg::with_name("pos")
                .short('p')
                .long("pos")
                .value_name("VAL")
                .possible_values(&["fast", "slow"])
                .help("Some vals")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .hide_env_values(true)
                .env("ENVVAR")
                .help("A coffeehouse, coffee shop, or café.")
                .takes_value(true),
        );
    assert!(test::compare_output(
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
            Arg::with_name("pos")
                .short('p')
                .long("pos")
                .value_name("VAL")
                .possible_values(&["fast", "slow"])
                .help("Some vals")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cafe")
                .short('c')
                .long("cafe")
                .value_name("FILE")
                .hide_possible_values(true)
                .env("ENVVAR")
                .help("A coffeehouse, coffee shop, or café.")
                .takes_value(true),
        );
    assert!(test::compare_output(
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
            Arg::with_name("no-proxy")
                .short('n')
                .long("no-proxy")
                .help("Do not use system proxy settings"),
        );

    assert!(test::compare_output(
        app,
        "test --help",
        CUSTOM_HELP_SECTION,
        false
    ));
}

static MULTIPLE_CUSTOM_HELP_SECTIONS: &'static str = "blorp 1.4
Will M.
does stuff

USAGE:
    test [OPTIONS] --fake <some>:<val> --birthday-song <song>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --fake <some>:<val>    some help
    -s, --speed <SPEED>        How fast? [possible values: fast, slow]

NETWORKING:
    -n, --no-proxy    Do not use system proxy settings

SPECIAL:
    -b, --birthday-song <song>    Change which song is played for birthdays";

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
            Arg::with_name("no-proxy")
                .short('n')
                .long("no-proxy")
                .help("Do not use system proxy settings"),
        )
        .help_heading("SPECIAL")
        .arg(Arg::from(
            "-b, --birthday-song <song> 'Change which song is played for birthdays'",
        ))
        .stop_custom_headings()
        .arg(
            Arg::with_name("speed")
                .long("speed")
                .short('s')
                .value_name("SPEED")
                .possible_values(&["fast", "slow"])
                .help("How fast?")
                .takes_value(true),
        );

    assert!(test::compare_output(
        app,
        "test --help",
        MULTIPLE_CUSTOM_HELP_SECTIONS,
        false
    ));
}

static ISSUE_897: &'static str = "ctest-foo 0.1
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
    assert!(test::compare_output(
        app,
        "ctest foo --help",
        ISSUE_897,
        false
    ));
}

static ISSUE_897_SHORT: &'static str = "ctest-foo 0.1
Long about foo

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
    assert!(test::compare_output(
        app,
        "ctest foo -h",
        ISSUE_897_SHORT,
        false
    ));
}

#[test]
fn issue_1487() {
    let app = App::new("test")
        .arg(Arg::with_name("arg1")
            .group("group1"))
        .arg(Arg::with_name("arg2")
            .group("group1"))
        .group(ArgGroup::with_name("group1")
            .args(&["arg1", "arg2"])
            .required(true));
    assert!(test::compare_output(app, "ctest -h", ISSUE_1487, false));
} 
