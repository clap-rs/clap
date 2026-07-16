use std::io::{Error, Write};

use clap::{Arg, Command, ValueHint, builder};

use crate::generator::{Generator, utils};

/// Generate fish completion file
///
/// Positional arguments with explicit possible values are completed as well as named options.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Fish;

impl Generator for Fish {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.fish")
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn Write) {
        self.try_generate(cmd, buf)
            .expect("failed to write completion file");
    }

    fn try_generate(&self, cmd: &Command, buf: &mut dyn Write) -> Result<(), Error> {
        let bin_name = cmd
            .get_bin_name()
            .expect("crate::generate should have set the bin_name");

        let name = escape_name(bin_name);
        let mut needs_fn_name = &format!("__fish_{name}_needs_command")[..];
        let mut using_fn_name = &format!("__fish_{name}_using_subcommand")[..];
        // Given `git --git-dir somedir status`, using `__fish_seen_subcommand_from` won't help us
        // find out `status` is the real subcommand, and not `somedir`. However, when there are no subcommands,
        // there is no need to use our custom stubs.
        if cmd.has_subcommands() {
            gen_subcommand_helpers(&name, cmd, buf, needs_fn_name, using_fn_name);
        } else {
            needs_fn_name = "__fish_use_subcommand";
            using_fn_name = "__fish_seen_subcommand_from";
        }

        let mut buffer = String::new();
        let mut positional_helper_index = 0;
        gen_fish_inner(
            bin_name,
            &[],
            cmd,
            &mut buffer,
            needs_fn_name,
            using_fn_name,
            &mut positional_helper_index,
        );
        write!(buf, "{buffer}")
    }
}

// Escape string inside single quotes
fn escape_string(string: &str, escape_comma: bool) -> String {
    let string = string.replace('\\', "\\\\").replace('\'', "\\'");
    if escape_comma {
        string.replace(',', "\\,")
    } else {
        string
    }
}

fn escape_help(help: &builder::StyledStr) -> String {
    escape_string(&help.to_string().replace('\n', " "), false)
}

fn escape_name(name: &str) -> String {
    name.replace('-', "_")
}

fn gen_fish_inner(
    root_command: &str,
    parent_commands: &[&str],
    cmd: &Command,
    buffer: &mut String,
    needs_fn_name: &str,
    using_fn_name: &str,
    positional_helper_index: &mut usize,
) {
    debug!("gen_fish_inner");
    // HACK: Assuming subcommands are only nested less than 3 levels as more than that is
    // unwieldy and takes more effort to support.
    // For example, `rustup toolchain help install` is the longest valid command line of `rustup`
    // that uses nested subcommands, and it cannot receive any flags to it.
    if parent_commands.len() > 2 {
        return;
    }
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
    //      -n "{needs_fn_name}"            # complete for command "myprog"
    //      -n "{using_fn_name} subcmd1"    # complete for command "myprog subcmd1"

    let mut basic_template = format!("complete -c {root_command}");
    add_command_condition(
        &mut basic_template,
        parent_commands,
        cmd,
        needs_fn_name,
        using_fn_name,
    );

    debug!("gen_fish_inner: parent_commands={parent_commands:?}");

    for option in cmd.get_opts() {
        let mut template = basic_template.clone();

        if let Some(shorts) = option.get_short_and_visible_aliases() {
            for short in shorts {
                template.push_str(format!(" -s {short}").as_str());
            }
        }

        if let Some(longs) = option.get_long_and_visible_aliases() {
            for long in longs {
                template.push_str(format!(" -l {}", escape_string(long, false)).as_str());
            }
        }

        if let Some(data) = option.get_help() {
            template.push_str(&format!(" -d '{}'", escape_help(data)));
        }

        template.push_str(value_completion(option).as_str());

        buffer.push_str(template.as_str());
        buffer.push('\n');
    }

    for flag in utils::flags(cmd) {
        let mut template = basic_template.clone();

        if let Some(shorts) = flag.get_short_and_visible_aliases() {
            for short in shorts {
                template.push_str(format!(" -s {short}").as_str());
            }
        }

        if let Some(longs) = flag.get_long_and_visible_aliases() {
            for long in longs {
                template.push_str(format!(" -l {}", escape_string(long, false)).as_str());
            }
        }

        if let Some(data) = flag.get_help() {
            template.push_str(&format!(" -d '{}'", escape_help(data)));
        }

        buffer.push_str(template.as_str());
        buffer.push('\n');
    }

    let positional_helper_name = format!(
        "__fish_{}_has_positional_{}",
        escape_name(root_command),
        *positional_helper_index
    );
    let mut has_positional_values = false;
    for (position, positional) in cmd.get_positionals().enumerate() {
        if utils::possible_values(positional).is_none() {
            continue;
        }

        if !has_positional_values {
            gen_positional_helper(
                root_command,
                parent_commands,
                cmd,
                buffer,
                &positional_helper_name,
            );
            *positional_helper_index += 1;
            has_positional_values = true;
        }

        let mut template = format!("complete -c {root_command}");
        if parent_commands.is_empty() {
            template.push_str(&format!(" -n \"{positional_helper_name} {position}\""));
        } else {
            add_command_condition(
                &mut template,
                parent_commands,
                cmd,
                needs_fn_name,
                using_fn_name,
            );
            let condition = template
                .strip_prefix(&format!("complete -c {root_command} -n \""))
                .and_then(|template| template.strip_suffix('"'))
                .expect("positionals use a command condition");
            template = format!(
                "complete -c {root_command} -n \"{condition}; and {positional_helper_name} {position}\""
            );
        }
        template.push_str(value_completion(positional).as_str());
        buffer.push_str(template.as_str());
        buffer.push('\n');
    }

    let has_positionals = cmd.get_positionals().next().is_some();
    if !has_positionals {
        basic_template.push_str(" -f");
    }
    for subcommand in cmd.get_subcommands() {
        for subcommand_name in subcommand.get_name_and_visible_aliases() {
            let mut template = basic_template.clone();

            template.push_str(format!(" -a \"{subcommand_name}\"").as_str());

            if let Some(data) = subcommand.get_about() {
                template.push_str(format!(" -d '{}'", escape_help(data)).as_str());
            }

            buffer.push_str(template.as_str());
            buffer.push('\n');
        }
    }

    // generate options of subcommands
    for subcommand in cmd.get_subcommands() {
        for subcommand_name in subcommand.get_name_and_visible_aliases() {
            let mut parent_commands: Vec<_> = parent_commands.into();
            parent_commands.push(subcommand_name);
            gen_fish_inner(
                root_command,
                &parent_commands,
                subcommand,
                buffer,
                needs_fn_name,
                using_fn_name,
                positional_helper_index,
            );
        }
    }
}

fn add_command_condition(
    template: &mut String,
    parent_commands: &[&str],
    cmd: &Command,
    needs_fn_name: &str,
    using_fn_name: &str,
) {
    if parent_commands.is_empty() {
        if cmd.has_subcommands() {
            template.push_str(&format!(" -n \"{needs_fn_name}\""));
        }
        return;
    }

    let mut out = String::from(using_fn_name);
    match parent_commands {
        [] => unreachable!(),
        [command] => {
            out.push_str(&format!(" {command}"));
            if cmd.has_subcommands() {
                out.push_str("; and not __fish_seen_subcommand_from");
            }
            let subcommands = cmd
                .get_subcommands()
                .flat_map(Command::get_name_and_visible_aliases);
            for name in subcommands {
                out.push_str(&format!(" {name}"));
            }
        }
        [command, subcommand] => out.push_str(&format!(
            " {command}; and __fish_seen_subcommand_from {subcommand}"
        )),
        _ => unreachable!(),
    }
    template.push_str(format!(" -n \"{out}\"").as_str());
}

fn gen_positional_helper(
    root_command: &str,
    parent_commands: &[&str],
    cmd: &Command,
    buffer: &mut String,
    helper_name: &str,
) {
    let mut body = format!(
        "function {helper_name}\n    set -l expected $argv[1]\n    set -l cmd (commandline -opc)\n    set -e cmd[1]\n"
    );

    if parent_commands.is_empty() {
        append_argparse(&mut body, &command_optspec(cmd), "cmd");
        if cmd.has_subcommands() {
            let subcommands = cmd
                .get_subcommands()
                .flat_map(Command::get_name_and_visible_aliases)
                .collect::<Vec<_>>()
                .join(" ");
            body.push_str(&format!(
                "    contains -- $argv[1] {subcommands}; and return 1\n"
            ));
        }
    } else {
        let global_optspec_fn = format!("__fish_{}_global_optspecs", escape_name(root_command));
        body.push_str(&format!(
            "    argparse -s ({global_optspec_fn}) -- $cmd 2>/dev/null\n    or return\n    set cmd $argv\n"
        ));
        for command in parent_commands {
            body.push_str(&format!(
                "    test \"$cmd[1]\" = \"{command}\"; or return\n    set -e cmd[1]\n"
            ));
        }
        append_argparse(&mut body, &command_optspec(cmd), "cmd");
    }

    body.push_str("    test (count $argv) -eq $expected\nend\n\n");
    buffer.push_str(&body);
}

fn append_argparse(buffer: &mut String, optspec: &str, command: &str) {
    if optspec.is_empty() {
        buffer.push_str(&format!("    set argv ${command}\n"));
    } else {
        buffer.push_str(&format!(
            "    argparse -s (string join \\n {optspec}) -- ${command} 2>/dev/null\n    or return\n"
        ));
    }
}

fn command_optspec(cmd: &Command) -> String {
    let mut optspecs = String::new();
    for option in cmd.get_arguments().filter(|arg| !arg.is_positional()) {
        optspecs.push(' ');
        let mut has_short = false;
        if let Some(short) = option.get_short() {
            has_short = true;
            optspecs.push(short);
        }

        if let Some(long) = option.get_long() {
            if has_short {
                optspecs.push('/');
            }
            optspecs.push_str(&escape_string(long, false));
        }

        if option
            .get_num_args()
            .map(|range| range.takes_values())
            .unwrap_or(true)
        {
            optspecs.push('=');
        }
    }
    optspecs
}

/// Print fish's helpers for easy handling subcommands.
fn gen_subcommand_helpers(
    bin_name: &str,
    cmd: &Command,
    buf: &mut dyn Write,
    needs_fn_name: &str,
    using_fn_name: &str,
) {
    let optspecs = command_optspec(cmd);
    let optspecs_fn_name = format!("__fish_{bin_name}_global_optspecs");
    write!(
        buf,
        "# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function {optspecs_fn_name}
    string join \\n{optspecs}
end

function {needs_fn_name}
    # Figure out if the current invocation already has a command.
    set -l cmd (commandline -opc)
    set -e cmd[1]
    argparse -s ({optspecs_fn_name}) -- $cmd 2>/dev/null
    or return
    if set -q argv[1]
        # Also print the command, so this can be used to figure out what it is.
        echo $argv[1]
        return 1
    end
    return 0
end

function {using_fn_name}
    set -l cmd ({needs_fn_name})
    test -z \"$cmd\"
    and return 1
    contains -- $cmd[1] $argv
end

"
        ).expect("failed to write completion file");
}

fn value_completion(option: &Arg) -> String {
    if !option.get_num_args().expect("built").takes_values() {
        return "".to_owned();
    }

    if let Some(data) = utils::possible_values(option) {
        // We return the possible values with their own empty description e.g. "a\t''\nb\t''"
        // this makes sure that a and b don't get the description of the option or argument
        format!(
            " -r -f -a \"{}\"",
            data.iter()
                .filter_map(|value| if value.is_hide_set() {
                    None
                } else {
                    // The help text after \t is wrapped in '' to make sure that the it is taken literally
                    // and there is no command substitution or variable expansion resulting in unexpected errors
                    Some(format!(
                        "{}\\t'{}'",
                        escape_string(value.get_name(), true).as_str(),
                        escape_help(value.get_help().unwrap_or_default())
                    ))
                })
                .collect::<Vec<_>>()
                .join("\n")
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
        .to_owned()
    }
}
