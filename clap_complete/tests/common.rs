use clap::builder::PossibleValue;

pub fn basic_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .arg(
            clap::Arg::new("config")
                .short('c')
                .global(true)
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("v")
                .short('v')
                .conflicts_with("config")
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand(
            clap::Command::new("test").about("Subcommand").arg(
                clap::Arg::new("debug")
                    .short('d')
                    .action(clap::ArgAction::Count),
            ),
        )
}

pub fn feature_sample_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .version("3.0")
        .propagate_version(true)
        .about("Tests completions")
        .arg(
            clap::Arg::new("file")
                .value_hint(clap::ValueHint::FilePath)
                .help("some input file"),
        )
        .arg(
            clap::Arg::new("config")
                .action(clap::ArgAction::Count)
                .help("some config file")
                .short('c')
                .visible_short_alias('C')
                .long("config")
                .visible_alias("conf"),
        )
        .arg(clap::Arg::new("choice").value_parser(["first", "second"]))
        .subcommand(
            clap::Command::new("test").about("tests things").arg(
                clap::Arg::new("case")
                    .long("case")
                    .action(clap::ArgAction::Set)
                    .help("the case to test"),
            ),
        )
}

pub fn special_commands_command(name: &'static str) -> clap::Command {
    feature_sample_command(name)
        .subcommand(
            clap::Command::new("some_cmd")
                .about("tests other things")
                .arg(
                    clap::Arg::new("config")
                        .long("config")
                        .hide(true)
                        .action(clap::ArgAction::Set)
                        .require_equals(true)
                        .help("the other case to test"),
                )
                .arg(clap::Arg::new("path").num_args(1..)),
        )
        .subcommand(clap::Command::new("some-cmd-with-hyphens").alias("hyphen"))
        .subcommand(clap::Command::new("some-hidden-cmd").hide(true))
}

pub fn quoting_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .version("3.0")
        .arg(
            clap::Arg::new("single-quotes")
                .long("single-quotes")
                .action(clap::ArgAction::SetTrue)
                .help("Can be 'always', 'auto', or 'never'"),
        )
        .arg(
            clap::Arg::new("double-quotes")
                .long("double-quotes")
                .action(clap::ArgAction::SetTrue)
                .help("Can be \"always\", \"auto\", or \"never\""),
        )
        .arg(
            clap::Arg::new("backticks")
                .long("backticks")
                .action(clap::ArgAction::SetTrue)
                .help("For more information see `echo test`"),
        )
        .arg(
            clap::Arg::new("backslash")
                .long("backslash")
                .action(clap::ArgAction::SetTrue)
                .help("Avoid '\\n'"),
        )
        .arg(
            clap::Arg::new("brackets")
                .long("brackets")
                .action(clap::ArgAction::SetTrue)
                .help("List packages [filter]"),
        )
        .arg(
            clap::Arg::new("expansions")
                .long("expansions")
                .action(clap::ArgAction::SetTrue)
                .help("Execute the shell command with $SHELL"),
        )
        .subcommands([
            clap::Command::new("cmd-single-quotes").about("Can be 'always', 'auto', or 'never'"),
            clap::Command::new("cmd-double-quotes")
                .about("Can be \"always\", \"auto\", or \"never\""),
            clap::Command::new("cmd-backticks").about("For more information see `echo test`"),
            clap::Command::new("cmd-backslash").about("Avoid '\\n'"),
            clap::Command::new("cmd-brackets").about("List packages [filter]"),
            clap::Command::new("cmd-expansions").about("Execute the shell command with $SHELL"),
        ])
}

pub fn aliases_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .version("3.0")
        .about("testing bash completions")
        .arg(
            clap::Arg::new("flag")
                .short('f')
                .visible_short_alias('F')
                .long("flag")
                .action(clap::ArgAction::SetTrue)
                .visible_alias("flg")
                .help("cmd flag"),
        )
        .arg(
            clap::Arg::new("option")
                .short('o')
                .visible_short_alias('O')
                .long("option")
                .visible_alias("opt")
                .help("cmd option")
                .action(clap::ArgAction::Set),
        )
        .arg(clap::Arg::new("positional"))
}

pub fn sub_subcommands_command(name: &'static str) -> clap::Command {
    feature_sample_command(name).subcommand(
        clap::Command::new("some_cmd")
            .about("top level subcommand")
            .visible_alias("some_cmd_alias")
            .subcommand(
                clap::Command::new("sub_cmd").about("sub-subcommand").arg(
                    clap::Arg::new("config")
                        .long("config")
                        .action(clap::ArgAction::Set)
                        .value_parser([
                            PossibleValue::new("Lest quotes, aren't escaped.")
                                .help("help,with,comma"),
                            PossibleValue::new("Second to trigger display of options"),
                        ])
                        .help("the other case to test"),
                ),
            ),
    )
}

pub fn value_hint_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .arg(
            clap::Arg::new("choice")
                .long("choice")
                .action(clap::ArgAction::Set)
                .value_parser(["bash", "fish", "zsh"]),
        )
        .arg(
            clap::Arg::new("unknown")
                .long("unknown")
                .value_hint(clap::ValueHint::Unknown),
        )
        .arg(
            clap::Arg::new("other")
                .long("other")
                .value_hint(clap::ValueHint::Other),
        )
        .arg(
            clap::Arg::new("path")
                .long("path")
                .short('p')
                .value_hint(clap::ValueHint::AnyPath),
        )
        .arg(
            clap::Arg::new("file")
                .long("file")
                .short('f')
                .value_hint(clap::ValueHint::FilePath),
        )
        .arg(
            clap::Arg::new("dir")
                .long("dir")
                .short('d')
                .value_hint(clap::ValueHint::DirPath),
        )
        .arg(
            clap::Arg::new("exe")
                .long("exe")
                .short('e')
                .value_hint(clap::ValueHint::ExecutablePath),
        )
        .arg(
            clap::Arg::new("cmd_name")
                .long("cmd-name")
                .value_hint(clap::ValueHint::CommandName),
        )
        .arg(
            clap::Arg::new("cmd")
                .long("cmd")
                .short('c')
                .value_hint(clap::ValueHint::CommandString),
        )
        .arg(
            clap::Arg::new("command_with_args")
                .action(clap::ArgAction::Set)
                .num_args(1..)
                .trailing_var_arg(true)
                .value_hint(clap::ValueHint::CommandWithArguments),
        )
        .arg(
            clap::Arg::new("user")
                .short('u')
                .long("user")
                .value_hint(clap::ValueHint::Username),
        )
        .arg(
            clap::Arg::new("host")
                .short('H')
                .long("host")
                .value_hint(clap::ValueHint::Hostname),
        )
        .arg(
            clap::Arg::new("url")
                .long("url")
                .value_hint(clap::ValueHint::Url),
        )
        .arg(
            clap::Arg::new("email")
                .long("email")
                .value_hint(clap::ValueHint::EmailAddress),
        )
}

pub fn value_terminator_command(name: &'static str) -> clap::Command {
    clap::Command::new(name).arg(
        clap::Arg::new("arguments")
            .help("multi-valued argument with a value terminator")
            .num_args(1..)
            .value_terminator(";"),
    )
}

pub fn assert_matches_path(
    expected_path: impl AsRef<std::path::Path>,
    gen: impl clap_complete::Generator,
    mut cmd: clap::Command,
    name: &'static str,
) {
    let mut buf = vec![];
    clap_complete::generate(gen, &mut cmd, name, &mut buf);

    snapbox::Assert::new()
        .action_env("SNAPSHOTS")
        .normalize_paths(false)
        .matches_path(expected_path, buf);
}
