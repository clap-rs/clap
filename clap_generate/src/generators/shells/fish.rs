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

        gen_fish_inner(command, app, command, &mut buffer);
        w!(buf, buffer.as_bytes());
    }
}

// Escape string inside single quotes
fn escape_string(string: &str) -> String {
    string.replace("\\", "\\\\").replace("'", "\\'")
}

fn gen_fish_inner(root_command: &str, app: &App, subcommand: &str, buffer: &mut String) {
    debugln!("Fish::gen_fish_inner;");
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

    if root_command == subcommand {
        basic_template.push_str("\"__fish_use_subcommand\"");
    } else {
        basic_template.push_str(format!("\"__fish_seen_subcommand_from {}\"", subcommand).as_str());
    }

    for option in opts!(app) {
        let mut template = basic_template.clone();

        if let Some(data) = option.short {
            template.push_str(format!(" -s {}", data).as_str());
        }

        if let Some(data) = option.long {
            template.push_str(format!(" -l {}", data).as_str());
        }

        if let Some(data) = option.help {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }

        if let Some(ref data) = option.possible_vals {
            template.push_str(format!(" -r -f -a \"{}\"", data.join(" ")).as_str());
        }

        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    for flag in flags!(app) {
        let mut template = basic_template.clone();

        if let Some(data) = flag.short {
            template.push_str(format!(" -s {}", data).as_str());
        }

        if let Some(data) = flag.long {
            template.push_str(format!(" -l {}", data).as_str());
        }

        if let Some(data) = flag.help {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str());
        }

        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    for subcommand in subcommands!(app) {
        let mut template = basic_template.clone();

        template.push_str(" -f");
        template.push_str(format!(" -a \"{}\"", &subcommand.name).as_str());

        if let Some(data) = subcommand.about {
            template.push_str(format!(" -d '{}'", escape_string(data)).as_str())
        }

        buffer.push_str(template.as_str());
        buffer.push_str("\n");
    }

    // generate options of subcommands
    for subapp in &app.subcommands {
        gen_fish_inner(root_command, subapp, &subapp.name, buffer);
    }
}
