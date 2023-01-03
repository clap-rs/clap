use clap::{arg, error::ErrorKind, Arg, ArgAction, ArgGroup, Command, Id};

use super::utils;

#[test]
fn required_group_missing_arg() {
    let result = Command::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!( -c --color "some other flag"))
        .group(ArgGroup::new("req").args(["flag", "color"]).required(true))
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command group: Argument group 'req' contains non-existent argument"]
fn non_existing_arg() {
    let _ = Command::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color "some other flag"))
        .group(ArgGroup::new("req").args(["flg", "color"]).required(true))
        .try_get_matches_from(vec![""]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command group: Argument group name must be unique\n\n\t'req' is already in use"]
fn unique_group_name() {
    let _ = Command::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color "some other flag"))
        .group(ArgGroup::new("req").args(["flag"]).required(true))
        .group(ArgGroup::new("req").args(["color"]).required(true))
        .try_get_matches_from(vec![""]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command group: Argument group name 'a' must not conflict with argument name"]
fn groups_new_of_arg_name() {
    let _ = Command::new("group")
        .arg(Arg::new("a").long("a").group("a"))
        .try_get_matches_from(vec!["", "--a"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Command group: Argument group name 'a' must not conflict with argument name"]
fn arg_group_new_of_arg_name() {
    let _ = Command::new("group")
        .arg(Arg::new("a").long("a").group("a"))
        .group(ArgGroup::new("a"))
        .try_get_matches_from(vec!["", "--a"]);
}

#[test]
fn group_single_value() {
    let res = Command::new("group")
        .arg(arg!(-c --color [color] "some option"))
        .arg(arg!(-n --hostname <name> "another option"))
        .group(ArgGroup::new("grp").args(["hostname", "color"]))
        .try_get_matches_from(vec!["", "-c", "blue"]);
    assert!(res.is_ok(), "{}", res.unwrap_err());

    let m = res.unwrap();
    assert!(m.contains_id("grp"));
    assert_eq!(m.get_one::<Id>("grp").map(|v| v.as_str()).unwrap(), "color");
}

#[test]
fn group_empty() {
    let res = Command::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color [color] "some option"))
        .arg(arg!(-n --hostname <name> "another option"))
        .group(ArgGroup::new("grp").args(["hostname", "color", "flag"]))
        .try_get_matches_from(vec![""]);
    assert!(res.is_ok(), "{}", res.unwrap_err());

    let m = res.unwrap();
    assert!(!m.contains_id("grp"));
    assert!(m.get_one::<String>("grp").map(|v| v.as_str()).is_none());
}

#[test]
fn group_required_flags_empty() {
    let result = Command::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color "some option"))
        .arg(arg!(-n --hostname <name> "another option"))
        .group(
            ArgGroup::new("grp")
                .required(true)
                .args(["hostname", "color", "flag"]),
        )
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_multi_value_single_arg() {
    let res = Command::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color <color> "some option").num_args(1..))
        .arg(arg!(-n --hostname <name> "another option"))
        .group(ArgGroup::new("grp").args(["hostname", "color", "flag"]))
        .try_get_matches_from(vec!["", "-c", "blue", "red", "green"]);
    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind());

    let m = res.unwrap();
    assert!(m.contains_id("grp"));
    assert_eq!(m.get_one::<Id>("grp").map(|v| v.as_str()).unwrap(), "color");
}

#[test]
fn empty_group() {
    let r = Command::new("empty_group")
        .arg(arg!(-f --flag "some flag"))
        .group(ArgGroup::new("vers").required(true))
        .try_get_matches_from(vec!["empty_prog"]);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
#[cfg(feature = "error-context")]
fn req_group_usage_string() {
    static REQ_GROUP_USAGE: &str = "error: the following required arguments were not provided:
  <base|--delete>

Usage: clap-test <base|--delete>

For more information, try '--help'.
";

    let cmd = Command::new("req_group")
        .arg(arg!([base] "Base commit"))
        .arg(arg!(
            -d --delete "Remove the base commit information"
        ))
        .group(
            ArgGroup::new("base_or_delete")
                .args(["base", "delete"])
                .required(true),
        );

    utils::assert_output(cmd, "clap-test", REQ_GROUP_USAGE, true);
}

#[test]
#[cfg(feature = "error-context")]
fn req_group_with_conflict_usage_string() {
    static REQ_GROUP_CONFLICT_USAGE: &str = "\
error: the argument '--delete' cannot be used with '[base]'

Usage: clap-test <base|--delete>

For more information, try '--help'.
";

    let cmd = Command::new("req_group")
        .arg(arg!([base] "Base commit").conflicts_with("delete"))
        .arg(arg!(
            -d --delete "Remove the base commit information"
        ))
        .group(
            ArgGroup::new("base_or_delete")
                .args(["base", "delete"])
                .required(true),
        );

    utils::assert_output(
        cmd,
        "clap-test --delete base",
        REQ_GROUP_CONFLICT_USAGE,
        true,
    );
}

#[test]
#[cfg(feature = "error-context")]
fn req_group_with_conflict_usage_string_only_options() {
    static REQ_GROUP_CONFLICT_ONLY_OPTIONS: &str = "\
error: the argument '--delete' cannot be used with '--all'

Usage: clap-test <--all|--delete>

For more information, try '--help'.
";

    let cmd = Command::new("req_group")
        .arg(arg!(-a --all "All").conflicts_with("delete"))
        .arg(arg!(
            -d --delete "Remove the base commit information"
        ))
        .group(
            ArgGroup::new("all_or_delete")
                .args(["all", "delete"])
                .required(true),
        );
    utils::assert_output(
        cmd,
        "clap-test --delete --all",
        REQ_GROUP_CONFLICT_ONLY_OPTIONS,
        true,
    );
}

#[test]
fn required_group_multiple_args() {
    let result = Command::new("group")
        .arg(arg!(-f --flag "some flag").action(ArgAction::SetTrue))
        .arg(arg!(-c --color "some other flag").action(ArgAction::SetTrue))
        .group(
            ArgGroup::new("req")
                .args(["flag", "color"])
                .required(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["group", "-f", "-c"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(*m.get_one::<bool>("flag").expect("defaulted by clap"));
    assert!(*m.get_one::<bool>("color").expect("defaulted by clap"));
    assert_eq!(
        &*m.get_many::<Id>("req")
            .unwrap()
            .map(|v| v.as_str())
            .collect::<Vec<_>>(),
        ["flag", "color"]
    );
}

#[test]
fn group_multiple_args_error() {
    let result = Command::new("group")
        .arg(arg!(-f --flag "some flag"))
        .arg(arg!(-c --color "some other flag"))
        .group(ArgGroup::new("req").args(["flag", "color"]))
        .try_get_matches_from(vec!["group", "-f", "-c"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn group_overrides_required() {
    let command = Command::new("group")
        .arg(arg!(--foo <FOO>).required(true))
        .arg(arg!(--bar <BAR>).required(true))
        .group(ArgGroup::new("group").args(["foo", "bar"]).required(true));
    let result = command.try_get_matches_from(vec!["group", "--foo", "value"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(m.contains_id("foo"));
    assert!(!m.contains_id("bar"));
}

#[test]
fn group_usage_use_val_name() {
    static GROUP_USAGE_USE_VAL_NAME: &str = "\
Usage: prog <A>

Arguments:
  [A]  

Options:
  -h, --help  Print help
";
    let cmd = Command::new("prog")
        .arg(Arg::new("a").value_name("A"))
        .group(ArgGroup::new("group").arg("a").required(true));
    utils::assert_output(cmd, "prog --help", GROUP_USAGE_USE_VAL_NAME, false);
}

#[test]
fn group_acts_like_arg() {
    let result = Command::new("prog")
        .arg(
            Arg::new("debug")
                .long("debug")
                .group("mode")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .group("mode")
                .action(ArgAction::SetTrue),
        )
        .try_get_matches_from(vec!["prog", "--debug"]);

    assert!(result.is_ok(), "{}", result.unwrap_err());
    let m = result.unwrap();
    assert!(m.contains_id("mode"));
    assert_eq!(m.get_one::<clap::Id>("mode").unwrap(), "debug");
}

#[test]
fn conflict_with_overlapping_group_in_error() {
    static ERR: &str = "\
error: the argument '--major' cannot be used with '--minor'

Usage: prog --major

For more information, try '--help'.
";

    let cmd = Command::new("prog")
        .group(ArgGroup::new("all").multiple(true))
        .arg(arg!(--major).group("vers").group("all"))
        .arg(arg!(--minor).group("vers").group("all"))
        .arg(arg!(--other).group("all"));

    utils::assert_output(cmd, "prog --major --minor", ERR, true);
}

#[test]
fn requires_group_with_overlapping_group_in_error() {
    static ERR: &str = "\
error: the following required arguments were not provided:
  <--in|--spec>

Usage: prog --config <--in|--spec>

For more information, try '--help'.
";

    let cmd = Command::new("prog")
        .group(ArgGroup::new("all").multiple(true))
        .group(ArgGroup::new("input").required(true))
        .arg(arg!(--in).group("input").group("all"))
        .arg(arg!(--spec).group("input").group("all"))
        .arg(arg!(--config).requires("input").group("all"));

    utils::assert_output(cmd, "prog --config", ERR, true);
}

/* This is used to be fixed in a hack, we need to find a better way to fix it.
#[test]
fn issue_1794() {
    let cmd = clap::Command::new("hello")
        .bin_name("deno")
        .arg(Arg::new("option1").long("option1").action(ArgAction::SetTrue))
        .arg(Arg::new("pos1").action(ArgAction::Set))
        .arg(Arg::new("pos2").action(ArgAction::Set))
        .group(
            ArgGroup::new("arg1")
                .args(["pos1", "option1"])
                .required(true),
        );

    let m = cmd.clone().try_get_matches_from(["cmd", "pos1", "pos2"]).unwrap();
    assert_eq!(m.get_one::<String>("pos1").map(|v| v.as_str()), Some("pos1"));
    assert_eq!(m.get_one::<String>("pos2").map(|v| v.as_str()), Some("pos2"));
    assert!(!*m.get_one::<bool>("option1").expect("defaulted by clap"));

    let m = cmd
        .clone()
        .try_get_matches_from(["cmd", "--option1", "positional"]).unwrap();
    assert_eq!(m.get_one::<String>("pos1").map(|v| v.as_str()), None);
    assert_eq!(m.get_one::<String>("pos2").map(|v| v.as_str()), Some("positional"));
    assert!(*m.get_one::<bool>("option1").expect("defaulted by clap"));
}
*/
