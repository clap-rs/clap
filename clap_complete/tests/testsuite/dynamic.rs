#![cfg(feature = "unstable-dynamic")]

#[test]
fn suggest_subcommand_subset() {
    let name = "test";
    let mut cmd = clap::Command::new(name)
        .subcommand(clap::Command::new("hello-world"))
        .subcommand(clap::Command::new("hello-moon"))
        .subcommand(clap::Command::new("goodbye-world"));

    let args = [name, "he"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>();
    let current_dir = None;

    let completions =
        clap_complete::dynamic::complete(&mut cmd, args, arg_index, current_dir).unwrap();
    let completions = completions
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    assert_eq!(completions, ["hello-moon", "hello-world", "help"]);
}

#[test]
fn suggest_long_flag_subset() {
    let name = "test";
    let mut cmd = clap::Command::new(name)
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

    let args = [name, "--he"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>();
    let current_dir = None;

    let completions =
        clap_complete::dynamic::complete(&mut cmd, args, arg_index, current_dir).unwrap();
    let completions = completions
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    assert_eq!(completions, ["--hello-world", "--hello-moon", "--help"]);
}

#[test]
fn suggest_possible_value_subset() {
    let name = "test";
    let mut cmd = clap::Command::new(name).arg(clap::Arg::new("hello-world").value_parser([
        "hello-world",
        "hello-moon",
        "goodbye-world",
    ]));

    let args = [name, "hello"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>();
    let current_dir = None;

    let completions =
        clap_complete::dynamic::complete(&mut cmd, args, arg_index, current_dir).unwrap();
    let completions = completions
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    assert_eq!(completions, ["hello-world", "hello-moon"]);
}

#[test]
fn suggest_additional_short_flags() {
    let name = "test";
    let mut cmd = clap::Command::new(name)
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

    let args = [name, "-a"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>();
    let current_dir = None;

    let completions =
        clap_complete::dynamic::complete(&mut cmd, args, arg_index, current_dir).unwrap();
    let completions = completions
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    assert_eq!(completions, ["-aa", "-ab", "-ac", "-ah"]);
}
