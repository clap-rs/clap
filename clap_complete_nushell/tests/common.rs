use clap::{builder::PossibleValue, Arg, ArgAction, Command, ValueHint};

pub fn basic_command(name: &'static str) -> Command {
    Command::new(name)
        .arg(
            Arg::new("config")
                .short('c')
                .global(true)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .conflicts_with("config")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("test")
                .about("Subcommand")
                .arg(Arg::new("debug").short('d').action(ArgAction::Count)),
        )
}

pub fn feature_sample_command(name: &'static str) -> Command {
    Command::new(name)
        .version("3.0")
        .propagate_version(true)
        .about("Tests completions")
        .arg(
            Arg::new("file")
                .value_hint(ValueHint::FilePath)
                .help("some input file"),
        )
        .arg(
            Arg::new("config")
                .action(ArgAction::Count)
                .help("some config file")
                .short('c')
                .visible_short_alias('C')
                .long("config")
                .visible_alias("conf"),
        )
        .arg(Arg::new("choice").value_parser(["first", "second"]))
        .subcommand(
            Command::new("test").about("tests things").arg(
                Arg::new("case")
                    .long("case")
                    .action(ArgAction::Set)
                    .help("the case to test"),
            ),
        )
}

pub fn special_commands_command(name: &'static str) -> Command {
    feature_sample_command(name)
        .subcommand(
            Command::new("some_cmd")
                .about("tests other things")
                .arg(
                    Arg::new("config")
                        .long("config")
                        .hide(true)
                        .action(ArgAction::Set)
                        .require_equals(true)
                        .help("the other case to test"),
                )
                .arg(Arg::new("path").num_args(1..)),
        )
        .subcommand(Command::new("some-cmd-with-hyphens").alias("hyphen"))
        .subcommand(Command::new("some-hidden-cmd").hide(true))
}

pub fn quoting_command(name: &'static str) -> Command {
    Command::new(name)
        .version("3.0")
        .arg(
            Arg::new("single-quotes")
                .long("single-quotes")
                .action(ArgAction::SetTrue)
                .help("Can be 'always', 'auto', or 'never'"),
        )
        .arg(
            Arg::new("double-quotes")
                .long("double-quotes")
                .action(ArgAction::SetTrue)
                .help("Can be \"always\", \"auto\", or \"never\""),
        )
        .arg(
            Arg::new("backticks")
                .long("backticks")
                .action(ArgAction::SetTrue)
                .help("For more information see `echo test`"),
        )
        .arg(
            Arg::new("backslash")
                .long("backslash")
                .action(ArgAction::SetTrue)
                .help("Avoid '\\n'"),
        )
        .arg(
            Arg::new("brackets")
                .long("brackets")
                .action(ArgAction::SetTrue)
                .help("List packages [filter]"),
        )
        .arg(
            Arg::new("expansions")
                .long("expansions")
                .action(ArgAction::SetTrue)
                .help("Execute the shell command with $SHELL"),
        )
        .subcommands([
            Command::new("cmd-single-quotes").about("Can be 'always', 'auto', or 'never'"),
            Command::new("cmd-double-quotes").about("Can be \"always\", \"auto\", or \"never\""),
            Command::new("cmd-backticks").about("For more information see `echo test`"),
            Command::new("cmd-backslash").about("Avoid '\\n'"),
            Command::new("cmd-brackets").about("List packages [filter]"),
            Command::new("cmd-expansions").about("Execute the shell command with $SHELL"),
        ])
}

pub fn aliases_command(name: &'static str) -> Command {
    Command::new(name)
        .version("3.0")
        .about("testing nushell completions")
        .arg(
            Arg::new("flag")
                .short('f')
                .visible_short_alias('F')
                .long("flag")
                .action(ArgAction::SetTrue)
                .visible_alias("flg")
                .help("cmd flag"),
        )
        .arg(
            Arg::new("option")
                .short('o')
                .visible_short_alias('O')
                .long("option")
                .visible_alias("opt")
                .help("cmd option")
                .action(ArgAction::Set),
        )
        .arg(Arg::new("positional"))
}

pub fn sub_subcommands_command(name: &'static str) -> Command {
    feature_sample_command(name).subcommand(
        Command::new("some_cmd")
            .about("top level subcommand")
            .visible_alias("some_cmd_alias")
            .subcommand(
                Command::new("sub_cmd").about("sub-subcommand").arg(
                    Arg::new("config")
                        .long("config")
                        .action(ArgAction::Set)
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

pub fn value_hint_command(name: &'static str) -> Command {
    Command::new(name)
        .arg(
            Arg::new("choice")
                .long("choice")
                .action(ArgAction::Set)
                .value_parser(["bash", "fish", "zsh"]),
        )
        .arg(
            Arg::new("unknown")
                .long("unknown")
                .value_hint(ValueHint::Unknown),
        )
        .arg(Arg::new("other").long("other").value_hint(ValueHint::Other))
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .value_hint(ValueHint::AnyPath),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .value_hint(ValueHint::DirPath),
        )
        .arg(
            Arg::new("exe")
                .long("exe")
                .short('e')
                .value_hint(ValueHint::ExecutablePath),
        )
        .arg(
            Arg::new("cmd_name")
                .long("cmd-name")
                .value_hint(ValueHint::CommandName),
        )
        .arg(
            Arg::new("cmd")
                .long("cmd")
                .short('c')
                .value_hint(ValueHint::CommandString),
        )
        .arg(
            Arg::new("command_with_args")
                .action(ArgAction::Set)
                .num_args(1..)
                .trailing_var_arg(true)
                .value_hint(ValueHint::CommandWithArguments),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .value_hint(ValueHint::Username),
        )
        .arg(
            Arg::new("host")
                .short('H')
                .long("host")
                .value_hint(ValueHint::Hostname),
        )
        .arg(Arg::new("url").long("url").value_hint(ValueHint::Url))
        .arg(
            Arg::new("email")
                .long("email")
                .value_hint(ValueHint::EmailAddress),
        )
}

pub fn assert_matches_path(
    expected_path: impl AsRef<std::path::Path>,
    gen: impl clap_complete::Generator,
    mut cmd: Command,
    name: &'static str,
) {
    let mut buf = vec![];
    clap_complete::generate(gen, &mut cmd, name, &mut buf);

    snapbox::Assert::new()
        .action_env("SNAPSHOTS")
        .normalize_paths(false)
        .matches_path(expected_path, buf);
}
