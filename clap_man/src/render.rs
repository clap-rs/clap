use clap::{AppSettings, ArgSettings};
use roff::{bold, escape, italic, list, paragraph};

pub(crate) fn subcommand_heading(app: &clap::App) -> String {
    match app.get_subommand_help_heading() {
        Some(title) => escape(title),
        None => "Subcommands".to_string(),
    }
}

pub(crate) fn about(app: &clap::App) -> String {
    match app.get_about().or_else(|| app.get_long_about()) {
        Some(about) => format!("{} - {}", app.get_name(), about),
        None => app.get_name().to_string(),
    }
}

pub(crate) fn description(app: &clap::App) -> Vec<String> {
    match app.get_long_about().or_else(|| app.get_about()) {
        Some(about) => about
            .lines()
            .filter_map(|l| (!l.trim().is_empty()).then(|| paragraph(l.trim())))
            .collect(),
        None => Vec::new(),
    }
}

pub(crate) fn synopsis(app: &clap::App) -> String {
    let mut res = String::new();

    res.push_str(&italic(app.get_name()));
    res.push(' ');

    for opt in app.get_arguments() {
        res.push_str(&match (opt.get_short(), opt.get_long()) {
            (Some(short), Some(long)) => format!("[-{}|--{}] ", short, long),
            (Some(short), None) => format!("[-{}] ", short),
            (None, Some(long)) => format!("[--{}] ", long),
            (None, None) => "".to_string(),
        });
    }

    for arg in app.get_positionals() {
        res.push_str(&format!("[{}] ", arg.get_name()));
    }

    if app.has_subcommands() {
        res.push_str(&format!("[{}] ", subcommand_heading(app).to_lowercase()));
    }

    res
}

pub(crate) fn options(app: &clap::App) -> Vec<String> {
    let mut res = Vec::new();
    let items: Vec<_> = app
        .get_arguments()
        .filter(|i| !i.is_set(ArgSettings::Hidden))
        .collect();

    for opt in items.iter().filter(|a| !a.is_positional()) {
        let mut body = Vec::new();

        let mut header = match (opt.get_short(), opt.get_long()) {
            (Some(short), Some(long)) => {
                vec![short_option(short), ", ".to_string(), long_option(long)]
            }
            (Some(short), None) => vec![short_option(short)],
            (None, Some(long)) => vec![long_option(long)],
            (None, None) => vec![],
        };

        if let Some(value) = &opt.get_value_names() {
            header.push(format!("={}", italic(&value.join(" "))));
        }

        if let Some(defs) = option_default_values(opt) {
            header.push(format!(" {}", defs));
        }

        if let Some(help) = opt.get_long_help().or_else(|| opt.get_help()) {
            body.push(help.to_string());
        }

        if let Some(env) = option_environment(opt) {
            body.push(env);
        }

        body.push("\n".to_string());

        res.push(list(&header, &body));
    }

    for pos in items.iter().filter(|a| a.is_positional()) {
        let required = pos.is_set(ArgSettings::Required);
        let (rhs, lhs) = if required { ("<", ">") } else { ("[", "]") };
        let name = format!("{}{}{}", rhs, pos.get_name(), lhs);

        let mut header = vec![bold(&name)];

        let mut body = Vec::new();

        if let Some(defs) = option_default_values(pos) {
            header.push(format!(" {}", defs));
        }

        if let Some(help) = pos.get_long_help().or_else(|| pos.get_help()) {
            body.push(help.to_string());
        }

        if let Some(env) = option_environment(pos) {
            body.push(env);
        }

        res.push(list(&header, &body))
    }

    res
}

pub(crate) fn subcommands(app: &clap::App, section: i8) -> Vec<String> {
    app.get_subcommands()
        .filter(|s| !s.is_set(AppSettings::Hidden))
        .map(|command| {
            let name = format!("{}-{}({})", app.get_name(), command.get_name(), section);

            let mut body = match command.get_long_about().or_else(|| command.get_about()) {
                Some(about) => about
                    .lines()
                    .filter_map(|l| (!l.trim().is_empty()).then(|| l.trim()))
                    .collect(),
                None => Vec::new(),
            };

            body.push("\n");

            list(&[bold(&name)], &body)
        })
        .collect()
}

pub(crate) fn version(app: &clap::App) -> String {
    format!("v{}", app.get_version().unwrap())
}

pub(crate) fn after_help(app: &clap::App) -> Vec<String> {
    match app.get_after_long_help().or_else(|| app.get_after_help()) {
        Some(about) => about
            .lines()
            .filter_map(|l| (!l.trim().is_empty()).then(|| paragraph(l.trim())))
            .collect(),
        None => Vec::new(),
    }
}

fn short_option(opt: char) -> String {
    format!("-{}", bold(&opt.to_string()))
}

fn long_option(opt: &str) -> String {
    format!("--{}", bold(&opt.to_string()))
}

fn option_environment(opt: &clap::Arg) -> Option<String> {
    if let Some(env) = opt.get_env() {
        return Some(paragraph(&format!(
            "May also be specified with the {} environment variable. ",
            bold(&env.to_string_lossy())
        )));
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
