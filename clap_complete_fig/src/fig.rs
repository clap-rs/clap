// Std
use std::io::Write;

// Internal
use clap::*;
use clap_complete::*;

/// Generate fig completion file
pub struct Fig;

impl Generator for Fig {
    fn file_name(&self, name: &str) -> String {
        format!("{}.ts", name)
    }

    fn generate(&self, app: &App, buf: &mut dyn Write) {
        let command = app.get_bin_name().unwrap();
        let mut buffer = String::new();

        buffer.push_str(&format!(
            "const completion: Fig.Spec = {{\n  name: \"{}\",\n",
            command
        ));

        buffer.push_str(&format!(
            "  description: \"{}\",\n",
            app.get_about().unwrap_or_default()
        ));

        gen_fig_inner(command, &[], 2, app, &mut buffer);

        buffer.push_str("};\n\nexport default completion;\n");

        buf.write_all(buffer.as_bytes())
            .expect("Failed to write to generated file");
    }
}

// Escape string inside double quotes
fn escape_string(string: &str) -> String {
    string.replace("\\", "\\\\").replace("\"", "\\\"")
}

fn gen_fig_inner(
    root_command: &str,
    parent_commands: &[&str],
    indent: usize,
    app: &App,
    buffer: &mut String,
) {
    if app.has_subcommands() {
        buffer.push_str(&format!("{:indent$}subcommands: [\n", "", indent = indent));
        // generate subcommands
        for subcommand in app.get_subcommands() {
            buffer.push_str(&format!(
                "{:indent$}{{\n{:indent$}  name: \"{}\",\n",
                "",
                "",
                subcommand.get_name(),
                indent = indent + 2
            ));

            if let Some(data) = subcommand.get_about() {
                buffer.push_str(&format!(
                    "{:indent$}description: \"{}\",\n",
                    "",
                    escape_string(data),
                    indent = indent + 4
                ));
            }

            let mut parent_commands: Vec<_> = parent_commands.into();
            parent_commands.push(subcommand.get_name());
            gen_fig_inner(
                root_command,
                &parent_commands,
                indent + 4,
                subcommand,
                buffer,
            );

            buffer.push_str(&format!("{:indent$}}},\n", "", indent = indent + 2));
        }
        buffer.push_str(&format!("{:indent$}],\n", "", indent = indent));
    }

    buffer.push_str(&gen_options(app, indent));

    let args = app.get_positionals().collect::<Vec<_>>();

    match args.len() {
        0 => {}
        1 => {
            buffer.push_str(&format!("{:indent$}args: ", "", indent = indent));

            buffer.push_str(&gen_args(args[0], indent));
        }
        _ => {
            buffer.push_str(&format!("{:indent$}args: [\n", "", indent = indent));
            for arg in args {
                buffer.push_str(&gen_args(arg, indent));
            }
            buffer.push_str(&format!("{:indent$}]\n", "", indent = indent));
        }
    };
}

fn gen_options(app: &App, indent: usize) -> String {
    let mut buffer = String::new();

    buffer.push_str(&format!("{:indent$}options: [\n", "", indent = indent));

    for option in app.get_opts() {
        buffer.push_str(&format!("{:indent$}{{\n", "", indent = indent + 2));

        let mut names = vec![];

        if let Some(shorts) = option.get_short_and_visible_aliases() {
            names.extend(shorts.iter().map(|short| format!("-{}", short)));
        }

        if let Some(longs) = option.get_long_and_visible_aliases() {
            names.extend(longs.iter().map(|long| format!("--{}", long)));
        }

        if names.len() > 1 {
            buffer.push_str(&format!("{:indent$}name: [", "", indent = indent + 4));

            buffer.push_str(
                &names
                    .iter()
                    .map(|name| format!("\"{}\"", name))
                    .collect::<Vec<_>>()
                    .join(", "),
            );

            buffer.push_str("],\n");
        } else {
            buffer.push_str(&format!(
                "{:indent$}name: \"{}\",\n",
                "",
                names[0],
                indent = indent + 4
            ));
        }

        if let Some(data) = option.get_help() {
            buffer.push_str(&format!(
                "{:indent$}description: \"{}\",\n",
                "",
                escape_string(data),
                indent = indent + 4
            ));
        }

        buffer.push_str(&format!("{:indent$}args: ", "", indent = indent + 4));

        buffer.push_str(&gen_args(option, indent + 4));

        buffer.push_str(&format!("{:indent$}}},\n", "", indent = indent + 2));
    }

    for flag in utils::flags(app) {
        buffer.push_str(&format!("{:indent$}{{\n", "", indent = indent + 2));

        let mut flags = vec![];

        if let Some(shorts) = flag.get_short_and_visible_aliases() {
            flags.extend(shorts.iter().map(|s| format!("-{}", s)));
        }

        if let Some(longs) = flag.get_long_and_visible_aliases() {
            flags.extend(longs.iter().map(|s| format!("--{}", s)));
        }

        if flags.len() > 1 {
            buffer.push_str(&format!("{:indent$}name: [", "", indent = indent + 4));

            buffer.push_str(
                &flags
                    .iter()
                    .map(|name| format!("\"{}\"", name))
                    .collect::<Vec<_>>()
                    .join(", "),
            );

            buffer.push_str("],\n");
        } else {
            buffer.push_str(&format!(
                "{:indent$}name: \"{}\",\n",
                "",
                flags[0],
                indent = indent + 4
            ));
        }

        if let Some(data) = flag.get_help() {
            buffer.push_str(&format!(
                "{:indent$}description: \"{}\",\n",
                "",
                escape_string(data).as_str(),
                indent = indent + 4
            ));
        }

        buffer.push_str(&format!("{:indent$}}},\n", "", indent = indent + 2));
    }

    buffer.push_str(&format!("{:indent$}],\n", "", indent = indent));

    buffer
}

fn gen_args(arg: &Arg, indent: usize) -> String {
    if !arg.is_set(ArgSettings::TakesValue) {
        return "".to_string();
    }

    let mut buffer = String::new();

    buffer.push_str(&format!(
        "{{\n{:indent$}  name: \"{}\",\n",
        "",
        arg.get_name(),
        indent = indent
    ));

    if arg.is_set(ArgSettings::MultipleValues) {
        buffer.push_str(&format!(
            "{:indent$}isVariadic: true,\n",
            "",
            indent = indent + 2
        ));
    }

    if !arg.is_set(ArgSettings::Required) {
        buffer.push_str(&format!(
            "{:indent$}isOptional: true,\n",
            "",
            indent = indent + 2
        ));
    }

    if let Some(data) = arg.get_possible_values() {
        buffer.push_str(&format!(
            "{:indent$}suggestions: [\n",
            "",
            indent = indent + 2
        ));

        for value in data {
            buffer.push_str(&format!(
                "{:indent$}{{\n{:indent$}  name: \"{}\",\n",
                "",
                "",
                value.get_name(),
                indent = indent + 4,
            ));

            if let Some(help) = value.get_help() {
                buffer.push_str(&format!(
                    "{:indent$}description: \"{}\",\n",
                    "",
                    escape_string(help),
                    indent = indent + 4
                ));
            }

            buffer.push_str(&format!("{:indent$}}},\n", "", indent = indent + 4));
        }

        buffer.push_str(&format!("{:indent$}]\n", "", indent = indent + 2));
    } else {
        match arg.get_value_hint() {
            ValueHint::AnyPath | ValueHint::FilePath | ValueHint::ExecutablePath => {
                buffer.push_str(&format!(
                    "{:indent$}template: \"filepaths\",\n",
                    "",
                    indent = indent + 2
                ));
            }
            ValueHint::DirPath => {
                buffer.push_str(&format!(
                    "{:indent$}template: \"folders\",\n",
                    "",
                    indent = indent + 2
                ));
            }
            ValueHint::CommandString | ValueHint::CommandName | ValueHint::CommandWithArguments => {
                buffer.push_str(&format!(
                    "{:indent$}isCommand: true,\n",
                    "",
                    indent = indent + 2
                ));
            }
            // Disable completion for others
            _ => (),
        };
    };

    buffer.push_str(&format!("{:indent$}}},\n", "", indent = indent));

    buffer
}
