#![cfg(feature = "unstable-dynamic")]

use std::path::Path;

use clap::{builder::PossibleValue, Command};

macro_rules! complete {
    ($cmd:expr, $input:expr$(, current_dir = $current_dir:expr)? $(,)?) => {
        {
            #[allow(unused)]
            let current_dir = None;
            $(let current_dir = $current_dir;)?
            complete(&mut $cmd, $input, current_dir)
        }
    }
}

#[test]
fn suggest_subcommand_subset() {
    let mut cmd = Command::new("exhaustive")
        .subcommand(Command::new("hello-world"))
        .subcommand(Command::new("hello-moon"))
        .subcommand(Command::new("goodbye-world"));

    snapbox::assert_eq(
        "hello-moon
hello-world
help\tPrint this message or the help of the given subcommand(s)",
        complete!(cmd, "he"),
    );
}

#[test]
fn suggest_long_flag_subset() {
    let mut cmd = Command::new("exhaustive")
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

    snapbox::assert_eq(
        "--hello-world
--hello-moon
--help\tPrint help",
        complete!(cmd, "--he"),
    );
}

#[test]
fn suggest_possible_value_subset() {
    let name = "exhaustive";
    let mut cmd = Command::new(name).arg(clap::Arg::new("hello-world").value_parser([
        PossibleValue::new("hello-world").help("Say hello to the world"),
        "hello-moon".into(),
        "goodbye-world".into(),
    ]));

    snapbox::assert_eq(
        "hello-world\tSay hello to the world
hello-moon",
        complete!(cmd, "hello"),
    );
}

#[test]
fn suggest_additional_short_flags() {
    let mut cmd = Command::new("exhaustive")
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

    snapbox::assert_eq(
        "-aa
-ab
-ac
-ah\tPrint help",
        complete!(cmd, "-a"),
    );
}

#[test]
fn suggest_subcommand_positional() {
    let mut cmd = Command::new("exhaustive").subcommand(Command::new("hello-world").arg(
        clap::Arg::new("hello-world").value_parser([
            PossibleValue::new("hello-world").help("Say hello to the world"),
            "hello-moon".into(),
            "goodbye-world".into(),
        ]),
    ));

    snapbox::assert_eq(
        "--help\tPrint help (see more with '--help')
-h\tPrint help (see more with '--help')
hello-world\tSay hello to the world
hello-moon
goodbye-world",
        complete!(cmd, "hello-world [TAB]"),
    );
}

fn complete(cmd: &mut Command, args: impl AsRef<str>, current_dir: Option<&Path>) -> String {
    let input = args.as_ref();
    let mut args = vec![std::ffi::OsString::from(cmd.get_name())];
    let arg_index;

    if let Some((prior, after)) = input.split_once("[TAB]") {
        args.extend(prior.split_whitespace().map(From::from));
        if prior.ends_with(char::is_whitespace) {
            args.push(std::ffi::OsString::default())
        }
        arg_index = args.len() - 1;
        // HACK: this cannot handle in-word '[TAB]'
        args.extend(after.split_whitespace().map(From::from));
    } else {
        args.extend(input.split_whitespace().map(From::from));
        if input.ends_with(char::is_whitespace) {
            args.push(std::ffi::OsString::default())
        }
        arg_index = args.len() - 1;
    }

    clap_complete::dynamic::complete(cmd, args, arg_index, current_dir)
        .unwrap()
        .into_iter()
        .map(|(compl, help)| {
            let compl = compl.to_str().unwrap();
            if let Some(help) = help {
                format!("{compl}\t{help}")
            } else {
                compl.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
