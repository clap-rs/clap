
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'exhaustive' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'exhaustive'
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
        'exhaustive' {
            [CompletionResult]::new('--generate', 'generate', [CompletionResultType]::ParameterName, 'generate')
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('action', 'action', [CompletionResultType]::ParameterValue, 'action')
            [CompletionResult]::new('quote', 'quote', [CompletionResultType]::ParameterValue, 'quote')
            [CompletionResult]::new('value', 'value', [CompletionResultType]::ParameterValue, 'value')
            [CompletionResult]::new('pacman', 'pacman', [CompletionResultType]::ParameterValue, 'pacman')
            [CompletionResult]::new('last', 'last', [CompletionResultType]::ParameterValue, 'last')
            [CompletionResult]::new('alias', 'alias', [CompletionResultType]::ParameterValue, 'alias')
            [CompletionResult]::new('hint', 'hint', [CompletionResultType]::ParameterValue, 'hint')
            [CompletionResult]::new('complete', 'complete', [CompletionResultType]::ParameterValue, 'Register shell completions for this program')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'exhaustive;action' {
            [CompletionResult]::new('--set', 'set', [CompletionResultType]::ParameterName, 'value')
            [CompletionResult]::new('--choice', 'choice', [CompletionResultType]::ParameterName, 'enum')
            [CompletionResult]::new('--set-true', 'set-true', [CompletionResultType]::ParameterName, 'bool')
            [CompletionResult]::new('--count', 'count', [CompletionResultType]::ParameterName, 'number')
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;quote' {
            [CompletionResult]::new('--choice', 'choice', [CompletionResultType]::ParameterName, 'choice')
            [CompletionResult]::new('--single-quotes', 'single-quotes', [CompletionResultType]::ParameterName, 'Can be ''always'', ''auto'', or ''never''')
            [CompletionResult]::new('--double-quotes', 'double-quotes', [CompletionResultType]::ParameterName, 'Can be "always", "auto", or "never"')
            [CompletionResult]::new('--backticks', 'backticks', [CompletionResultType]::ParameterName, 'For more information see `echo test`')
            [CompletionResult]::new('--backslash', 'backslash', [CompletionResultType]::ParameterName, 'Avoid ''\n''')
            [CompletionResult]::new('--brackets', 'brackets', [CompletionResultType]::ParameterName, 'List packages [filter]')
            [CompletionResult]::new('--expansions', 'expansions', [CompletionResultType]::ParameterName, 'Execute the shell command with $SHELL')
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('cmd-single-quotes', 'cmd-single-quotes', [CompletionResultType]::ParameterValue, 'Can be ''always'', ''auto'', or ''never''')
            [CompletionResult]::new('cmd-double-quotes', 'cmd-double-quotes', [CompletionResultType]::ParameterValue, 'Can be "always", "auto", or "never"')
            [CompletionResult]::new('cmd-backticks', 'cmd-backticks', [CompletionResultType]::ParameterValue, 'For more information see `echo test`')
            [CompletionResult]::new('cmd-backslash', 'cmd-backslash', [CompletionResultType]::ParameterValue, 'Avoid ''\n''')
            [CompletionResult]::new('cmd-brackets', 'cmd-brackets', [CompletionResultType]::ParameterValue, 'List packages [filter]')
            [CompletionResult]::new('cmd-expansions', 'cmd-expansions', [CompletionResultType]::ParameterValue, 'Execute the shell command with $SHELL')
            [CompletionResult]::new('escape-help', 'escape-help', [CompletionResultType]::ParameterValue, '\tab	"'' New Line')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'exhaustive;quote;cmd-single-quotes' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;quote;cmd-double-quotes' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;quote;cmd-backticks' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;quote;cmd-backslash' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;quote;cmd-brackets' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;quote;cmd-expansions' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;quote;escape-help' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;quote;help' {
            [CompletionResult]::new('cmd-single-quotes', 'cmd-single-quotes', [CompletionResultType]::ParameterValue, 'Can be ''always'', ''auto'', or ''never''')
            [CompletionResult]::new('cmd-double-quotes', 'cmd-double-quotes', [CompletionResultType]::ParameterValue, 'Can be "always", "auto", or "never"')
            [CompletionResult]::new('cmd-backticks', 'cmd-backticks', [CompletionResultType]::ParameterValue, 'For more information see `echo test`')
            [CompletionResult]::new('cmd-backslash', 'cmd-backslash', [CompletionResultType]::ParameterValue, 'Avoid ''\n''')
            [CompletionResult]::new('cmd-brackets', 'cmd-brackets', [CompletionResultType]::ParameterValue, 'List packages [filter]')
            [CompletionResult]::new('cmd-expansions', 'cmd-expansions', [CompletionResultType]::ParameterValue, 'Execute the shell command with $SHELL')
            [CompletionResult]::new('escape-help', 'escape-help', [CompletionResultType]::ParameterValue, '\tab	"'' New Line')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'exhaustive;quote;help;cmd-single-quotes' {
            break
        }
        'exhaustive;quote;help;cmd-double-quotes' {
            break
        }
        'exhaustive;quote;help;cmd-backticks' {
            break
        }
        'exhaustive;quote;help;cmd-backslash' {
            break
        }
        'exhaustive;quote;help;cmd-brackets' {
            break
        }
        'exhaustive;quote;help;cmd-expansions' {
            break
        }
        'exhaustive;quote;help;escape-help' {
            break
        }
        'exhaustive;quote;help;help' {
            break
        }
        'exhaustive;value' {
            [CompletionResult]::new('--delim', 'delim', [CompletionResultType]::ParameterName, 'delim')
            [CompletionResult]::new('--tuple', 'tuple', [CompletionResultType]::ParameterName, 'tuple')
            [CompletionResult]::new('--require-eq', 'require-eq', [CompletionResultType]::ParameterName, 'require-eq')
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;pacman' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('one', 'one', [CompletionResultType]::ParameterValue, 'one')
            [CompletionResult]::new('two', 'two', [CompletionResultType]::ParameterValue, 'two')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'exhaustive;pacman;one' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;pacman;two' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;pacman;help' {
            [CompletionResult]::new('one', 'one', [CompletionResultType]::ParameterValue, 'one')
            [CompletionResult]::new('two', 'two', [CompletionResultType]::ParameterValue, 'two')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'exhaustive;pacman;help;one' {
            break
        }
        'exhaustive;pacman;help;two' {
            break
        }
        'exhaustive;pacman;help;help' {
            break
        }
        'exhaustive;last' {
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;alias' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('-O', 'O ', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('--option', 'option', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('--opt', 'opt', [CompletionResultType]::ParameterName, 'cmd option')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('-F', 'F ', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('--flag', 'flag', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('--flg', 'flg', [CompletionResultType]::ParameterName, 'cmd flag')
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;hint' {
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
            [CompletionResult]::new('-H', 'H ', [CompletionResultType]::ParameterName, 'H')
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'host')
            [CompletionResult]::new('--url', 'url', [CompletionResultType]::ParameterName, 'url')
            [CompletionResult]::new('--email', 'email', [CompletionResultType]::ParameterName, 'email')
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;complete' {
            [CompletionResult]::new('--shell', 'shell', [CompletionResultType]::ParameterName, 'Specify shell to complete for')
            [CompletionResult]::new('--register', 'register', [CompletionResultType]::ParameterName, 'Path to write completion-registration to')
            [CompletionResult]::new('--global', 'global', [CompletionResultType]::ParameterName, 'everywhere')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'exhaustive;help' {
            [CompletionResult]::new('action', 'action', [CompletionResultType]::ParameterValue, 'action')
            [CompletionResult]::new('quote', 'quote', [CompletionResultType]::ParameterValue, 'quote')
            [CompletionResult]::new('value', 'value', [CompletionResultType]::ParameterValue, 'value')
            [CompletionResult]::new('pacman', 'pacman', [CompletionResultType]::ParameterValue, 'pacman')
            [CompletionResult]::new('last', 'last', [CompletionResultType]::ParameterValue, 'last')
            [CompletionResult]::new('alias', 'alias', [CompletionResultType]::ParameterValue, 'alias')
            [CompletionResult]::new('hint', 'hint', [CompletionResultType]::ParameterValue, 'hint')
            [CompletionResult]::new('complete', 'complete', [CompletionResultType]::ParameterValue, 'Register shell completions for this program')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'exhaustive;help;action' {
            break
        }
        'exhaustive;help;quote' {
            [CompletionResult]::new('cmd-single-quotes', 'cmd-single-quotes', [CompletionResultType]::ParameterValue, 'Can be ''always'', ''auto'', or ''never''')
            [CompletionResult]::new('cmd-double-quotes', 'cmd-double-quotes', [CompletionResultType]::ParameterValue, 'Can be "always", "auto", or "never"')
            [CompletionResult]::new('cmd-backticks', 'cmd-backticks', [CompletionResultType]::ParameterValue, 'For more information see `echo test`')
            [CompletionResult]::new('cmd-backslash', 'cmd-backslash', [CompletionResultType]::ParameterValue, 'Avoid ''\n''')
            [CompletionResult]::new('cmd-brackets', 'cmd-brackets', [CompletionResultType]::ParameterValue, 'List packages [filter]')
            [CompletionResult]::new('cmd-expansions', 'cmd-expansions', [CompletionResultType]::ParameterValue, 'Execute the shell command with $SHELL')
            [CompletionResult]::new('escape-help', 'escape-help', [CompletionResultType]::ParameterValue, '\tab	"'' New Line')
            break
        }
        'exhaustive;help;quote;cmd-single-quotes' {
            break
        }
        'exhaustive;help;quote;cmd-double-quotes' {
            break
        }
        'exhaustive;help;quote;cmd-backticks' {
            break
        }
        'exhaustive;help;quote;cmd-backslash' {
            break
        }
        'exhaustive;help;quote;cmd-brackets' {
            break
        }
        'exhaustive;help;quote;cmd-expansions' {
            break
        }
        'exhaustive;help;quote;escape-help' {
            break
        }
        'exhaustive;help;value' {
            break
        }
        'exhaustive;help;pacman' {
            [CompletionResult]::new('one', 'one', [CompletionResultType]::ParameterValue, 'one')
            [CompletionResult]::new('two', 'two', [CompletionResultType]::ParameterValue, 'two')
            break
        }
        'exhaustive;help;pacman;one' {
            break
        }
        'exhaustive;help;pacman;two' {
            break
        }
        'exhaustive;help;last' {
            break
        }
        'exhaustive;help;alias' {
            break
        }
        'exhaustive;help;hint' {
            break
        }
        'exhaustive;help;complete' {
            break
        }
        'exhaustive;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
function prompt {
    '% '
}
Set-PSReadLineOption -PredictionSource None