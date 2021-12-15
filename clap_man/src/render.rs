use std::io::Write;

use crate::man::{bold, document_title, escape, indent, italic, list, section_title};

use clap::ArgSettings;

fn app_name(app: &clap::App) -> String {
    if let Some(name) = app.get_bin_name() {
        escape(name)
    } else {
        escape(app.get_name())
    }
}

fn subcommand_heading(app: &clap::App) -> String {
    match app.get_subommand_help_heading() {
        Some(title) => escape(title),
        None => "Subcommands".to_string(),
    }
}

pub(crate) fn header(app: &clap::App, section: i8, manual: Option<String>, buf: &mut dyn Write) {
    write!(buf, "{}", document_title(app, section, manual, None)).unwrap();
}

pub(crate) fn about(app: &clap::App, buf: &mut dyn Write) {
    write!(buf, "{}", section_title("Name")).unwrap();
    write!(buf, "{}", bold(&app_name(app))).unwrap();

    if let Some(about) = app.get_about() {
        write!(buf, r" \- {}", escape(about)).unwrap();
    }

    writeln!(buf).unwrap();
}

pub(crate) fn description(app: &clap::App, buf: &mut dyn Write) {
    if let Some(about) = app.get_long_about() {
        writeln!(buf, "{}{}", section_title("Description"), escape(about)).unwrap();
    }
}

pub(crate) fn synopsis(app: &clap::App, buf: &mut dyn Write) {
    write!(
        buf,
        "{}{} ",
        section_title("Synopsis"),
        italic(&app_name(app))
    )
    .unwrap();

    for opt in app.get_arguments() {
        let options = match (opt.get_short(), opt.get_long()) {
            (Some(short), Some(long)) => format!("[-{}|--{}]", short, long),
            (Some(short), None) => format!("[-{}]", short),
            (None, Some(long)) => format!("[--{}]", long),
            (None, None) => "".to_string(),
        };

        write!(buf, "{} ", options).unwrap();
    }

    for arg in app.get_positionals() {
        write!(buf, "{} ", format!("[{}]", arg.get_name())).unwrap();
    }

    if app.has_subcommands() {
        write!(buf, "[{}]", subcommand_heading(app).to_lowercase()).unwrap();
    }

    writeln!(buf).unwrap();
}

// TODO: handle positional arguments in a different way?
pub(crate) fn options(app: &clap::App, buf: &mut dyn Write) {
    let items: Vec<_> = app
        .get_arguments()
        .filter(|i| !i.is_set(ArgSettings::Hidden))
        .collect();

    if items.is_empty() {
        return;
    }

    write!(buf, "{}", section_title("Options")).unwrap();
    for opt in items.iter().filter(|a| !a.is_positional()) {
        let args = match (opt.get_short(), opt.get_long()) {
            (Some(short), Some(long)) => format!("-{}, --{}", short, long),
            (Some(short), None) => format!("-{}", short),
            (None, Some(long)) => format!("--{}", long),
            (None, None) => "".to_string(),
        };

        write!(buf, "{}", bold(&args)).unwrap();

        if let Some(value) = &opt.get_value_names() {
            write!(buf, " {}", italic(&value.join(" "))).unwrap();
        }

        writeln!(buf).unwrap();

        if let Some(help) = opt.get_help() {
            write!(buf, "{}", indent(4, help)).unwrap();
        }

        if let Some(help) = opt.get_long_help() {
            write!(buf, "\n.sp\n.sp\n{}", indent(4, help)).unwrap();
        }

        writeln!(buf, "\n").unwrap();
    }

    for pos in items.iter().filter(|a| a.is_positional()) {
        let required = pos.is_set(ArgSettings::Required);
        let (rhs, lhs) = if required { ("<", ">") } else { ("[", "]") };
        let name = format!("{}{}{}", rhs, pos.get_name(), lhs);

        writeln!(buf, "{}", bold(&name)).unwrap();

        if let Some(help) = pos.get_help() {
            write!(buf, "{}", indent(4, help)).unwrap();
        }

        if let Some(help) = pos.get_long_help() {
            write!(buf, "\n.sp\n.sp\n{}", indent(4, help)).unwrap();
        }

        writeln!(buf, "\n").unwrap();
    }
}

pub(crate) fn subcommands(app: &clap::App, section: i8, buf: &mut dyn Write) {
    let commands: Vec<_> = app.get_subcommands().collect();

    if commands.is_empty() {
        return;
    }

    write!(buf, "{}", section_title(&subcommand_heading(app))).unwrap();
    for command in commands {
        let name = format!("{}-{}", app_name(app), command.get_name());
        writeln!(buf, "{}({})", bold(&escape(&name)), section).unwrap();

        if let Some(about) = command.get_about() {
            writeln!(buf, ".br\n{}\n", about).unwrap();
        }
    }
}

pub(crate) fn authors(app: &clap::App, buf: &mut dyn Write) {
    if let Some(authors) = app.get_author() {
        let separator = ['\n', ':'].iter().find(|&&s| authors.contains(s));
        let authors = if let Some(sep) = separator {
            authors.split(*sep).collect()
        } else {
            vec![authors]
        };

        let title = if authors.len() > 1 {
            "Authors"
        } else {
            "Author"
        };

        write!(buf, "{}", list(title, authors)).unwrap();
    }
}

pub(crate) fn version(app: &clap::App, buf: &mut dyn Write) {
    if let Some(version) = app.get_version() {
        write!(buf, "{}", section_title("Version")).unwrap();
        writeln!(buf, "v{}", escape(version)).unwrap();
    }
}

pub(crate) fn custom_sections(sections: Vec<(String, Vec<String>)>, buf: &mut dyn Write) {
    for (title, body) in sections {
        write!(buf, "{}", section_title(&title)).unwrap();

        for paragraph in body {
            writeln!(buf, ".sp\n{}", escape(&paragraph)).unwrap();
        }
    }
}

pub(crate) fn after_help(app: &clap::App, buf: &mut dyn Write) {
    if let Some(help) = app.get_after_help() {
        write!(buf, "{}", section_title("Extra")).unwrap();
        writeln!(buf, ".sp\n{}", escape(help)).unwrap();

        if let Some(long) = app.get_after_long_help() {
            writeln!(buf, ".sp\n{}", escape(long)).unwrap();
        }
    }
}
