#![allow(clippy::bool_assert_comparison)]

use clap::Arg;
use clap::ArgAction;
use clap::Command;

#[test]
fn set() {
    let cmd = Command::new("test").arg(Arg::new("mammal").long("mammal").action(ArgAction::Set));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(matches.get_one::<String>("mammal"), None);
    assert_eq!(matches.contains_id("mammal"), false);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), None);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal", "dog"])
        .unwrap();
    assert_eq!(matches.get_one::<String>("mammal").unwrap(), "dog");
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(2));

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal", "dog", "--mammal", "cat"])
        .unwrap();
    assert_eq!(matches.get_one::<String>("mammal").unwrap(), "cat");
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(4));
}

#[test]
fn append() {
    let cmd = Command::new("test").arg(Arg::new("mammal").long("mammal").action(ArgAction::Append));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(matches.get_one::<String>("mammal"), None);
    assert_eq!(matches.contains_id("mammal"), false);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), None);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal", "dog"])
        .unwrap();
    assert_eq!(matches.get_one::<String>("mammal").unwrap(), "dog");
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(
        matches.indices_of("mammal").unwrap().collect::<Vec<_>>(),
        vec![2]
    );

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal", "dog", "--mammal", "cat"])
        .unwrap();
    assert_eq!(
        matches
            .get_many::<String>("mammal")
            .unwrap()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
        vec!["dog", "cat"]
    );
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(
        matches.indices_of("mammal").unwrap().collect::<Vec<_>>(),
        vec![2, 4]
    );
}

#[test]
fn set_true() {
    let cmd =
        Command::new("test").arg(Arg::new("mammal").long("mammal").action(ArgAction::SetTrue));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(2));
}

#[test]
fn set_true_with_explicit_default_value() {
    let cmd = Command::new("test").arg(
        Arg::new("mammal")
            .long("mammal")
            .action(ArgAction::SetTrue)
            .default_value("false"),
    );

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));
}

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

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), false);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);

    let matches = cmd.clone().try_get_matches_from(["test", "--dog"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), true);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), false);
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

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), false);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);

    let matches = cmd.clone().try_get_matches_from(["test", "--dog"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), true);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), false);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);
}

#[test]
fn set_true_with_required_if_eq() {
    let cmd = Command::new("test")
        .arg(
            Arg::new("mammal")
                .long("mammal")
                .action(ArgAction::SetTrue)
                .required_if_eq("dog", "true"),
        )
        .arg(Arg::new("dog").long("dog").action(ArgAction::SetTrue));

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), false);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);

    cmd.clone()
        .try_get_matches_from(["test", "--dog"])
        .unwrap_err();

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--dog", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), true);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);
}

#[test]
fn set_false() {
    let cmd = Command::new("test").arg(
        Arg::new("mammal")
            .long("mammal")
            .action(ArgAction::SetFalse),
    );

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(2));
}

#[test]
fn set_false_with_explicit_default_value() {
    let cmd = Command::new("test").arg(
        Arg::new("mammal")
            .long("mammal")
            .action(ArgAction::SetFalse)
            .default_value("true"),
    );

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));
}

#[test]
fn set_false_with_default_value_if_present() {
    let cmd = Command::new("test")
        .arg(
            Arg::new("mammal")
                .long("mammal")
                .action(ArgAction::SetFalse)
                .default_value_if("dog", None, Some("false")),
        )
        .arg(Arg::new("dog").long("dog").action(ArgAction::SetFalse));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), true);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);

    let matches = cmd.clone().try_get_matches_from(["test", "--dog"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), false);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), true);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);
}

#[test]
fn set_false_with_default_value_if_value() {
    let cmd = Command::new("test")
        .arg(
            Arg::new("mammal")
                .long("mammal")
                .action(ArgAction::SetFalse)
                .default_value_if("dog", Some("false"), Some("false")),
        )
        .arg(Arg::new("dog").long("dog").action(ArgAction::SetFalse));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), true);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), true);

    let matches = cmd.clone().try_get_matches_from(["test", "--dog"]).unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), false);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<bool>("dog").unwrap(), true);
    assert_eq!(*matches.get_one::<bool>("mammal").unwrap(), false);
}

#[test]
fn count() {
    let cmd = Command::new("test").arg(Arg::new("mammal").long("mammal").action(ArgAction::Count));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 0);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 1);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 2);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(2));
}

#[test]
fn count_with_explicit_default_value() {
    let cmd = Command::new("test").arg(
        Arg::new("mammal")
            .long("mammal")
            .action(ArgAction::Count)
            .default_value("10"),
    );

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 1);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 10);
    assert_eq!(matches.contains_id("mammal"), true);
    #[allow(deprecated)]
    {
        assert_eq!(matches.occurrences_of("mammal"), 0);
    }
    assert_eq!(matches.index_of("mammal"), Some(1));
}

#[test]
fn count_with_default_value_if_present() {
    let cmd = Command::new("test")
        .arg(
            Arg::new("mammal")
                .long("mammal")
                .action(ArgAction::Count)
                .default_value_if("dog", None, Some("10")),
        )
        .arg(Arg::new("dog").long("dog").action(ArgAction::Count));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<u8>("dog").unwrap(), 0);
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 0);

    let matches = cmd.clone().try_get_matches_from(["test", "--dog"]).unwrap();
    assert_eq!(*matches.get_one::<u8>("dog").unwrap(), 1);
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 10);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<u8>("dog").unwrap(), 0);
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 1);
}

#[test]
fn count_with_default_value_if_value() {
    let cmd = Command::new("test")
        .arg(
            Arg::new("mammal")
                .long("mammal")
                .action(ArgAction::Count)
                .default_value_if("dog", Some("2"), Some("10")),
        )
        .arg(Arg::new("dog").long("dog").action(ArgAction::Count));

    let matches = cmd.clone().try_get_matches_from(["test"]).unwrap();
    assert_eq!(*matches.get_one::<u8>("dog").unwrap(), 0);
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 0);

    let matches = cmd.clone().try_get_matches_from(["test", "--dog"]).unwrap();
    assert_eq!(*matches.get_one::<u8>("dog").unwrap(), 1);
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 0);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--dog", "--dog"])
        .unwrap();
    assert_eq!(*matches.get_one::<u8>("dog").unwrap(), 2);
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 10);

    let matches = cmd
        .clone()
        .try_get_matches_from(["test", "--mammal"])
        .unwrap();
    assert_eq!(*matches.get_one::<u8>("dog").unwrap(), 0);
    assert_eq!(*matches.get_one::<u8>("mammal").unwrap(), 1);
}
