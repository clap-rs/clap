mod utils;

use std::str;

use clap::{App, AppSettings, Arg};

static NO_DERIVE_ORDER: &str = "test 1.2

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

static DERIVE_ORDER: &str = "test 1.2

USAGE:
    test [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --flag_b     first flag
        --flag_a     second flag

OPTIONS:
        --option_b <option_b>    first option
        --option_a <option_a>    second option";

static UNIFIED_HELP: &str = "test 1.2

USAGE:
    test [OPTIONS]

OPTIONS:
        --flag_a                 second flag
        --flag_b                 first flag
    -h, --help                   Prints help information
        --option_a <option_a>    second option
        --option_b <option_b>    first option
    -V, --version                Prints version information";

static UNIFIED_HELP_AND_DERIVE: &str = "test 1.2

USAGE:
    test [OPTIONS]

OPTIONS:
    -h, --help                   Prints help information
    -V, --version                Prints version information
        --flag_b                 first flag
        --option_b <option_b>    first option
        --flag_a                 second flag
        --option_a <option_a>    second option";

static DERIVE_ORDER_SC_PROP: &str = "test-sub 1.2

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

static UNIFIED_SC_PROP: &str = "test-sub 1.2

USAGE:
    test sub [OPTIONS]

OPTIONS:
        --flag_a                 second flag
        --flag_b                 first flag
    -h, --help                   Prints help information
        --option_a <option_a>    second option
        --option_b <option_b>    first option
    -V, --version                Prints version information";

static UNIFIED_DERIVE_SC_PROP: &str = "test-sub 1.2

USAGE:
    test sub [OPTIONS]

OPTIONS:
        --flag_b                 first flag
        --option_b <option_b>    first option
        --flag_a                 second flag
        --option_a <option_a>    second option
    -h, --help                   Prints help information
    -V, --version                Prints version information";

static UNIFIED_DERIVE_SC_PROP_EXPLICIT_ORDER: &str = "test-sub 1.2

USAGE:
    test sub [OPTIONS]

OPTIONS:
        --flag_a                 second flag
        --flag_b                 first flag
        --option_b <option_b>    first option
        --option_a <option_a>    second option
    -h, --help                   Prints help information
    -V, --version                Prints version information";

static PREFER_USER_HELP_DERIVE_ORDER: &str = "test 1.2

USAGE:
    test [FLAGS]

FLAGS:
    -V, --version    Prints version information
    -h, --help       Prints help message
        --flag_b     first flag
        --flag_a     second flag";

static PREFER_USER_HELP_SUBCMD_DERIVE_ORDER: &str = "test-sub 1.2

USAGE:
    test sub [FLAGS]

FLAGS:
    -h, --help       Prints help message
        --flag_b     first flag
        --flag_a     second flag
    -V, --version    Prints version information";

#[test]
fn no_derive_order() {
    let app = App::new("test").version("1.2").args(&[
        Arg::new("flag_b").long("flag_b").about("first flag"),
        Arg::new("option_b")
            .long("option_b")
            .takes_value(true)
            .about("first option"),
        Arg::new("flag_a").long("flag_a").about("second flag"),
        Arg::new("option_a")
            .long("option_a")
            .takes_value(true)
            .about("second option"),
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
            Arg::new("flag_b").long("flag_b").about("first flag"),
            Arg::new("option_b")
                .long("option_b")
                .takes_value(true)
                .about("first option"),
            Arg::new("flag_a").long("flag_a").about("second flag"),
            Arg::new("option_a")
                .long("option_a")
                .takes_value(true)
                .about("second option"),
        ]);

    assert!(utils::compare_output(
        app,
        "test --help",
        DERIVE_ORDER,
        false
    ));
}

#[test]
fn unified_help() {
    let app = App::new("test")
        .setting(AppSettings::UnifiedHelpMessage)
        .version("1.2")
        .args(&[
            Arg::new("flag_b").long("flag_b").about("first flag"),
            Arg::new("option_b")
                .long("option_b")
                .takes_value(true)
                .about("first option"),
            Arg::new("flag_a").long("flag_a").about("second flag"),
            Arg::new("option_a")
                .long("option_a")
                .takes_value(true)
                .about("second option"),
        ]);

    assert!(utils::compare_output(
        app,
        "test --help",
        UNIFIED_HELP,
        false
    ));
}

#[test]
fn unified_help_and_derive_order() {
    let app = App::new("test")
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version("1.2")
        .args(&[
            Arg::new("flag_b").long("flag_b").about("first flag"),
            Arg::new("option_b")
                .long("option_b")
                .takes_value(true)
                .about("first option"),
            Arg::new("flag_a").long("flag_a").about("second flag"),
            Arg::new("option_a")
                .long("option_a")
                .takes_value(true)
                .about("second option"),
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
        .version("1.2")
        .subcommand(
            App::new("sub").version("1.2").args(&[
                Arg::new("flag_b").long("flag_b").about("first flag"),
                Arg::new("option_b")
                    .long("option_b")
                    .takes_value(true)
                    .about("first option"),
                Arg::new("flag_a").long("flag_a").about("second flag"),
                Arg::new("option_a")
                    .long("option_a")
                    .takes_value(true)
                    .about("second option"),
            ]),
        );

    assert!(utils::compare_output(
        app,
        "test sub --help",
        DERIVE_ORDER_SC_PROP,
        false
    ));
}

#[test]
fn unified_help_subcommand_propagate() {
    let app = App::new("test")
        .global_setting(AppSettings::UnifiedHelpMessage)
        .subcommand(
            App::new("sub").version("1.2").args(&[
                Arg::new("flag_b").long("flag_b").about("first flag"),
                Arg::new("option_b")
                    .long("option_b")
                    .takes_value(true)
                    .about("first option"),
                Arg::new("flag_a").long("flag_a").about("second flag"),
                Arg::new("option_a")
                    .long("option_a")
                    .takes_value(true)
                    .about("second option"),
            ]),
        );

    assert!(utils::compare_output(
        app,
        "test sub --help",
        UNIFIED_SC_PROP,
        false
    ));
}

#[test]
fn unified_help_and_derive_order_subcommand_propagate() {
    let app = App::new("test")
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .subcommand(
            App::new("sub").version("1.2").args(&[
                Arg::new("flag_b").long("flag_b").about("first flag"),
                Arg::new("option_b")
                    .long("option_b")
                    .takes_value(true)
                    .about("first option"),
                Arg::new("flag_a").long("flag_a").about("second flag"),
                Arg::new("option_a")
                    .long("option_a")
                    .takes_value(true)
                    .about("second option"),
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
fn unified_help_and_derive_order_subcommand_propagate_with_explicit_display_order() {
    let app = App::new("test")
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .subcommand(
            App::new("sub").version("1.2").args(&[
                Arg::new("flag_b").long("flag_b").about("first flag"),
                Arg::new("option_b")
                    .long("option_b")
                    .takes_value(true)
                    .about("first option"),
                Arg::new("flag_a")
                    .long("flag_a")
                    .about("second flag")
                    .display_order(0),
                Arg::new("option_a")
                    .long("option_a")
                    .takes_value(true)
                    .about("second option"),
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
                .about("Prints help message"),
            Arg::new("flag_b").long("flag_b").about("first flag"),
            Arg::new("flag_a").long("flag_a").about("second flag"),
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
                    .about("Prints help message"),
                Arg::new("flag_b").long("flag_b").about("first flag"),
                Arg::new("flag_a").long("flag_a").about("second flag"),
            ]),
        );

    assert!(utils::compare_output(
        app,
        "test sub --help",
        PREFER_USER_HELP_SUBCMD_DERIVE_ORDER,
        false
    ));
}
