use super::utils;

use clap::{arg, Arg, Command};

static HIDDEN_ARGS: &str = "test 1.4
Kevin K.
tests stuff

USAGE:
    test [OPTIONS]

OPTIONS:
    -F, --flag2           some other flag
    -h, --help            Print help information
        --option <opt>    some option
    -V, --version         Print version information
";

#[test]
fn hide_args() {
    let cmd = Command::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.4")
        .args(&[
            arg!(-f --flag "some flag").hide(true),
            arg!(-F --flag2 "some other flag"),
            arg!(--option <opt> "some option").required(false),
            Arg::new("DUMMY").hide(true),
        ]);
    utils::assert_output(cmd, "test --help", HIDDEN_ARGS, false);
}

static HIDDEN_SHORT_ARGS: &str = "test 2.31.2
Steve P.
hides short args

USAGE:
    test [OPTIONS]

OPTIONS:
    -h, --help       Print help information
    -v, --visible    This text should be visible
    -V, --version    Print version information
";

static HIDDEN_SHORT_ARGS_LONG_HELP: &str = "test 2.31.2
Steve P.
hides short args

USAGE:
    test [OPTIONS]

OPTIONS:
    -c, --config
            Some help text describing the --config arg

    -h, --help
            Print help information

    -v, --visible
            This text should be visible

    -V, --version
            Print version information
";

/// Ensure hide with short option
#[test]
fn hide_short_args() {
    let cmd = Command::new("test")
        .about("hides short args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hide_short_help(true)
                .help("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .help("This text should be visible"),
        ]);

    utils::assert_output(cmd, "test -h", HIDDEN_SHORT_ARGS, false);
}

/// Ensure visible with opposite option
#[test]
fn hide_short_args_long_help() {
    let cmd = Command::new("test")
        .about("hides short args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hide_short_help(true)
                .help("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .help("This text should be visible"),
        ]);

    utils::assert_output(cmd, "test --help", HIDDEN_SHORT_ARGS_LONG_HELP, false);
}

static HIDDEN_LONG_ARGS: &str = "test 2.31.2
Steve P.
hides long args

USAGE:
    test [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

    -v, --visible
            This text should be visible

    -V, --version
            Print version information
";

#[test]
fn hide_long_args() {
    let cmd = Command::new("test")
        .about("hides long args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hide_long_help(true)
                .help("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .help("This text should be visible"),
        ]);

    utils::assert_output(cmd, "test --help", HIDDEN_LONG_ARGS, false);
}

static HIDDEN_LONG_ARGS_SHORT_HELP: &str = "test 2.31.2
Steve P.
hides long args

USAGE:
    test [OPTIONS]

OPTIONS:
    -c, --config     Some help text describing the --config arg
    -h, --help       Print help information
    -v, --visible    This text should be visible
    -V, --version    Print version information
";

#[test]
fn hide_long_args_short_help() {
    let cmd = Command::new("test")
        .about("hides long args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hide_long_help(true)
                .help("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .help("This text should be visible"),
        ]);

    utils::assert_output(cmd, "test -h", HIDDEN_LONG_ARGS_SHORT_HELP, false);
}

static HIDDEN_POS_ARGS: &str = "test 1.4

USAGE:
    test [another]

ARGS:
    <another>    another pos

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

#[test]
fn hide_pos_args() {
    let cmd = Command::new("test").version("1.4").args(&[
        Arg::new("pos").help("some pos").hide(true),
        Arg::new("another").help("another pos"),
    ]);

    utils::assert_output(cmd, "test --help", HIDDEN_POS_ARGS, false);
}

static HIDDEN_SUBCMDS: &str = "test 1.4

USAGE:
    test

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
";

#[test]
fn hide_subcmds() {
    let cmd = Command::new("test")
        .version("1.4")
        .subcommand(Command::new("sub").hide(true));

    utils::assert_output(cmd, "test --help", HIDDEN_SUBCMDS, false);
}

static HIDDEN_OPT_ARGS_ONLY: &str = "test 1.4

USAGE:
    test

After help
";

#[test]
fn hide_opt_args_only() {
    let cmd = Command::new("test")
        .version("1.4")
        .after_help("After help")
        .mut_arg("help", |a| a.hide(true))
        .mut_arg("version", |a| a.hide(true))
        .arg(
            arg!(--option <opt> "some option")
                .required(false)
                .hide(true),
        );

    utils::assert_output(cmd, "test --help", HIDDEN_OPT_ARGS_ONLY, false);
}

static HIDDEN_POS_ARGS_ONLY: &str = "test 1.4

USAGE:
    test

After help
";

#[test]
fn hide_pos_args_only() {
    let cmd = Command::new("test")
        .version("1.4")
        .after_help("After help")
        .mut_arg("help", |a| a.hide(true))
        .mut_arg("version", |a| a.hide(true))
        .args(&[Arg::new("pos").help("some pos").hide(true)]);

    utils::assert_output(cmd, "test --help", HIDDEN_POS_ARGS_ONLY, false);
}

static HIDDEN_SUBCMDS_ONLY: &str = "test 1.4

USAGE:
    test

After help
";

#[test]
fn hide_subcmds_only() {
    let cmd = Command::new("test")
        .version("1.4")
        .after_help("After help")
        .mut_arg("help", |a| a.hide(true))
        .mut_arg("version", |a| a.hide(true))
        .subcommand(Command::new("sub").hide(true));

    utils::assert_output(cmd, "test --help", HIDDEN_SUBCMDS_ONLY, false);
}
