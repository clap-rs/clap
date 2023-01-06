use clap::{arg, error::ErrorKind, Arg, ArgAction, ArgGroup, Command};

#[cfg(feature = "error-context")]
use super::utils;

#[test]
fn flag_conflict() {
    let result = Command::new("flag_conflict")
        .arg(arg!(-f --flag "some flag").conflicts_with("other"))
        .arg(arg!(-o --other "some flag"))
        .try_get_matches_from(vec!["myprog", "-f", "-o"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn flag_conflict_2() {
    let result = Command::new("flag_conflict")
        .arg(arg!(-f --flag "some flag").conflicts_with("other"))
        .arg(arg!(-o --other "some flag"))
        .try_get_matches_from(vec!["myprog", "-o", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn flag_conflict_with_all() {
    let result = Command::new("flag_conflict")
        .arg(arg!(-f --flag "some flag").conflicts_with_all(["other"]))
        .arg(arg!(-o --other "some flag"))
        .try_get_matches_from(vec!["myprog", "-o", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn exclusive_flag() {
    let cmd = Command::new("flag_conflict")
        .arg(arg!(-f --flag "some flag").exclusive(true))
        .arg(arg!(-o --other "some flag"));
    let result = cmd.clone().try_get_matches_from(vec!["myprog", "-f"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());

    let result = cmd.clone().try_get_matches_from(vec!["myprog", "-o", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn exclusive_option() {
    let result = Command::new("flag_conflict")
        .arg(arg!(-f --flag <VALUE> "some flag").exclusive(true))
        .arg(arg!(-o --other <VALUE> "some flag"))
        .try_get_matches_from(vec!["myprog", "-o=val1", "-f=val2"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn not_exclusive_with_defaults() {
    let result = Command::new("flag_conflict")
        .arg(arg!(-f --flag <VALUE> "some flag").exclusive(true))
        .arg(
            arg!(-o --other <VALUE> "some flag")
                .required(false)
                .default_value("val1"),
        )
        .try_get_matches_from(vec!["myprog", "-f=val2"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
}

#[test]
fn not_exclusive_with_group() {
    let cmd = Command::new("test")
        .group(clap::ArgGroup::new("test").arg("foo"))
        .arg(
            clap::Arg::new("foo")
                .long("foo")
                .exclusive(true)
                .action(clap::ArgAction::SetTrue),
        );
    let result = cmd.try_get_matches_from(vec!["test", "--foo"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
}

#[test]
fn default_doesnt_activate_exclusive() {
    let result = Command::new("flag_conflict")
        .arg(
            arg!(-f --flag <VALUE> "some flag")
                .exclusive(true)
                .default_value("val2"),
        )
        .arg(arg!(-o --other <VALUE> "some flag").default_value("val1"))
        .try_get_matches_from(vec!["myprog"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
}

#[test]
fn arg_conflicts_with_group() {
    let mut cmd = Command::new("group_conflict")
        .arg(arg!(-f --flag "some flag").conflicts_with("gr"))
        .group(ArgGroup::new("gr").arg("some").arg("other"))
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"));

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--other", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "-f", "--some"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--some"]);
    if let Err(err) = result {
        panic!("{}", err);
    }

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--other"]);
    if let Err(err) = result {
        panic!("{}", err);
    }

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--flag"]);
    if let Err(err) = result {
        panic!("{}", err);
    }
}

#[test]
fn arg_conflicts_with_group_with_multiple_sources() {
    let mut cmd = clap::Command::new("group_conflict")
        .arg(clap::arg!(-f --flag "some flag").conflicts_with("gr"))
        .group(clap::ArgGroup::new("gr").multiple(true))
        .arg(clap::arg!(--some <name> "some arg").group("gr"))
        .arg(
            clap::arg!(--other <secs> "other arg")
                .default_value("1000")
                .group("gr"),
        );

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "-f"]);
    if let Err(err) = result {
        panic!("{}", err);
    }

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--some", "usb1"]);
    if let Err(err) = result {
        panic!("{}", err);
    }

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--some", "usb1", "--other", "40"]);
    if let Err(err) = result {
        panic!("{}", err);
    }

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "-f", "--some", "usb1"]);
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn group_conflicts_with_arg() {
    let mut cmd = Command::new("group_conflict")
        .arg(arg!(-f --flag "some flag"))
        .group(
            ArgGroup::new("gr")
                .arg("some")
                .arg("other")
                .conflicts_with("flag"),
        )
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"));

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--other", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "-f", "--some"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--some"]);
    if let Err(err) = result {
        panic!("{}", err);
    }

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--other"]);
    if let Err(err) = result {
        panic!("{}", err);
    }

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--flag"]);
    if let Err(err) = result {
        panic!("{}", err);
    }
}

#[test]
fn arg_conflicts_with_required_group() {
    let mut cmd = Command::new("group_conflict")
        .arg(arg!(-f --flag "some flag").conflicts_with("gr"))
        .group(ArgGroup::new("gr").required(true).arg("some").arg("other"))
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"));

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--other", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "-f", "--some"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--some"]);
    if let Err(err) = result {
        panic!("{}", err);
    }

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--other"]);
    if let Err(err) = result {
        panic!("{}", err);
    }
}

#[test]
fn arg_conflicts_with_group_with_required_memeber() {
    let mut cmd = Command::new("group_conflict")
        .arg(arg!(-f --flag "some flag").conflicts_with("gr"))
        .group(ArgGroup::new("gr").arg("some").arg("other"))
        .arg(arg!(--some "some arg").required(true))
        .arg(arg!(--other "other arg"));

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--other", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "-f", "--some"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--some"]);
    if let Err(err) = result {
        panic!("{}", err);
    }

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--flag"]);
    if let Err(err) = result {
        panic!("{}", err);
    }
}

#[test]
fn required_group_conflicts_with_arg() {
    let mut cmd = Command::new("group_conflict")
        .arg(arg!(-f --flag "some flag"))
        .group(
            ArgGroup::new("gr")
                .required(true)
                .arg("some")
                .arg("other")
                .conflicts_with("flag"),
        )
        .arg(arg!(--some "some arg"))
        .arg(arg!(--other "other arg"));

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--other", "-f"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "-f", "--some"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--some"]);
    if let Err(err) = result {
        panic!("{}", err);
    }

    let result = cmd.try_get_matches_from_mut(vec!["myprog", "--other"]);
    if let Err(err) = result {
        panic!("{}", err);
    }
}

#[test]
fn get_arg_conflicts_with_group() {
    let flag = arg!(--flag).conflicts_with("gr");
    let mut cmd = Command::new("group_conflict")
        .arg(&flag)
        .group(ArgGroup::new("gr").arg("some").arg("other"))
        .arg(arg!(--some))
        .arg(arg!(--other));

    cmd.build();

    let result = cmd.get_arg_conflicts_with(&flag);

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].get_id(), "some");
    assert_eq!(result[1].get_id(), "other");
}

#[test]
#[cfg(feature = "error-context")]
fn conflict_output() {
    static CONFLICT_ERR: &str = "\
error: the argument '--flag...' cannot be used with '-F'

Usage: clap-test --flag... --long-option-2 <option2> <positional> <positional2> [positional3]...

For more information, try '--help'.
";

    utils::assert_output(
        utils::complex_app(),
        "clap-test val1 fa --flag --long-option-2 val2 -F",
        CONFLICT_ERR,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn conflict_output_rev() {
    static CONFLICT_ERR_REV: &str = "\
error: the argument '-F' cannot be used with '--flag...'

Usage: clap-test -F --long-option-2 <option2> <positional> <positional2> [positional3]...

For more information, try '--help'.
";

    utils::assert_output(
        utils::complex_app(),
        "clap-test val1 fa -F --long-option-2 val2 --flag",
        CONFLICT_ERR_REV,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn conflict_output_repeat() {
    static ERR: &str = "\
error: the argument '-F' cannot be used multiple times

Usage: clap-test [OPTIONS] [positional] [positional2] [positional3]... [COMMAND]

For more information, try '--help'.
";

    utils::assert_output(utils::complex_app(), "clap-test -F -F", ERR, true);
}

#[test]
#[cfg(feature = "error-context")]
fn conflict_output_with_required() {
    static CONFLICT_ERR: &str = "\
error: the argument '--flag...' cannot be used with '-F'

Usage: clap-test --flag... --long-option-2 <option2> <positional> <positional2> [positional3]...

For more information, try '--help'.
";

    utils::assert_output(
        utils::complex_app(),
        "clap-test val1 --flag --long-option-2 val2 -F",
        CONFLICT_ERR,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn conflict_output_rev_with_required() {
    static CONFLICT_ERR_REV: &str = "\
error: the argument '-F' cannot be used with '--flag...'

Usage: clap-test -F --long-option-2 <option2> <positional> <positional2> [positional3]...

For more information, try '--help'.
";

    utils::assert_output(
        utils::complex_app(),
        "clap-test val1 -F --long-option-2 val2 --flag",
        CONFLICT_ERR_REV,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn conflict_output_three_conflicting() {
    static CONFLICT_ERR_THREE: &str = "\
error: the argument '--one' cannot be used with:
  --two
  --three

Usage: three_conflicting_arguments --one

For more information, try '--help'.
";

    let cmd = Command::new("three_conflicting_arguments")
        .arg(
            Arg::new("one")
                .long("one")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["two", "three"]),
        )
        .arg(
            Arg::new("two")
                .long("two")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["one", "three"]),
        )
        .arg(
            Arg::new("three")
                .long("three")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["one", "two"]),
        );
    utils::assert_output(
        cmd,
        "three_conflicting_arguments --one --two --three",
        CONFLICT_ERR_THREE,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn two_conflicting_arguments() {
    let a = Command::new("two_conflicting_arguments")
        .arg(
            Arg::new("develop")
                .long("develop")
                .action(ArgAction::SetTrue)
                .conflicts_with("production"),
        )
        .arg(
            Arg::new("production")
                .long("production")
                .action(ArgAction::SetTrue)
                .conflicts_with("develop"),
        )
        .try_get_matches_from(vec!["", "--develop", "--production"]);

    assert!(a.is_err());
    let a = a.unwrap_err();
    assert!(
        a.to_string()
            .contains("the argument \'--develop\' cannot be used with \'--production\'"),
        "{}",
        a
    );
}

#[test]
#[cfg(feature = "error-context")]
fn three_conflicting_arguments() {
    let a = Command::new("three_conflicting_arguments")
        .arg(
            Arg::new("one")
                .long("one")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["two", "three"]),
        )
        .arg(
            Arg::new("two")
                .long("two")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["one", "three"]),
        )
        .arg(
            Arg::new("three")
                .long("three")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["one", "two"]),
        )
        .try_get_matches_from(vec!["", "--one", "--two", "--three"]);

    assert!(a.is_err());
    let a = a.unwrap_err();
    assert!(
        a.to_string()
            .contains("the argument \'--one\' cannot be used with:"),
        "{}",
        a
    );
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument 'config' cannot conflict with itself"]
fn self_conflicting_arg() {
    let _ = Command::new("prog")
        .arg(Arg::new("config").long("config").conflicts_with("config"))
        .try_get_matches_from(vec!["", "--config"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument or group 'extra' specified in 'conflicts_with*' for 'config' does not exist"]
fn conflicts_with_invalid_arg() {
    let _ = Command::new("prog")
        .arg(Arg::new("config").long("config").conflicts_with("extra"))
        .try_get_matches_from(vec!["", "--config"]);
}

#[test]
fn conflict_with_unused_default() {
    let result = Command::new("conflict")
        .arg(arg!(-o --opt <opt> "some opt").default_value("default"))
        .arg(
            arg!(-f --flag "some flag")
                .conflicts_with("opt")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["myprog", "-f"]);

    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();

    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("default")
    );
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn conflicts_with_alongside_default() {
    let result = Command::new("conflict")
        .arg(
            arg!(-o --opt <opt> "some opt")
                .default_value("default")
                .conflicts_with("flag"),
        )
        .arg(arg!(-f --flag "some flag").action(ArgAction::SetTrue))
        .try_get_matches_from(vec!["myprog", "-f"]);

    assert!(
        result.is_ok(),
        "conflicts_with should ignore default_value: {:?}",
        result.unwrap_err()
    );
    let m = result.unwrap();

    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("default")
    );
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn group_in_conflicts_with() {
    let result = Command::new("conflict")
        .arg(
            Arg::new("opt")
                .long("opt")
                .default_value("default")
                .group("one"),
        )
        .arg(
            Arg::new("flag")
                .long("flag")
                .conflicts_with("one")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["myprog", "--flag"]);

    assert!(
        result.is_ok(),
        "conflicts_with on an arg group should ignore default_value: {:?}",
        result.unwrap_err()
    );
    let m = result.unwrap();

    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("default")
    );
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn group_conflicts_with_default_value() {
    let result = Command::new("conflict")
        .arg(
            Arg::new("opt")
                .long("opt")
                .default_value("default")
                .group("one"),
        )
        .arg(
            Arg::new("flag")
                .long("flag")
                .group("one")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["myprog", "--flag"]);

    assert!(
        result.is_ok(),
        "arg group count should ignore default_value: {:?}",
        result.unwrap_err()
    );
    let m = result.unwrap();

    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("default")
    );
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn group_conflicts_with_default_arg() {
    let result = Command::new("conflict")
        .arg(Arg::new("opt").long("opt").default_value("default"))
        .arg(
            Arg::new("flag")
                .long("flag")
                .group("one")
                .action(ArgAction::SetTrue),
        )
        .group(ArgGroup::new("one").conflicts_with("opt"))
        .try_get_matches_from(vec!["myprog", "--flag"]);

    assert!(
        result.is_ok(),
        "arg group conflicts_with should ignore default_value: {:?}",
        result.unwrap_err()
    );
    let m = result.unwrap();

    assert_eq!(
        m.get_one::<String>("opt").map(|v| v.as_str()),
        Some("default")
    );
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
}

#[test]
fn exclusive_with_required() {
    let cmd = Command::new("bug")
        .arg(
            Arg::new("test")
                .long("test")
                .action(ArgAction::SetTrue)
                .exclusive(true),
        )
        .arg(Arg::new("input").action(ArgAction::Set).required(true));

    cmd.clone()
        .try_get_matches_from(["bug", "--test", "required"])
        .unwrap_err();

    cmd.clone()
        .try_get_matches_from(["bug", "required"])
        .unwrap();

    cmd.clone().try_get_matches_from(["bug", "--test"]).unwrap();
}

#[test]
fn args_negate_subcommands_one_level() {
    let res = Command::new("disablehelp")
        .args_conflicts_with_subcommands(true)
        .subcommand_negates_reqs(true)
        .arg(arg!(<arg1> "some arg"))
        .arg(arg!(<arg2> "some arg"))
        .subcommand(
            Command::new("sub1").subcommand(Command::new("sub2").subcommand(Command::new("sub3"))),
        )
        .try_get_matches_from(vec!["", "pickles", "sub1"]);
    assert!(res.is_ok(), "error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert_eq!(
        m.get_one::<String>("arg2").map(|v| v.as_str()),
        Some("sub1")
    );
}

#[test]
fn args_negate_subcommands_two_levels() {
    let res = Command::new("disablehelp")
        .args_conflicts_with_subcommands(true)
        .subcommand_negates_reqs(true)
        .arg(arg!(<arg1> "some arg"))
        .arg(arg!(<arg2> "some arg"))
        .subcommand(
            Command::new("sub1")
                .args_conflicts_with_subcommands(true)
                .subcommand_negates_reqs(true)
                .arg(arg!(<arg> "some"))
                .arg(arg!(<arg2> "some"))
                .subcommand(Command::new("sub2").subcommand(Command::new("sub3"))),
        )
        .try_get_matches_from(vec!["", "sub1", "arg", "sub2"]);
    assert!(res.is_ok(), "error: {:?}", res.unwrap_err().kind());
    let m = res.unwrap();
    assert_eq!(
        m.subcommand_matches("sub1")
            .unwrap()
            .get_one::<String>("arg2")
            .map(|v| v.as_str()),
        Some("sub2")
    );
}

#[test]
#[cfg(feature = "error-context")]
fn subcommand_conflict_error_message() {
    static CONFLICT_ERR: &str = "\
error: unexpected argument 'sub1'

Usage: test [OPTIONS]
       test <COMMAND>

For more information, try '--help'.
";

    let cmd = Command::new("test")
        .args_conflicts_with_subcommands(true)
        .arg(arg!(-p --place <"place id"> "Place ID to open"))
        .subcommand(
            Command::new("sub1").subcommand(Command::new("sub2").subcommand(Command::new("sub3"))),
        );

    utils::assert_output(cmd, "test --place id sub1", CONFLICT_ERR, true);
}

#[test]
fn subcommand_conflict_negates_required() {
    let cmd = Command::new("test")
        .args_conflicts_with_subcommands(true)
        .subcommand(Command::new("config"))
        .arg(arg!(-p --place <"place id"> "Place ID to open").required(true));

    let result = cmd.try_get_matches_from(["test", "config"]);
    assert!(
        result.is_ok(),
        "args_conflicts_with_subcommands should ignore required: {}",
        result.unwrap_err()
    );
    let m = result.unwrap();
    assert_eq!(m.subcommand_name().unwrap(), "config");
}
