use clap::{error::ErrorKind, Command};

// #6227: regression guard for the default behavior (no opt-in).
// Without `help_subcommand(true)`, the root still gets a `help` subcommand
// because it has children, but leaf children do not.
#[test]
fn default_behavior_unchanged_leaf_has_no_help_subcommand() {
    let mut cmd = Command::new("myprog")
        .subcommand(Command::new("one"))
        .subcommand(Command::new("two"));

    // Root accepts `help`.
    let res_root = cmd.try_get_matches_from_mut(vec!["myprog", "help"]);
    assert!(
        res_root.is_err(),
        "root `help` should short-circuit with DisplayHelp"
    );
    assert_eq!(res_root.unwrap_err().kind(), ErrorKind::DisplayHelp);

    // Leaf `one` does NOT have `help`.
    let res_leaf = cmd.try_get_matches_from_mut(vec!["myprog", "one", "help"]);
    assert!(res_leaf.is_err(), "leaf `one help` should fail by default");
    let err = res_leaf.unwrap_err();
    assert!(
        matches!(
            err.kind(),
            ErrorKind::UnknownArgument | ErrorKind::InvalidSubcommand
        ),
        "expected UnknownArgument or InvalidSubcommand, got {:?}",
        err.kind()
    );
}

// #6227: enabled at root, leaves get `help` too.
#[cfg(feature = "unstable-v5")]
#[test]
fn help_subcommand_true_propagates_to_leaves() {
    let mut cmd = Command::new("myprog")
        .help_subcommand(true)
        .subcommand(Command::new("one"))
        .subcommand(Command::new("two"));

    let res_one = cmd.try_get_matches_from_mut(vec!["myprog", "one", "help"]);
    assert!(
        res_one.is_err(),
        "with help_subcommand(true), `one help` should short-circuit with DisplayHelp"
    );
    assert_eq!(res_one.unwrap_err().kind(), ErrorKind::DisplayHelp);

    let res_two = cmd.try_get_matches_from_mut(vec!["myprog", "two", "help"]);
    assert!(res_two.is_err());
    assert_eq!(res_two.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

// #6227: enabled at root, nested leaves also get `help`.
#[cfg(feature = "unstable-v5")]
#[test]
fn help_subcommand_true_propagates_through_nested_subcommands() {
    let mut cmd = Command::new("myprog").help_subcommand(true).subcommand(
        Command::new("outer")
            .subcommand(Command::new("inner_a"))
            .subcommand(Command::new("inner_b")),
    );

    let res = cmd.try_get_matches_from_mut(vec!["myprog", "outer", "inner_a", "help"]);
    assert!(res.is_err(), "deeply nested leaf should also accept `help`");
    assert_eq!(res.unwrap_err().kind(), ErrorKind::DisplayHelp);
}

// #6227: explicit `false` on a child overrides parent's `true`.
#[cfg(feature = "unstable-v5")]
#[test]
fn help_subcommand_child_override_wins_over_parent() {
    let mut cmd = Command::new("myprog").help_subcommand(true).subcommand(
        Command::new("quiet")
            .help_subcommand(false)
            .subcommand(Command::new("leaf")),
    );

    // `quiet` itself has children, but `help_subcommand(false)` suppresses it.
    let res = cmd.try_get_matches_from_mut(vec!["myprog", "quiet", "help"]);
    assert!(
        res.is_err(),
        "child `help_subcommand(false)` should suppress `help` even when parent enabled it"
    );
    let err = res.unwrap_err();
    assert!(
        matches!(
            err.kind(),
            ErrorKind::UnknownArgument | ErrorKind::InvalidSubcommand
        ),
        "expected UnknownArgument or InvalidSubcommand, got {:?}",
        err.kind()
    );
}

// #6227: `Option::<bool>::None` resets the tri-state choice.
#[cfg(feature = "unstable-v5")]
#[test]
fn help_subcommand_reset_via_none() {
    // First set to true, then reset to None: default behavior is restored, so
    // leaf `one` no longer accepts `help`.
    let mut cmd = Command::new("myprog")
        .help_subcommand(true)
        .help_subcommand(Option::<bool>::None)
        .subcommand(Command::new("one"));

    let res = cmd.try_get_matches_from_mut(vec!["myprog", "one", "help"]);
    assert!(
        res.is_err(),
        "after reset, leaf `one help` should fail like default"
    );
    let err = res.unwrap_err();
    assert!(
        matches!(
            err.kind(),
            ErrorKind::UnknownArgument | ErrorKind::InvalidSubcommand
        ),
        "expected UnknownArgument or InvalidSubcommand, got {:?}",
        err.kind()
    );
}

// #6227: explicit `Resettable::Reset` is accepted via the IntoResettable path.
#[cfg(feature = "unstable-v5")]
#[test]
fn help_subcommand_reset_via_resettable() {
    use clap::builder::Resettable;

    let mut cmd = Command::new("myprog")
        .help_subcommand(true)
        .help_subcommand(Resettable::<bool>::Reset)
        .subcommand(Command::new("one"));

    let res = cmd.try_get_matches_from_mut(vec!["myprog", "one", "help"]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(matches!(
        err.kind(),
        ErrorKind::UnknownArgument | ErrorKind::InvalidSubcommand
    ));
}
