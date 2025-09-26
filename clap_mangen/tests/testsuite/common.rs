use clap::builder::PossibleValue;
use snapbox::prelude::*;

pub(crate) fn basic_command(name: &'static str) -> clap::Command {
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
            clap::Command::new("test")
                .about("Subcommand\nwith a second line")
                .arg(
                    clap::Arg::new("debug")
                        .short('d')
                        .action(clap::ArgAction::Count),
                ),
        )
}

pub(crate) fn feature_sample_command(name: &'static str) -> clap::Command {
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

pub(crate) fn special_commands_command(name: &'static str) -> clap::Command {
    feature_sample_command(name)
        .subcommand(
            clap::Command::new("some_cmd")
                .about("tests other things")
                .arg(
                    clap::Arg::new("config")
                        .long("config")
                        .action(clap::ArgAction::Set)
                        .help("the other case to test"),
                ),
        )
        .subcommand(clap::Command::new("some-cmd-with-hyphens").alias("hyphen"))
}

pub(crate) fn quoting_command(name: &'static str) -> clap::Command {
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

pub(crate) fn aliases_command(name: &'static str) -> clap::Command {
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

pub(crate) fn sub_subcommands_command(name: &'static str) -> clap::Command {
    feature_sample_command(name).subcommand(
        clap::Command::new("some_cmd")
            .about("top level subcommand")
            .subcommand(
                clap::Command::new("sub_cmd").about("sub-subcommand").arg(
                    clap::Arg::new("config")
                        .long("config")
                        .action(clap::ArgAction::Set)
                        .value_parser([PossibleValue::new("Lest quotes aren't escaped.")])
                        .help("the other case to test"),
                ),
            ),
    )
}

pub(crate) fn value_hint_command(name: &'static str) -> clap::Command {
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

pub(crate) fn hidden_option_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .arg(
            clap::Arg::new("config")
                .long("config")
                .action(clap::ArgAction::Set),
        )
        .arg(
            clap::Arg::new("no-config")
                .long("no-config")
                .hide(true)
                .overrides_with("config"),
        )
}

pub(crate) fn env_value_command(name: &'static str) -> clap::Command {
    clap::Command::new(name).arg(
        clap::Arg::new("config")
            .short('c')
            .long_help("Set configuration file path")
            .required(false)
            .action(clap::ArgAction::Set)
            .default_value("config.toml")
            .env("CONFIG_FILE"),
    )
}

pub(crate) fn assert_matches(expected: impl IntoData, cmd: clap::Command) {
    let mut buf = vec![];
    clap_mangen::Man::new(cmd).render(&mut buf).unwrap();

    snapbox::Assert::new()
        .action_env(snapbox::assert::DEFAULT_ACTION_ENV)
        .normalize_paths(false)
        .eq(buf, expected);
}

pub(crate) fn possible_values_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .arg(
            clap::Arg::new("choice")
                .long("choice")
                .action(clap::ArgAction::Set)
                .value_parser(["bash", "fish", "zsh"]),
        )
        .arg(
            clap::Arg::new("method")
                .long("method")
                .action(clap::ArgAction::Set)
                .value_parser([
                    PossibleValue::new("fast").help("use the Fast method"),
                    PossibleValue::new("slow").help("use the slow method"),
                    PossibleValue::new("normal")
                        .help("use normal mode")
                        .hide(true),
                ]),
        )
        .arg(
            clap::Arg::new("positional_choice")
                .action(clap::ArgAction::Set)
                .help("Pick the Position you want the command to run in")
                .value_parser([
                    PossibleValue::new("left").help("run left adjusted"),
                    PossibleValue::new("right"),
                    PossibleValue::new("center").hide(true),
                ]),
        )
}

pub(crate) fn value_name_without_arg(name: &'static str) -> clap::Command {
    clap::Command::new(name).arg(
        clap::Arg::new("flag")
            .long("flag")
            .value_name("SPURIOUS")
            .action(clap::ArgAction::SetTrue),
    )
}

pub(crate) fn help_headings(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .arg(
            clap::Arg::new("recursive")
                .long("recursive")
                .short('r')
                .action(clap::ArgAction::SetTrue),
        )
        .next_help_heading("Conflict Options")
        .arg(
            clap::Arg::new("force")
                .long("force")
                .short('f')
                .action(clap::ArgAction::SetTrue),
        )
        .next_help_heading("Hidden Options")
        .arg(
            clap::Arg::new("debug")
                .long("debug")
                .short('d')
                .hide(true)
                .action(clap::ArgAction::SetTrue),
        )
        .next_help_heading("Global Options")
        .arg(
            clap::Arg::new("color")
                .global(true)
                .value_parser(["always", "never", "auto"]),
        )
}

pub(crate) fn value_with_required_equals(name: &'static str) -> clap::Command {
    clap::Command::new(name).arg(
        clap::Arg::new("config")
            .long("config")
            .value_name("FILE")
            .require_equals(true)
            .help("Optional config file"),
    )
}

pub(crate) fn optional_value_with_required_equals(name: &'static str) -> clap::Command {
    clap::Command::new(name).arg(
        clap::Arg::new("config")
            .long("config")
            .value_name("FILE")
            .require_equals(true)
            .num_args(0..=1)
            .help("Optional config file"),
    )
}

pub(crate) fn optional_value(name: &'static str) -> clap::Command {
    clap::Command::new(name).arg(
        clap::Arg::new("config")
            .long("config")
            .value_name("FILE")
            .num_args(0..=1)
            .help("Optional config file"),
    )
}

pub(crate) fn multiple_optional_values(name: &'static str) -> clap::Command {
    clap::Command::new(name).arg(
        clap::Arg::new("config")
            .long("config")
            .value_names(["FILE1", "FILE2"])
            .num_args(0..=2)
            .help("Optional config file"),
    )
}

pub(crate) fn variadic_values(name: &'static str) -> clap::Command {
    clap::Command::new(name).arg(
        clap::Arg::new("config")
            .long("config")
            .value_names(["FILE1", "FILE2"])
            .require_equals(false)
            .num_args(3)
            .help("Optional config file"),
    )
}

pub(crate) fn display_order(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .next_display_order(None)
        .arg(clap::Arg::new("c").short('c'))
        .arg(clap::Arg::new("b").short('b'))
        .arg(clap::Arg::new("a").short('a'))
        .arg(clap::Arg::new("aa").long("aa"))
        .arg(clap::Arg::new("0").short('0'))
}
