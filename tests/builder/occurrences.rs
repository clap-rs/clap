#![cfg(feature = "unstable-grouped")]

use clap::{Arg, ArgAction, ArgMatches, Command};

fn occurrences_as_vec_vec<'a>(m: &'a ArgMatches, name: &str) -> Vec<Vec<&'a String>> {
    m.get_occurrences(name)
        .unwrap()
        .map(Iterator::collect)
        .collect()
}

#[test]
fn grouped_value_works() {
    let m = Command::new("cli")
        .arg(
            Arg::new("option")
                .long("option")
                .action(ArgAction::Set)
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .try_get_matches_from([
            "cli",
            "--option",
            "fr_FR:mon option 1",
            "en_US:my option 1",
            "--option",
            "fr_FR:mon option 2",
            "en_US:my option 2",
        ])
        .unwrap();
    let grouped_vals = occurrences_as_vec_vec(&m, "option");
    assert_eq!(
        grouped_vals,
        vec![
            vec!["fr_FR:mon option 1", "en_US:my option 1",],
            vec!["fr_FR:mon option 2", "en_US:my option 2",],
        ]
    );
}

#[test]
fn issue_1026() {
    let m = Command::new("cli")
        .arg(Arg::new("server").short('s').action(ArgAction::Set))
        .arg(Arg::new("user").short('u').action(ArgAction::Set))
        .arg(
            Arg::new("target")
                .long("target")
                .action(ArgAction::Set)
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .try_get_matches_from([
            "backup", "-s", "server", "-u", "user", "--target", "target1", "file1", "file2",
            "file3", "--target", "target2", "file4", "file5", "file6", "file7", "--target",
            "target3", "file8",
        ])
        .unwrap();
    let grouped_vals = occurrences_as_vec_vec(&m, "target");
    assert_eq!(
        grouped_vals,
        vec![
            vec!["target1", "file1", "file2", "file3"],
            vec!["target2", "file4", "file5", "file6", "file7",],
            vec!["target3", "file8"]
        ]
    );
}

#[test]
fn grouped_value_long_flag_delimiter() {
    let m = Command::new("myapp")
        .arg(
            Arg::new("option")
                .long("option")
                .action(ArgAction::Set)
                .value_delimiter(',')
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec![
            "myapp",
            "--option=hmm",
            "--option=val1,val2,val3",
            "--option",
            "alice,bob",
        ])
        .unwrap();
    let grouped_vals = occurrences_as_vec_vec(&m, "option");
    assert_eq!(
        grouped_vals,
        vec![
            vec!["hmm"],
            vec!["val1", "val2", "val3"],
            vec!["alice", "bob"]
        ]
    );
}

#[test]
fn grouped_value_short_flag_delimiter() {
    let m = Command::new("myapp")
        .arg(
            Arg::new("option")
                .short('o')
                .action(ArgAction::Set)
                .value_delimiter(',')
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["myapp", "-o=foo", "-o=val1,val2,val3", "-o=bar"])
        .unwrap();
    let grouped_vals = occurrences_as_vec_vec(&m, "option");
    assert_eq!(
        grouped_vals,
        vec![vec!["foo"], vec!["val1", "val2", "val3"], vec!["bar"]]
    );
}

#[test]
fn grouped_value_positional_arg() {
    let m = Command::new("multiple_values")
        .arg(
            Arg::new("pos")
                .help("multiple positionals")
                .action(ArgAction::Set)
                .num_args(1..),
        )
        .try_get_matches_from(vec![
            "myprog", "val1", "val2", "val3", "val4", "val5", "val6",
        ])
        .unwrap();
    let grouped_vals = occurrences_as_vec_vec(&m, "pos");
    assert_eq!(
        grouped_vals,
        vec![vec!["val1", "val2", "val3", "val4", "val5", "val6"]]
    );
}

#[test]
fn grouped_value_multiple_positional_arg() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos1").help("multiple positionals"))
        .arg(
            Arg::new("pos2")
                .help("multiple positionals")
                .action(ArgAction::Set)
                .num_args(1..),
        )
        .try_get_matches_from(vec![
            "myprog", "val1", "val2", "val3", "val4", "val5", "val6",
        ])
        .unwrap();
    let grouped_vals = occurrences_as_vec_vec(&m, "pos2");
    assert_eq!(
        grouped_vals,
        vec![vec!["val2", "val3", "val4", "val5", "val6"]]
    );
}

#[test]
fn grouped_value_multiple_positional_arg_last_multiple() {
    let m = Command::new("multiple_values")
        .arg(Arg::new("pos1").help("multiple positionals"))
        .arg(
            Arg::new("pos2")
                .help("multiple positionals")
                .action(ArgAction::Set)
                .num_args(1..)
                .last(true),
        )
        .try_get_matches_from(vec![
            "myprog", "val1", "--", "val2", "val3", "val4", "val5", "val6",
        ])
        .unwrap();
    let grouped_vals = occurrences_as_vec_vec(&m, "pos2");
    assert_eq!(
        grouped_vals,
        vec![vec!["val2", "val3", "val4", "val5", "val6"]]
    );
}

#[test]
fn grouped_interleaved_positional_values() {
    let cmd = clap::Command::new("foo")
        .arg(clap::Arg::new("pos").num_args(1..))
        .arg(
            clap::Arg::new("flag")
                .short('f')
                .long("flag")
                .action(ArgAction::Set)
                .action(ArgAction::Append),
        );

    let m = cmd
        .try_get_matches_from(["foo", "1", "2", "-f", "a", "3", "-f", "b", "4"])
        .unwrap();

    let pos = occurrences_as_vec_vec(&m, "pos");
    assert_eq!(pos, vec![vec!["1", "2"], vec!["3"], vec!["4"]]);

    let flag = occurrences_as_vec_vec(&m, "flag");
    assert_eq!(flag, vec![vec!["a"], vec!["b"]]);
}

#[test]
fn grouped_interleaved_positional_occurrences() {
    let cmd = clap::Command::new("foo")
        .arg(clap::Arg::new("pos").num_args(1..))
        .arg(
            clap::Arg::new("flag")
                .short('f')
                .long("flag")
                .action(ArgAction::Set)
                .action(ArgAction::Append),
        );

    let m = cmd
        .try_get_matches_from(["foo", "1", "2", "-f", "a", "3", "-f", "b", "4"])
        .unwrap();

    let pos = occurrences_as_vec_vec(&m, "pos");
    assert_eq!(pos, vec![vec!["1", "2"], vec!["3"], vec!["4"]]);

    let flag = occurrences_as_vec_vec(&m, "flag");
    assert_eq!(flag, vec![vec!["a"], vec!["b"]]);
}

#[test]
fn issue_2171() {
    let schema = Command::new("ripgrep#1701 reproducer")
        .args_override_self(true)
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("search_zip")
                .short('z')
                .long("search-zip")
                .action(ArgAction::SetTrue),
        );

    let test_args = [
        vec!["reproducer", "-pz", "-p"],
        vec!["reproducer", "-pzp"],
        vec!["reproducer", "-zpp"],
        vec!["reproducer", "-pp", "-z"],
        vec!["reproducer", "-p", "-p", "-z"],
        vec!["reproducer", "-p", "-pz"],
        vec!["reproducer", "-ppz"],
    ];

    for argv in test_args {
        let _ = schema.clone().try_get_matches_from(argv).unwrap();
    }
}
