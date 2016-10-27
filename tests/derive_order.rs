extern crate clap;

use std::str;

use clap::{App, Arg, SubCommand, AppSettings};

fn condense_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize(s: &str) -> Vec<String> {
    s.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).map(condense_whitespace).collect()
}

fn get_help(app: App, args: &[&'static str]) -> Vec<String> {
    let res = app.get_matches_from_safe(args.iter().chain(["--help"].iter()));
    normalize(&*res.unwrap_err().message)
}

#[test]
fn no_derive_order() {
    let app = App::new("test")
        .args(&[
            Arg::with_name("flag_b").long("flag_b").help("first flag"),
            Arg::with_name("option_b").long("option_b").takes_value(true).help("first option"),
            Arg::with_name("flag_a").long("flag_a").help("second flag"),
            Arg::with_name("option_a").long("option_a").takes_value(true).help("second option"),
        ]);

    assert_eq!(get_help(app, &["test"]), normalize("
        test

        USAGE:
            test [FLAGS] [OPTIONS]

        FLAGS:
                --flag_a     second flag
                --flag_b     first flag
            -h, --help       Prints help information
            -V, --version    Prints version information

        OPTIONS:
                --option_a <option_a>    second option
                --option_b <option_b>    first option
    "));
}

#[test]
fn derive_order() {
    let app = App::new("test")
        .setting(AppSettings::DeriveDisplayOrder)
        .args(&[
            Arg::with_name("flag_b").long("flag_b").help("first flag"),
            Arg::with_name("option_b").long("option_b").takes_value(true).help("first option"),
            Arg::with_name("flag_a").long("flag_a").help("second flag"),
            Arg::with_name("option_a").long("option_a").takes_value(true).help("second option"),
        ]);

    assert_eq!(get_help(app, &["test"]), normalize("
        test

        USAGE:
            test [FLAGS] [OPTIONS]

        FLAGS:
                --flag_b     first flag
                --flag_a     second flag
            -h, --help       Prints help information
            -V, --version    Prints version information

        OPTIONS:
                --option_b <option_b>    first option
                --option_a <option_a>    second option
    "));
}

#[test]
fn unified_help() {
    let app = App::new("test")
        .setting(AppSettings::UnifiedHelpMessage)
        .args(&[
            Arg::with_name("flag_b").long("flag_b").help("first flag"),
            Arg::with_name("option_b").long("option_b").takes_value(true).help("first option"),
            Arg::with_name("flag_a").long("flag_a").help("second flag"),
            Arg::with_name("option_a").long("option_a").takes_value(true).help("second option"),
        ]);

    assert_eq!(get_help(app, &["test"]), normalize("
        test

        USAGE:
            test [OPTIONS]

        OPTIONS:
                --flag_a     second flag
                --flag_b     first flag
            -h, --help       Prints help information
                --option_a <option_a>    second option
                --option_b <option_b>    first option
            -V, --version    Prints version information
    "));
}

#[test]
fn unified_help_and_derive_order() {
    let app = App::new("test")
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .args(&[
            Arg::with_name("flag_b").long("flag_b").help("first flag"),
            Arg::with_name("option_b").long("option_b").takes_value(true).help("first option"),
            Arg::with_name("flag_a").long("flag_a").help("second flag"),
            Arg::with_name("option_a").long("option_a").takes_value(true).help("second option"),
        ]);

    assert_eq!(get_help(app, &["test"]), normalize("
        test

        USAGE:
            test [OPTIONS]

        OPTIONS:
                --flag_b     first flag
                --option_b <option_b>    first option
                --flag_a     second flag
                --option_a <option_a>    second option
            -h, --help       Prints help information
            -V, --version    Prints version information
    "));
}

#[test]
fn derive_order_subcommand_propagate() {
    let app = App::new("test")
        .global_setting(AppSettings::DeriveDisplayOrder)
        .subcommand(SubCommand::with_name("sub")
            .args(&[
                Arg::with_name("flag_b").long("flag_b").help("first flag"),
                Arg::with_name("option_b").long("option_b").takes_value(true).help("first option"),
                Arg::with_name("flag_a").long("flag_a").help("second flag"),
                Arg::with_name("option_a").long("option_a").takes_value(true).help("second option"),
            ]));

    assert_eq!(get_help(app, &["test", "sub"]), normalize("
        test-sub

        USAGE:
            test sub [FLAGS] [OPTIONS]

        FLAGS:
                --flag_b     first flag
                --flag_a     second flag
            -h, --help       Prints help information
            -V, --version    Prints version information

        OPTIONS:
                --option_b <option_b>    first option
                --option_a <option_a>    second option
    "));
}

#[test]
fn unified_help_subcommand_propagate() {
    let app = App::new("test")
        .global_setting(AppSettings::UnifiedHelpMessage)
        .subcommand(SubCommand::with_name("sub")
            .args(&[
                Arg::with_name("flag_b").long("flag_b").help("first flag"),
                Arg::with_name("option_b").long("option_b").takes_value(true).help("first option"),
                Arg::with_name("flag_a").long("flag_a").help("second flag"),
                Arg::with_name("option_a").long("option_a").takes_value(true).help("second option"),
            ]));

    assert_eq!(get_help(app, &["test", "sub"]), normalize("
        test-sub

        USAGE:
            test sub [OPTIONS]

        OPTIONS:
                --flag_a     second flag
                --flag_b     first flag
            -h, --help       Prints help information
                --option_a <option_a>    second option
                --option_b <option_b>    first option
            -V, --version    Prints version information

    "));
}

#[test]
fn unified_help_and_derive_order_subcommand_propagate() {
    let app = App::new("test")
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .subcommand(SubCommand::with_name("sub")
            .args(&[
                Arg::with_name("flag_b").long("flag_b").help("first flag"),
                Arg::with_name("option_b").long("option_b").takes_value(true).help("first option"),
                Arg::with_name("flag_a").long("flag_a").help("second flag"),
                Arg::with_name("option_a").long("option_a").takes_value(true).help("second option"),
            ]));

    assert_eq!(get_help(app, &["test", "sub"]), normalize("
        test-sub

        USAGE:
            test sub [OPTIONS]

        OPTIONS:
                --flag_b     first flag
                --option_b <option_b>    first option
                --flag_a     second flag
                --option_a <option_a>    second option
            -h, --help       Prints help information
            -V, --version    Prints version information

    "));
}

#[test]
fn unified_help_and_derive_order_subcommand_propagate_with_explicit_display_order() {
    let app = App::new("test")
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .subcommand(SubCommand::with_name("sub")
            .args(&[
                Arg::with_name("flag_b").long("flag_b").help("first flag"),
                Arg::with_name("option_b").long("option_b").takes_value(true).help("first option"),
                Arg::with_name("flag_a").long("flag_a").help("second flag").display_order(0),
                Arg::with_name("option_a").long("option_a").takes_value(true).help("second option"),
            ]));

    assert_eq!(get_help(app, &["test", "sub"]), normalize("
        test-sub

        USAGE:
            test sub [OPTIONS]

        OPTIONS:
                --flag_a     second flag
                --flag_b     first flag
                --option_b <option_b>    first option
                --option_a <option_a>    second option
            -h, --help       Prints help information
            -V, --version    Prints version information

    "));
}
