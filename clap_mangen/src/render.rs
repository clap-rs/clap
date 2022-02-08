use clap::{AppSettings, ArgSettings};
use roff::{bold, italic, roman, Inline, Roff};

pub(crate) fn subcommand_heading(app: &clap::App) -> String {
    match app.get_subommand_help_heading() {
        Some(title) => title.to_string(),
        None => "SUBCOMMANDS".to_string(),
    }
}

pub(crate) fn about(roff: &mut Roff, app: &clap::App) {
    let s = match app.get_about().or_else(|| app.get_long_about()) {
        Some(about) => format!("{} - {}", app.get_name(), about),
        None => app.get_name().to_string(),
    };
    roff.text([roman(&s)]);
}

pub(crate) fn description(roff: &mut Roff, app: &clap::App) {
    if let Some(about) = app.get_long_about().or_else(|| app.get_about()) {
        for line in about.lines() {
            if line.trim().is_empty() {
                roff.control("PP", []);
            } else {
                roff.text([roman(line)]);
            }
        }
    }
}

pub(crate) fn synopsis(roff: &mut Roff, app: &clap::App) {
    let mut line = vec![bold(app.get_name()), roman(" ")];

    for opt in app.get_arguments() {
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

    for arg in app.get_positionals() {
        let (lhs, rhs) = option_markers(arg);
        line.push(roman(lhs));
        line.push(italic(arg.get_name()));
        line.push(roman(rhs));
        line.push(roman(" "));
    }

    if app.has_subcommands() {
        let (lhs, rhs) = subcommand_markers(app);
        line.push(roman(lhs));
        line.push(italic(
            &app.get_subcommand_value_name()
                .unwrap_or(&subcommand_heading(app))
                .to_lowercase(),
        ));
        line.push(roman(rhs));
    }

    roff.text(line);
}

pub(crate) fn options(roff: &mut Roff, app: &clap::App) {
    let items: Vec<_> = app
        .get_arguments()
        .filter(|i| !i.is_set(ArgSettings::Hidden))
        .collect();

    for opt in items.iter().filter(|a| !a.is_positional()) {
        let mut body = vec![];

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

        if let Some(help) = opt.get_long_help().or_else(|| opt.get_help()) {
            body.push(roman(help));
        }

        if let Some(mut env) = option_environment(opt) {
            body.append(&mut env);
        }

        roff.control("TP", []);
        roff.text(header);
        roff.text(body);
    }

    for pos in items.iter().filter(|a| a.is_positional()) {
        let (lhs, rhs) = option_markers(pos);
        let name = format!("{}{}{}", lhs, pos.get_name(), rhs);

        let mut header = vec![bold(&name)];

        let mut body = vec![];

        if let Some(defs) = option_default_values(pos) {
            header.push(roman(&format!(" {}", defs)));
        }

        if let Some(help) = pos.get_long_help().or_else(|| pos.get_help()) {
            body.push(roman(&help.to_string()));
        }

        if let Some(mut env) = option_environment(pos) {
            body.append(&mut env);
        }

        roff.control("TP", []);
        roff.text(body);
    }
}

pub(crate) fn subcommands(roff: &mut Roff, app: &clap::App, section: &str) {
    for sub in app
        .get_subcommands()
        .filter(|s| !s.is_set(AppSettings::Hidden))
    {
        roff.control("TP", []);

        let name = format!("{}-{}({})", app.get_name(), sub.get_name(), section);
        roff.text([roman(&name)]);

        if let Some(about) = sub.get_about().or_else(|| sub.get_long_about()) {
            for line in about.lines() {
                roff.text([roman(line)]);
            }
        }
    }
}

pub(crate) fn version(app: &clap::App) -> String {
    format!(
        "v{}",
        app.get_long_version()
            .or_else(|| app.get_version())
            .unwrap()
    )
}

pub(crate) fn after_help(roff: &mut Roff, app: &clap::App) {
    if let Some(about) = app.get_after_long_help().or_else(|| app.get_after_help()) {
        for line in about.lines() {
            roff.text([roman(line)]);
        }
    }
}

fn subcommand_markers(cmd: &clap::App) -> (&'static str, &'static str) {
    markers(
        cmd.is_set(AppSettings::SubcommandRequired)
            || cmd.is_set(AppSettings::SubcommandRequiredElseHelp),
    )
}

fn option_markers(opt: &clap::Arg) -> (&'static str, &'static str) {
    markers(opt.is_set(ArgSettings::Required))
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
    if opt.is_set(ArgSettings::HideEnv) {
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
