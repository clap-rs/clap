mod utils;

use clap::{App, AppSettings, Arg};

static HIDDEN_ARGS: &str = "test 1.4

Kevin K.

tests stuff

USAGE:
    test [FLAGS] [OPTIONS]

FLAGS:
    -F, --flag2      some other flag
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --option <opt>    some option";

#[test]
fn hidden_args() {
    let app = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.4")
        .args(&[
            Arg::from("-f, --flag 'some flag'").hidden(true),
            Arg::from("-F, --flag2 'some other flag'"),
            Arg::from("--option [opt] 'some option'"),
            Arg::new("DUMMY").hidden(true),
        ]);
    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_ARGS,
        false
    ));
}

static HIDDEN_SHORT_ARGS: &str = "test 2.31.2

Steve P.

hides short args

USAGE:
    test [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -v, --visible    This text should be visible
    -V, --version    Prints version information";

static HIDDEN_SHORT_ARGS_LONG_HELP: &str = "test 2.31.2

Steve P.

hides short args

USAGE:
    test [FLAGS]

FLAGS:
    -c, --config
            Some help text describing the --config arg

    -h, --help
            Prints help information

    -v, --visible
            This text should be visible

    -V, --version
            Prints version information";

/// Ensure hidden with short option
#[test]
fn hidden_short_args() {
    let app = App::new("test")
        .about("hides short args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hidden_short_help(true)
                .about("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .about("This text should be visible"),
        ]);

    assert!(utils::compare_output(
        app,
        "test -h",
        HIDDEN_SHORT_ARGS,
        false
    ));
}

/// Ensure visible with opposite option
#[test]
fn hidden_short_args_long_help() {
    let app = App::new("test")
        .about("hides short args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hidden_short_help(true)
                .about("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .about("This text should be visible"),
        ]);

    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_SHORT_ARGS_LONG_HELP,
        false
    ));
}

static HIDDEN_LONG_ARGS: &str = "test 2.31.2

Steve P.

hides long args

USAGE:
    test [FLAGS]

FLAGS:
    -h, --help
            Prints help information

    -v, --visible
            This text should be visible

    -V, --version
            Prints version information";

#[test]
fn hidden_long_args() {
    let app = App::new("test")
        .about("hides long args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hidden_long_help(true)
                .about("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .about("This text should be visible"),
        ]);

    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_LONG_ARGS,
        false
    ));
}

static HIDDEN_LONG_ARGS_SHORT_HELP: &str = "test 2.31.2

Steve P.

hides long args

USAGE:
    test [FLAGS]

FLAGS:
    -c, --config     Some help text describing the --config arg
    -h, --help       Prints help information
    -v, --visible    This text should be visible
    -V, --version    Prints version information";

#[test]
fn hidden_long_args_short_help() {
    let app = App::new("test")
        .about("hides long args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hidden_long_help(true)
                .about("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .about("This text should be visible"),
        ]);

    assert!(utils::compare_output(
        app,
        "test -h",
        HIDDEN_LONG_ARGS_SHORT_HELP,
        false
    ));
}

static HIDDEN_FLAG_ARGS: &str = "test 1.4

USAGE:
    test [OPTIONS]

OPTIONS:
        --option <opt>    some option";

#[test]
fn hidden_flag_args() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .mut_arg("help", |a| a.hidden(true))
        .args(&[Arg::from("--option [opt] 'some option'")]);

    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_FLAG_ARGS,
        false
    ));
}

static HIDDEN_OPT_ARGS: &str = "test 1.4

USAGE:
    test [FLAGS]

FLAGS:
        --flag    some flag
    -h, --help    Prints help information";

#[test]
fn hidden_opt_args() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .args(&[
            Arg::from("--flag 'some flag'"),
            Arg::from("--option [opt] 'some option'").hidden(true),
        ]);

    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_OPT_ARGS,
        false
    ));
}

static HIDDEN_POS_ARGS: &str = "test 1.4

USAGE:
    test [another]

ARGS:
    <another>    another pos

FLAGS:
    -h, --help    Prints help information";

#[test]
fn hidden_pos_args() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .args(&[
            Arg::new("pos").about("some pos").hidden(true),
            Arg::new("another").about("another pos"),
        ]);

    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_POS_ARGS,
        false
    ));
}

static HIDDEN_SUBCMDS: &str = "test 1.4

USAGE:
    test

FLAGS:
    -h, --help    Prints help information";

#[test]
fn hidden_subcmds() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .subcommand(App::new("sub").setting(AppSettings::Hidden));

    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_SUBCMDS,
        false
    ));
}

static HIDDEN_FLAG_ARGS_ONLY: &str = "test 1.4

USAGE:
    test

After help";

#[test]
fn hidden_flag_args_only() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .after_help("After help")
        .mut_arg("help", |a| a.hidden(true));

    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_FLAG_ARGS_ONLY,
        false
    ));
}

static HIDDEN_OPT_ARGS_ONLY: &str = "test 1.4

USAGE:
    test

After help";

#[test]
fn hidden_opt_args_only() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .after_help("After help")
        .mut_arg("help", |a| a.hidden(true))
        .args(&[Arg::from("--option [opt] 'some option'").hidden(true)]);

    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_OPT_ARGS_ONLY,
        false
    ));
}

static HIDDEN_POS_ARGS_ONLY: &str = "test 1.4

USAGE:
    test

After help";

#[test]
fn hidden_pos_args_only() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .after_help("After help")
        .mut_arg("help", |a| a.hidden(true))
        .args(&[Arg::new("pos").about("some pos").hidden(true)]);

    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_POS_ARGS_ONLY,
        false
    ));
}

static HIDDEN_SUBCMDS_ONLY: &str = "test 1.4

USAGE:
    test

After help";

#[test]
fn hidden_subcmds_only() {
    let app = App::new("test")
        .version("1.4")
        .setting(AppSettings::DisableVersionFlag)
        .after_help("After help")
        .mut_arg("help", |a| a.hidden(true))
        .subcommand(App::new("sub").setting(AppSettings::Hidden));

    assert!(utils::compare_output(
        app,
        "test --help",
        HIDDEN_SUBCMDS_ONLY,
        false
    ));
}
