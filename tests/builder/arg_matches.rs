use clap::{arg, value_parser, Command};
#[cfg(debug_assertions)]
use clap::{Arg, ArgAction};

#[test]
fn ids() {
    let m = Command::new("test")
        .arg(arg!(--color <when>).value_parser(["auto", "always", "never"]))
        .arg(arg!(--config <path>).value_parser(value_parser!(std::path::PathBuf)))
        .try_get_matches_from(["test", "--config=config.toml", "--color=auto"])
        .unwrap();
    assert_eq!(
        m.ids().map(|id| id.as_str()).collect::<Vec<_>>(),
        ["config", "color"]
    );
    assert_eq!(m.ids().len(), 2);
}

#[test]
fn ids_ignore_unused() {
    let m = Command::new("test")
        .arg(arg!(--color <when>).value_parser(["auto", "always", "never"]))
        .arg(arg!(--config <path>).value_parser(value_parser!(std::path::PathBuf)))
        .try_get_matches_from(["test", "--config=config.toml"])
        .unwrap();
    assert_eq!(
        m.ids().map(|id| id.as_str()).collect::<Vec<_>>(),
        ["config"]
    );
    assert_eq!(m.ids().len(), 1);
}

#[test]
fn ids_ignore_overridden() {
    let m = Command::new("test")
        .arg(arg!(--color <when>).value_parser(["auto", "always", "never"]))
        .arg(
            arg!(--config <path>)
                .value_parser(value_parser!(std::path::PathBuf))
                .overrides_with("color"),
        )
        .try_get_matches_from(["test", "--config=config.toml", "--color=auto"])
        .unwrap();
    assert_eq!(m.ids().map(|id| id.as_str()).collect::<Vec<_>>(), ["color"]);
    assert_eq!(m.ids().len(), 1);
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "Unknown argument or group id.  Make sure you are using the argument id and not the short or long flags"]
fn arg_matches_if_present_wrong_arg() {
    let m = Command::new("test")
        .arg(Arg::new("flag").short('f').action(ArgAction::SetTrue))
        .try_get_matches_from(["test", "-f"])
        .unwrap();

    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    m.contains_id("f");
}

#[test]
#[cfg(debug_assertions)]
#[should_panic = "Mismatch between definition and access of `o`. Unknown argument or group id.  Make sure you are using the argument id and not the short or long flags"]
fn arg_matches_value_of_wrong_arg() {
    let m = Command::new("test")
        .arg(Arg::new("opt").short('o').action(ArgAction::Set))
        .try_get_matches_from(["test", "-o", "val"])
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
        .try_get_matches_from(["test", "speed"])
        .unwrap();

    assert!(m.subcommand_matches("speed").is_some());
    m.subcommand_matches("seed");
}
