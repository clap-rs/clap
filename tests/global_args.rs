mod utils;

use clap::{App, Arg};

fn get_app() -> App<'static> {
    App::new("myprog")
        .arg(
            Arg::new("GLOBAL_ARG")
                .long("global-arg")
                .about("Specifies something needed by the subcommands")
                .global(true)
                .takes_value(true)
                .default_value("default_value"),
        )
        .arg(
            Arg::new("GLOBAL_FLAG")
                .long("global-flag")
                .about("Specifies something needed by the subcommands")
                .multiple(true)
                .global(true),
        )
        .subcommand(App::new("outer").subcommand(App::new("inner")))
}

#[test]
fn issue_1076() {
    let mut app = get_app();
    let _ = app.try_get_matches_from_mut(vec!["myprog"]);
    let _ = app.try_get_matches_from_mut(vec!["myprog"]);
    let _ = app.try_get_matches_from_mut(vec!["myprog"]);
}

#[test]
fn propagate_global_arg_in_subcommand_to_subsubcommand_1385() {
    let m1 = App::new("foo")
        .subcommand(
            App::new("sub1")
                .arg(Arg::new("arg1").long("arg1").takes_value(true).global(true))
                .subcommand(App::new("sub1a")),
        )
        .get_matches_from(&["foo", "sub1", "--arg1", "v1", "sub1a"]);
    assert_eq!(
        "v1",
        m1.subcommand_matches("sub1")
            .unwrap()
            .subcommand_matches("sub1a")
            .unwrap()
            .value_of("arg1")
            .unwrap()
    );
}

#[test]
fn propagate_global_arg_in_subcommand_to_subsubcommand_2053() {
    let m = App::new("opts")
        .arg(Arg::from("--global-flag").global(true))
        .arg(
            Arg::from("--global-str <global-str>")
                .required(false)
                .global(true),
        )
        .subcommand(
            App::new("test")
                .arg(Arg::from("--sub-flag").global(true))
                .arg(
                    Arg::from("--sub-str <sub-str>")
                        .required(false)
                        .global(true),
                )
                .subcommand(App::new("test")),
        )
        .get_matches_from(&[
            "app",
            "test",
            "test",
            "--global-flag",
            "--global-str",
            "hello",
            "--sub-flag",
            "--sub-str",
            "world",
        ]);
    assert_eq!(
        Some("world"),
        m.subcommand_matches("test").unwrap().value_of("sub-str")
    );
}
