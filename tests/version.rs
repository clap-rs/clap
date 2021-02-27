mod utils;

use std::str;

use clap::{App, Arg, ErrorKind};

static VERSION: &str = "clap-test v1.4.8\n";

#[test]
fn version_short() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .long_version("1.3 (abcdef12)")
        .try_get_matches_from(vec!["myprog", "-V"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");
}

#[test]
fn version_long() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .long_version("1.3 (abcdef12)")
        .try_get_matches_from(vec!["myprog", "--version"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3 (abcdef12)\n");
}

#[test]
fn complex_version_output() {
    let mut a = App::new("clap-test").version("v1.4.8");
    let _ = a.try_get_matches_from_mut(vec![""]);

    // Now we check the output of print_version()
    assert_eq!(a.render_version(), VERSION);
}

fn prefer_user_app() -> App<'static> {
    App::new("test")
        .version("1.3")
        .arg(
            Arg::new("version1")
                .long("version")
                .short('V')
                .about("some version"),
        )
        .subcommand(
            App::new("foo").arg(
                Arg::new("version1")
                    .long("version")
                    .short('V')
                    .about("some version"),
            ),
        )
}

#[test]
fn prefer_user_version_long() {
    let m = prefer_user_app().try_get_matches_from(vec!["test", "--version"]);

    assert!(m.is_ok());
    assert!(m.unwrap().is_present("version1"));
}

#[test]
fn prefer_user_version_short() {
    let m = prefer_user_app().try_get_matches_from(vec!["test", "-V"]);

    assert!(m.is_ok());
    assert!(m.unwrap().is_present("version1"));
}

#[test]
fn prefer_user_subcmd_version_long() {
    let m = prefer_user_app().try_get_matches_from(vec!["test", "foo", "--version"]);

    assert!(m.is_ok());
    assert!(m
        .unwrap()
        .subcommand_matches("foo")
        .unwrap()
        .is_present("version1"));
}

#[test]
fn prefer_user_subcmd_version_short() {
    let m = prefer_user_app().try_get_matches_from(vec!["test", "foo", "-V"]);

    assert!(m.is_ok());
    assert!(m
        .unwrap()
        .subcommand_matches("foo")
        .unwrap()
        .is_present("version1"));
}

static OVERRIDE_VERSION_SHORT: &str = "test 1.3

USAGE:
    test

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints version information";

#[test]
fn override_version_short() {
    let app = App::new("test")
        .version("1.3")
        .mut_arg("version", |a| a.short('v'));

    let m = app.clone().try_get_matches_from(vec!["test", "-v"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");

    let m = app.clone().try_get_matches_from(vec!["test", "--version"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");

    assert!(utils::compare_output(
        app,
        "test -h",
        OVERRIDE_VERSION_SHORT,
        false
    ));
}

static OVERRIDE_VERSION_LONG: &str = "test 1.3

USAGE:
    test [FLAGS]

FLAGS:
    -h, --help    Prints help information
    -V, --vers    Prints version information";

#[test]
fn override_version_long() {
    let app = App::new("test")
        .version("1.3")
        .mut_arg("version", |a| a.long("vers"));

    let m = app.clone().try_get_matches_from(vec!["test", "-V"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");

    let m = app.clone().try_get_matches_from(vec!["test", "--vers"]);

    assert!(m.is_err());
    let err = m.unwrap_err();
    assert_eq!(err.kind, ErrorKind::DisplayVersion);
    assert_eq!(err.to_string(), "test 1.3\n");

    assert!(utils::compare_output(
        app,
        "test -h",
        OVERRIDE_VERSION_LONG,
        false
    ));
}

static OVERRIDE_VERSION_ABOUT: &str = "test 1.3

USAGE:
    test

FLAGS:
    -h, --help       Prints help information
    -V, --version    version info";

#[test]
fn override_version_about() {
    let app = App::new("test")
        .version("1.3")
        .mut_arg("version", |a| a.about("version info"));

    assert!(utils::compare_output(
        app.clone(),
        "test -h",
        OVERRIDE_VERSION_ABOUT,
        false
    ));
    assert!(utils::compare_output(
        app,
        "test --help",
        OVERRIDE_VERSION_ABOUT,
        false
    ));
}

static VERSION_ABOUT_MULTI_SC: &str = "myapp-subcmd-multi 1.0

USAGE:
    myapp subcmd multi

FLAGS:
    -h, --help       Prints help information
    -V, --version    Print custom version about text";

#[test]
fn version_about_multi_subcmd() {
    let app = App::new("myapp")
        .mut_arg("version", |a| a.about("Print custom version about text"))
        .subcommand(App::new("subcmd").subcommand(App::new("multi").version("1.0")));

    assert!(utils::compare_output(
        app.clone(),
        "myapp help subcmd multi",
        VERSION_ABOUT_MULTI_SC,
        false
    ));
    assert!(utils::compare_output(
        app.clone(),
        "myapp subcmd multi -h",
        VERSION_ABOUT_MULTI_SC,
        false
    ));
    assert!(utils::compare_output(
        app,
        "myapp subcmd multi --help",
        VERSION_ABOUT_MULTI_SC,
        false
    ));
}

static VERSION_ABOUT_MULTI_SC_OVERRIDE: &str = "myapp-subcmd-multi 1.0

USAGE:
    myapp subcmd multi

FLAGS:
    -h, --help       Prints help information
    -V, --version    Print custom version about text from multi";

#[test]
fn version_about_multi_subcmd_override() {
    let app = App::new("myapp")
        .mut_arg("version", |a| a.about("Print custom version about text"))
        .subcommand(App::new("subcmd").subcommand(
            App::new("multi").version("1.0").mut_arg("version", |a| {
                a.about("Print custom version about text from multi")
            }),
        ));

    assert!(utils::compare_output(
        app,
        "myapp help subcmd multi",
        VERSION_ABOUT_MULTI_SC_OVERRIDE,
        false
    ));
}
