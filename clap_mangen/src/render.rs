use clap::{Arg, ArgAction};
use roff::{bold, italic, roman, Inline, Roff};

pub(crate) fn subcommand_heading(cmd: &clap::Command) -> &str {
    match cmd.get_subcommand_help_heading() {
        Some(title) => title,
        None => "SUBCOMMANDS",
    }
}

pub(crate) fn about(roff: &mut Roff, cmd: &clap::Command) {
    let name = cmd.get_display_name().unwrap_or_else(|| cmd.get_name());
    let s = match cmd.get_about().or_else(|| cmd.get_long_about()) {
        Some(about) => format!("{name} - {about}"),
        None => name.to_owned(),
    };
    roff.text([roman(s)]);
}

pub(crate) fn description(roff: &mut Roff, cmd: &clap::Command) {
    if let Some(about) = cmd.get_long_about().or_else(|| cmd.get_about()) {
        for line in about.to_string().lines() {
            if line.trim().is_empty() {
                roff.control("PP", []);
            } else {
                roff.text([roman(line)]);
            }
        }
    }
}

pub(crate) fn synopsis(roff: &mut Roff, cmd: &clap::Command) {
    let name = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());
    let mut line = vec![bold(name), roman(" ")];

    for opt in cmd.get_arguments().filter(|i| !i.is_hide_set()) {
        let (lhs, rhs) = option_markers(opt);
        match (opt.get_short(), opt.get_long()) {
            (Some(short), Some(long)) => {
                line.push(roman(lhs));
                line.push(bold(format!("-{short}")));
                line.push(roman("|"));
                line.push(bold(format!("--{long}",)));
                line.push(roman(rhs));
            }
            (Some(short), None) => {
                line.push(roman(lhs));
                line.push(bold(format!("-{short} ")));
                line.push(roman(rhs));
            }
            (None, Some(long)) => {
                line.push(roman(lhs));
                line.push(bold(format!("--{long}")));
                line.push(roman(rhs));
            }
            (None, None) => continue,
        };

        if matches!(opt.get_action(), ArgAction::Count) {
            line.push(roman("..."));
        }
        line.push(roman(" "));
    }

    for arg in cmd.get_positionals() {
        let (lhs, rhs) = option_markers(arg);
        line.push(roman(lhs));
        if let Some(value) = arg.get_value_names() {
            line.push(italic(value.join(" ")));
        } else {
            line.push(italic(arg.get_id().as_str()));
        }
        line.push(roman(rhs));
        line.push(roman(" "));
    }

    if cmd.has_subcommands() {
        let (lhs, rhs) = subcommand_markers(cmd);
        line.push(roman(lhs));
        line.push(italic(
            cmd.get_subcommand_value_name()
                .unwrap_or_else(|| subcommand_heading(cmd))
                .to_lowercase(),
        ));
        line.push(roman(rhs));
    }

    roff.text(line);
}

pub(crate) fn options(roff: &mut Roff, items: &[&Arg]) {
    for opt in items.iter().filter(|a| !a.is_positional()) {
        let mut header = match (opt.get_short(), opt.get_long()) {
            (Some(short), Some(long)) => {
                vec![short_option(short), roman(", "), long_option(long)]
            }
            (Some(short), None) => vec![short_option(short)],
            (None, Some(long)) => vec![long_option(long)],
            (None, None) => vec![],
        };

        let arg_range = opt.get_num_args().expect("built");
        if arg_range.takes_values() {
            if let Some(value_names) = &opt.get_value_names() {
                let (lhs, rhs) = option_value_markers(opt);

                header.push(roman(lhs));
                for (i, name) in value_names.iter().enumerate() {
                    if i > 0 {
                        header.push(italic(" "));
                    }

                    let mut val = format!("<{name}>");

                    // If this is the last value and it's variadic, add "..."
                    let is_last = i == value_names.len() - 1;

                    if is_last && arg_range.max_values() > value_names.len() {
                        val.push_str("...");
                    }
                    header.push(italic(val));
                }
                header.push(roman(rhs));
            }
        }

        if let Some(defs) = option_default_values(opt) {
            header.push(roman(" "));
            header.push(roman(defs));
        }

        let mut body = vec![];
        let mut arg_help_written = false;
        if let Some(help) = option_help(opt) {
            arg_help_written = true;
            body.push(roman(help.to_string()));
        }

        roff.control("TP", []);
        roff.text(header);
        roff.text(body);

        possible_options(roff, opt, arg_help_written);

        if let Some(env) = option_environment(opt) {
            roff.control("RS", []);
            roff.text(env);
            roff.control("RE", []);
        }
    }

    for pos in items.iter().filter(|a| a.is_positional()) {
        let mut header = vec![];
        let (lhs, rhs) = option_markers(pos);
        header.push(roman(lhs));
        if let Some(value) = pos.get_value_names() {
            header.push(italic(value.join(" ")));
        } else {
            header.push(italic(pos.get_id().as_str()));
        };
        header.push(roman(rhs));

        if let Some(defs) = option_default_values(pos) {
            header.push(roman(format!(" {defs}")));
        }

        let mut body = vec![];
        let mut arg_help_written = false;
        if let Some(help) = option_help(pos) {
            body.push(roman(help.to_string()));
            arg_help_written = true;
        }

        roff.control("TP", []);
        roff.text(header);
        roff.text(body);

        if let Some(env) = option_environment(pos) {
            roff.control("RS", []);
            roff.text(env);
            roff.control("RE", []);
        }

        possible_options(roff, pos, arg_help_written);
    }
}

fn possible_options(roff: &mut Roff, arg: &Arg, arg_help_written: bool) {
    if let Some((possible_values_text, with_help)) = get_possible_values(arg) {
        if arg_help_written {
            // It looks nice to have a separation between the help and the values
            roff.text([Inline::LineBreak]);
        }
        if with_help {
            roff.text([Inline::LineBreak, italic("Possible values:")]);

            // Need to indent twice to get it to look right, because .TP heading indents, but
            // that indent doesn't Carry over to the .IP for the bullets. The standard shift
            // size is 7 for terminal devices
            roff.control("RS", ["14"]);
            for line in possible_values_text {
                roff.control("IP", ["\\(bu", "2"]);
                roff.text([roman(line)]);
            }
            roff.control("RE", []);
        } else {
            let possible_value_text: Vec<Inline> = vec![
                Inline::LineBreak,
                roman("["),
                italic("possible values: "),
                roman(possible_values_text.join(", ")),
                roman("]"),
            ];
            roff.text(possible_value_text);
        }
    }
}

pub(crate) fn subcommands(roff: &mut Roff, cmd: &clap::Command, section: &str) {
    for sub in cmd.get_subcommands().filter(|s| !s.is_hide_set()) {
        roff.control("TP", []);

        let name = format!(
            "{}-{}({})",
            cmd.get_display_name().unwrap_or_else(|| cmd.get_name()),
            sub.get_name(),
            section
        );
        roff.text([roman(name)]);

        if let Some(about) = sub.get_about().or_else(|| sub.get_long_about()) {
            for line in about.to_string().lines() {
                roff.text([roman(line)]);
            }
        }
    }
}

pub(crate) fn version(cmd: &clap::Command) -> String {
    format!(
        "v{}",
        cmd.get_long_version()
            .or_else(|| cmd.get_version())
            .unwrap()
    )
}

pub(crate) fn after_help(roff: &mut Roff, cmd: &clap::Command) {
    if let Some(about) = cmd.get_after_long_help().or_else(|| cmd.get_after_help()) {
        let content = about.to_string();
        let lines: Vec<&str> = content.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();

            // Skip empty lines
            if line.is_empty() {
                roff.control("PP", []);
                i += 1;
                continue;
            }

            if let Some(item_content) = line.strip_prefix("- ") {
                // list item after removing "- "
                // Look for the pattern: "- OPTION PARAM description"
                if let Some((option, description)) = parse_list_item(item_content) {
                    roff.control("TP", []);
                    roff.text([bold(option)]);
                    roff.text([roman(description)]);
                } else {
                    // Fallback: just render as bold item if it doesn't confirm to our parsing rules
                    roff.control("TP", []);
                    roff.text([bold(item_content)]);
                }
            } else {
                // Regular paragraph text
                roff.text([roman(line)]);
            }

            i += 1;
        }
    }
}

/// Parse a list item to extract option and description
/// Examples:
/// "- -b FILE FILE exists and is block special" -> ("-b", "FILE FILE exists and is block special")
/// "- STRING equivalent to -n STRING" -> ("STRING", "equivalent to -n STRING")
fn parse_list_item(content: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = content.splitn(3, ' ').collect();

    match parts.as_slice() {
        [option, param, _description] => {
            // Check if this looks like an option with parameter
            // e.g. "-b FILE FILE exists and is block special"
            if option.starts_with('-') && param.chars().all(|c| c.is_uppercase() || c == '_') {
                Some((*option, &content[option.len() + param.len() + 2..]))
            } else {
                // e.g. "- STRING equivalent to -n STRING"
                Some((*option, &content[option.len() + 1..]))
            }
        }
        [option, description] => {
            // e.g. "- STRING equivalent to -n STRING"
            Some((*option, *description))
        }
        // anything else is not a list item
        _ => None,
    }
}

fn subcommand_markers(cmd: &clap::Command) -> (&'static str, &'static str) {
    markers(cmd.is_subcommand_required_set())
}

fn option_markers(opt: &Arg) -> (&'static str, &'static str) {
    markers(opt.is_required_set())
}

fn markers(required: bool) -> (&'static str, &'static str) {
    if required {
        ("<", ">")
    } else {
        ("[", "]")
    }
}

fn option_value_markers(arg: &Arg) -> (&'static str, &'static str) {
    let range = arg.get_num_args().expect("built");

    if !range.takes_values() {
        return ("", ""); // no value, so nothing to render
    }

    let required = range.min_values() > 0;
    let require_equals = arg.is_require_equals_set();

    match (required, require_equals) {
        // Required, no equals: <VALUE>
        (true, false) => (" ", ""),

        // Optional, no equals: [<VALUE>]
        (false, false) => (" [", "]"),

        // Optional, with equals: [=<VALUE>]
        (false, true) => ("[=", "]"),

        // Required, with equals
        (true, true) => ("=", ""),
    }
}

fn short_option(opt: char) -> Inline {
    bold(format!("-{opt}"))
}

fn long_option(opt: &str) -> Inline {
    bold(format!("--{opt}"))
}

fn option_help(opt: &Arg) -> Option<&clap::builder::StyledStr> {
    if !opt.is_hide_long_help_set() {
        let long_help = opt.get_long_help();
        if long_help.is_some() {
            return long_help;
        }
    }
    if !opt.is_hide_short_help_set() {
        return opt.get_help();
    }

    None
}

fn option_environment(opt: &Arg) -> Option<Vec<Inline>> {
    if opt.is_hide_env_set() {
        return None;
    } else if let Some(env) = opt.get_env() {
        return Some(vec![
            roman("May also be specified with the "),
            bold(env.to_string_lossy().into_owned()),
            roman(" environment variable. "),
        ]);
    }

    None
}

fn option_default_values(opt: &Arg) -> Option<String> {
    if opt.is_hide_default_value_set() || !opt.get_num_args().expect("built").takes_values() {
        return None;
    } else if !opt.get_default_values().is_empty() {
        let values = opt
            .get_default_values()
            .iter()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>()
            .join(",");

        return Some(format!("[default: {values}]"));
    }

    None
}

fn get_possible_values(arg: &Arg) -> Option<(Vec<String>, bool)> {
    if arg.is_hide_possible_values_set() {
        return None;
    }

    let possibles = &arg.get_possible_values();
    let possibles: Vec<&clap::builder::PossibleValue> =
        possibles.iter().filter(|pos| !pos.is_hide_set()).collect();

    if !possibles.is_empty() {
        return Some(format_possible_values(&possibles));
    }
    None
}

fn format_possible_values(possibles: &Vec<&clap::builder::PossibleValue>) -> (Vec<String>, bool) {
    let mut lines = vec![];
    let with_help = possibles.iter().any(|p| p.get_help().is_some());
    if with_help {
        for value in possibles {
            let val_name = value.get_name();
            match value.get_help() {
                Some(help) => lines.push(format!("{val_name}: {help}")),
                None => lines.push(val_name.to_string()),
            }
        }
    } else {
        lines.append(&mut possibles.iter().map(|p| p.get_name().to_string()).collect());
    }
    (lines, with_help)
}
