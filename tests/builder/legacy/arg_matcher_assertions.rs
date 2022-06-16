#[cfg(debug_assertions)]
use clap::{Arg, Command};

#[test]
#[cfg(debug_assertions)]
#[should_panic = "`f` is not an id of an argument or a group."]
fn arg_matches_if_present_wrong_arg() {
    let m = Command::new("test")
        .arg(Arg::new("flag").short('f'))
        .try_get_matches_from(&["test", "-f"])
        .unwrap();

    assert!(m.is_present("flag"));
    m.is_present("f");
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "`o` is not an id of an argument or a group."]
fn arg_matches_value_of_wrong_arg() {
    let m = Command::new("test")
        .arg(Arg::new("opt").short('o').takes_value(true))
        .try_get_matches_from(&["test", "-o", "val"])
        .unwrap();

    assert_eq!(m.value_of("opt"), Some("val"));
    m.value_of("o");
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "`seed` is not a name of a subcommand."]
fn arg_matches_subcommand_matches_wrong_sub() {
    let m = Command::new("test")
        .subcommand(Command::new("speed"))
        .try_get_matches_from(&["test", "speed"])
        .unwrap();

    assert!(m.subcommand_matches("speed").is_some());
    m.subcommand_matches("seed");
}
