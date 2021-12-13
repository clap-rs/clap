#[cfg(debug_assertions)]
use clap::{App, Arg};

#[test]
#[cfg(debug_assertions)]
#[should_panic = "'f' is not a name of an argument or a group."]
fn arg_matches_if_present_wrong_arg() {
    let m = App::new("test")
        .arg(Arg::new("flag").short('f'))
        .get_matches_from(&["test", "-f"]);

    assert!(m.is_present("flag"));
    m.is_present("f");
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "'o' is not a name of an argument or a group."]
fn arg_matches_value_of_wrong_arg() {
    let m = App::new("test")
        .arg(Arg::new("opt").short('o').takes_value(true))
        .get_matches_from(&["test", "-o", "val"]);

    assert_eq!(m.value_of("opt"), Some("val"));
    m.value_of("o");
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "'seed' is not a name of a subcommand."]
fn arg_matches_subcommand_matches_wrong_sub() {
    let m = App::new("test")
        .subcommand(App::new("speed"))
        .get_matches_from(&["test", "speed"]);

    assert!(m.subcommand_matches("speed").is_some());
    m.subcommand_matches("seed");
}
