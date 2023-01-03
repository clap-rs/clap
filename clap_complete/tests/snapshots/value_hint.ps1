
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
            [CompletionResult]::new('--choice', 'choice', [CompletionResultType]::ParameterName, 'choice')
            [CompletionResult]::new('--unknown', 'unknown', [CompletionResultType]::ParameterName, 'unknown')
            [CompletionResult]::new('--other', 'other', [CompletionResultType]::ParameterName, 'other')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'p')
            [CompletionResult]::new('--path', 'path', [CompletionResultType]::ParameterName, 'path')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'f')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'file')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'd')
            [CompletionResult]::new('--dir', 'dir', [CompletionResultType]::ParameterName, 'dir')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--exe', 'exe', [CompletionResultType]::ParameterName, 'exe')
            [CompletionResult]::new('--cmd-name', 'cmd-name', [CompletionResultType]::ParameterName, 'cmd-name')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--cmd', 'cmd', [CompletionResultType]::ParameterName, 'cmd')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'u')
            [CompletionResult]::new('--user', 'user', [CompletionResultType]::ParameterName, 'user')
            [CompletionResult]::new('-H', 'H', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'host')
            [CompletionResult]::new('--url', 'url', [CompletionResultType]::ParameterName, 'url')
            [CompletionResult]::new('--email', 'email', [CompletionResultType]::ParameterName, 'email')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
