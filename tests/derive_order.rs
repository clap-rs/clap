extern crate clap;
extern crate regex;

use std::str;

use clap::{App, Arg, SubCommand, AppSettings};

include!("../clap-test.rs");

static NO_DERIVE_ORDER: &'static str = "test 1.2

USAGE:
    test [FLAGS] [OPTIONS]

FLAGS:
        --flag_a     second flag
        --flag_b     first flag
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --option_a <option_a>    second option
        --option_b <option_b>    first option";

static DERIVE_ORDER: &'static str = "test 1.2

USAGE:
    test [FLAGS] [OPTIONS]

FLAGS:
        --flag_b     first flag
        --flag_a     second flag
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --option_b <option_b>    first option
        --option_a <option_a>    second option";

static UNIFIED_HELP: &'static str = "test 1.2

USAGE:
    test [OPTIONS]

OPTIONS:
        --flag_a                 second flag
        --flag_b                 first flag
    -h, --help                   Prints help information
        --option_a <option_a>    second option
        --option_b <option_b>    first option
    -V, --version                Prints version information";

static UNIFIED_HELP_AND_DERIVE: &'static str = "test 1.2

USAGE:
    test [OPTIONS]

OPTIONS:
        --flag_b                 first flag
        --option_b <option_b>    first option
        --flag_a                 second flag
        --option_a <option_a>    second option
    -h, --help                   Prints help information
    -V, --version                Prints version information";

static DERIVE_ORDER_SC_PROP: &'static str = "test-sub 1.2

USAGE:
    test sub [FLAGS] [OPTIONS]

FLAGS:
        --flag_b     first flag
        --flag_a     second flag
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --option_b <option_b>    first option
        --option_a <option_a>    second option";

static UNIFIED_SC_PROP: &'static str = "test-sub 1.2

USAGE:
    test sub [OPTIONS]

OPTIONS:
        --flag_a                 second flag
        --flag_b                 first flag
    -h, --help                   Prints help information
        --option_a <option_a>    second option
        --option_b <option_b>    first option
    -V, --version                Prints version information";

static UNIFIED_DERIVE_SC_PROP: &'static str = "test-sub 1.2

USAGE:
    test sub [OPTIONS]

OPTIONS:
        --flag_b                 first flag
        --option_b <option_b>    first option
        --flag_a                 second flag
        --option_a <option_a>    second option
    -h, --help                   Prints help information
    -V, --version                Prints version information";

static UNIFIED_DERIVE_SC_PROP_EXPLICIT_ORDER: &'static str = "test-sub 1.2

USAGE:
    test sub [OPTIONS]

OPTIONS:
        --flag_a                 second flag
        --flag_b                 first flag
        --option_b <option_b>    first option
        --option_a <option_a>    second option
    -h, --help                   Prints help information
    -V, --version                Prints version information";

#[test]
fn no_derive_order() {
    let app = App::new("test")
        .version("1.2")
        .args(&[Arg::new("flag_b")
                    .long("flag_b")
                    .help("first flag"),
                Arg::new("option_b")
                    .long("option_b")
                    .set(ArgSettings::TakesValue)
                    .help("first option"),
                Arg::new("flag_a")
                    .long("flag_a")
                    .help("second flag"),
                Arg::new("option_a")
                    .long("option_a")
                    .set(ArgSettings::TakesValue)
                    .help("second option")]);

    assert!(test::compare_output(app, "test --help", NO_DERIVE_ORDER, false));
}

#[test]
fn derive_order() {
    let app = App::new("test")
        .set(AppSettings::DeriveDisplayOrder)
        .version("1.2")
        .args(&[Arg::new("flag_b")
                    .long("flag_b")
                    .help("first flag"),
                Arg::new("option_b")
                    .long("option_b")
                    .set(ArgSettings::TakesValue)
                    .help("first option"),
                Arg::new("flag_a")
                    .long("flag_a")
                    .help("second flag"),
                Arg::new("option_a")
                    .long("option_a")
                    .set(ArgSettings::TakesValue)
                    .help("second option")]);

    assert!(test::compare_output(app, "test --help", DERIVE_ORDER, false));
}

#[test]
fn unified_help() {
    let app = App::new("test")
        .set(AppSettings::UnifiedHelpMessage)
        .version("1.2")
        .args(&[Arg::new("flag_b")
                    .long("flag_b")
                    .help("first flag"),
                Arg::new("option_b")
                    .long("option_b")
                    .set(ArgSettings::TakesValue)
                    .help("first option"),
                Arg::new("flag_a")
                    .long("flag_a")
                    .help("second flag"),
                Arg::new("option_a")
                    .long("option_a")
                    .set(ArgSettings::TakesValue)
                    .help("second option")]);

    assert!(test::compare_output(app, "test --help", UNIFIED_HELP, false));
}

#[test]
fn unified_help_and_derive_order() {
    let app = App::new("test")
        .set(AppSettings::DeriveDisplayOrder)
        .set(AppSettings::UnifiedHelpMessage)
        .version("1.2")
        .args(&[Arg::new("flag_b")
                    .long("flag_b")
                    .help("first flag"),
                Arg::new("option_b")
                    .long("option_b")
                    .set(ArgSettings::TakesValue)
                    .help("first option"),
                Arg::new("flag_a")
                    .long("flag_a")
                    .help("second flag"),
                Arg::new("option_a")
                    .long("option_a")
                    .set(ArgSettings::TakesValue)
                    .help("second option")]);

    assert!(test::compare_output(app, "test --help", UNIFIED_HELP_AND_DERIVE, false));
}

#[test]
fn derive_order_subcommand_propagate() {
    let app = App::new("test")
        .set_global(AppSettings::DeriveDisplayOrder)
        .version("1.2")
        .subcommand(App::new("sub")
                        .version("1.2")
                        .args(&[Arg::new("flag_b")
                                    .long("flag_b")
                                    .help("first flag"),
                                Arg::new("option_b")
                                    .long("option_b")
                                    .set(ArgSettings::TakesValue)
                                    .help("first option"),
                                Arg::new("flag_a")
                                    .long("flag_a")
                                    .help("second flag"),
                                Arg::new("option_a")
                                    .long("option_a")
                                    .set(ArgSettings::TakesValue)
                                    .help("second option")]));

    assert!(test::compare_output(app, "test sub --help", DERIVE_ORDER_SC_PROP, false));
}

#[test]
fn unified_help_subcommand_propagate() {
    let app = App::new("test")
        .set_global(AppSettings::UnifiedHelpMessage)
        .subcommand(App::new("sub")
                        .version("1.2")
                        .args(&[Arg::new("flag_b")
                                    .long("flag_b")
                                    .help("first flag"),
                                Arg::new("option_b")
                                    .long("option_b")
                                    .set(ArgSettings::TakesValue)
                                    .help("first option"),
                                Arg::new("flag_a")
                                    .long("flag_a")
                                    .help("second flag"),
                                Arg::new("option_a")
                                    .long("option_a")
                                    .set(ArgSettings::TakesValue)
                                    .help("second option")]));

    assert!(test::compare_output(app, "test sub --help", UNIFIED_SC_PROP, false));
}

#[test]
fn unified_help_and_derive_order_subcommand_propagate() {
    let app = App::new("test")
        .set_global(AppSettings::DeriveDisplayOrder)
        .set_global(AppSettings::UnifiedHelpMessage)
        .subcommand(App::new("sub")
                        .version("1.2")
                        .args(&[Arg::new("flag_b")
                                    .long("flag_b")
                                    .help("first flag"),
                                Arg::new("option_b")
                                    .long("option_b")
                                    .set(ArgSettings::TakesValue)
                                    .help("first option"),
                                Arg::new("flag_a")
                                    .long("flag_a")
                                    .help("second flag"),
                                Arg::new("option_a")
                                    .long("option_a")
                                    .set(ArgSettings::TakesValue)
                                    .help("second option")]));

    assert!(test::compare_output(app, "test sub --help", UNIFIED_DERIVE_SC_PROP, false));
}

#[test]
fn unified_help_and_derive_order_subcommand_propagate_with_explicit_display_order() {
    let app = App::new("test")
        .set_global(AppSettings::DeriveDisplayOrder)
        .set_global(AppSettings::UnifiedHelpMessage)
        .subcommand(App::new("sub")
                        .version("1.2")
                        .args(&[Arg::new("flag_b")
                                    .long("flag_b")
                                    .help("first flag"),
                                Arg::new("option_b")
                                    .long("option_b")
                                    .set(ArgSettings::TakesValue)
                                    .help("first option"),
                                Arg::new("flag_a")
                                    .long("flag_a")
                                    .help("second flag")
                                    .display_order(0),
                                Arg::new("option_a")
                                    .long("option_a")
                                    .set(ArgSettings::TakesValue)
                                    .help("second option")]));

    assert!(test::compare_output(app,
                                 "test sub --help",
                                 UNIFIED_DERIVE_SC_PROP_EXPLICIT_ORDER,
                                 false));
}
