extern crate regex;
extern crate clap;

use clap::{App, Arg, SubCommand, Shell};
use regex::Regex;

static BASH: &'static str = r#"_myapp() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            myapp)
                cmd="myapp"
                ;;
            
            help)
                cmd+="__help"
                ;;
            test)
                cmd+="__test"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        myapp)
            opts=" -h -V  --help --version  <file>  test help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        
        myapp__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        myapp__test)
            opts=" -h -V  --help --version --case  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --case)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
    esac
}

complete -F _myapp -o bashdefault -o default myapp
"#;

static ZSH: &'static str = r#"#compdef myapp

autoload -U is-at-least

_myapp() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- some input file:_files' \
":: :_myapp_commands" \
"*::: :->myapp" \
&& ret=0
    case $state in
    (myapp)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:myapp-command-$line[2]:"
        case $line[2] in
            (test)
_arguments "${_arguments_options[@]}" \
'--case=[the case to test]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_myapp_commands] )) ||
_myapp_commands() {
    local commands; commands=(
        "test:tests things" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'myapp commands' commands "$@"
}
(( $+functions[_myapp__help_commands] )) ||
_myapp__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'myapp help commands' commands "$@"
}
(( $+functions[_myapp__test_commands] )) ||
_myapp__test_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'myapp test commands' commands "$@"
}

_myapp "$@""#;

static FISH: &'static str = r#"complete -c myapp -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c myapp -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
complete -c myapp -n "__fish_use_subcommand" -f -a "test" -d 'tests things'
complete -c myapp -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c myapp -n "__fish_seen_subcommand_from test" -l case -d 'the case to test'
complete -c myapp -n "__fish_seen_subcommand_from test" -s h -l help -d 'Prints help information'
complete -c myapp -n "__fish_seen_subcommand_from test" -s V -l version -d 'Prints version information'
complete -c myapp -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c myapp -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
"#;

static POWERSHELL: &'static str = r#"
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
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'my_app' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'tests things')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'my_app;test' {
            [CompletionResult]::new('--case', 'case', [CompletionResultType]::ParameterName, 'the case to test')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'my_app;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
"#;

static ELVISH: &'static str = r#"
edit:completion:arg-completer[my_app] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'my_app'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'my_app'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand test 'tests things'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'my_app;test'= {
            cand --case 'the case to test'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'my_app;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
"#;

static ELVISH_SPECIAL_CMDS: &'static str = r#"
edit:completion:arg-completer[my_app] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'my_app'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'my_app'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand test 'tests things'
            cand some_cmd 'tests other things'
            cand some-cmd-with-hypens 'some-cmd-with-hypens'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'my_app;test'= {
            cand --case 'the case to test'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'my_app;some_cmd'= {
            cand --config 'the other case to test'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'my_app;some-cmd-with-hypens'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'my_app;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
"#;

static POWERSHELL_SPECIAL_CMDS: &'static str = r#"
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
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'my_app' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'tests things')
            [CompletionResult]::new('some_cmd', 'some_cmd', [CompletionResultType]::ParameterValue, 'tests other things')
            [CompletionResult]::new('some-cmd-with-hypens', 'some-cmd-with-hypens', [CompletionResultType]::ParameterValue, 'some-cmd-with-hypens')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'my_app;test' {
            [CompletionResult]::new('--case', 'case', [CompletionResultType]::ParameterName, 'the case to test')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'my_app;some_cmd' {
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'the other case to test')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'my_app;some-cmd-with-hypens' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'my_app;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
"#;

static ZSH_SPECIAL_CMDS: &'static str = r#"#compdef my_app

autoload -U is-at-least

_my_app() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::file -- some input file:_files' \
":: :_my_app_commands" \
"*::: :->my_app" \
&& ret=0
    case $state in
    (my_app)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my_app-command-$line[2]:"
        case $line[2] in
            (test)
_arguments "${_arguments_options[@]}" \
'--case=[the case to test]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(some_cmd)
_arguments "${_arguments_options[@]}" \
'--config=[the other case to test]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(some-cmd-with-hypens)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_my_app_commands] )) ||
_my_app_commands() {
    local commands; commands=(
        "test:tests things" \
"some_cmd:tests other things" \
"some-cmd-with-hypens:" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'my_app commands' commands "$@"
}
(( $+functions[_my_app__help_commands] )) ||
_my_app__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'my_app help commands' commands "$@"
}
(( $+functions[_my_app__some-cmd-with-hypens_commands] )) ||
_my_app__some-cmd-with-hypens_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'my_app some-cmd-with-hypens commands' commands "$@"
}
(( $+functions[_my_app__some_cmd_commands] )) ||
_my_app__some_cmd_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'my_app some_cmd commands' commands "$@"
}
(( $+functions[_my_app__test_commands] )) ||
_my_app__test_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'my_app test commands' commands "$@"
}

_my_app "$@""#;

static FISH_SPECIAL_CMDS: &'static str = r#"complete -c my_app -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
complete -c my_app -n "__fish_use_subcommand" -f -a "test" -d 'tests things'
complete -c my_app -n "__fish_use_subcommand" -f -a "some_cmd" -d 'tests other things'
complete -c my_app -n "__fish_use_subcommand" -f -a "some-cmd-with-hypens"
complete -c my_app -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c my_app -n "__fish_seen_subcommand_from test" -l case -d 'the case to test'
complete -c my_app -n "__fish_seen_subcommand_from test" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_seen_subcommand_from test" -s V -l version -d 'Prints version information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd" -l config -d 'the other case to test'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd" -s V -l version -d 'Prints version information'
complete -c my_app -n "__fish_seen_subcommand_from some-cmd-with-hypens" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_seen_subcommand_from some-cmd-with-hypens" -s V -l version -d 'Prints version information'
complete -c my_app -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
"#;

static BASH_SPECIAL_CMDS: &'static str = r#"_my_app() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            my_app)
                cmd="my_app"
                ;;
            
            help)
                cmd+="__help"
                ;;
            some-cmd-with-hypens)
                cmd+="__some__cmd__with__hypens"
                ;;
            some_cmd)
                cmd+="__some_cmd"
                ;;
            test)
                cmd+="__test"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        my_app)
            opts=" -h -V  --help --version  <file>  test some_cmd some-cmd-with-hypens help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        
        my_app__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        my_app__some__cmd__with__hypens)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        my_app__some_cmd)
            opts=" -h -V  --help --version --config  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --config)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        my_app__test)
            opts=" -h -V  --help --version --case  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --case)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
    esac
}

complete -F _my_app -o bashdefault -o default my_app
"#;

static FISH_SPECIAL_HELP: &'static str = r#"complete -c my_app -n "__fish_use_subcommand" -l single-quotes -d 'Can be \'always\', \'auto\', or \'never\''
complete -c my_app -n "__fish_use_subcommand" -l double-quotes -d 'Can be "always", "auto", or "never"'
complete -c my_app -n "__fish_use_subcommand" -l backticks -d 'For more information see `echo test`'
complete -c my_app -n "__fish_use_subcommand" -l backslash -d 'Avoid \'\\n\''
complete -c my_app -n "__fish_use_subcommand" -l brackets -d 'List packages [filter]'
complete -c my_app -n "__fish_use_subcommand" -l expansions -d 'Execute the shell command with $SHELL'
complete -c my_app -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
"#;

static ZSH_SPECIAL_HELP: &'static str = r#"#compdef my_app

autoload -U is-at-least

_my_app() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'--single-quotes[Can be '\''always'\'', '\''auto'\'', or '\''never'\'']' \
'--double-quotes[Can be "always", "auto", or "never"]' \
'--backticks[For more information see `echo test`]' \
'--backslash[Avoid '\''\\n'\'']' \
'--brackets[List packages \[filter\]]' \
'--expansions[Execute the shell command with $SHELL]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
    
}

(( $+functions[_my_app_commands] )) ||
_my_app_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'my_app commands' commands "$@"
}

_my_app "$@""#;

fn compare(left: &str, right: &str) -> bool {
    let b = left == right;
    if !b {
        let re = Regex::new(" ").unwrap();
        println!();
        println!("--> left");
        // println!("{}", left);
        println!("{}", re.replace_all(left, "\u{2022}"));
        println!("--> right");
        println!("{}", re.replace_all(right, "\u{2022}"));
        // println!("{}", right);
        println!("--")
    }
    b
}

fn build_app() -> App<'static, 'static> { build_app_with_name("myapp") }

fn build_app_with_name(s: &'static str) -> App<'static, 'static> {
    App::new(s)
        .about("Tests completions")
        .arg(Arg::with_name("file").help("some input file"))
        .subcommand(SubCommand::with_name("test")
            .about("tests things")
            .arg(Arg::with_name("case")
                .long("case")
                .takes_value(true)
                .help("the case to test")))
}

fn build_app_special_commands() -> App<'static, 'static> {
    build_app_with_name("my_app")
        .subcommand(SubCommand::with_name("some_cmd")
                    .about("tests other things")
                    .arg(Arg::with_name("config")
                         .long("--config")
                         .takes_value(true)
                         .help("the other case to test")))
        .subcommand(SubCommand::with_name("some-cmd-with-hypens"))
}

fn build_app_special_help() -> App<'static, 'static> {
    App::new("my_app")
        .arg(Arg::with_name("single-quotes")
            .long("single-quotes")
            .help("Can be 'always', 'auto', or 'never'"))
        .arg(Arg::with_name("double-quotes")
            .long("double-quotes")
            .help("Can be \"always\", \"auto\", or \"never\""))
        .arg(Arg::with_name("backticks")
            .long("backticks")
            .help("For more information see `echo test`"))
        .arg(Arg::with_name("backslash")
            .long("backslash")
            .help("Avoid '\\n'"))
        .arg(Arg::with_name("brackets")
            .long("brackets")
            .help("List packages [filter]"))
        .arg(Arg::with_name("expansions")
            .long("expansions")
            .help("Execute the shell command with $SHELL"))
}

#[test]
fn bash() {
    let mut app = build_app();
    let mut buf = vec![];
    app.gen_completions_to("myapp", Shell::Bash, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, BASH));
}

#[test]
fn zsh() {
    let mut app = build_app();
    let mut buf = vec![];
    app.gen_completions_to("myapp", Shell::Zsh, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, ZSH));
}

#[test]
fn fish() {
    let mut app = build_app();
    let mut buf = vec![];
    app.gen_completions_to("myapp", Shell::Fish, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, FISH));
}

#[test]
fn powershell() {
    let mut app = build_app();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::PowerShell, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, POWERSHELL));
}

#[test]
fn elvish() {
    let mut app = build_app();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::Elvish, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, ELVISH));
}

#[test]
fn elvish_with_special_commands() {
    let mut app = build_app_special_commands();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::Elvish, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, ELVISH_SPECIAL_CMDS));
}

#[test]
fn powershell_with_special_commands() {
    let mut app = build_app_special_commands();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::PowerShell, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, POWERSHELL_SPECIAL_CMDS));
}

#[test]
fn bash_with_special_commands() {
    let mut app = build_app_special_commands();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::Bash, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, BASH_SPECIAL_CMDS));
}

#[test]
fn fish_with_special_commands() {
    let mut app = build_app_special_commands();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::Fish, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, FISH_SPECIAL_CMDS));
}

#[test]
fn zsh_with_special_commands() {
    let mut app = build_app_special_commands();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::Zsh, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, ZSH_SPECIAL_CMDS));
}

#[test]
fn fish_with_special_help() {
    let mut app = build_app_special_help();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::Fish, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, FISH_SPECIAL_HELP));
}

#[test]
fn zsh_with_special_help() {
    let mut app = build_app_special_help();
    let mut buf = vec![];
    app.gen_completions_to("my_app", Shell::Zsh, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert!(compare(&*string, ZSH_SPECIAL_HELP));
}
