// Std
use std::io::Write;

// Internal
use app::parser::Parser;
use INTERNAL_ERROR_MSG;

pub struct PowerShellGen<'a, 'b>
where
    'a: 'b,
{
    p: &'b Parser<'a, 'b>,
}

impl<'a, 'b> PowerShellGen<'a, 'b> {
    pub fn new(p: &'b Parser<'a, 'b>) -> Self { PowerShellGen { p: p } }

    pub fn generate_to<W: Write>(&self, buf: &mut W) {
        let bin_name = self.p.meta.bin_name.as_ref().unwrap();

        let mut names = vec![];
        let (subcommands_detection_cases, subcommands_cases) =
            generate_inner(self.p, "", &mut names);

        let mut bin_names = vec![bin_name.to_string(), format!("./{0}", bin_name)];
        if cfg!(windows) {
            bin_names.push(format!("{0}.exe", bin_name));
            bin_names.push(format!(r".\{0}", bin_name));
            bin_names.push(format!(r".\{0}.exe", bin_name));
            bin_names.push(format!(r"./{0}.exe", bin_name));
        }

        let bin_names = bin_names.iter().fold(String::new(), |previous, current| {
            format!("{0}, '{1}'", previous, current)
        });
        let bin_names = bin_names.trim_left_matches(", ");

        let result = format!(r#"
@({bin_names}) | %{{
    Register-ArgumentCompleter -Native -CommandName $_ -ScriptBlock {{
        param($wordToComplete, $commandAst, $cursorPosition)

        $command = '_{bin_name}'
        $commandAst.CommandElements |
            Select-Object -Skip 1 |
            %{{
                switch ($_.ToString()) {{
{subcommands_detection_cases}
                    default {{ 
                        break
                    }}
                }}
            }}

        $completions = @()

        switch ($command) {{
{subcommands_cases}
        }}

        $completions |
            ?{{ $_ -like "$wordToComplete*" }} |
            Sort-Object |
            %{{ New-Object System.Management.Automation.CompletionResult $_, $_, 'ParameterValue', $_ }}
    }}
}}
"#,
            bin_names = bin_names,
            bin_name = bin_name,
            subcommands_detection_cases = subcommands_detection_cases,
            subcommands_cases = subcommands_cases
        );

        w!(buf, result.as_bytes());
    }
}

fn generate_inner<'a, 'b, 'p>(
    p: &'p Parser<'a, 'b>,
    previous_command_name: &str,
    names: &mut Vec<&'p str>,
) -> (String, String) {
    debugln!("PowerShellGen::generate_inner;");
    let command_name = if previous_command_name.is_empty() {
        format!(
            "{}_{}",
            previous_command_name,
            &p.meta.bin_name.as_ref().expect(INTERNAL_ERROR_MSG)
        )
    } else {
        format!("{}_{}", previous_command_name, &p.meta.name)
    };

    let mut subcommands_detection_cases = if !names.contains(&&*p.meta.name) {
        names.push(&*p.meta.name);
        format!(
            r"
                    '{0}' {{
                        $command += '_{0}'
                        break
                    }}
",
            &p.meta.name
        )
    } else {
        String::new()
    };

    let mut completions = String::new();
    for subcommand in &p.subcommands {
        completions.push_str(&format!("'{}', ", &subcommand.p.meta.name));
    }
    for short in shorts!(p) {
        completions.push_str(&format!("'-{}', ", short));
    }
    for long in longs!(p) {
        completions.push_str(&format!("'--{}', ", long));
    }

    let mut subcommands_cases = format!(
        r"
            '{}' {{
                $completions = @({})
            }}
",
        &command_name,
        completions.trim_right_matches(", ")
    );

    for subcommand in &p.subcommands {
        let (subcommand_subcommands_detection_cases, subcommand_subcommands_cases) =
            generate_inner(&subcommand.p, &command_name, names);
        subcommands_detection_cases.push_str(&subcommand_subcommands_detection_cases);
        subcommands_cases.push_str(&subcommand_subcommands_cases);
    }

    (subcommands_detection_cases, subcommands_cases)
}
