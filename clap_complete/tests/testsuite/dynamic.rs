#![cfg(feature = "unstable-dynamic")]

use std::fs;
use std::path::Path;

use clap::{builder::PossibleValue, Command};
use clap_complete::dynamic::{ArgValueCompleter, CompletionCandidate, CustomCompleter};
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

    assert_data_eq!(complete!(cmd, "he"), snapbox::str![[r#"
hello-moon
hello-world
help	Print this message or the help of the given subcommand(s)
"#]],);
}

#[test]
fn suggest_hidden_long_flags() {
    let mut cmd = Command::new("exhaustive")
        .arg(clap::Arg::new("hello-world-visible").long("hello-world-visible"))
        .arg(
            clap::Arg::new("hello-world-hidden")
                .long("hello-world-hidden")
                .hide(true),
        );

    assert_data_eq!(complete!(cmd, "--hello-world"), snapbox::str!["--hello-world-visible"]);

    assert_data_eq!(complete!(cmd, "--hello-world-h"), snapbox::str!["--hello-world-hidden"]);
}

#[test]
fn suggest_hidden_subcommand_and_aliases() {
    let mut cmd = Command::new("exhaustive")
        .subcommand(
            Command::new("test_visible")
                .visible_alias("test_visible-alias_visible")
                .alias("test_visible-alias_hidden"),
        )
        .subcommand(
            Command::new("test_hidden")
                .visible_alias("test_hidden-alias_visible")
                .alias("test_hidden-alias_hidden")
                .hide(true),
        );

    assert_data_eq!(complete!(cmd, "test"), snapbox::str![[r#"
test_visible
test_visible-alias_visible
"#]]);

    assert_data_eq!(complete!(cmd, "test_h"), snapbox::str![[r#"
test_hidden
test_hidden-alias_hidden
test_hidden-alias_visible
"#]]);

    assert_data_eq!(complete!(cmd, "test_hidden-alias_h"), snapbox::str!["test_hidden-alias_hidden"]);
}

#[test]
fn suggest_subcommand_aliases() {
    let mut cmd = Command::new("exhaustive")
        .subcommand(
            Command::new("hello-world")
                .visible_alias("hello-world-foo")
                .alias("hidden-world"),
        )
        .subcommand(
            Command::new("hello-moon")
                .visible_alias("hello-moon-foo")
                .alias("hidden-moon"),
        )
        .subcommand(
            Command::new("goodbye-world")
                .visible_alias("goodbye-world-foo")
                .alias("hidden-goodbye"),
        );

    assert_data_eq!(complete!(cmd, "hello"), snapbox::str![[r#"
hello-moon
hello-moon-foo
hello-world
hello-world-foo
"#]],);
}

#[test]
fn suggest_hidden_possible_value() {
    let mut cmd = Command::new("exhaustive").arg(
        clap::Arg::new("possible_value").long("test").value_parser([
            PossibleValue::new("test-visible").help("Say hello to the world"),
            PossibleValue::new("test-hidden")
                .help("Say hello to the moon")
                .hide(true),
        ]),
    );

    assert_data_eq!(complete!(cmd, "--test=test"), snapbox::str!["--test=test-visible	Say hello to the world"]);

    assert_data_eq!(complete!(cmd, "--test=test-h"), snapbox::str!["--test=test-hidden	Say hello to the moon"]);
}

#[test]
fn suggest_hidden_long_flag_aliases() {
    let mut cmd = Command::new("exhaustive")
        .arg(
            clap::Arg::new("test_visible")
                .long("test_visible")
                .visible_alias("test_visible-alias_visible")
                .alias("test_visible-alias_hidden"),
        )
        .arg(
            clap::Arg::new("test_hidden")
                .long("test_hidden")
                .visible_alias("test_hidden-alias_visible")
                .alias("test_hidden-alias_hidden")
                .hide(true),
        );

    assert_data_eq!(complete!(cmd, "--test"), snapbox::str![[r#"
--test_visible
--test_visible-alias_visible
"#]]);

    assert_data_eq!(complete!(cmd, "--test_h"), snapbox::str![[r#"
--test_hidden
--test_hidden-alias_visible
--test_hidden-alias_hidden
"#]]);

    assert_data_eq!(complete!(cmd, "--test_visible-alias_h"), snapbox::str!["--test_visible-alias_hidden"]);

    assert_data_eq!(complete!(cmd, "--test_hidden-alias_h"), snapbox::str!["--test_hidden-alias_hidden"]);
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

    assert_data_eq!(complete!(cmd, "--he"), snapbox::str![[r#"
--hello-world
--hello-moon
--help	Print help
"#]],);
}

#[test]
fn suggest_possible_value_subset() {
    let name = "exhaustive";
    let mut cmd = Command::new(name).arg(clap::Arg::new("hello-world").value_parser([
        PossibleValue::new("hello-world").help("Say hello to the world"),
        "hello-moon".into(),
        "goodbye-world".into(),
    ]));

    assert_data_eq!(complete!(cmd, "hello"), snapbox::str![[r#"
hello-world	Say hello to the world
hello-moon
"#]],);
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

    assert_data_eq!(complete!(cmd, "-a"), snapbox::str![[r#"
-aa
-ab
-ac
-ah	Print help
"#]],);
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

    assert_data_eq!(complete!(cmd, "hello-world [TAB]"), snapbox::str![[r#"
--help	Print help (see more with '--help')
-h	Print help (see more with '--help')
hello-world	Say hello to the world
hello-moon
goodbye-world
"#]],);
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
        .arg(clap::Arg::new("positional").value_parser(["pos_a", "pos_b", "pos_c"]))
        .args_conflicts_with_subcommands(true);

    let testdir = snapbox::dir::DirRoot::mutable_temp().unwrap();
    let testdir_path = testdir.path().unwrap();

    fs::write(testdir_path.join("a_file"), "").unwrap();
    fs::write(testdir_path.join("b_file"), "").unwrap();
    fs::create_dir_all(testdir_path.join("c_dir")).unwrap();
    fs::create_dir_all(testdir_path.join("d_dir")).unwrap();

    assert_data_eq!(
        complete!(cmd, "--input [TAB]", current_dir = Some(testdir_path)),
        snapbox::str![[r#"
a_file
b_file
c_dir/
d_dir/
"#]],
    );

    assert_data_eq!(
        complete!(cmd, "-i [TAB]", current_dir = Some(testdir_path)),
        snapbox::str![[r#"
a_file
b_file
c_dir/
d_dir/
"#]],
    );

    assert_data_eq!(
        complete!(cmd, "--input a[TAB]", current_dir = Some(testdir_path)),
        snapbox::str!["a_file"],
    );

    assert_data_eq!(
        complete!(cmd, "-i b[TAB]", current_dir = Some(testdir_path)),
        snapbox::str!["b_file"],
    );

    assert_data_eq!(complete!(cmd, "--format [TAB]"), snapbox::str![[r#"
json
yaml
toml
"#]],);

    assert_data_eq!(complete!(cmd, "-F [TAB]"), snapbox::str![[r#"
json
yaml
toml
"#]],);

    assert_data_eq!(complete!(cmd, "--format j[TAB]"), snapbox::str!["json"],);

    assert_data_eq!(complete!(cmd, "-F j[TAB]"), snapbox::str!["json"],);

    assert_data_eq!(complete!(cmd, "--format t[TAB]"), snapbox::str!["toml"],);

    assert_data_eq!(complete!(cmd, "-F t[TAB]"), snapbox::str!["toml"],);

    assert_data_eq!(complete!(cmd, "-cccF [TAB]"), snapbox::str![[r#"
json
yaml
toml
"#]]);

    assert_data_eq!(complete!(cmd, "--input a_file [TAB]"), snapbox::str![[r#"
--input
--format
--count
--help	Print help
-i
-F
-c
-h	Print help
pos_a
pos_b
pos_c
"#]]);

    assert_data_eq!(
        complete!(cmd, "-ci[TAB]", current_dir = Some(testdir_path)),
        snapbox::str![[r#"
-cia_file
-cib_file
-cic_dir/
-cid_dir/
"#]]
    );

    assert_data_eq!(
        complete!(cmd, "-ci=[TAB]", current_dir = Some(testdir_path)),
        snapbox::str![[r#"
-ci=a_file
-ci=b_file
-ci=c_dir/
-ci=d_dir/
"#]]
    );

    assert_data_eq!(
        complete!(cmd, "-ci=a[TAB]", current_dir = Some(testdir_path)),
        snapbox::str!["-ci=a_file"]
    );

    assert_data_eq!(
        complete!(cmd, "-ciF[TAB]", current_dir = Some(testdir_path)),
        snapbox::str![]
    );

    assert_data_eq!(
        complete!(cmd, "-ciF=[TAB]", current_dir = Some(testdir_path)),
        snapbox::str![]
    );
}

#[test]
fn suggest_argument_multi_values() {
    let mut cmd = Command::new("dynamic")
        .arg(
            clap::Arg::new("certain-num")
                .long("certain-num")
                .short('Y')
                .value_parser(["val1", "val2", "val3"])
                .num_args(3),
        )
        .arg(
            clap::Arg::new("uncertain-num")
                .long("uncertain-num")
                .short('N')
                .value_parser(["val1", "val2", "val3"])
                .num_args(1..=3),
        );

    assert_data_eq!(complete!(cmd, "--certain-num [TAB]"), snapbox::str![[r#"
val1
val2
val3
"#]]);

    assert_data_eq!(complete!(cmd, "--certain-num val1 [TAB]"), snapbox::str![[r#"
val1
val2
val3
"#]]);

    assert_data_eq!(
        complete!(cmd, "--certain-num val1 val2 val3 [TAB]"),
        snapbox::str![[r#"
--certain-num
--uncertain-num
--help	Print help
-Y
-N
-h	Print help
"#]]
    );

    assert_data_eq!(complete!(cmd, "--uncertain-num [TAB]"), snapbox::str![[r#"
val1
val2
val3
"#]]);

    assert_data_eq!(
        complete!(cmd, "--uncertain-num val1 [TAB]"),
        snapbox::str![[r#"
val1
val2
val3
--certain-num
--uncertain-num
--help	Print help
-Y
-N
-h	Print help
"#]]
    );

    assert_data_eq!(
        complete!(cmd, "--uncertain-num val1 val2 val3 [TAB]"),
        snapbox::str![[r#"
--certain-num
--uncertain-num
--help	Print help
-Y
-N
-h	Print help
"#]]
    );

    assert_data_eq!(complete!(cmd, "-Y [TAB]"), snapbox::str![[r#"
val1
val2
val3
"#]]);

    assert_data_eq!(complete!(cmd, "-Y val1 [TAB]"), snapbox::str![[r#"
val1
val2
val3
"#]]);

    assert_data_eq!(complete!(cmd, "-Y val1 val2 val3 [TAB]"), snapbox::str![[r#"
--certain-num
--uncertain-num
--help	Print help
-Y
-N
-h	Print help
"#]]);

    assert_data_eq!(complete!(cmd, "-N [TAB]"), snapbox::str![[r#"
val1
val2
val3
"#]]);

    assert_data_eq!(complete!(cmd, "-N val1 [TAB]"), snapbox::str![[r#"
val1
val2
val3
--certain-num
--uncertain-num
--help	Print help
-Y
-N
-h	Print help
"#]]);

    assert_data_eq!(complete!(cmd, "-N val1 val2 val3 [TAB]"), snapbox::str![[r#"
--certain-num
--uncertain-num
--help	Print help
-Y
-N
-h	Print help
"#]]);
}

#[test]
fn suggest_custom_arg_value() {
    #[derive(Debug)]
    struct MyCustomCompleter {}

    impl CustomCompleter for MyCustomCompleter {
        fn candidates(&self) -> Vec<CompletionCandidate> {
            vec![
                CompletionCandidate::new("custom1"),
                CompletionCandidate::new("custom2"),
                CompletionCandidate::new("custom3"),
            ]
        }
    }

    let mut cmd = Command::new("dynamic").arg(
        clap::Arg::new("custom")
            .long("custom")
            .add::<ArgValueCompleter>(ArgValueCompleter::new(MyCustomCompleter {})),
    );

    assert_data_eq!(complete!(cmd, "--custom [TAB]"), snapbox::str![[r#"
custom1
custom2
custom3
"#]],);
}

#[test]
fn suggest_multi_positional() {
    let mut cmd = Command::new("dynamic")
        .arg(
            clap::Arg::new("positional")
                .value_parser(["pos_1, pos_2, pos_3"])
                .index(1),
        )
        .arg(
            clap::Arg::new("positional-2")
                .value_parser(["pos_a", "pos_b", "pos_c"])
                .index(2)
                .num_args(3),
        )
        .arg(
            clap::Arg::new("--format")
                .long("format")
                .short('F')
                .value_parser(["json", "yaml", "toml"]),
        );

    assert_data_eq!(complete!(cmd, "pos_1 pos_a [TAB]"), snapbox::str![[r#"
pos_a
pos_b
pos_c
"#]]);

    assert_data_eq!(complete!(cmd, "pos_1 pos_a pos_b [TAB]"), snapbox::str![[r#"
pos_a
pos_b
pos_c
"#]]);

    assert_data_eq!(complete!(cmd, "--format json pos_1 [TAB]"), snapbox::str![[r#"
--format
--help	Print help
-F
-h	Print help
pos_a
pos_b
pos_c
"#]]);

    assert_data_eq!(
        complete!(cmd, "--format json pos_1 pos_a [TAB]"),
        snapbox::str![[r#"
pos_a
pos_b
pos_c
"#]]
    );

    assert_data_eq!(
        complete!(cmd, "--format json pos_1 pos_a pos_b pos_c [TAB]"),
        snapbox::str![[r#"
--format
--help	Print help
-F
-h	Print help
"#]]
    );

    assert_data_eq!(
        complete!(cmd, "--format json -- pos_1 pos_a [TAB]"),
        snapbox::str![[r#"
pos_a
pos_b
pos_c
"#]]
    );

    assert_data_eq!(
        complete!(cmd, "--format json -- pos_1 pos_a pos_b [TAB]"),
        snapbox::str![[r#"
pos_a
pos_b
pos_c
"#]]
    );

    assert_data_eq!(
        complete!(cmd, "--format json -- pos_1 pos_a pos_b pos_c [TAB]"),
        snapbox::str![]
    );
}

#[test]
fn suggest_delimiter_values() {
    let mut cmd = Command::new("delimiter")
        .arg(
            clap::Arg::new("delimiter")
                .long("delimiter")
                .short('D')
                .value_parser([
                    PossibleValue::new("comma"),
                    PossibleValue::new("space"),
                    PossibleValue::new("tab"),
                ])
                .value_delimiter(','),
        )
        .arg(
            clap::Arg::new("pos")
                .index(1)
                .value_parser(["a_pos", "b_pos", "c_pos"])
                .value_delimiter(','),
        );

    assert_data_eq!(
        complete!(cmd, "--delimiter [TAB]"),
        snapbox::str![
            "comma
space
tab"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "--delimiter=[TAB]"),
        snapbox::str![
            "--delimiter=comma
--delimiter=space
--delimiter=tab"
        ]
    );

    assert_data_eq!(complete!(cmd, "--delimiter c[TAB]"), snapbox::str!["comma"]);

    assert_data_eq!(
        complete!(cmd, "--delimiter=c[TAB]"),
        snapbox::str!["--delimiter=comma"]
    );

    assert_data_eq!(
        complete!(cmd, "--delimiter comma,[TAB]"),
        snapbox::str![
            "comma,comma
comma,space
comma,tab"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "--delimiter=comma,[TAB]"),
        snapbox::str![
            "--delimiter=comma,comma
--delimiter=comma,space
--delimiter=comma,tab
--delimiter=comma,a_pos
--delimiter=comma,b_pos
--delimiter=comma,c_pos"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "--delimiter comma,s[TAB]"),
        snapbox::str!["comma,space"]
    );

    assert_data_eq!(
        complete!(cmd, "--delimiter=comma,s[TAB]"),
        snapbox::str!["--delimiter=comma,space"]
    );

    assert_data_eq!(
        complete!(cmd, "-D [TAB]"),
        snapbox::str![
            "comma
space
tab"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "-D=[TAB]"),
        snapbox::str![
            "-D=comma
-D=space
-D=tab"
        ]
    );

    assert_data_eq!(complete!(cmd, "-D c[TAB]"), snapbox::str!["comma"]);

    assert_data_eq!(complete!(cmd, "-D=c[TAB]"), snapbox::str!["-D=comma"]);

    assert_data_eq!(
        complete!(cmd, "-D comma,[TAB]"),
        snapbox::str![
            "comma,comma
comma,space
comma,tab"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "-D=comma,[TAB]"),
        snapbox::str![
            "-D=comma,comma
-D=comma,space
-D=comma,tab
-D=comma,a_pos
-D=comma,b_pos
-D=comma,c_pos"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "-D comma,s[TAB]"),
        snapbox::str!["comma,space"]
    );

    assert_data_eq!(
        complete!(cmd, "-D=comma,s[TAB]"),
        snapbox::str!["-D=comma,space"]
    );

    assert_data_eq!(
        complete!(cmd, "-- [TAB]"),
        snapbox::str![
            "--delimiter
--help\tPrint help
-D
-h\tPrint help
a_pos
b_pos
c_pos"
        ]
    );

    assert_data_eq!(
        complete!(cmd, " -- a_pos,[TAB]"),
        snapbox::str![
            "a_pos,a_pos
a_pos,b_pos
a_pos,c_pos"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "-- a_pos,b[TAB]"),
        snapbox::str!["a_pos,b_pos"]
    );
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
        .map(|candidate| {
            let compl = candidate.get_content().to_str().unwrap();
            if let Some(help) = candidate.get_help() {
                format!("{compl}\t{help}")
            } else {
                compl.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
