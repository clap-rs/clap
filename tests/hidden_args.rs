mod utils;

use clap::{App, Arg};

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
            Arg::with_name("DUMMY").hidden(true),
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
    -V, --version    Prints version information
    -v, --visible    This text should be visible";

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

    -V, --version    
            Prints version information

    -v, --visible    
            This text should be visible";

/// Ensure hidden with short option
#[test]
fn hidden_short_args() {
    let app = App::new("test")
        .about("hides short args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::with_name("cfg")
                .short('c')
                .long("config")
                .hidden_short_help(true)
                .about("Some help text describing the --config arg"),
            Arg::with_name("visible")
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
            Arg::with_name("cfg")
                .short('c')
                .long("config")
                .hidden_short_help(true)
                .about("Some help text describing the --config arg"),
            Arg::with_name("visible")
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

    -V, --version    
            Prints version information

    -v, --visible    
            This text should be visible";

#[test]
fn hidden_long_args() {
    let app = App::new("test")
        .about("hides long args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::with_name("cfg")
                .short('c')
                .long("config")
                .hidden_long_help(true)
                .about("Some help text describing the --config arg"),
            Arg::with_name("visible")
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
    -V, --version    Prints version information
    -v, --visible    This text should be visible";

#[test]
fn hidden_long_args_short_help() {
    let app = App::new("test")
        .about("hides long args")
        .author("Steve P.")
        .version("2.31.2")
        .args(&[
            Arg::with_name("cfg")
                .short('c')
                .long("config")
                .hidden_long_help(true)
                .about("Some help text describing the --config arg"),
            Arg::with_name("visible")
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
