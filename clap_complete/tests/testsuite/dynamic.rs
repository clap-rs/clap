#![cfg(feature = "unstable-dynamic")]

use std::{ffi::OsString, iter};

use clap_complete::dynamic::ShowOptions;

fn assert_complete<'a>(
    cmd: &mut clap::Command,
    args: impl IntoIterator<Item = &'a str>,
    expected: impl AsRef<[&'a str]>,
    options: ShowOptions,
) {
    let args: Vec<_> = iter::once(OsString::from(cmd.get_name()))
        .chain(args.into_iter().map(OsString::from))
        .collect();
    let arg_index = args.len() - 1;
    let current_dir = None;

    let completions =
        clap_complete::dynamic::complete(cmd, args, arg_index, current_dir, options).unwrap();

    let completions = completions
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    assert_eq!(completions, expected.as_ref());
}

#[test]
fn suggest_subcommand_subset() {
    let mut cmd = clap::Command::new("exhaustive")
        .subcommand(clap::Command::new("hello-world"))
        .subcommand(clap::Command::new("hello-moon"))
        .subcommand(clap::Command::new("goodbye-world"));

    assert_complete(
        &mut cmd,
        ["he"],
        ["hello-moon", "hello-world", "help"],
        ShowOptions::Always,
    );
}

#[test]
fn suggest_long_flag_subset() {
    let mut cmd = clap::Command::new("exhaustive")
        .arg(
            clap::Arg::new("hello-world")
                .long("hello-world")
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("hello-moon")
                .long("hello-moon")
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("goodbye-world")
                .long("goodbye-world")
                .action(clap::ArgAction::Count),
        );

    assert_complete(
        &mut cmd,
        ["--he"],
        ["--hello-world", "--hello-moon", "--help"],
        ShowOptions::Always,
    );
}

#[test]
fn suggest_possible_value_subset() {
    let mut cmd =
        clap::Command::new("exhaustive").arg(clap::Arg::new("hello-world").value_parser([
            "hello-world",
            "hello-moon",
            "goodbye-world",
        ]));

    assert_complete(
        &mut cmd,
        ["hello"],
        ["hello-world", "hello-moon"],
        ShowOptions::Always,
    );
}

#[test]
fn suggest_additional_short_flags() {
    let mut cmd = clap::Command::new("exhaustive")
        .arg(
            clap::Arg::new("a")
                .short('a')
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("b")
                .short('b')
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("c")
                .short('c')
                .action(clap::ArgAction::Count),
        );
    assert_complete(
        &mut cmd,
        ["-a"],
        ["-aa", "-ab", "-ac", "-ah"],
        ShowOptions::Always,
    );
}

#[test]
fn suggest_flags_only_on_dash() {
    let mut cmd = clap::Command::new("exhaustive")
        .arg(clap::Arg::new("a").short('a'))
        .arg(clap::Arg::new("b").long("b"));

    assert_complete(&mut cmd, [""], [], ShowOptions::ExactDash);
    assert_complete(&mut cmd, ["-"], ["-a", "-h"], ShowOptions::ExactDash);
    assert_complete(&mut cmd, ["--"], ["--b", "--help"], ShowOptions::ExactDash);

    assert_complete(
        &mut cmd,
        ["-"],
        ["--b", "--help", "-a", "-h"],
        ShowOptions::MinOneDash,
    );

    assert_complete(
        &mut cmd,
        [""],
        ["--b", "--help", "-a", "-h"],
        ShowOptions::Always,
    );
}
