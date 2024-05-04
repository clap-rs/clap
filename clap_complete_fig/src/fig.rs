#![allow(clippy::write_with_newline)]

use std::fmt::Write;

// Internal
use clap::{builder, Arg, ArgAction, Command, ValueHint};
use clap_complete::{generator, Generator};

/// Generate fig completion file
pub struct Fig;

impl Generator for Fig {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.ts")
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn std::io::Write) {
        let command = cmd.get_bin_name().unwrap();
        let mut buffer = String::new();

        write!(
            &mut buffer,
            "const completion: Fig.Spec = {{\n  name: \"{}\",\n",
            escape_string(command)
        )
        .unwrap();

        write!(
            &mut buffer,
            "  description: \"{}\",\n",
            escape_string(&cmd.get_about().unwrap_or_default().to_string())
        )
        .unwrap();

        gen_fig_inner(&[], 2, cmd, &mut buffer);

        write!(&mut buffer, "}};\n\nexport default completion;\n").unwrap();

        buf.write_all(buffer.as_bytes())
            .expect("Failed to write to generated file");
    }
}

// Escape string inside double quotes and convert whitespace
fn escape_string(string: &str) -> String {
    string
        .replace('\\', "\\\\")
        .replace('\"', "\\\"")
        .replace('\t', "    ")
        .replace('\n', " ")
        .replace('\r', "")
}

fn gen_fig_inner(parent_commands: &[&str], indent: usize, cmd: &Command, buffer: &mut String) {
    if cmd.has_subcommands() {
        write!(buffer, "{:indent$}subcommands: [\n", "").unwrap();
        // generate subcommands
        for subcommand in cmd.get_subcommands() {
            let mut aliases: Vec<&str> = subcommand.get_all_aliases().collect();
            if !aliases.is_empty() {
                aliases.insert(0, subcommand.get_name());

                write!(
                    buffer,
                    "{:indent$}{{\n{:indent$}  name: [",
                    "",
                    "",
                    indent = indent + 2
                )
                .unwrap();

                buffer.push_str(
                    &aliases
                        .iter()
                        .map(|name| format!("\"{}\"", escape_string(name)))
                        .collect::<Vec<_>>()
                        .join(", "),
                );

                write!(buffer, "],\n").unwrap();
            } else {
                write!(
                    buffer,
                    "{:indent$}{{\n{:indent$}  name: \"{}\",\n",
                    "",
                    "",
                    escape_string(subcommand.get_name()),
                    indent = indent + 2
                )
                .unwrap();
            }

            if let Some(data) = subcommand.get_about() {
                write!(
                    buffer,
                    "{:indent$}description: \"{}\",\n",
                    "",
                    escape_string(&data.to_string()),
                    indent = indent + 4
                )
                .unwrap();
            }

            if subcommand.is_hide_set() {
                write!(buffer, "{:indent$}hidden: true,\n", "", indent = indent + 4).unwrap();
            }

            let mut parent_commands: Vec<_> = parent_commands.into();
            parent_commands.push(subcommand.get_name());
            gen_fig_inner(&parent_commands, indent + 4, subcommand, buffer);

            write!(buffer, "{:indent$}}},\n", "", indent = indent + 2).unwrap();
        }
        write!(buffer, "{:indent$}],\n", "").unwrap();
    }

    buffer.push_str(&gen_options(cmd, indent));

    let args = cmd.get_positionals().collect::<Vec<_>>();

    match args.len() {
        0 => {}
        1 => {
            write!(buffer, "{:indent$}args: ", "").unwrap();

            buffer.push_str(&gen_args(args[0], indent));
        }
        _ => {
            write!(buffer, "{:indent$}args: [\n", "").unwrap();
            for arg in args {
                write!(buffer, "{:indent$}", "", indent = indent + 2).unwrap();
                buffer.push_str(&gen_args(arg, indent + 2));
            }
            write!(buffer, "{:indent$}]\n", "").unwrap();
        }
    };
}

fn gen_options(cmd: &Command, indent: usize) -> String {
    let mut buffer = String::new();

    let flags = generator::utils::flags(cmd);

    if cmd.get_opts().next().is_some() || !flags.is_empty() {
        write!(&mut buffer, "{:indent$}options: [\n", "").unwrap();

        for option in cmd.get_opts() {
            write!(&mut buffer, "{:indent$}{{\n", "", indent = indent + 2).unwrap();

            let mut names = vec![];

            if let Some(shorts) = option.get_short_and_visible_aliases() {
                names.extend(
                    shorts
                        .iter()
                        .map(|short| format!("-{}", escape_string(&short.to_string()))),
                );
            }

            if let Some(longs) = option.get_long_and_visible_aliases() {
                names.extend(
                    longs
                        .iter()
                        .map(|long| format!("--{}", escape_string(long))),
                );
            }

            if names.len() > 1 {
                write!(&mut buffer, "{:indent$}name: [", "", indent = indent + 4).unwrap();

                buffer.push_str(
                    &names
                        .iter()
                        .map(|name| format!("\"{}\"", escape_string(name)))
                        .collect::<Vec<_>>()
                        .join(", "),
                );

                buffer.push_str("],\n");
            } else {
                write!(
                    &mut buffer,
                    "{:indent$}name: \"{}\",\n",
                    "",
                    escape_string(&names[0]),
                    indent = indent + 4
                )
                .unwrap();
            }

            if let Some(data) = option.get_help() {
                write!(
                    &mut buffer,
                    "{:indent$}description: \"{}\",\n",
                    "",
                    escape_string(&data.to_string()),
                    indent = indent + 4
                )
                .unwrap();
            }

            if option.is_hide_set() {
                write!(
                    &mut buffer,
                    "{:indent$}hidden: true,\n",
                    "",
                    indent = indent + 4
                )
                .unwrap();
            }

            let conflicts = arg_conflicts(cmd, option);

            if !conflicts.is_empty() {
                write!(
                    &mut buffer,
                    "{:indent$}exclusiveOn: [\n",
                    "",
                    indent = indent + 4
                )
                .unwrap();

                for conflict in conflicts {
                    write!(
                        &mut buffer,
                        "{:indent$}\"{}\",\n",
                        "",
                        escape_string(&conflict),
                        indent = indent + 6
                    )
                    .unwrap();
                }

                write!(&mut buffer, "{:indent$}],\n", "", indent = indent + 4).unwrap();
            }

            if let ArgAction::Set | ArgAction::Append | ArgAction::Count = option.get_action() {
                write!(
                    &mut buffer,
                    "{:indent$}isRepeatable: true,\n",
                    "",
                    indent = indent + 4
                )
                .unwrap();
            }

            if option.is_require_equals_set() {
                write!(
                    &mut buffer,
                    "{:indent$}requiresEquals: true,\n",
                    "",
                    indent = indent + 4
                )
                .unwrap();
            }

            write!(&mut buffer, "{:indent$}args: ", "", indent = indent + 4).unwrap();

            buffer.push_str(&gen_args(option, indent + 4));

            write!(&mut buffer, "{:indent$}}},\n", "", indent = indent + 2).unwrap();
        }

        for flag in generator::utils::flags(cmd) {
            write!(&mut buffer, "{:indent$}{{\n", "", indent = indent + 2).unwrap();

            let mut flags = vec![];

            if let Some(shorts) = flag.get_short_and_visible_aliases() {
                flags.extend(shorts.iter().map(|s| format!("-{s}")));
            }

            if let Some(longs) = flag.get_long_and_visible_aliases() {
                flags.extend(longs.iter().map(|s| format!("--{s}")));
            }

            if flags.len() > 1 {
                write!(&mut buffer, "{:indent$}name: [", "", indent = indent + 4).unwrap();

                buffer.push_str(
                    &flags
                        .iter()
                        .map(|name| format!("\"{}\"", escape_string(name)))
                        .collect::<Vec<_>>()
                        .join(", "),
                );

                buffer.push_str("],\n");
            } else {
                write!(
                    &mut buffer,
                    "{:indent$}name: \"{}\",\n",
                    "",
                    escape_string(&flags[0]),
                    indent = indent + 4
                )
                .unwrap();
            }

            if let Some(data) = flag.get_help() {
                write!(
                    &mut buffer,
                    "{:indent$}description: \"{}\",\n",
                    "",
                    escape_string(&data.to_string()).as_str(),
                    indent = indent + 4
                )
                .unwrap();
            }

            let conflicts = arg_conflicts(cmd, &flag);

            if !conflicts.is_empty() {
                write!(
                    &mut buffer,
                    "{:indent$}exclusiveOn: [\n",
                    "",
                    indent = indent + 4
                )
                .unwrap();

                for conflict in conflicts {
                    write!(
                        &mut buffer,
                        "{:indent$}\"{}\",\n",
                        "",
                        escape_string(&conflict),
                        indent = indent + 6
                    )
                    .unwrap();
                }

                write!(&mut buffer, "{:indent$}],\n", "", indent = indent + 4).unwrap();
            }

            if let ArgAction::Set | ArgAction::Append | ArgAction::Count = flag.get_action() {
                write!(
                    &mut buffer,
                    "{:indent$}isRepeatable: true,\n",
                    "",
                    indent = indent + 4
                )
                .unwrap();
            }

            write!(&mut buffer, "{:indent$}}},\n", "", indent = indent + 2).unwrap();
        }

        write!(&mut buffer, "{:indent$}],\n", "").unwrap();
    }

    buffer
}

fn gen_args(arg: &Arg, indent: usize) -> String {
    if !arg.get_num_args().expect("built").takes_values() {
        return "".to_string();
    }

    let mut buffer = String::new();

    write!(
        &mut buffer,
        "{{\n{:indent$}  name: \"{}\",\n",
        "",
        escape_string(arg.get_id().as_str()),
        indent = indent
    )
    .unwrap();

    let num_args = arg.get_num_args().expect("built");
    if num_args != builder::ValueRange::EMPTY && num_args != builder::ValueRange::SINGLE {
        write!(
            &mut buffer,
            "{:indent$}isVariadic: true,\n",
            "",
            indent = indent + 2
        )
        .unwrap();
    }

    if !arg.is_required_set() {
        write!(
            &mut buffer,
            "{:indent$}isOptional: true,\n",
            "",
            indent = indent + 2
        )
        .unwrap();
    }

    if let Some(data) = generator::utils::possible_values(arg) {
        write!(
            &mut buffer,
            "{:indent$}suggestions: [\n",
            "",
            indent = indent + 2
        )
        .unwrap();

        for value in data {
            if let Some(help) = value.get_help() {
                write!(
                    &mut buffer,
                    "{:indent$}{{\n{:indent$}  name: \"{}\",\n",
                    "",
                    "",
                    escape_string(value.get_name()),
                    indent = indent + 4,
                )
                .unwrap();

                write!(
                    &mut buffer,
                    "{:indent$}description: \"{}\",\n",
                    "",
                    escape_string(&help.to_string()),
                    indent = indent + 6
                )
                .unwrap();

                write!(&mut buffer, "{:indent$}}},\n", "", indent = indent + 4).unwrap();
            } else {
                write!(
                    &mut buffer,
                    "{:indent$}\"{}\",\n",
                    "",
                    escape_string(value.get_name()),
                    indent = indent + 4,
                )
                .unwrap();
            }
        }

        write!(&mut buffer, "{:indent$}],\n", "", indent = indent + 2).unwrap();
    } else {
        match arg.get_value_hint() {
            ValueHint::AnyPath | ValueHint::FilePath | ValueHint::ExecutablePath => {
                write!(
                    &mut buffer,
                    "{:indent$}template: \"filepaths\",\n",
                    "",
                    indent = indent + 2
                )
                .unwrap();
            }
            ValueHint::DirPath => {
                write!(
                    &mut buffer,
                    "{:indent$}template: \"folders\",\n",
                    "",
                    indent = indent + 2
                )
                .unwrap();
            }
            ValueHint::CommandString | ValueHint::CommandName | ValueHint::CommandWithArguments => {
                write!(
                    &mut buffer,
                    "{:indent$}isCommand: true,\n",
                    "",
                    indent = indent + 2
                )
                .unwrap();
            }
            // Disable completion for others
            _ => (),
        };
    };

    write!(&mut buffer, "{:indent$}}},\n", "").unwrap();

    buffer
}

fn arg_conflicts(cmd: &Command, arg: &Arg) -> Vec<String> {
    let mut res = vec![];

    for conflict in cmd.get_arg_conflicts_with(arg) {
        if let Some(s) = conflict.get_short() {
            res.push(format!("-{}", escape_string(&s.to_string())));
        }

        if let Some(l) = conflict.get_long() {
            res.push(format!("--{}", escape_string(l)));
        }
    }

    res
}
