use std::io::Write;

use clap::*;

use crate::generator::{utils, Generator};
use crate::INTERNAL_ERROR_MSG;

/// Generate zsh completion file
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Zsh;

impl Generator for Zsh {
    fn file_name(&self, name: &str) -> String {
        format!("_{}", name)
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn Write) {
        let bin_name = cmd
            .get_bin_name()
            .expect("crate::generate should have set the bin_name");

        w!(
            buf,
            format!(
                "#compdef {name}

autoload -U is-at-least

_{name}() {{
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext=\"$curcontext\" state line
    {initial_args}{subcommands}
}}

{subcommand_details}

_{name} \"$@\"
",
                name = bin_name,
                initial_args = get_args_of(cmd, None),
                subcommands = get_subcommands_of(cmd),
                subcommand_details = subcommand_details(cmd)
            )
            .as_bytes()
        );
    }
}

// Displays the commands of a subcommand
// (( $+functions[_[bin_name_underscore]_commands] )) ||
// _[bin_name_underscore]_commands() {
//     local commands; commands=(
//         '[arg_name]:[arg_help]'
//     )
//     _describe -t commands '[bin_name] commands' commands "$@"
//
// Where the following variables are present:
//    [bin_name_underscore]: The full space delineated bin_name, where spaces have been replaced by
//                           underscore characters
//    [arg_name]: The name of the subcommand
//    [arg_help]: The help message of the subcommand
//    [bin_name]: The full space delineated bin_name
//
// Here's a snippet from rustup:
//
// (( $+functions[_rustup_commands] )) ||
// _rustup_commands() {
//     local commands; commands=(
//      'show:Show the active and installed toolchains'
//      'update:Update Rust toolchains'
//      # ... snip for brevity
//      'help:Print this message or the help of the given subcommand(s)'
//     )
//     _describe -t commands 'rustup commands' commands "$@"
//
fn subcommand_details(p: &Command) -> String {
    debug!("subcommand_details");

    let bin_name = p
        .get_bin_name()
        .expect("crate::generate should have set the bin_name");

    let mut ret = vec![];

    // First we do ourself
    let parent_text = format!(
        "\
(( $+functions[_{bin_name_underscore}_commands] )) ||
_{bin_name_underscore}_commands() {{
    local commands; commands=({subcommands_and_args})
    _describe -t commands '{bin_name} commands' commands \"$@\"
}}",
        bin_name_underscore = bin_name.replace(' ', "__"),
        bin_name = bin_name,
        subcommands_and_args = subcommands_of(p)
    );
    ret.push(parent_text);

    // Next we start looping through all the children, grandchildren, etc.
    let mut all_subcommands = utils::all_subcommands(p);

    all_subcommands.sort();
    all_subcommands.dedup();

    for &(_, ref bin_name) in &all_subcommands {
        debug!("subcommand_details:iter: bin_name={}", bin_name);

        ret.push(format!(
            "\
(( $+functions[_{bin_name_underscore}_commands] )) ||
_{bin_name_underscore}_commands() {{
    local commands; commands=({subcommands_and_args})
    _describe -t commands '{bin_name} commands' commands \"$@\"
}}",
            bin_name_underscore = bin_name.replace(' ', "__"),
            bin_name = bin_name,
            subcommands_and_args =
                subcommands_of(parser_of(p, bin_name).expect(INTERNAL_ERROR_MSG))
        ));
    }

    ret.join("\n")
}

// Generates subcommand completions in form of
//
//         '[arg_name]:[arg_help]'
//
// Where:
//    [arg_name]: the subcommand's name
//    [arg_help]: the help message of the subcommand
//
// A snippet from rustup:
//         'show:Show the active and installed toolchains'
//      'update:Update Rust toolchains'
fn subcommands_of(p: &Command) -> String {
    debug!("subcommands_of");

    let mut segments = vec![];

    fn add_subcommands(subcommand: &Command, name: &str, ret: &mut Vec<String>) {
        debug!("add_subcommands");

        let text = format!(
            "'{name}:{help}' \\",
            name = name,
            help = escape_help(subcommand.get_about().unwrap_or(""))
        );

        if !text.is_empty() {
            ret.push(text);
        }
    }

    // The subcommands
    for command in p.get_subcommands() {
        debug!("subcommands_of:iter: subcommand={}", command.get_name());

        add_subcommands(command, command.get_name(), &mut segments);

        for alias in command.get_visible_aliases() {
            add_subcommands(command, alias, &mut segments);
        }
    }

    // Surround the text with newlines for proper formatting.
    // We need this to prevent weirdly formatted `command=(\n        \n)` sections.
    // When there are no (sub-)commands.
    if !segments.is_empty() {
        segments.insert(0, "".to_string());
        segments.push("    ".to_string());
    }

    segments.join("\n")
}

// Get's the subcommand section of a completion file
// This looks roughly like:
//
// case $state in
// ([bin_name]_args)
//     curcontext=\"${curcontext%:*:*}:[name_hyphen]-command-$words[1]:\"
//     case $line[1] in
//
//         ([name])
//         _arguments -C -s -S \
//             [subcommand_args]
//         && ret=0
//
//         [RECURSIVE_CALLS]
//
//         ;;",
//
//         [repeat]
//
//     esac
// ;;
// esac",
//
// Where the following variables are present:
//    [name] = The subcommand name in the form of "install" for "rustup toolchain install"
//    [bin_name] = The full space delineated bin_name such as "rustup toolchain install"
//    [name_hyphen] = The full space delineated bin_name, but replace spaces with hyphens
//    [repeat] = From the same recursive calls, but for all subcommands
//    [subcommand_args] = The same as zsh::get_args_of
fn get_subcommands_of(parent: &Command) -> String {
    debug!(
        "get_subcommands_of: Has subcommands...{:?}",
        parent.has_subcommands()
    );

    if !parent.has_subcommands() {
        return String::new();
    }

    let subcommand_names = utils::subcommands(parent);
    let mut all_subcommands = vec![];

    for &(ref name, ref bin_name) in &subcommand_names {
        debug!(
            "get_subcommands_of:iter: parent={}, name={}, bin_name={}",
            parent.get_name(),
            name,
            bin_name,
        );
        let mut segments = vec![format!("({})", name)];
        let subcommand_args = get_args_of(
            parser_of(parent, &*bin_name).expect(INTERNAL_ERROR_MSG),
            Some(parent),
        );

        if !subcommand_args.is_empty() {
            segments.push(subcommand_args);
        }

        // Get the help text of all child subcommands.
        let children = get_subcommands_of(parser_of(parent, &*bin_name).expect(INTERNAL_ERROR_MSG));

        if !children.is_empty() {
            segments.push(children);
        }

        segments.push(String::from(";;"));
        all_subcommands.push(segments.join("\n"));
    }

    let parent_bin_name = parent
        .get_bin_name()
        .expect("crate::generate should have set the bin_name");

    format!(
        "
    case $state in
    ({name})
        words=($line[{pos}] \"${{words[@]}}\")
        (( CURRENT += 1 ))
        curcontext=\"${{curcontext%:*:*}}:{name_hyphen}-command-$line[{pos}]:\"
        case $line[{pos}] in
            {subcommands}
        esac
    ;;
esac",
        name = parent.get_name(),
        name_hyphen = parent_bin_name.replace(' ', "-"),
        subcommands = all_subcommands.join("\n"),
        pos = parent.get_positionals().count() + 1
    )
}

// Get the Command for a given subcommand tree.
//
// Given the bin_name "a b c" and the Command for "a" this returns the "c" Command.
// Given the bin_name "a b c" and the Command for "b" this returns the "c" Command.
fn parser_of<'help, 'cmd>(
    parent: &'cmd Command<'help>,
    bin_name: &str,
) -> Option<&'cmd Command<'help>> {
    debug!("parser_of: p={}, bin_name={}", parent.get_name(), bin_name);

    if bin_name == parent.get_bin_name().unwrap_or(&String::new()) {
        return Some(parent);
    }

    for subcommand in parent.get_subcommands() {
        if let Some(ret) = parser_of(subcommand, bin_name) {
            return Some(ret);
        }
    }

    None
}

// Writes out the args section, which ends up being the flags, opts and positionals, and a jump to
// another ZSH function if there are subcommands.
// The structure works like this:
//    ([conflicting_args]) [multiple] arg [takes_value] [[help]] [: :(possible_values)]
//       ^-- list '-v -h'    ^--'*'          ^--'+'                   ^-- list 'one two three'
//
// An example from the rustup command:
//
// _arguments -C -s -S \
//         '(-h --help --verbose)-v[Enable verbose output]' \
//         '(-V -v --version --verbose --help)-h[Print help information]' \
//      # ... snip for brevity
//         ':: :_rustup_commands' \    # <-- displays subcommands
//         '*::: :->rustup' \          # <-- displays subcommand args and child subcommands
//     && ret=0
//
// The args used for _arguments are as follows:
//    -C: modify the $context internal variable
//    -s: Allow stacking of short args (i.e. -a -b -c => -abc)
//    -S: Do not complete anything after '--' and treat those as argument values
fn get_args_of(parent: &Command, p_global: Option<&Command>) -> String {
    debug!("get_args_of");

    let mut segments = vec![String::from("_arguments \"${_arguments_options[@]}\" \\")];
    let opts = write_opts_of(parent, p_global);
    let flags = write_flags_of(parent, p_global);
    let positionals = write_positionals_of(parent);

    if !opts.is_empty() {
        segments.push(opts);
    }

    if !flags.is_empty() {
        segments.push(flags);
    }

    if !positionals.is_empty() {
        segments.push(positionals);
    }

    if parent.has_subcommands() {
        let parent_bin_name = parent
            .get_bin_name()
            .expect("crate::generate should have set the bin_name");
        let subcommand_bin_name = format!(
            "\":: :_{name}_commands\" \\",
            name = parent_bin_name.replace(' ', "__")
        );
        segments.push(subcommand_bin_name);

        let subcommand_text = format!("\"*::: :->{name}\" \\", name = parent.get_name());
        segments.push(subcommand_text);
    };

    segments.push(String::from("&& ret=0"));
    segments.join("\n")
}

// Uses either `possible_vals` or `value_hint` to give hints about possible argument values
fn value_completion(arg: &Arg) -> Option<String> {
    if let Some(values) = &arg.get_possible_values() {
        if values
            .iter()
            .any(|value| !value.is_hide_set() && value.get_help().is_some())
        {
            Some(format!(
                "(({}))",
                values
                    .iter()
                    .filter_map(|value| {
                        if value.is_hide_set() {
                            None
                        } else {
                            Some(format!(
                                r#"{name}\:"{tooltip}""#,
                                name = escape_value(value.get_name()),
                                tooltip = value.get_help().map(escape_help).unwrap_or_default()
                            ))
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            ))
        } else {
            Some(format!(
                "({})",
                values
                    .iter()
                    .filter(|pv| !pv.is_hide_set())
                    .map(PossibleValue::get_name)
                    .collect::<Vec<_>>()
                    .join(" ")
            ))
        }
    } else {
        // NB! If you change this, please also update the table in `ValueHint` documentation.
        Some(
            match arg.get_value_hint() {
                ValueHint::Unknown => {
                    return None;
                }
                ValueHint::Other => "( )",
                ValueHint::AnyPath => "_files",
                ValueHint::FilePath => "_files",
                ValueHint::DirPath => "_files -/",
                ValueHint::ExecutablePath => "_absolute_command_paths",
                ValueHint::CommandName => "_command_names -e",
                ValueHint::CommandString => "_cmdstring",
                ValueHint::CommandWithArguments => "_cmdambivalent",
                ValueHint::Username => "_users",
                ValueHint::Hostname => "_hosts",
                ValueHint::Url => "_urls",
                ValueHint::EmailAddress => "_email_addresses",
                _ => {
                    return None;
                }
            }
            .to_string(),
        )
    }
}

/// Escape help string inside single quotes and brackets
fn escape_help(string: &str) -> String {
    string
        .replace('\\', "\\\\")
        .replace('\'', "'\\''")
        .replace('[', "\\[")
        .replace(']', "\\]")
}

/// Escape value string inside single quotes and parentheses
fn escape_value(string: &str) -> String {
    string
        .replace('\\', "\\\\")
        .replace('\'', "'\\''")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace(' ', "\\ ")
}

fn write_opts_of(p: &Command, p_global: Option<&Command>) -> String {
    debug!("write_opts_of");

    let mut ret = vec![];

    for o in p.get_opts() {
        debug!("write_opts_of:iter: o={}", o.get_id());

        let help = o.get_help().map_or(String::new(), escape_help);
        let conflicts = arg_conflicts(p, o, p_global);

        let multiple = if o.is_multiple_occurrences_set() {
            "*"
        } else {
            ""
        };

        let vn = match o.get_value_names() {
            None => " ".to_string(),
            Some(val) => val[0].to_string(),
        };
        let vc = match value_completion(o) {
            Some(val) => format!(":{}:{}", vn, val),
            None => format!(":{}: ", vn),
        };
        let vc = match o.get_num_vals() {
            Some(num_vals) => vc.repeat(num_vals),
            None => vc,
        };

        if let Some(shorts) = o.get_short_and_visible_aliases() {
            for short in shorts {
                let s = format!(
                    "'{conflicts}{multiple}-{arg}+[{help}]{value_completion}' \\",
                    conflicts = conflicts,
                    multiple = multiple,
                    arg = short,
                    value_completion = vc,
                    help = help
                );

                debug!("write_opts_of:iter: Wrote...{}", &*s);
                ret.push(s);
            }
        }
        if let Some(longs) = o.get_long_and_visible_aliases() {
            for long in longs {
                let l = format!(
                    "'{conflicts}{multiple}--{arg}=[{help}]{value_completion}' \\",
                    conflicts = conflicts,
                    multiple = multiple,
                    arg = long,
                    value_completion = vc,
                    help = help
                );

                debug!("write_opts_of:iter: Wrote...{}", &*l);
                ret.push(l);
            }
        }
    }

    ret.join("\n")
}

fn arg_conflicts(cmd: &Command, arg: &Arg, app_global: Option<&Command>) -> String {
    fn push_conflicts(conflicts: &[&Arg], res: &mut Vec<String>) {
        for conflict in conflicts {
            if let Some(s) = conflict.get_short() {
                res.push(format!("-{}", s));
            }

            if let Some(l) = conflict.get_long() {
                res.push(format!("--{}", l));
            }
        }
    }

    let mut res = vec![];
    match (app_global, arg.is_global_set()) {
        (Some(x), true) => {
            let conflicts = x.get_arg_conflicts_with(arg);

            if conflicts.is_empty() {
                return String::new();
            }

            push_conflicts(&conflicts, &mut res);
        }
        (_, _) => {
            let conflicts = cmd.get_arg_conflicts_with(arg);

            if conflicts.is_empty() {
                return String::new();
            }

            push_conflicts(&conflicts, &mut res);
        }
    };

    format!("({})", res.join(" "))
}

fn write_flags_of(p: &Command, p_global: Option<&Command>) -> String {
    debug!("write_flags_of;");

    let mut ret = vec![];

    for f in utils::flags(p) {
        debug!("write_flags_of:iter: f={}", f.get_id());

        let help = f.get_help().map_or(String::new(), escape_help);
        let conflicts = arg_conflicts(p, &f, p_global);

        let multiple = if f.is_multiple_occurrences_set() {
            "*"
        } else {
            ""
        };

        if let Some(short) = f.get_short() {
            let s = format!(
                "'{conflicts}{multiple}-{arg}[{help}]' \\",
                multiple = multiple,
                conflicts = conflicts,
                arg = short,
                help = help
            );

            debug!("write_flags_of:iter: Wrote...{}", &*s);

            ret.push(s);

            if let Some(short_aliases) = f.get_visible_short_aliases() {
                for alias in short_aliases {
                    let s = format!(
                        "'{conflicts}{multiple}-{arg}[{help}]' \\",
                        multiple = multiple,
                        conflicts = conflicts,
                        arg = alias,
                        help = help
                    );

                    debug!("write_flags_of:iter: Wrote...{}", &*s);

                    ret.push(s);
                }
            }
        }

        if let Some(long) = f.get_long() {
            let l = format!(
                "'{conflicts}{multiple}--{arg}[{help}]' \\",
                conflicts = conflicts,
                multiple = multiple,
                arg = long,
                help = help
            );

            debug!("write_flags_of:iter: Wrote...{}", &*l);

            ret.push(l);

            if let Some(aliases) = f.get_visible_aliases() {
                for alias in aliases {
                    let l = format!(
                        "'{conflicts}{multiple}--{arg}[{help}]' \\",
                        conflicts = conflicts,
                        multiple = multiple,
                        arg = alias,
                        help = help
                    );

                    debug!("write_flags_of:iter: Wrote...{}", &*l);

                    ret.push(l);
                }
            }
        }
    }

    ret.join("\n")
}

fn write_positionals_of(p: &Command) -> String {
    debug!("write_positionals_of;");

    let mut ret = vec![];

    for arg in p.get_positionals() {
        debug!("write_positionals_of:iter: arg={}", arg.get_id());

        let cardinality = if arg.is_multiple_values_set() || arg.is_multiple_occurrences_set() {
            "*:"
        } else if !arg.is_required_set() {
            ":"
        } else {
            ""
        };

        let a = format!(
            "'{cardinality}:{name}{help}:{value_completion}' \\",
            cardinality = cardinality,
            name = arg.get_id(),
            help = arg
                .get_help()
                .map_or("".to_owned(), |v| " -- ".to_owned() + v)
                .replace('[', "\\[")
                .replace(']', "\\]")
                .replace('\'', "'\\''")
                .replace(':', "\\:"),
            value_completion = value_completion(arg).unwrap_or_else(|| "".to_string())
        );

        debug!("write_positionals_of:iter: Wrote...{}", a);

        ret.push(a);
    }

    ret.join("\n")
}
