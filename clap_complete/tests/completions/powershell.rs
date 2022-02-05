use super::*;

fn build_app() -> App<'static> {
    build_app_with_name("myapp")
}

fn build_app_with_name(s: &'static str) -> App<'static> {
    App::new(s)
        .version("3.0")
        .setting(AppSettings::PropagateVersion)
        .about("Tests completions")
        .arg(
            Arg::new("file")
                .value_hint(ValueHint::FilePath)
                .help("some input file"),
        )
        .arg(
            Arg::new("config")
                .help("some config file")
                .short('c')
                .visible_short_alias('C')
                .long("config")
                .visible_alias("conf"),
        )
        .subcommand(
            App::new("test").about("tests things").arg(
                Arg::new("case")
                    .long("case")
                    .takes_value(true)
                    .help("the case to test"),
            ),
        )
}

#[test]
fn powershell() {
    let mut app = build_app();
    common(PowerShell, &mut app, "my_app", POWERSHELL);
}

static POWERSHELL: &str = r#"
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'my_app' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'my_app'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'my_app' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('-C', 'C', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('--conf', 'conf', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'tests things')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'my_app;test' {
            [CompletionResult]::new('--case', 'case', [CompletionResultType]::ParameterName, 'the case to test')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            break
        }
        'my_app;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
"#;

#[test]
fn powershell_with_special_commands() {
    let mut app = build_app_special_commands();
    common(PowerShell, &mut app, "my_app", POWERSHELL_SPECIAL_CMDS);
}

fn build_app_special_commands() -> App<'static> {
    build_app_with_name("my_app")
        .subcommand(
            App::new("some_cmd").about("tests other things").arg(
                Arg::new("config")
                    .long("--config")
                    .takes_value(true)
                    .help("the other case to test"),
            ),
        )
        .subcommand(App::new("some-cmd-with-hyphens").alias("hyphen"))
}

static POWERSHELL_SPECIAL_CMDS: &str = r#"
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'my_app' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'my_app'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'my_app' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('-C', 'C', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('--conf', 'conf', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'tests things')
            [CompletionResult]::new('some_cmd', 'some_cmd', [CompletionResultType]::ParameterValue, 'tests other things')
            [CompletionResult]::new('some-cmd-with-hyphens', 'some-cmd-with-hyphens', [CompletionResultType]::ParameterValue, 'some-cmd-with-hyphens')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'my_app;test' {
            [CompletionResult]::new('--case', 'case', [CompletionResultType]::ParameterName, 'the case to test')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            break
        }
        'my_app;some_cmd' {
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'the other case to test')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            break
        }
        'my_app;some-cmd-with-hyphens' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            break
        }
        'my_app;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
"#;

#[test]
fn powershell_with_aliases() {
    let mut app = build_app_with_aliases();
    common(PowerShell, &mut app, "cmd", POWERSHELL_ALIASES);
}

fn build_app_with_aliases() -> App<'static> {
    App::new("cmd")
        .version("3.0")
        .about("testing bash completions")
        .arg(
            Arg::new("flag")
                .short('f')
                .visible_short_alias('F')
                .long("flag")
                .visible_alias("flg")
                .help("cmd flag"),
        )
        .arg(
            Arg::new("option")
                .short('o')
                .visible_short_alias('O')
                .long("option")
                .visible_alias("opt")
                .help("cmd option")
                .takes_value(true),
        )
        .arg(Arg::new("positional"))
}

static POWERSHELL_ALIASES: &str = r#"
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'cmd' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'cmd'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'cmd' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('-O', 'O', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('--option', 'option', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('--opt', 'opt', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('-F', 'F', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('--flag', 'flag', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('--flg', 'flg', [CompletionResultType]::ParameterName, 'cmd flag')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
"#;
