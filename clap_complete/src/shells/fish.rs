use std::io::Write;

use clap::*;

use crate::generator::{utils, Generator};

/// Generate fish completion file
///
/// Note: The fish generator currently only supports named options (-o/--option), not positional arguments.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Fish;

impl Generator for Fish {
    fn file_name(&self, name: &str) -> String {
        format!("{}.fish", name)
    }

    fn generate(&self, app: &App, buf: &mut dyn Write) {
        let command = app.get_bin_name().unwrap();
        let mut buffer = String::new();

        gen_fish_inner(command, &[], app, &mut buffer);
        w!(buf, buffer.as_bytes());
    }
}

// Escape string inside single quotes
fn escape_string(string: &str) -> String {
    string.replace('\\', "\\\\").replace('\'', "\\'")
}

fn gen_fish_inner(root_command: &str, parent_commands: &[&str], app: &App, buffer: &mut String) {
    debug!("gen_fish_inner");
    // example :
    //
    // complete
    //      -c {command}
    //      -d "{description}"
    //      -s {short}
    //      -l {long}
    //      -a "{possible_arguments}"
    //      -r # if require parameter
    //      -f # don't use file completion
    //      -n "__fish_use_subcommand"               # complete for command "myprog"
    //      -n "__fish_seen_subcommand_from subcmd1" # complete for command "myprog subcmd1"

    let mut basic_template = format!("complete -c {}", root_command);

    if parent_commands.is_empty() {
        if app.has_subcommands() {
            basic_template.push_str(" -n \"__fish_use_subcommand\"");
        }
    } else {
        basic_template.push_str(
            format!(
                " -n \"{}\"",
                parent_commands
                    .iter()
                    .map(|command| format!("__fish_seen_subcommand_from {}", command))
                    .chain(
                        app.get_subcommands()
                            .map(|command| format!("not __fish_seen_subcommand_from {}", command))
                    )
                    .collect::<Vec<_>>()
                    .join("; and ")
            )
            .as_str(),
        );
    }

    debug!("gen_fish_inner: parent_commands={:?}", parent_commands);

    for option in app.get_opts() {
        let mut template = basic_template.clone();

        if let Some(shorts) = option.get_short_and_visible_aliases() {
            for short in shorts {
                template.push_str(format!(" -s {}", short).as_str());
            }
        }

        if let Some(longs) = option.get_long_and_visible_aliases() {
            for long in longs {
                template.push_str(format!(" -l {}", escape_string(long)).as_str());
            }
        }

        if let Some(data) = option.get_help() {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }

        template.push_str(value_completion(option).as_str());

        buffer.push_str(template.as_str());
        buffer.push('\n');
    }

    for flag in utils::flags(app) {
        let mut template = basic_template.clone();

        if let Some(shorts) = flag.get_short_and_visible_aliases() {
            for short in shorts {
                template.push_str(format!(" -s {}", short).as_str());
            }
        }

        if let Some(longs) = flag.get_long_and_visible_aliases() {
            for long in longs {
                template.push_str(format!(" -l {}", escape_string(long)).as_str());
            }
        }

        if let Some(data) = flag.get_help() {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }

        buffer.push_str(template.as_str());
        buffer.push('\n');
    }

    for subcommand in app.get_subcommands() {
        let mut template = basic_template.clone();

        template.push_str(" -f");
        template.push_str(format!(" -a \"{}\"", &subcommand.get_name()).as_str());

        if let Some(data) = subcommand.get_about() {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str())
        }

        buffer.push_str(template.as_str());
        buffer.push('\n');
    }

    // generate options of subcommands
    for subcommand in app.get_subcommands() {
        let mut parent_commands: Vec<_> = parent_commands.into();
        parent_commands.push(subcommand.get_name());
        gen_fish_inner(root_command, &parent_commands, subcommand, buffer);
    }
}

fn value_completion(option: &Arg) -> String {
    if !option.is_set(ArgSettings::TakesValue) {
        return "".to_string();
    }

    if let Some(data) = option.get_possible_values() {
        // We return the possible values with their own empty description e.g. {a\t,b\t}
        // this makes sure that a and b don't get the description of the option or argument
        format!(
            " -r -f -a \"{{{}}}\"",
            data.iter()
                .filter_map(|value| if value.is_hidden() {
                    None
                } else {
                    Some(format!(
                        "{}\t{}",
                        value.get_name(),
                        value.get_help().unwrap_or_default()
                    ))
                })
                .collect::<Vec<_>>()
                .join(",")
        )
    } else {
        // NB! If you change this, please also update the table in `ValueHint` documentation.
        match option.get_value_hint() {
            ValueHint::Unknown => " -r",
            // fish has no built-in support to distinguish these
            ValueHint::AnyPath | ValueHint::FilePath | ValueHint::ExecutablePath => " -r -F",
            ValueHint::DirPath => " -r -f -a \"(__fish_complete_directories)\"",
            // It seems fish has no built-in support for completing command + arguments as
            // single string (CommandString). Complete just the command name.
            ValueHint::CommandString | ValueHint::CommandName => {
                " -r -f -a \"(__fish_complete_command)\""
            }
            ValueHint::Username => " -r -f -a \"(__fish_complete_users)\"",
            ValueHint::Hostname => " -r -f -a \"(__fish_print_hostnames)\"",
            // Disable completion for others
            _ => " -r -f",
        }
        .to_string()
    }
}
