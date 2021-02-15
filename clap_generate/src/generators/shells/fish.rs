// Std
use std::io::Write;

// Internal
use crate::Generator;
use clap::*;

/// Generate fish completion file
///
/// Note: The fish generator currently only supports named options (-o/--option), not positional arguments.
pub struct Fish;

impl Generator for Fish {
    fn file_name(name: &str) -> String {
        format!("{}.fish", name)
    }

    fn generate(app: &App, buf: &mut dyn Write) {
        let command = app.get_bin_name().unwrap();
        let mut buffer = String::new();

        gen_fish_inner(command, app, &mut buffer);
        w!(buf, buffer.as_bytes());
    }
}

// Escape string inside single quotes
fn escape_string(string: &str) -> String {
    string.replace("\\", "\\\\").replace("'", "\\'")
}

fn gen_fish_inner(root_command: &str, app: &App, buffer: &mut String) {
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

    let mut basic_template = format!("complete -c {} -n ", root_command);
    let mut bin_name = app.get_bin_name().unwrap();

    if root_command == bin_name {
        basic_template.push_str("\"__fish_use_subcommand\"");
    } else {
        bin_name = &app.get_name();
        basic_template.push_str(format!("\"__fish_seen_subcommand_from {}\"", bin_name).as_str());
    }

    debug!("gen_fish_inner: bin_name={}", bin_name);

    for option in app.get_opts() {
        let mut template = basic_template.clone();

        if let Some(data) = option.get_short() {
            template.push_str(format!(" -s {}", data).as_str());

            if let Some(short_aliases) = option.get_visible_short_aliases() {
                for data in short_aliases {
                    template.push_str(format!(" -s {}", data).as_str());
                }
            }
        }

        if let Some(data) = option.get_long() {
            template.push_str(format!(" -l {}", data).as_str());

            if let Some(aliases) = option.get_visible_aliases() {
                for data in aliases {
                    template.push_str(format!(" -l {}", data).as_str());
                }
            }
        }

        if let Some(data) = option.get_about() {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }

        template.push_str(value_completion(option).as_str());

        buffer.push_str(template.as_str());
        buffer.push('\n');
    }

    for flag in Fish::flags(app) {
        let mut template = basic_template.clone();

        if let Some(data) = flag.get_short() {
            template.push_str(format!(" -s {}", data).as_str());

            if let Some(short_aliases) = flag.get_visible_short_aliases() {
                for data in short_aliases {
                    template.push_str(format!(" -s {}", data).as_str());
                }
            }
        }

        if let Some(data) = flag.get_long() {
            template.push_str(format!(" -l {}", data).as_str());

            if let Some(aliases) = flag.get_visible_aliases() {
                for data in aliases {
                    template.push_str(format!(" -l {}", data).as_str());
                }
            }
        }

        if let Some(data) = flag.get_about() {
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
        gen_fish_inner(root_command, subcommand, buffer);
    }
}

fn value_completion(option: &Arg) -> String {
    if !option.is_set(ArgSettings::TakesValue) {
        return "".to_string();
    }

    if let Some(ref data) = option.get_possible_values() {
        format!(" -r -f -a \"{}\"", data.join(" "))
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
