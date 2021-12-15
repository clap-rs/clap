pub(crate) fn bold(input: &str) -> String {
    format!(r"\fB{}\fR", input)
}

pub(crate) fn italic(input: &str) -> String {
    format!(r"\fI{}\fR", input)
}

pub(crate) fn escape(str: &str) -> String {
    str.replace("-", r"\-")
}

pub(crate) fn indent(level: i8, input: &str) -> String {
    format!(".RS {}\n{}\n.RE", level, input)
}

pub(crate) fn section_title(title: &str) -> String {
    format!(".SH \"{}\"\n", title.to_uppercase())
}

pub(crate) fn document_title(
    app: &clap::App,
    section: i8,
    manual: Option<String>,
    date: Option<String>,
) -> String {
    let version = match app.get_version() {
        Some(version) => format!(r#" "{} v{}""#, app.get_name(), version),
        None => r#" """#.to_string(),
    };

    let manual = match manual {
        Some(manual) => format!(r#" "{}""#, manual),
        None => r#" """#.to_string(),
    };

    let date = match date {
        Some(date) => format!(r#" "{}""#, date),
        None => r#" """#.to_string(),
    };

    let name = if let Some(name) = app.get_bin_name() {
        name
    } else {
        app.get_name()
    };

    format!(".TH {} {} {} {} {}\n", name, section, date, version, manual)
}

pub(crate) fn list(title: &str, list: Vec<impl Into<String>>) -> String {
    let mut res = String::new();

    res.push_str(&section_title(&escape(title)));
    res.push_str(".P\n.RS 2\n.nf\n");
    for item in list {
        res.push_str(&escape(&item.into()));
        res.push('\n');
    }

    res
}
