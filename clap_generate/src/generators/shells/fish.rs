// Std
use std::io::Write;

// Internal
use crate::Generator;
use clap::*;

/// Generate fish completion file
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

    for option in app.get_opts_no_heading() {
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
        }

        if let Some(data) = option.get_about() {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }

        if let Some(ref data) = option.get_possible_values() {
            template.push_str(format!(" -r -f -a \"{}\"", data.join(" ")).as_str());
        }

        buffer.push_str(template.as_str());
        buffer.push_str("\n");
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
        }

        if let Some(data) = flag.get_about() {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }

        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    for subcommand in app.get_subcommands() {
        let mut template = basic_template.clone();

        template.push_str(" -f");
        template.push_str(format!(" -a \"{}\"", &subcommand.get_name()).as_str());

        if let Some(data) = subcommand.get_about() {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str())
        }

        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    // generate options of subcommands
    for subcommand in app.get_subcommands() {
        gen_fish_inner(root_command, subcommand, buffer);
    }
}
