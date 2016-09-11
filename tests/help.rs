extern crate clap;
extern crate regex;

include!("../clap-test.rs");

use clap::{App, SubCommand, ErrorKind, Arg};

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

static AFTER_HELP: &'static str = "some text that comes before the help

clap-test v1.4.8
tests clap library

USAGE:
    clap-test

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

some text that comes after the help";

static SC_HELP: &'static str = "subcmd 0.1
Kevin K. <kbknapp@gmail.com>
tests subcommands

USAGE:
    subcmd [FLAGS] [OPTIONS] [--] [scpositional]

FLAGS:
    -f, --flag    tests flags

OPTIONS:
    -o, --option <scoption>...    tests options

ARGS:
    <scpositional>    tests positionals";

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
                         tea. Many cafés also serve some type of
                         food, such as light snacks, muffins, or
                         pastries.";

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
    let m = test::complex_app()
        .get_matches_from_safe(vec!["clap-test", "subcmd", "-h"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn subcommand_long_help() {
    let m = test::complex_app()
        .get_matches_from_safe(vec!["clap-test", "subcmd", "--help"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn subcommand_help_rev() {
    let m = test::complex_app()
        .get_matches_from_safe(vec!["clap-test", "help", "subcmd"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::HelpDisplayed);
}

#[test]
fn complex_help_output() {
    test::check_help(test::complex_app(), HELP);
}

#[test]
fn after_and_before_help_output() {
    let app = App::new("clap-test")
        .version("v1.4.8")
        .about("tests clap library")
        .before_help("some text that comes before the help")
        .after_help("some text that comes after the help");
    test::check_help(app, AFTER_HELP);
}

#[test]
fn multi_level_sc_help() {
    let app = App::new("ctest")
        .subcommand(SubCommand::with_name("subcmd")
            .subcommand(SubCommand::with_name("multi")
                .about("tests subcommands")
                .author("Kevin K. <kbknapp@gmail.com>")
                .version("0.1")
                .args_from_usage("
                    -f, --flag                    'tests flags'
                    -o, --option [scoption]...    'tests options'
                ")));
    test::check_err_output(app, "ctest help subcmd multi", MULTI_SC_HELP, false);
}

#[test]
fn no_wrap_help() {
    let app = App::new("ctest")
        .set_term_width(0)
        .help(MULTI_SC_HELP);
    test::check_err_output(app, "ctest --help", MULTI_SC_HELP, false);
}

#[test]
fn complex_subcommand_help_output() {
    let mut a = test::complex_app();
    let _ = a.get_matches_from_safe_borrow(vec![""]);
    let sc = a.p.subcommands.iter().filter(|s| s.p.meta.name == "subcmd").next().unwrap();
    // Now we check the output of print_help()
    let mut help = vec![];
    sc.write_help(&mut help).ok().expect("failed to print help");
    assert_eq!(&*String::from_utf8(help).unwrap(), SC_HELP);
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
    test::check_err_output(app, "ctest --help", ISSUE_626_CUTOFF, false);
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
    test::check_err_output(app, "ctest --help", HIDE_POS_VALS, false);
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
    test::check_err_output(app, "ctest --help", ISSUE_626_PANIC, false);
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
