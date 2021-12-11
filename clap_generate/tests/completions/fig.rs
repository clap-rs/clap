use super::*;
use crate::Fig;

fn build_app() -> App<'static> {
    build_app_with_name("myapp")
}

fn build_app_with_name(s: &'static str) -> App<'static> {
    App::new(s)
        .version("3.0")
        .setting(AppSettings::PropagateVersion)
        .about("Tests completions")
        .arg(
            Arg::new("file")
                .value_hint(ValueHint::FilePath)
                .help("some input file"),
        )
        .subcommand(
            App::new("test").about("tests things").arg(
                Arg::new("case")
                    .long("case")
                    .takes_value(true)
                    .help("the case to test"),
            ),
        )
}

#[test]
fn fig() {
    let mut app = build_app();
    common(Fig, &mut app, "myapp", FIG);
}

static FIG: &str = r#"const completion: Fig.Spec = {
  name: "myapp",
  description: "Tests completions",
  subcommands: [
    {
      name: "test",
      description: "tests things",
      options: [
        {
          name: "--case",
          description: "the case to test",
          args: {
            name: "case",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
      ],
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
  ],
  args: {
    name: "file",
    isOptional: true,
    template: "filepaths",
  },
};

export default completion;
"#;

#[test]
fn fig_with_special_commands() {
    let mut app = build_app_special_commands();
    common(Fig, &mut app, "my_app", FIG_SPECIAL_CMDS);
}

fn build_app_special_commands() -> App<'static> {
    build_app_with_name("my_app")
        .subcommand(
            App::new("some_cmd").about("tests other things").arg(
                Arg::new("config")
                    .long("--config")
                    .takes_value(true)
                    .help("the other case to test"),
            ),
        )
        .subcommand(App::new("some-cmd-with-hyphens").alias("hyphen"))
}

static FIG_SPECIAL_CMDS: &str = r#"const completion: Fig.Spec = {
  name: "my_app",
  description: "Tests completions",
  subcommands: [
    {
      name: "test",
      description: "tests things",
      options: [
        {
          name: "--case",
          description: "the case to test",
          args: {
            name: "case",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "some_cmd",
      description: "tests other things",
      options: [
        {
          name: "--config",
          description: "the other case to test",
          args: {
            name: "config",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "some-cmd-with-hyphens",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
      ],
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
  ],
  args: {
    name: "file",
    isOptional: true,
    template: "filepaths",
  },
};

export default completion;
"#;

#[test]
fn fig_with_special_help() {
    let mut app = build_app_special_help();
    common(Fig, &mut app, "my_app", FIG_SPECIAL_HELP);
}

fn build_app_special_help() -> App<'static> {
    App::new("my_app")
        .version("3.0")
        .arg(
            Arg::new("single-quotes")
                .long("single-quotes")
                .help("Can be 'always', 'auto', or 'never'"),
        )
        .arg(
            Arg::new("double-quotes")
                .long("double-quotes")
                .help("Can be \"always\", \"auto\", or \"never\""),
        )
        .arg(
            Arg::new("backticks")
                .long("backticks")
                .help("For more information see `echo test`"),
        )
        .arg(Arg::new("backslash").long("backslash").help("Avoid '\\n'"))
        .arg(
            Arg::new("brackets")
                .long("brackets")
                .help("List packages [filter]"),
        )
        .arg(
            Arg::new("expansions")
                .long("expansions")
                .help("Execute the shell command with $SHELL"),
        )
}

static FIG_SPECIAL_HELP: &str = r#"const completion: Fig.Spec = {
  name: "my_app",
  description: "",
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
    {
      name: "--single-quotes",
      description: "Can be 'always', 'auto', or 'never'",
    },
    {
      name: "--double-quotes",
      description: "Can be \"always\", \"auto\", or \"never\"",
    },
    {
      name: "--backticks",
      description: "For more information see `echo test`",
    },
    {
      name: "--backslash",
      description: "Avoid '\\n'",
    },
    {
      name: "--brackets",
      description: "List packages [filter]",
    },
    {
      name: "--expansions",
      description: "Execute the shell command with $SHELL",
    },
  ],
};

export default completion;
"#;

#[test]
fn fig_with_aliases() {
    let mut app = build_app_with_aliases();
    common(Fig, &mut app, "cmd", FIG_ALIASES);
}

fn build_app_with_aliases() -> App<'static> {
    App::new("cmd")
        .version("3.0")
        .about("testing bash completions")
        .arg(
            Arg::new("flag")
                .short('f')
                .visible_short_alias('F')
                .long("flag")
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
                .takes_value(true),
        )
        .arg(Arg::new("positional"))
}

static FIG_ALIASES: &str = r#"const completion: Fig.Spec = {
  name: "cmd",
  description: "testing bash completions",
  options: [
    {
      name: ["-o", "-O", "--option", "--opt"],
      description: "cmd option",
      args: {
        name: "option",
        isOptional: true,
      },
    },
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
    {
      name: ["-f", "-F", "--flag", "--flg"],
      description: "cmd flag",
    },
  ],
  args: {
    name: "positional",
    isOptional: true,
  },
};

export default completion;
"#;

#[test]
fn fig_with_sub_subcommands() {
    let mut app = build_app_sub_subcommands();
    common(Fig, &mut app, "my_app", FIG_SUB_SUBCMDS);
}

fn build_app_sub_subcommands() -> App<'static> {
    build_app_with_name("my_app").subcommand(
        App::new("some_cmd")
            .about("top level subcommand")
            .subcommand(
                App::new("sub_cmd").about("sub-subcommand").arg(
                    Arg::new("config")
                        .long("--config")
                        .takes_value(true)
                        .help("the other case to test"),
                ),
            ),
    )
}

static FIG_SUB_SUBCMDS: &str = r#"const completion: Fig.Spec = {
  name: "my_app",
  description: "Tests completions",
  subcommands: [
    {
      name: "test",
      description: "tests things",
      options: [
        {
          name: "--case",
          description: "the case to test",
          args: {
            name: "case",
            isOptional: true,
          },
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "some_cmd",
      description: "top level subcommand",
      subcommands: [
        {
          name: "sub_cmd",
          description: "sub-subcommand",
          options: [
            {
              name: "--config",
              description: "the other case to test",
              args: {
                name: "config",
                isOptional: true,
              },
            },
            {
              name: ["-h", "--help"],
              description: "Print help information",
            },
            {
              name: ["-V", "--version"],
              description: "Print version information",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help information",
            },
            {
              name: ["-V", "--version"],
              description: "Print version information",
            },
          ],
        },
      ],
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      options: [
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
      ],
    },
  ],
  options: [
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
  ],
  args: {
    name: "file",
    isOptional: true,
    template: "filepaths",
  },
};

export default completion;
"#;

pub fn build_app_with_value_hints() -> App<'static> {
    App::new("my_app")
        .setting(AppSettings::DisableVersionFlag)
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::new("choice")
                .long("choice")
                .possible_values(["bash", "fish", "zsh"]),
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
                .takes_value(true)
                .multiple_values(true)
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
                .short('h')
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

static FIG_VALUE_HINTS: &str = r#"const completion: Fig.Spec = {
  name: "my_app",
  description: "",
  options: [
    {
      name: "--choice",
      args: {
        name: "choice",
        isOptional: true,
        suggestions: [
          {
            name: "bash",
          },
          {
            name: "fish",
          },
          {
            name: "zsh",
          },
        ]
      },
    },
    {
      name: "--unknown",
      args: {
        name: "unknown",
        isOptional: true,
      },
    },
    {
      name: "--other",
      args: {
        name: "other",
        isOptional: true,
      },
    },
    {
      name: ["-p", "--path"],
      args: {
        name: "path",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: ["-f", "--file"],
      args: {
        name: "file",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: ["-d", "--dir"],
      args: {
        name: "dir",
        isOptional: true,
        template: "folders",
      },
    },
    {
      name: ["-e", "--exe"],
      args: {
        name: "exe",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: "--cmd-name",
      args: {
        name: "cmd_name",
        isOptional: true,
        isCommand: true,
      },
    },
    {
      name: ["-c", "--cmd"],
      args: {
        name: "cmd",
        isOptional: true,
        isCommand: true,
      },
    },
    {
      name: ["-u", "--user"],
      args: {
        name: "user",
        isOptional: true,
      },
    },
    {
      name: ["-h", "--host"],
      args: {
        name: "host",
        isOptional: true,
      },
    },
    {
      name: "--url",
      args: {
        name: "url",
        isOptional: true,
      },
    },
    {
      name: "--email",
      args: {
        name: "email",
        isOptional: true,
      },
    },
    {
      name: "--help",
      description: "Print help information",
    },
  ],
  args: {
    name: "command_with_args",
    isVariadic: true,
    isOptional: true,
    isCommand: true,
  },
};

export default completion;
"#;

#[test]
fn fig_with_value_hints() {
    let mut app = build_app_with_value_hints();
    common(Fig, &mut app, "my_app", FIG_VALUE_HINTS);
}
