// Std
use std::io::Write;

// Internal
use crate::Generator;
use crate::INTERNAL_ERROR_MSG;
use clap::*;

/// Generate elvish completion file
pub struct Elvish;

impl Generator for Elvish {
    fn file_name(name: &str) -> String {
        format!("{}.elv", name)
    }

    fn generate(app: &App, buf: &mut dyn Write) {
        let bin_name = app.get_bin_name().unwrap();

        let mut names = vec![];
        let subcommands_cases = generate_inner(app, "", &mut names);

        let result = format!(
            r#"
edit:completion:arg-completer[{bin_name}] = [@words]{{
    fn spaces [n]{{
        repeat $n ' ' | joins ''
    }}
    fn cand [text desc]{{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }}
    command = '{bin_name}'
    for word $words[1:-1] {{
        if (has-prefix $word '-') {{
            break
        }}
        command = $command';'$word
    }}
    completions = [{subcommands_cases}
    ]
    $completions[$command]
}}
"#,
            bin_name = bin_name,
            subcommands_cases = subcommands_cases
        );

        w!(buf, result.as_bytes());
    }
}

// Escape string inside single quotes
fn escape_string(string: &str) -> String {
    string.replace("'", "''")
}

fn get_tooltip<T: ToString>(help: Option<&str>, data: T) -> String {
    match help {
        Some(help) => escape_string(help),
        _ => data.to_string(),
    }
}

fn generate_inner<'help>(
    p: &App<'help>,
    previous_command_name: &str,
    names: &mut Vec<&'help str>,
) -> String {
    debug!("generate_inner");

    let command_name = if previous_command_name.is_empty() {
        p.get_bin_name().expect(INTERNAL_ERROR_MSG).to_string()
    } else {
        format!("{};{}", previous_command_name, &p.get_name())
    };

    let mut completions = String::new();
    let preamble = String::from("\n            cand ");

    for option in p.get_opts() {
        if let Some(shorts) = option.get_short_and_visible_aliases() {
            let tooltip = get_tooltip(option.get_about(), shorts[0]);
            for short in shorts {
                completions.push_str(&preamble);
                completions.push_str(format!("-{} '{}'", short, tooltip).as_str());
            }
        }

        if let Some(longs) = option.get_long_and_visible_aliases() {
            let tooltip = get_tooltip(option.get_about(), longs[0]);
            for long in longs {
                completions.push_str(&preamble);
                completions.push_str(format!("--{} '{}'", long, tooltip).as_str());
            }
        }
    }

    for flag in Elvish::flags(p) {
        if let Some(shorts) = flag.get_short_and_visible_aliases() {
            let tooltip = get_tooltip(flag.get_about(), shorts[0]);
            for short in shorts {
                completions.push_str(&preamble);
                completions.push_str(format!("-{} '{}'", short, tooltip).as_str());
            }
        }

        if let Some(longs) = flag.get_long_and_visible_aliases() {
            let tooltip = get_tooltip(flag.get_about(), longs[0]);
            for long in longs {
                completions.push_str(&preamble);
                completions.push_str(format!("--{} '{}'", long, tooltip).as_str());
            }
        }
    }

    for subcommand in p.get_subcommands() {
        let data = &subcommand.get_name();
        let tooltip = get_tooltip(subcommand.get_about(), data);

        completions.push_str(&preamble);
        completions.push_str(format!("{} '{}'", data, tooltip).as_str());
    }

    let mut subcommands_cases = format!(
        r"
        &'{}'= {{{}
        }}",
        &command_name, completions
    );

    for subcommand in p.get_subcommands() {
        let subcommand_subcommands_cases = generate_inner(&subcommand, &command_name, names);
        subcommands_cases.push_str(&subcommand_subcommands_cases);
    }

    subcommands_cases
}
