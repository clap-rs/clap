extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, AppSettings, SubCommand, ErrorKind, Arg};

static HELP: &'static str = "clap-test v1.4.8
Kevin K. <kbknapp@gmail.com>
tests clap library

USAGE:
    clap-test [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

FLAGS:
    -f, --flag       tests flags
    -F               tests flags with exclusions
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -O, --Option <option3>           specific vals [values: fast, slow]
        --long-option-2 <option2>    tests long options with exclusions
        --maxvals3 <maxvals>...      Tests 3 max vals
        --minvals2 <minvals>...      Tests 2 min vals
        --multvals <one> <two>       Tests mutliple values, not mult occs
        --multvalsmo <one> <two>     Tests mutliple values, and mult occs
    -o, --option <opt>...            tests options

ARGS:
    <positional>        tests positionals
    <positional2>       tests positionals with exclusions
    <positional3>...    tests specific values [values: vi, emacs]

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    subcmd    tests subcommands";

static SC_NEGATES_REQS: &'static str = "prog 1.0

USAGE:
    prog --opt <FILE> [PATH]
    prog [PATH] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <FILE>    tests options

ARGS:
    <PATH>    

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test";

static ARGS_NEGATE_SC: &'static str = "prog 1.0

USAGE:
    prog [FLAGS] [OPTIONS] [PATH]
    prog <SUBCOMMAND>

FLAGS:
    -f, --flag       testing flags
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --opt <FILE>    tests options

ARGS:
    <PATH>    

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

FLAGS:
    -f, --flag       tests flags
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --option <scoption>...    tests options

ARGS:
    <scpositional>    tests positionals";

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
    -p, --pos <VAL>      Some vals [values: fast, slow]";

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


static ISSUE_688: &'static str = "ctest 0.1

USAGE:
    ctest [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --filter <filter>    Sets the filter, or sampling method, to use for interpolation when resizing the particle
                             images. The default is Linear (Bilinear). [values: Nearest, Linear, Cubic, Gaussian,
                             Lanczos3]";

static ISSUE_702: &'static str = "myapp 1.0
foo
bar

USAGE:
    myapp [OPTIONS] [--] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --label <label>...    a label
    -o, --other <other>       some other option
    -s, --some <some>         some option

ARGS:
    <arg1>       some option
    <arg2>...    some option";

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

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some";

static LAST_ARG_SC: &'static str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] [-- <ARGS>...]
    last <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    some";

static LAST_ARG_REQ: &'static str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] -- <ARGS>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some";

static LAST_ARG_REQ_SC: &'static str = "last 0.1

USAGE:
    last <TARGET> [CORPUS] -- <ARGS>...
    last <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <TARGET>     some
    <CORPUS>     some
    <ARGS>...    some

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

#[test]
fn help_short() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn help_long() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn help_no_subcommand() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::UnknownArgument);
}

#[test]
fn help_subcommand() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .subcommand(SubCommand::with_name("test")
            .about("tests things")
            .arg_from_usage("-v --verbose 'with verbosity'"))
        .get_matches_from_safe(vec!["myprog", "help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn subcommand_short_help() {
    let m = test::complex_app().get_matches_from_safe(vec!["clap-test", "subcmd", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn subcommand_long_help() {
    let m = test::complex_app().get_matches_from_safe(vec!["clap-test", "subcmd", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn subcommand_help_rev() {
    let m = test::complex_app().get_matches_from_safe(vec!["clap-test", "help", "subcmd"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn complex_help_output() {
    assert!(test::compare_output(test::complex_app(), "clap-test --help", HELP, false));
}

#[test]
fn after_and_before_help_output() {
    let app = App::new("clap-test")
        .version("v1.4.8")
        .about("tests clap library")
        .before_help("some text that comes before the help")
        .after_help("some text that comes after the help");
    assert!(test::compare_output(app, "clap-test --help", AFTER_HELP, false));
}

#[test]
fn multi_level_sc_help() {
    let app = App::new("ctest")
        .subcommand(SubCommand::with_name("subcmd").subcommand(SubCommand::with_name("multi")
            .about("tests subcommands")
            .author("Kevin K. <kbknapp@gmail.com>")
            .version("0.1")
            .args_from_usage("
                    -f, --flag                    'tests flags'
                    -o, --option [scoption]...    'tests options'
                ")));
    assert!(test::compare_output(app, "ctest help subcmd multi", MULTI_SC_HELP, false));
}

#[test]
fn no_wrap_help() {
    let app = App::new("ctest")
        .set_term_width(0)
        .help(MULTI_SC_HELP);
    assert!(test::compare_output(app, "ctest --help", MULTI_SC_HELP, false));
}

#[test]
fn complex_subcommand_help_output() {
    let a = test::complex_app();
    assert!(test::compare_output(a, "clap-test subcmd --help", SC_HELP, false));
}


#[test]
fn issue_626_unicode_cutoff() {
    let app = App::new("ctest")
        .version("0.1")
        .set_term_width(70)
        .arg(Arg::with_name("cafe")
            .short("c")
            .long("cafe")
            .value_name("FILE")
            .help("A coffeehouse, coffee shop, or café is an establishment \
           which primarily serves hot coffee, related coffee beverages \
           (e.g., café latte, cappuccino, espresso), tea, and other hot \
           beverages. Some coffeehouses also serve cold beverages such as \
           iced coffee and iced tea. Many cafés also serve some type of \
           food, such as light snacks, muffins, or pastries.")
            .takes_value(true));
    assert!(test::compare_output(app, "ctest --help", ISSUE_626_CUTOFF, false));
}

#[test]
fn hide_possible_vals() {
    let app = App::new("ctest")
        .version("0.1")
        .arg(Arg::with_name("pos")
            .short("p")
            .long("pos")
            .value_name("VAL")
            .possible_values(&["fast", "slow"])
            .help("Some vals")
            .takes_value(true))
        .arg(Arg::with_name("cafe")
            .short("c")
            .long("cafe")
            .value_name("FILE")
            .hide_possible_values(true)
            .possible_values(&["fast", "slow"])
            .help("A coffeehouse, coffee shop, or café.")
            .takes_value(true));
    assert!(test::compare_output(app, "ctest --help", HIDE_POS_VALS, false));
}

#[test]
fn issue_626_panic() {
    let app = App::new("ctest")
        .version("0.1")
        .set_term_width(52)
        .arg(Arg::with_name("cafe")
           .short("c")
           .long("cafe")
           .value_name("FILE")
           .help("La culture du café est très développée dans de nombreux pays à climat chaud d'Amérique, \
           d'Afrique et d'Asie, dans des plantations qui sont cultivées pour les marchés d'exportation. \
           Le café est souvent une contribution majeure aux exportations des régions productrices.")
           .takes_value(true));
    assert!(test::compare_output(app, "ctest --help", ISSUE_626_PANIC, false));
}

#[test]
fn issue_626_variable_panic() {
    for i in 10..320 {
        let _ = App::new("ctest")
            .version("0.1")
            .set_term_width(i)
            .arg(Arg::with_name("cafe")
               .short("c")
               .long("cafe")
               .value_name("FILE")
               .help("La culture du café est très développée dans de nombreux pays à climat chaud d'Amérique, \
               d'Afrique et d'Asie, dans des plantations qui sont cultivées pour les marchés d'exportation. \
               Le café est souvent une contribution majeure aux exportations des régions productrices.")
               .takes_value(true))
            .get_matches_from_safe(vec!["ctest", "--help"]);
    }
}

#[test]
fn final_word_wrapping() {
    let app = App::new("ctest").version("0.1").set_term_width(24);
    assert!(test::compare_output(app, "ctest --help", FINAL_WORD_WRAPPING, false));
}

#[test]
fn old_newline_chars() {
    let app = App::new("ctest")
        .version("0.1")
        .arg(Arg::with_name("mode")
            .short("m")
            .help("Some help with some wrapping{n}(Defaults to something)"));
    assert!(test::compare_output(app, "ctest --help", OLD_NEWLINE_CHARS, false));
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
                images. The default is Linear (Bilinear). [values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
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
                images. The default is Linear (Bilinear). [values: Nearest, Linear, Cubic, Gaussian, Lanczos3]")
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
        .arg(Arg::with_name("arg2")
            .multiple(true)
            .help("some option"))
        .arg(Arg::with_name("some")
            .help("some option")
            .short("s")
            .long("some")
            .takes_value(true))
        .arg(Arg::with_name("other")
            .help("some other option")
            .short("o")
            .long("other")
            .takes_value(true))
        .arg(Arg::with_name("label")
            .help("a label")
            .short("l")
            .long("label")
            .multiple(true)
            .takes_value(true));
    assert!(test::compare_output(app, "myapp --help", ISSUE_702, false));
}

#[test]
fn issue_760() {
    let app = App::new("ctest")
        .version("0.1")
        .arg(Arg::with_name("option")
            .help("tests options")
            .short("o")
            .long("option")
            .takes_value(true)
            .multiple(true)
            .number_of_values(1))
        .arg(Arg::with_name("opt")
            .help("tests options")
            .short("O")
            .long("opt")
            .takes_value(true));
    assert!(test::compare_output(app, "ctest --help", ISSUE_760, false));
}

#[test]
fn ripgrep_usage() {
    let app = App::new("ripgrep")
        .version("0.5")
        .usage("rg [OPTIONS] <pattern> [<path> ...]
    rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
    rg [OPTIONS] --files [<path> ...]
    rg [OPTIONS] --type-list");

    assert!(test::compare_output(app, "ripgrep --help", RIPGREP_USAGE, false));
}

#[test]
fn hidden_args() {
    let app = App::new("prog")
        .version("1.0")
        .args_from_usage("-f, --flag 'testing flags'
                          -o, --opt [FILE] 'tests options'")
        .arg(Arg::with_name("pos").hidden(true));
    assert!(test::compare_output(app, "prog --help", HIDDEN_ARGS, false));
}

#[test]
fn sc_negates_reqs() {
    let app = App::new("prog")
        .version("1.0")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg_from_usage("-o, --opt <FILE> 'tests options'")
        .arg(Arg::with_name("PATH"))
        .subcommand(SubCommand::with_name("test"));
    assert!(test::compare_output(app, "prog --help", SC_NEGATES_REQS, false));
}

#[test]
fn args_negate_sc() {
    let app = App::new("prog")
        .version("1.0")
        .setting(AppSettings::ArgsNegateSubcommands)
        .args_from_usage("-f, --flag 'testing flags'
                          -o, --opt [FILE] 'tests options'")
        .arg(Arg::with_name("PATH"))
        .subcommand(SubCommand::with_name("test"));
    assert!(test::compare_output(app, "prog --help", ARGS_NEGATE_SC, false));
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
        .help_short("H")
        .help_message("Print help information")
        .version_short("v")
        .version_message("Print version information");
    assert!(test::compare_output(app, "customize --help", CUSTOM_VERSION_AND_HELP, false));
}

#[test]
fn last_arg_mult_usage() {
    let app = App::new("last")
            .version("0.1")
            .arg(Arg::with_name("TARGET").required(true).help("some"))
            .arg(Arg::with_name("CORPUS").help("some"))
            .arg(Arg::with_name("ARGS").multiple(true).last(true).help("some"));
    assert!(test::compare_output(app, "last --help", LAST_ARG, false));
}

#[test]
fn last_arg_mult_usage_req() {
    let app = App::new("last")
            .version("0.1")
            .arg(Arg::with_name("TARGET").required(true).help("some"))
            .arg(Arg::with_name("CORPUS").help("some"))
            .arg(Arg::with_name("ARGS").multiple(true).last(true).required(true).help("some"));
    assert!(test::compare_output(app, "last --help", LAST_ARG_REQ, false));
}

#[test]
fn last_arg_mult_usage_req_with_sc() {
    let app = App::new("last")
            .version("0.1")
            .setting(AppSettings::SubcommandsNegateReqs)
            .arg(Arg::with_name("TARGET").required(true).help("some"))
            .arg(Arg::with_name("CORPUS").help("some"))
            .arg(Arg::with_name("ARGS").multiple(true).last(true).required(true).help("some"))
            .subcommand(SubCommand::with_name("test").about("some"));
    assert!(test::compare_output(app, "last --help", LAST_ARG_REQ_SC, false));
}

#[test]
fn last_arg_mult_usage_with_sc() {
    let app = App::new("last")
            .version("0.1")
            .setting(AppSettings::ArgsNegateSubcommands)
            .arg(Arg::with_name("TARGET").required(true).help("some"))
            .arg(Arg::with_name("CORPUS").help("some"))
            .arg(Arg::with_name("ARGS").multiple(true).last(true).help("some"))
            .subcommand(SubCommand::with_name("test").about("some"));
    assert!(test::compare_output(app, "last --help", LAST_ARG_SC, false));
}


#[test]
fn hidden_default_val() {
    let app1 = App::new("default")
        .version("0.1")
        .set_term_width(120)
        .arg(Arg::with_name("argument")
             .help("Pass an argument to the program. [default: default-argument]")
             .long("arg")
             .default_value("default-argument")
             .hide_default_value(true));
    assert!(test::compare_output(app1, "default --help", HIDE_DEFAULT_VAL, false));

    let app2 = App::new("default")
        .version("0.1")
        .set_term_width(120)
        .arg(Arg::with_name("argument")
             .help("Pass an argument to the program.")
             .long("arg")
             .default_value("default-argument"));
    assert!(test::compare_output(app2, "default --help", HIDE_DEFAULT_VAL, false));
}
