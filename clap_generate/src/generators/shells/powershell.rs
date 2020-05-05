// Std
use std::io::Write;

// Internal
use crate::Generator;
use crate::INTERNAL_ERROR_MSG;
use clap::*;

/// Generate powershell completion file
pub struct PowerShell;

impl Generator for PowerShell {
    fn file_name(name: &str) -> String {
        format!("_{}.ps1", name)
    }

    fn generate(app: &App, buf: &mut dyn Write) {
        let bin_name = app.get_bin_name().unwrap();

        let mut names = vec![];
        let subcommands_cases = generate_inner(app, "", &mut names);

        let result = format!(
            r#"
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName '{bin_name}' -ScriptBlock {{
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        '{bin_name}'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {{
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {{
                break
        }}
        $element.Value
    }}) -join ';'

    $completions = @(switch ($command) {{{subcommands_cases}
    }})

    $completions.Where{{ $_.CompletionText -like "$wordToComplete*" }} |
        Sort-Object -Property ListItemText
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
        Some(help) => escape_string(&help),
        _ => data.to_string(),
    }
}

fn generate_inner<'b>(
    p: &'b App<'b>,
    previous_command_name: &str,
    names: &mut Vec<&'b str>,
) -> String {
    debug!("generate_inner");

    let command_name = if previous_command_name.is_empty() {
        p.get_bin_name().expect(INTERNAL_ERROR_MSG).to_string()
    } else {
        format!("{};{}", previous_command_name, &p.get_name())
    };

    let mut completions = String::new();
    let preamble = String::from("\n            [CompletionResult]::new(");

    for option in opts!(p) {
        if let Some(data) = option.get_short() {
            let tooltip = get_tooltip(option.get_about(), data);

            completions.push_str(&preamble);
            completions.push_str(
                format!(
                    "'-{}', '{}', {}, '{}')",
                    data, data, "[CompletionResultType]::ParameterName", tooltip
                )
                .as_str(),
            );
        }

        if let Some(data) = option.get_long() {
            let tooltip = get_tooltip(option.get_about(), data);

            completions.push_str(&preamble);
            completions.push_str(
                format!(
                    "'--{}', '{}', {}, '{}')",
                    data, data, "[CompletionResultType]::ParameterName", tooltip
                )
                .as_str(),
            );
        }
    }

    for flag in PowerShell::flags(p) {
        if let Some(data) = flag.get_short() {
            let tooltip = get_tooltip(flag.get_about(), data);

            completions.push_str(&preamble);
            completions.push_str(
                format!(
                    "'-{}', '{}', {}, '{}')",
                    data, data, "[CompletionResultType]::ParameterName", tooltip
                )
                .as_str(),
            );
        }

        if let Some(data) = flag.get_long() {
            let tooltip = get_tooltip(flag.get_about(), data);

            completions.push_str(&preamble);
            completions.push_str(
                format!(
                    "'--{}', '{}', {}, '{}')",
                    data, data, "[CompletionResultType]::ParameterName", tooltip
                )
                .as_str(),
            );
        }
    }

    for subcommand in p.get_subcommands() {
        let data = &subcommand.get_name();
        let tooltip = get_tooltip(subcommand.get_about(), data);

        completions.push_str(&preamble);
        completions.push_str(
            format!(
                "'{}', '{}', {}, '{}')",
                data, data, "[CompletionResultType]::ParameterValue", tooltip
            )
            .as_str(),
        );
    }

    let mut subcommands_cases = format!(
        r"
        '{}' {{{}
            break
        }}",
        &command_name, completions
    );

    for subcommand in p.get_subcommands() {
        let subcommand_subcommands_cases = generate_inner(&subcommand, &command_name, names);
        subcommands_cases.push_str(&subcommand_subcommands_cases);
    }

    subcommands_cases
}
