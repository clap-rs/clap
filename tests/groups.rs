mod utils;

use clap::{App, Arg, ArgGroup, ErrorKind};

static REQ_GROUP_USAGE: &str = "error: The following required arguments were not provided:
    <base|--delete>

USAGE:
    clap-test <base|--delete>

For more information try --help";

static REQ_GROUP_CONFLICT_USAGE: &str =
    "error: The argument '<base>' cannot be used with '--delete'

USAGE:
    clap-test <base|--delete>

For more information try --help";

#[allow(unused)]
static REQ_GROUP_CONFLICT_REV: &str = "error: The argument '--delete' cannot be used with '<base>'

USAGE:
    clap-test <base|--delete>

For more information try --help";

static REQ_GROUP_CONFLICT_ONLY_OPTIONS: &str =
    "error: Found argument '--all' which wasn't expected, or isn't valid in this context

If you tried to supply `--all` as a PATTERN use `-- --all`

USAGE:
    clap-test <-a|--delete>

For more information try --help";

// FIXME: This message has regressed after https://github.com/clap-rs/clap/pull/1856
//        Need to roll back somehow.
static REQ_GROUP_CONFLICT_REV_DEGRADED: &str =
    "error: Found argument 'base' which wasn't expected, or isn't valid in this context

If you tried to supply `base` as a PATTERN use `-- base`

USAGE:
    clap-test <base|--delete>

For more information try --help";

#[test]
fn required_group_missing_arg() {
    let result = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg(" -c, --color 'some other flag'")
        .group(
            ArgGroup::with_name("req")
                .args(&["flag", "color"])
                .required(true),
        )
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument group 'req' contains non-existent argument"]
fn non_existing_arg() {
    let _ = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color 'some other flag'")
        .group(
            ArgGroup::with_name("req")
                .args(&["flg", "color"])
                .required(true),
        )
        .try_get_matches_from(vec![""]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument group name must be unique\n\n\t'req' is already in use"]
fn unique_group_name() {
    let _ = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color 'some other flag'")
        .group(ArgGroup::with_name("req").args(&["flag"]).required(true))
        .group(ArgGroup::with_name("req").args(&["color"]).required(true))
        .try_get_matches_from(vec![""]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument group name '' must not conflict with argument name"]
fn groups_with_name_of_arg_name() {
    let _ = App::new("group")
        .arg(Arg::with_name("a").long("a").group("a"))
        .try_get_matches_from(vec!["", "--a"]);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument group name 'a' must not conflict with argument name"]
fn arg_group_with_name_of_arg_name() {
    let _ = App::new("group")
        .arg(Arg::with_name("a").long("a").group("a"))
        .group(ArgGroup::with_name("a"))
        .try_get_matches_from(vec!["", "--a"]);
}

#[test]
fn group_single_value() {
    let res = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color [color] 'some option'")
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec!["", "-c", "blue"]);
    assert!(res.is_ok());

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert_eq!(m.value_of("grp").unwrap(), "blue");
}

#[test]
fn group_single_flag() {
    let res = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color [color] 'some option'")
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec!["", "-f"]);
    assert!(res.is_ok());

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_empty() {
    let res = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color [color] 'some option'")
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec![""]);
    assert!(res.is_ok());

    let m = res.unwrap();
    assert!(!m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_reqired_flags_empty() {
    let result = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color 'some option'")
        .group(
            ArgGroup::with_name("grp")
                .required(true)
                .args(&["flag", "color"]),
        )
        .try_get_matches_from(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_multi_value_single_arg() {
    let res = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color [color]... 'some option'")
        .group(ArgGroup::with_name("grp").args(&["flag", "color"]))
        .try_get_matches_from(vec!["", "-c", "blue", "red", "green"]);
    assert!(res.is_ok(), "{:?}", res.unwrap_err().kind);

    let m = res.unwrap();
    assert!(m.is_present("grp"));
    assert_eq!(
        &*m.values_of("grp").unwrap().collect::<Vec<_>>(),
        &["blue", "red", "green"]
    );
}

#[test]
fn empty_group() {
    let r = App::new("empty_group")
        .arg(Arg::from("-f, --flag 'some flag'"))
        .group(ArgGroup::with_name("vers").required(true))
        .try_get_matches_from(vec!["empty_prog"]);
    assert!(r.is_err());
    let err = r.err().unwrap();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn req_group_usage_string() {
    let app = App::new("req_group")
        .arg("[base] 'Base commit'")
        .arg("-d, --delete 'Remove the base commit information'")
        .group(
            ArgGroup::with_name("base_or_delete")
                .args(&["base", "delete"])
                .required(true),
        );

    assert!(utils::compare_output(
        app,
        "clap-test",
        REQ_GROUP_USAGE,
        true
    ));
}

#[test]
fn req_group_with_conflict_usage_string() {
    let app = App::new("req_group")
        .arg(Arg::from("[base] 'Base commit'").conflicts_with("delete"))
        .arg(Arg::from(
            "-d, --delete 'Remove the base commit information'",
        ))
        .group(
            ArgGroup::with_name("base_or_delete")
                .args(&["base", "delete"])
                .required(true),
        );

    assert!(utils::compare_output2(
        app,
        "clap-test --delete base",
        REQ_GROUP_CONFLICT_REV_DEGRADED,
        REQ_GROUP_CONFLICT_USAGE,
        true
    ));
}

#[test]
fn req_group_with_conflict_usage_string_only_options() {
    let app = App::new("req_group")
        .arg(Arg::from("<all> -a, -all 'All'").conflicts_with("delete"))
        .arg(Arg::from(
            "<delete> -d, --delete 'Remove the base commit information'",
        ))
        .group(
            ArgGroup::with_name("all_or_delete")
                .args(&["all", "delete"])
                .required(true),
        );
    assert!(utils::compare_output2(
        app,
        "clap-test --delete --all",
        REQ_GROUP_CONFLICT_ONLY_OPTIONS,
        REQ_GROUP_CONFLICT_USAGE,
        true
    ));
}

#[test]
fn required_group_multiple_args() {
    let result = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color 'some other flag'")
        .group(
            ArgGroup::with_name("req")
                .args(&["flag", "color"])
                .required(true)
                .multiple(true),
        )
        .try_get_matches_from(vec!["group", "-f", "-c"]);
    assert!(result.is_ok());
    let m = result.unwrap();
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn group_multiple_args_error() {
    let result = App::new("group")
        .arg("-f, --flag 'some flag'")
        .arg("-c, --color 'some other flag'")
        .group(ArgGroup::with_name("req").args(&["flag", "color"]))
        .try_get_matches_from(vec!["group", "-f", "-c"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn group_acts_like_arg() {
    let m = App::new("prog")
        .arg(Arg::with_name("debug").long("debug").group("mode"))
        .arg(Arg::with_name("verbose").long("verbose").group("mode"))
        .get_matches_from(vec!["prog", "--debug"]);
    assert!(m.is_present("mode"));
}

#[test]
fn issue_1794() {
    let app = clap::App::new("hello")
        .bin_name("deno")
        .arg(Arg::with_name("option1").long("option1").takes_value(false))
        .arg(Arg::with_name("pos1").takes_value(true))
        .arg(Arg::with_name("pos2").takes_value(true))
        .group(
            ArgGroup::with_name("arg1")
                .args(&["pos1", "option1"])
                .required(true),
        );

    let m = app.clone().get_matches_from(&["app", "pos1", "pos2"]);
    assert_eq!(m.value_of("pos1"), Some("pos1"));
    assert_eq!(m.value_of("pos2"), Some("pos2"));
    assert!(!m.is_present("option1"));

    let m = app
        .clone()
        .get_matches_from(&["app", "--option1", "positional"]);
    assert_eq!(m.value_of("pos1"), None);
    assert_eq!(m.value_of("pos2"), Some("positional"));
    assert!(m.is_present("option1"));
}
