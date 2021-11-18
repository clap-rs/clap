mod utils;

use std::str;

use clap::{App, AppSettings, Arg};

static NO_DERIVE_ORDER: &str = "test 1.2

USAGE:
    test [OPTIONS]

OPTIONS:
        --flag_a                 second flag
        --flag_b                 first flag
    -h, --help                   Print help information
        --option_a <option_a>    second option
        --option_b <option_b>    first option
    -V, --version                Print version information
";

static UNIFIED_HELP_AND_DERIVE: &str = "test 1.2

USAGE:
    test [OPTIONS]

OPTIONS:
        --flag_b                 first flag
        --option_b <option_b>    first option
        --flag_a                 second flag
        --option_a <option_a>    second option
    -h, --help                   Print help information
    -V, --version                Print version information
";

static UNIFIED_DERIVE_SC_PROP: &str = "test-sub 1.2

USAGE:
    test sub [OPTIONS]

OPTIONS:
        --flag_b                 first flag
        --option_b <option_b>    first option
        --flag_a                 second flag
        --option_a <option_a>    second option
    -h, --help                   Print help information
    -V, --version                Print version information
";

static UNIFIED_DERIVE_SC_PROP_EXPLICIT_ORDER: &str = "test-sub 1.2

USAGE:
    test sub [OPTIONS]

OPTIONS:
        --flag_a                 second flag
        --flag_b                 first flag
        --option_b <option_b>    first option
        --option_a <option_a>    second option
    -h, --help                   Print help information
    -V, --version                Print version information
";

static PREFER_USER_HELP_DERIVE_ORDER: &str = "test 1.2

USAGE:
    test [OPTIONS]

OPTIONS:
    -h, --help       Print help message
        --flag_b     first flag
        --flag_a     second flag
    -V, --version    Print version information
";

static PREFER_USER_HELP_SUBCMD_DERIVE_ORDER: &str = "test-sub 1.2

USAGE:
    test sub [OPTIONS]

OPTIONS:
    -h, --help       Print help message
        --flag_b     first flag
        --flag_a     second flag
    -V, --version    Print version information
";

#[test]
fn no_derive_order() {
    let app = App::new("test").version("1.2").args(&[
        Arg::new("flag_b").long("flag_b").help("first flag"),
        Arg::new("option_b")
            .long("option_b")
            .takes_value(true)
            .help("first option"),
        Arg::new("flag_a").long("flag_a").help("second flag"),
        Arg::new("option_a")
            .long("option_a")
            .takes_value(true)
            .help("second option"),
    ]);

    assert!(utils::compare_output(
        app,
        "test --help",
        NO_DERIVE_ORDER,
        false
    ));
}

#[test]
fn derive_order() {
    let app = App::new("test")
        .setting(AppSettings::DeriveDisplayOrder)
        .version("1.2")
        .args(&[
            Arg::new("flag_b").long("flag_b").help("first flag"),
            Arg::new("option_b")
                .long("option_b")
                .takes_value(true)
                .help("first option"),
            Arg::new("flag_a").long("flag_a").help("second flag"),
            Arg::new("option_a")
                .long("option_a")
                .takes_value(true)
                .help("second option"),
        ]);

    assert!(utils::compare_output(
        app,
        "test --help",
        UNIFIED_HELP_AND_DERIVE,
        false
    ));
}

#[test]
fn derive_order_subcommand_propagate() {
    let app = App::new("test")
        .global_setting(AppSettings::DeriveDisplayOrder)
        .subcommand(
            App::new("sub").version("1.2").args(&[
                Arg::new("flag_b").long("flag_b").help("first flag"),
                Arg::new("option_b")
                    .long("option_b")
                    .takes_value(true)
                    .help("first option"),
                Arg::new("flag_a").long("flag_a").help("second flag"),
                Arg::new("option_a")
                    .long("option_a")
                    .takes_value(true)
                    .help("second option"),
            ]),
        );

    assert!(utils::compare_output(
        app,
        "test sub --help",
        UNIFIED_DERIVE_SC_PROP,
        false
    ));
}

#[test]
fn derive_order_subcommand_propagate_with_explicit_display_order() {
    let app = App::new("test")
        .global_setting(AppSettings::DeriveDisplayOrder)
        .subcommand(
            App::new("sub").version("1.2").args(&[
                Arg::new("flag_b").long("flag_b").help("first flag"),
                Arg::new("option_b")
                    .long("option_b")
                    .takes_value(true)
                    .help("first option"),
                Arg::new("flag_a")
                    .long("flag_a")
                    .help("second flag")
                    .display_order(0),
                Arg::new("option_a")
                    .long("option_a")
                    .takes_value(true)
                    .help("second option"),
            ]),
        );

    assert!(utils::compare_output(
        app,
        "test sub --help",
        UNIFIED_DERIVE_SC_PROP_EXPLICIT_ORDER,
        false
    ));
}

#[test]
fn prefer_user_help_with_derive_order() {
    let app = App::new("test")
        .setting(AppSettings::DeriveDisplayOrder)
        .version("1.2")
        .args(&[
            Arg::new("help")
                .long("help")
                .short('h')
                .help("Print help message"),
            Arg::new("flag_b").long("flag_b").help("first flag"),
            Arg::new("flag_a").long("flag_a").help("second flag"),
        ]);

    assert!(utils::compare_output(
        app,
        "test --help",
        PREFER_USER_HELP_DERIVE_ORDER,
        false
    ));
}

#[test]
fn prefer_user_help_in_subcommand_with_derive_order() {
    let app = App::new("test")
        .global_setting(AppSettings::DeriveDisplayOrder)
        .subcommand(
            App::new("sub").version("1.2").args(&[
                Arg::new("help")
                    .long("help")
                    .short('h')
                    .help("Print help message"),
                Arg::new("flag_b").long("flag_b").help("first flag"),
                Arg::new("flag_a").long("flag_a").help("second flag"),
            ]),
        );

    assert!(utils::compare_output(
        app,
        "test sub --help",
        PREFER_USER_HELP_SUBCMD_DERIVE_ORDER,
        false
    ));
}
