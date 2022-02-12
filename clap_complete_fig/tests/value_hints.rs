use clap::{Arg, Command, ValueHint};
use clap_complete_fig::Fig;
use completions::common;

mod completions;

pub fn build_app_with_value_hints() -> Command<'static> {
    Command::new("my_app")
        .disable_version_flag(true)
        .trailing_var_arg(true)
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
