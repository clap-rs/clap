
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
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('-O', '-O ', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('--option', '--option', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('--opt', '--opt', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('-F', '-F ', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('--flag', '--flag', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('--flg', '--flg', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    if ($wordToComplete -notlike "-*") {
        $completions = $completions.Where{ $_.CompletionText -notlike "-*" }
    }

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
