use clap::AppSettings;
use roff::{bold, italic, roman, Inline, Roff};

pub(crate) fn subcommand_heading(cmd: &clap::Command) -> String {
    match cmd.get_subcommand_help_heading() {
        Some(title) => title.to_string(),
        None => "SUBCOMMANDS".to_string(),
    }
}

pub(crate) fn about(roff: &mut Roff, cmd: &clap::Command) {
    let s = match cmd.get_about().or_else(|| cmd.get_long_about()) {
        Some(about) => format!("{} - {}", cmd.get_name(), about),
        None => cmd.get_name().to_string(),
    };
    roff.text([roman(&s)]);
}

pub(crate) fn description(roff: &mut Roff, cmd: &clap::Command) {
    if let Some(about) = cmd.get_long_about().or_else(|| cmd.get_about()) {
        for line in about.lines() {
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
                line.push(bold(&format!("-{}", short)));
                line.push(roman("|"));
                line.push(bold(&format!("--{}", long)));
                line.push(roman(rhs));
                line.push(roman(" "));
            }
            (Some(short), None) => {
                line.push(roman(lhs));
                line.push(bold(&format!("-{} ", short)));
                line.push(roman(rhs));
                line.push(roman(" "));
            }
            (None, Some(long)) => {
                line.push(roman(lhs));
                line.push(bold(&format!("--{}", long)));
                line.push(roman(rhs));
                line.push(roman(" "));
            }
            (None, None) => (),
        };
    }

    for arg in cmd.get_positionals() {
        let (lhs, rhs) = option_markers(arg);
        line.push(roman(lhs));
        if let Some(value) = arg.get_value_names() {
            line.push(italic(value.join(" ")));
        } else {
            line.push(italic(arg.get_id()));
        }
        line.push(roman(rhs));
        line.push(roman(" "));
    }

    if cmd.has_subcommands() {
        let (lhs, rhs) = subcommand_markers(cmd);
        line.push(roman(lhs));
        line.push(italic(
            &cmd.get_subcommand_value_name()
                .unwrap_or(&subcommand_heading(cmd))
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

        if let Some(value) = &opt.get_value_names() {
            header.push(roman("="));
            header.push(italic(&value.join(" ")));
        }

        if let Some(defs) = option_default_values(opt) {
            header.push(roman(" "));
            header.push(roman(&defs));
        }

        let mut body = vec![];
        if let Some(help) = opt.get_long_help().or_else(|| opt.get_help()) {
            body.push(roman(help));
        }

        roff.control("TP", []);
        roff.text(header);
        roff.text(body);

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
            header.push(italic(pos.get_id()));
        };
        header.push(roman(rhs));

        if let Some(defs) = option_default_values(pos) {
            header.push(roman(&format!(" {}", defs)));
        }

        let mut body = vec![];
        if let Some(help) = pos.get_long_help().or_else(|| pos.get_help()) {
            body.push(roman(&help.to_string()));
        }

        roff.control("TP", []);
        roff.text(header);
        roff.text(body);

        if let Some(env) = option_environment(pos) {
            roff.control("RS", []);
            roff.text(env);
            roff.control("RE", []);
        }
    }
}

pub(crate) fn subcommands(roff: &mut Roff, cmd: &clap::Command, section: &str) {
    for sub in cmd.get_subcommands().filter(|s| !s.is_hide_set()) {
        roff.control("TP", []);

        let name = format!("{}-{}({})", cmd.get_name(), sub.get_name(), section);
        roff.text([roman(&name)]);

        if let Some(about) = sub.get_about().or_else(|| sub.get_long_about()) {
            for line in about.lines() {
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
        for line in about.lines() {
            roff.text([roman(line)]);
        }
    }
}

fn subcommand_markers(cmd: &clap::Command) -> (&'static str, &'static str) {
    #[allow(deprecated)]
    markers(cmd.is_subcommand_required_set() || cmd.is_set(AppSettings::SubcommandRequiredElseHelp))
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
    bold(&format!("-{}", opt))
}

fn long_option(opt: &str) -> Inline {
    bold(&format!("--{}", opt))
}

fn option_environment(opt: &clap::Arg) -> Option<Vec<Inline>> {
    if opt.is_hide_env_set() {
        return None;
    } else if let Some(env) = opt.get_env() {
        return Some(vec![
            roman("May also be specified with the "),
            bold(env.to_string_lossy().to_owned()),
            roman(" environment variable. "),
        ]);
    }

    None
}

fn option_default_values(opt: &clap::Arg) -> Option<String> {
    if !opt.get_default_values().is_empty() {
        let values = opt
            .get_default_values()
            .iter()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>()
            .join(",");

        return Some(format!("[default: {}]", values));
    }

    None
}
