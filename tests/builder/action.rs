use clap::builder::ArgAction;
use clap::Arg;
use clap::Command;

#[test]
fn set_true_with_default_value_if_present() {
    let cmd = Command::new("test")
        .arg(
            Arg::new("mammal")
                .long("mammal")
                .action(ArgAction::SetTrue)
                .default_value_if("dog", None, Some("true")),
        )
        .arg(Arg::new("dog").long("dog").action(ArgAction::SetTrue));

    let matches = cmd.clone().try_get_matches_from(["test", "--dog"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), true);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(matches.get_one::<bool>("dog"), None);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);
}

#[test]
fn set_true_with_default_value_if_value() {
    let cmd = Command::new("test")
        .arg(
            Arg::new("mammal")
                .long("mammal")
                .action(ArgAction::SetTrue)
                .default_value_if("dog", Some("true"), Some("true")),
        )
        .arg(Arg::new("dog").long("dog").action(ArgAction::SetTrue));

    let matches = cmd.clone().try_get_matches_from(["test", "--dog"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), true);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(matches.get_one::<bool>("dog"), None);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);
}
