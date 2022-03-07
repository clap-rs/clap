
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
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--single-quotes', 'single-quotes', [CompletionResultType]::ParameterName, 'Can be ''always'', ''auto'', or ''never''')
            [CompletionResult]::new('--double-quotes', 'double-quotes', [CompletionResultType]::ParameterName, 'Can be "always", "auto", or "never"')
            [CompletionResult]::new('--backticks', 'backticks', [CompletionResultType]::ParameterName, 'For more information see `echo test`')
            [CompletionResult]::new('--backslash', 'backslash', [CompletionResultType]::ParameterName, 'Avoid ''/n''')
            [CompletionResult]::new('--brackets', 'brackets', [CompletionResultType]::ParameterName, 'List packages [filter]')
            [CompletionResult]::new('--expansions', 'expansions', [CompletionResultType]::ParameterName, 'Execute the shell command with $SHELL')
            [CompletionResult]::new('cmd-single-quotes', 'cmd-single-quotes', [CompletionResultType]::ParameterValue, 'Can be ''always'', ''auto'', or ''never''')
            [CompletionResult]::new('cmd-double-quotes', 'cmd-double-quotes', [CompletionResultType]::ParameterValue, 'Can be "always", "auto", or "never"')
            [CompletionResult]::new('cmd-backticks', 'cmd-backticks', [CompletionResultType]::ParameterValue, 'For more information see `echo test`')
            [CompletionResult]::new('cmd-backslash', 'cmd-backslash', [CompletionResultType]::ParameterValue, 'Avoid ''/n''')
            [CompletionResult]::new('cmd-brackets', 'cmd-brackets', [CompletionResultType]::ParameterValue, 'List packages [filter]')
            [CompletionResult]::new('cmd-expansions', 'cmd-expansions', [CompletionResultType]::ParameterValue, 'Execute the shell command with $SHELL')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'my-app;cmd-single-quotes' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'my-app;cmd-double-quotes' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'my-app;cmd-backticks' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'my-app;cmd-backslash' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'my-app;cmd-brackets' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'my-app;cmd-expansions' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'my-app;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
