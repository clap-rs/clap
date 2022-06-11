#[cfg(debug_assertions)]
use clap::{Arg, ArgAction, Command};

#[test]
#[cfg(debug_assertions)]
#[should_panic = "Unknown argument or group id.  Make sure you are using the argument id and not the short or long flags"]
fn arg_matches_if_present_wrong_arg() {
    let m = Command::new("test")
        .arg(Arg::new("flag").short('f').action(ArgAction::SetTrue))
        .try_get_matches_from(&["test", "-f"])
        .unwrap();

    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    m.contains_id("f");
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "Mismatch between definition and access of `o`. Unknown argument or group id.  Make sure you are using the argument id and not the short or long flags"]
fn arg_matches_value_of_wrong_arg() {
    let m = Command::new("test")
        .arg(Arg::new("opt").short('o').takes_value(true))
        .try_get_matches_from(&["test", "-o", "val"])
        .unwrap();

    assert_eq!(m.get_one::<String>("opt").map(|v| v.as_str()), Some("val"));
    m.get_one::<String>("o").map(|v| v.as_str());
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
