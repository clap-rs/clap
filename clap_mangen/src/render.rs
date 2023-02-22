use clap::ArgAction;
use roff::{bold, italic, roman, Inline, Roff};

pub(crate) fn subcommand_heading(cmd: &clap::Command) -> &str {
    match cmd.get_subcommand_help_heading() {
        Some(title) => title,
        None => "SUBCOMMANDS",
    }
}

pub(crate) fn about(roff: &mut Roff, cmd: &clap::Command) {
    let s = match cmd.get_about().or_else(|| cmd.get_long_about()) {
        Some(about) => format!("{} - {}", cmd.get_name(), about),
        None => cmd.get_name().to_string(),
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
    let mut line = vec![bold(cmd.get_name()), roman(" ")];

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
            line.push(roman("..."))
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

pub(crate) fn options(roff: &mut Roff, cmd: &clap::Command) {
    let items: Vec<_> = cmd.get_arguments().filter(|i| !i.is_hide_set()).collect();

    for opt in items.iter().filter(|a| !a.is_positional()) {
        let mut header = match (opt.get_short(), opt.get_long()) {
            (Some(short), Some(long)) => {
                vec![short_option(short), roman(", "), long_option(long)]
            }
            (Some(short), None) => vec![short_option(short)],
            (None, Some(long)) => vec![long_option(long)],
            (None, None) => vec![],
        };

        if opt.get_action().takes_values() {
            if let Some(value) = &opt.get_value_names() {
                header.push(roman("="));
                header.push(italic(value.join(" ")));
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

        if let Some((possible_values_text, with_help)) = get_possible_values(opt) {
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
        // If possible options are available
        if let Some((possible_values_text, with_help)) = get_possible_values(pos) {
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
        for line in about.to_string().lines() {
            roff.text([roman(line)]);
        }
    }
}

fn subcommand_markers(cmd: &clap::Command) -> (&'static str, &'static str) {
    markers(cmd.is_subcommand_required_set())
}

fn option_markers(opt: &clap::Arg) -> (&'static str, &'static str) {
    markers(opt.is_required_set())
}

fn markers(required: bool) -> (&'static str, &'static str) {
    if required {
        ("<", ">")
    } else {
        ("[", "]")
    }
}

fn short_option(opt: char) -> Inline {
    bold(format!("-{opt}"))
}

fn long_option(opt: &str) -> Inline {
    bold(format!("--{opt}"))
}

fn option_help(opt: &clap::Arg) -> Option<&clap::builder::StyledStr> {
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

fn option_environment(opt: &clap::Arg) -> Option<Vec<Inline>> {
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

fn option_default_values(opt: &clap::Arg) -> Option<String> {
    if opt.is_hide_default_value_set() || !opt.get_action().takes_values() {
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

fn get_possible_values(arg: &clap::Arg) -> Option<(Vec<String>, bool)> {
    let possibles = &arg.get_possible_values();
    let possibles: Vec<&clap::builder::PossibleValue> =
        possibles.iter().filter(|pos| !pos.is_hide_set()).collect();

    if !(possibles.is_empty() || arg.is_hide_possible_values_set()) {
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
