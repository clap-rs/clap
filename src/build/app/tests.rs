use crate::{App, AppSettings};

#[test]
fn global_version() {
    let mut app = App::new("global_version")
        .setting(AppSettings::GlobalVersion)
        .version("1.1")
        .subcommand(App::new("sub1"));
    app._propagate();
    assert_eq!(app.subcommands[0].version, Some("1.1"));
}

#[test]
fn global_setting() {
    let mut app = App::new("test")
        .global_setting(AppSettings::ColoredHelp)
        .subcommand(App::new("subcmd"));
    app._propagate();
    assert!(app
        .subcommands
        .iter()
        .find(|s| s.name == "subcmd")
        .unwrap()
        .is_set(AppSettings::ColoredHelp));
}

#[test]
fn global_settings() {
    let mut app = App::new("test")
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::TrailingVarArg)
        .subcommand(App::new("subcmd"));
    app._propagate();
    assert!(app
        .subcommands
        .iter()
        .find(|s| s.name == "subcmd")
        .unwrap()
        .is_set(AppSettings::ColoredHelp));
    assert!(app
        .subcommands
        .iter()
        .find(|s| s.name == "subcmd")
        .unwrap()
        .is_set(AppSettings::TrailingVarArg));
}

// This test will *fail to compile* if App is not Send + Sync
#[test]
fn app_send_sync() {
    fn foo<T: Send + Sync>(_: T) {}
    foo(App::new("test"))
}

#[test]
fn issue_2090() {
    let mut app = App::new("app")
        .global_setting(AppSettings::DisableVersionFlag)
        .subcommand(App::new("sub"));
    app._build();

    assert!(app.subcommands[0].is_set(AppSettings::DisableVersionFlag));
}

#[test]
fn long_help_message() {
    let mut app = App::new("app")
        .setting(AppSettings::ColorNever)
        .global_setting(AppSettings::DisableVersionFlag)
        .subcommand(App::new("sub")
            .long_about("long about sub")
            .about("short about sub"));
    app._build();

    let mut bytes = Vec::<u8>::new();

    app.write_long_help(&mut bytes).unwrap();

    assert_eq!(
        "app 

USAGE:
    app [SUBCOMMAND]

FLAGS:
    -h, --help
            Prints help information

SUBCOMMANDS:
    help
            Prints this message or the help of the given subcommand(s)
    sub
            short about sub
",
        std::str::from_utf8(bytes.as_slice()).unwrap()
    );
}

#[test]
fn short_help_message() {
    let mut app = App::new("app")
        .setting(AppSettings::ColorNever)
        .global_setting(AppSettings::DisableVersionFlag)
        .subcommand(App::new("sub")
            .long_about("long about sub")
            .about("short about sub"));
    app._build();

    let mut bytes = Vec::<u8>::new();

    app.write_help(&mut bytes).unwrap();

    assert_eq!(
        "app 

USAGE:
    app [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    sub     short about sub
",
        std::str::from_utf8(bytes.as_slice()).unwrap()
    );
}
