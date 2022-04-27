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
        .map(|s| std::ffi::OsString::from(s))
        .collect::<Vec<_>>();
    let comp_type = clap_complete::dynamic::bash::CompType::default();
    let trailing_space = true;
    let current_dir = None;

    let completions = clap_complete::dynamic::bash::complete(
        &mut cmd,
        args,
        arg_index,
        comp_type,
        trailing_space,
        current_dir,
    )
    .unwrap();
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
                .long("--hello-world")
                .multiple_occurrences(true),
        )
        .arg(
            clap::Arg::new("hello-moon")
                .long("--hello-moon")
                .multiple_occurrences(true),
        )
        .arg(
            clap::Arg::new("goodbye-world")
                .long("--goodbye-world")
                .multiple_occurrences(true),
        );

    let args = [name, "--he"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(|s| std::ffi::OsString::from(s))
        .collect::<Vec<_>>();
    let comp_type = clap_complete::dynamic::bash::CompType::default();
    let trailing_space = true;
    let current_dir = None;

    let completions = clap_complete::dynamic::bash::complete(
        &mut cmd,
        args,
        arg_index,
        comp_type,
        trailing_space,
        current_dir,
    )
    .unwrap();
    let completions = completions
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    assert_eq!(completions, ["--help", "--hello-world", "--hello-moon"]);
}

#[test]
fn suggest_possible_value_subset() {
    let name = "test";
    let mut cmd = clap::Command::new(name).arg(clap::Arg::new("hello-world").possible_values([
        "hello-world",
        "hello-moon",
        "goodbye-world",
    ]));

    let args = [name, "hello"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(|s| std::ffi::OsString::from(s))
        .collect::<Vec<_>>();
    let comp_type = clap_complete::dynamic::bash::CompType::default();
    let trailing_space = true;
    let current_dir = None;

    let completions = clap_complete::dynamic::bash::complete(
        &mut cmd,
        args,
        arg_index,
        comp_type,
        trailing_space,
        current_dir,
    )
    .unwrap();
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
        .arg(clap::Arg::new("a").short('a').multiple_occurrences(true))
        .arg(clap::Arg::new("b").short('b').multiple_occurrences(true))
        .arg(clap::Arg::new("c").short('c').multiple_occurrences(true));

    let args = [name, "-a"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(|s| std::ffi::OsString::from(s))
        .collect::<Vec<_>>();
    let comp_type = clap_complete::dynamic::bash::CompType::default();
    let trailing_space = true;
    let current_dir = None;

    let completions = clap_complete::dynamic::bash::complete(
        &mut cmd,
        args,
        arg_index,
        comp_type,
        trailing_space,
        current_dir,
    )
    .unwrap();
    let completions = completions
        .into_iter()
        .map(|s| s.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    assert_eq!(completions, ["-ah", "-aa", "-ab", "-ac"]);
}
