
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'my-app' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'my-app'
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
        'my-app' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('-C', 'C', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('--conf', 'conf', [CompletionResultType]::ParameterName, 'some config file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'tests things')
            [CompletionResult]::new('some_cmd', 'some_cmd', [CompletionResultType]::ParameterValue, 'tests other things')
            [CompletionResult]::new('some-cmd-with-hyphens', 'some-cmd-with-hyphens', [CompletionResultType]::ParameterValue, 'some-cmd-with-hyphens')
            [CompletionResult]::new('some-hidden-cmd', 'some-hidden-cmd', [CompletionResultType]::ParameterValue, 'some-hidden-cmd')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'my-app;test' {
            [CompletionResult]::new('--case', 'case', [CompletionResultType]::ParameterName, 'the case to test')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'my-app;some_cmd' {
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'the other case to test')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'my-app;some-cmd-with-hyphens' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'my-app;some-hidden-cmd' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'my-app;help' {
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'tests things')
            [CompletionResult]::new('some_cmd', 'some_cmd', [CompletionResultType]::ParameterValue, 'tests other things')
            [CompletionResult]::new('some-cmd-with-hyphens', 'some-cmd-with-hyphens', [CompletionResultType]::ParameterValue, 'some-cmd-with-hyphens')
            [CompletionResult]::new('some-hidden-cmd', 'some-hidden-cmd', [CompletionResultType]::ParameterValue, 'some-hidden-cmd')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'my-app;help;test' {
            break
        }
        'my-app;help;some_cmd' {
            break
        }
        'my-app;help;some-cmd-with-hyphens' {
            break
        }
        'my-app;help;some-hidden-cmd' {
            break
        }
        'my-app;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
