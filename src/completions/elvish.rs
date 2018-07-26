// Std
use std::io::Write;

// Internal
use build::App;
use INTERNAL_ERROR_MSG;

pub struct ElvishGen<'a, 'b, 'c>
where
    'a: 'b,
    'b: 'c,
{
    app: &'c App<'a, 'b>,
}

impl<'a, 'b, 'c> ElvishGen<'a, 'b, 'c> {
    pub fn new(p: &'c App<'a, 'b>) -> Self { ElvishGen { app: p } }

    pub fn generate_to<W: Write>(&self, buf: &mut W) {
        let bin_name = self.app.bin_name.as_ref().unwrap();

        let mut names = vec![];
        let subcommands_cases = generate_inner(self.app, "", &mut names);

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
fn escape_string(string: &str) -> String { string.replace("'", "''") }

fn get_tooltip<T: ToString>(help: Option<&str>, data: T) -> String {
    match help {
        Some(help) => escape_string(help),
        _ => data.to_string(),
    }
}

fn generate_inner<'a, 'b, 'c>(
    p: &'c App<'a, 'b>,
    previous_command_name: &str,
    names: &mut Vec<&'a str>,
) -> String
where
    'a: 'b,
    'b: 'c,
{
    debugln!("ElvishGen::generate_inner;");
    let command_name = if previous_command_name.is_empty() {
        p.bin_name.as_ref().expect(INTERNAL_ERROR_MSG).clone()
    } else {
        format!("{};{}", previous_command_name, &p.name)
    };

    let mut completions = String::new();
    let preamble = String::from("\n            cand ");

    for (_, option) in opts!(p) {
        if let Some(data) = option.short {
            let tooltip = get_tooltip(option.help, data);
            completions.push_str(&preamble);
            completions.push_str(format!("-{} '{}'", data, tooltip).as_str());
        }
        if let Some(data) = option.long {
            let tooltip = get_tooltip(option.help, data);
            completions.push_str(&preamble);
            completions.push_str(format!("--{} '{}'", data, tooltip).as_str());
        }
    }

    for (_, flag) in flags!(p) {
        if let Some(data) = flag.short {
            let tooltip = get_tooltip(flag.help, data);
            completions.push_str(&preamble);
            completions.push_str(format!("-{} '{}'", data, tooltip).as_str());
        }
        if let Some(data) = flag.long {
            let tooltip = get_tooltip(flag.help, data);
            completions.push_str(&preamble);
            completions.push_str(format!("--{} '{}'", data, tooltip).as_str());
        }
    }

    for subcommand in &p.subcommands {
        let data = &subcommand.name;
        let tooltip = get_tooltip(subcommand.about, data);
        completions.push_str(&preamble);
        completions.push_str(format!("{} '{}'", data, tooltip).as_str());
    }

    let mut subcommands_cases = format!(
        r"
        &'{}'= {{{}
        }}",
        &command_name, completions
    );

    for subcommand in &p.subcommands {
        let subcommand_subcommands_cases = generate_inner(&subcommand, &command_name, names);
        subcommands_cases.push_str(&subcommand_subcommands_cases);
    }

    subcommands_cases
}
