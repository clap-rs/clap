#![cfg(feature = "unstable-dynamic")]

use std::fs;
use std::path::Path;

use clap::{builder::PossibleValue, Command};
use snapbox::assert_data_eq;

macro_rules! complete {
    ($cmd:expr, $input:expr$(, current_dir = $current_dir:expr)? $(,)?) => {
        {
            #[allow(unused)]
            let current_dir: Option<&Path> = None;
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

    assert_data_eq!(
        complete!(cmd, "he"),
        snapbox::str![
            "hello-moon
hello-world
help\tPrint this message or the help of the given subcommand(s)"
        ],
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

    assert_data_eq!(
        complete!(cmd, "--he"),
        snapbox::str![
            "--hello-world
--hello-moon
--help\tPrint help"
        ],
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

    assert_data_eq!(
        complete!(cmd, "hello"),
        snapbox::str![
            "hello-world\tSay hello to the world
hello-moon"
        ],
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

    assert_data_eq!(
        complete!(cmd, "-a"),
        snapbox::str![
            "-aa
-ab
-ac
-ah\tPrint help"
        ],
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

    assert_data_eq!(
        complete!(cmd, "hello-world [TAB]"),
        snapbox::str![
            "--help\tPrint help (see more with '--help')
-h\tPrint help (see more with '--help')
hello-world\tSay hello to the world
hello-moon
goodbye-world"
        ],
    );
}

#[test]
fn suggest_argument_value() {
    let mut cmd = Command::new("dynamic")
        .arg(
            clap::Arg::new("input")
                .long("input")
                .short('i')
                .value_hint(clap::ValueHint::FilePath),
        )
        .arg(
            clap::Arg::new("format")
                .long("format")
                .short('F')
                .value_parser(["json", "yaml", "toml"]),
        )
        .arg(
            clap::Arg::new("count")
                .long("count")
                .short('c')
                .action(clap::ArgAction::Count),
        )
        .args_conflicts_with_subcommands(true);

    let testdir = snapbox::dir::DirRoot::mutable_temp().unwrap();
    let testdir_path = testdir.path().unwrap();

    fs::write(testdir_path.join("a_file"), "").unwrap();
    fs::write(testdir_path.join("b_file"), "").unwrap();
    fs::create_dir_all(testdir_path.join("c_dir")).unwrap();
    fs::create_dir_all(testdir_path.join("d_dir")).unwrap();

    assert_data_eq!(
        complete!(cmd, "--input [TAB]", current_dir = Some(testdir_path)),
        snapbox::str![
            "a_file
b_file
c_dir/
d_dir/"
        ],
    );

    assert_data_eq!(
        complete!(cmd, "-i [TAB]", current_dir = Some(testdir_path)),
        snapbox::str![
            "a_file
b_file
c_dir/
d_dir/"
        ],
    );

    assert_data_eq!(
        complete!(cmd, "--input a[TAB]", current_dir = Some(testdir_path)),
        snapbox::str!["a_file"],
    );

    assert_data_eq!(
        complete!(cmd, "-i b[TAB]", current_dir = Some(testdir_path)),
        snapbox::str!["b_file"],
    );

    assert_data_eq!(
        complete!(cmd, "--format [TAB]"),
        snapbox::str![
            "json
yaml
toml"
        ],
    );

    assert_data_eq!(
        complete!(cmd, "-F [TAB]"),
        snapbox::str![
            "json
yaml
toml"
        ],
    );

    assert_data_eq!(complete!(cmd, "--format j[TAB]"), snapbox::str!["json"],);

    assert_data_eq!(complete!(cmd, "-F j[TAB]"), snapbox::str!["json"],);

    assert_data_eq!(complete!(cmd, "--format t[TAB]"), snapbox::str!["toml"],);

    assert_data_eq!(complete!(cmd, "-F t[TAB]"), snapbox::str!["toml"],);

    assert_data_eq!(
        complete!(cmd, "-chi [TAB]", current_dir = Some(testdir_path)),
        snapbox::str![
            "a_file
b_file
c_dir/
d_dir/"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "-chi a[TAB]", current_dir = Some(testdir_path)),
        snapbox::str!["a_file"],
    );

    assert_data_eq!(
        complete!(cmd, "-chF [TAB]"),
        snapbox::str![
            "json
yaml
toml"
        ]
    );

    assert_data_eq!(complete!(cmd, "-chF j[TAB]"), snapbox::str!["json"]);

    // NOTE: Treat `F` as a value of `-i`, so pressing [TAB] will complete other arguments and subcommands.
    assert_data_eq!(
        complete!(cmd, "-ciF [TAB]"),
        snapbox::str![
            "--input
--format
--count
--help	Print help
-i
-F
-c
-h	Print help"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "-ci[TAB]", current_dir = Some(testdir_path)),
        snapbox::str![
            "-cii
-ciF
-cic
-cih	Print help
-cia_file
-cib_file
-cic_dir/
-cid_dir/"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "-ci=[TAB]", current_dir = Some(testdir_path)),
        snapbox::str![
            "-ci=i
-ci=F
-ci=c
-ci=h	Print help
-ci=a_file
-ci=b_file
-ci=c_dir/
-ci=d_dir/"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "-ci=a[TAB]", current_dir = Some(testdir_path)),
        snapbox::str![
            "-ci=ai
-ci=aF
-ci=ac
-ci=ah	Print help
-ci=a_file"
        ]
    )
}

fn complete(cmd: &mut Command, args: impl AsRef<str>, current_dir: Option<&Path>) -> String {
    let input = args.as_ref();
    let mut args = vec![std::ffi::OsString::from(cmd.get_name())];
    let arg_index;

    if let Some((prior, after)) = input.split_once("[TAB]") {
        args.extend(prior.split_whitespace().map(From::from));
        if prior.ends_with(char::is_whitespace) {
            args.push(std::ffi::OsString::default());
        }
        arg_index = args.len() - 1;
        // HACK: this cannot handle in-word '[TAB]'
        args.extend(after.split_whitespace().map(From::from));
    } else {
        args.extend(input.split_whitespace().map(From::from));
        if input.ends_with(char::is_whitespace) {
            args.push(std::ffi::OsString::default());
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
