#![cfg(feature = "unstable-grouped")]

use clap::{Arg, ArgAction, Command};

#[test]
fn grouped_value_works() {
    let m = Command::new("cli")
        .arg(
            Arg::new("option")
                .long("option")
                .takes_value(true)
                .multiple_values(true)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(&[
            "cli",
            "--option",
            "fr_FR:mon option 1",
            "en_US:my option 1",
            "--option",
            "fr_FR:mon option 2",
            "en_US:my option 2",
        ])
        .unwrap();
    let grouped_vals: Vec<_> = m.grouped_values_of("option").unwrap().collect();
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
        .arg(Arg::new("server").short('s').takes_value(true))
        .arg(Arg::new("user").short('u').takes_value(true))
        .arg(
            Arg::new("target")
                .long("target")
                .takes_value(true)
                .multiple_values(true)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(&[
            "backup", "-s", "server", "-u", "user", "--target", "target1", "file1", "file2",
            "file3", "--target", "target2", "file4", "file5", "file6", "file7", "--target",
            "target3", "file8",
        ])
        .unwrap();
    let grouped_vals: Vec<_> = m.grouped_values_of("target").unwrap().collect();
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
                .takes_value(true)
                .use_value_delimiter(true)
                .multiple_values(true)
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
    let grouped_vals: Vec<_> = m.grouped_values_of("option").unwrap().collect();
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
                .takes_value(true)
                .use_value_delimiter(true)
                .multiple_values(true)
                .action(ArgAction::Append),
        )
        .try_get_matches_from(vec!["myapp", "-o=foo", "-o=val1,val2,val3", "-o=bar"])
        .unwrap();
    let grouped_vals: Vec<_> = m.grouped_values_of("option").unwrap().collect();
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
                .takes_value(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec![
            "myprog", "val1", "val2", "val3", "val4", "val5", "val6",
        ])
        .unwrap();
    let grouped_vals: Vec<_> = m.grouped_values_of("pos").unwrap().collect();
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
                .takes_value(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec![
            "myprog", "val1", "val2", "val3", "val4", "val5", "val6",
        ])
        .unwrap();
    let grouped_vals: Vec<_> = m.grouped_values_of("pos2").unwrap().collect();
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
                .takes_value(true)
                .multiple_values(true)
                .last(true),
        )
        .try_get_matches_from(vec![
            "myprog", "val1", "--", "val2", "val3", "val4", "val5", "val6",
        ])
        .unwrap();
    let grouped_vals: Vec<_> = m.grouped_values_of("pos2").unwrap().collect();
    assert_eq!(
        grouped_vals,
        vec![vec!["val2", "val3", "val4", "val5", "val6"]]
    );
}

#[test]
fn grouped_interleaved_positional_values() {
    let cmd = clap::Command::new("foo")
        .arg(clap::Arg::new("pos").multiple_values(true))
        .arg(
            clap::Arg::new("flag")
                .short('f')
                .long("flag")
                .takes_value(true)
                .action(ArgAction::Append),
        );

    let m = cmd
        .try_get_matches_from(["foo", "1", "2", "-f", "a", "3", "-f", "b", "4"])
        .unwrap();

    let pos: Vec<_> = m.grouped_values_of("pos").unwrap().collect();
    assert_eq!(pos, vec![vec!["1", "2", "3", "4"]]);

    let flag: Vec<_> = m.grouped_values_of("flag").unwrap().collect();
    assert_eq!(flag, vec![vec!["a"], vec!["b"]]);
}

#[test]
fn grouped_interleaved_positional_occurrences() {
    let cmd = clap::Command::new("foo")
        .arg(clap::Arg::new("pos").multiple_values(true))
        .arg(
            clap::Arg::new("flag")
                .short('f')
                .long("flag")
                .takes_value(true)
                .action(ArgAction::Append),
        );

    let m = cmd
        .try_get_matches_from(["foo", "1", "2", "-f", "a", "3", "-f", "b", "4"])
        .unwrap();

    let pos: Vec<_> = m.grouped_values_of("pos").unwrap().collect();
    assert_eq!(pos, vec![vec!["1", "2", "3", "4"]]);

    let flag: Vec<_> = m.grouped_values_of("flag").unwrap().collect();
    assert_eq!(flag, vec![vec!["a"], vec!["b"]]);
}

#[test]
fn issue_1374() {
    let cmd = Command::new("MyApp").arg(
        Arg::new("input")
            .takes_value(true)
            .long("input")
            .overrides_with("input")
            .min_values(0)
            .action(ArgAction::Append),
    );
    let matches = cmd
        .clone()
        .try_get_matches_from(&["MyApp", "--input", "a", "b", "c", "--input", "d"])
        .unwrap();
    let vs = matches
        .get_many::<String>("input")
        .unwrap()
        .map(|v| v.as_str());
    assert_eq!(vs.collect::<Vec<_>>(), vec!["a", "b", "c", "d"]);
    let matches = cmd
        .clone()
        .try_get_matches_from(&["MyApp", "--input", "a", "b", "--input", "c", "d"])
        .unwrap();
    let vs = matches
        .get_many::<String>("input")
        .unwrap()
        .map(|v| v.as_str());
    assert_eq!(vs.collect::<Vec<_>>(), vec!["a", "b", "c", "d"]);
}

#[test]
fn issue_2171_deprecated() {
    #![allow(deprecated)]
    let schema = Command::new("ripgrep#1701 reproducer")
        .args_override_self(true)
        .arg(Arg::new("pretty").short('p').long("pretty"))
        .arg(Arg::new("search_zip").short('z').long("search-zip"));

    let test_args = &[
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

#[test]
fn issue_2171() {
    let schema = Command::new("ripgrep#1701 reproducer")
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

    let test_args = &[
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
