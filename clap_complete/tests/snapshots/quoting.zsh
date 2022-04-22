#compdef my-app

autoload -U is-at-least

_my-app() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" /
'-h[Print help information]' /
'--help[Print help information]' /
'-V[Print version information]' /
'--version[Print version information]' /
'--single-quotes[Can be '/''always'/'', '/''auto'/'', or '/''never'/'']' /
'--double-quotes[Can be "always", "auto", or "never"]' /
'--backticks[For more information see `echo test`]' /
'--backslash[Avoid '/''//n'/'']' /
'--brackets[List packages /[filter/]]' /
'--expansions[Execute the shell command with $SHELL]' /
":: :_my-app_commands" /
"*::: :->my-app" /
&& ret=0
    case $state in
    (my-app)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-command-$line[1]:"
        case $line[1] in
            (cmd-single-quotes)
_arguments "${_arguments_options[@]}" /
'-h[Print help information]' /
'--help[Print help information]' /
&& ret=0
;;
(cmd-double-quotes)
_arguments "${_arguments_options[@]}" /
'-h[Print help information]' /
'--help[Print help information]' /
&& ret=0
;;
(cmd-backticks)
_arguments "${_arguments_options[@]}" /
'-h[Print help information]' /
'--help[Print help information]' /
&& ret=0
;;
(cmd-backslash)
_arguments "${_arguments_options[@]}" /
'-h[Print help information]' /
'--help[Print help information]' /
&& ret=0
;;
(cmd-brackets)
_arguments "${_arguments_options[@]}" /
'-h[Print help information]' /
'--help[Print help information]' /
&& ret=0
;;
(cmd-expansions)
_arguments "${_arguments_options[@]}" /
'-h[Print help information]' /
'--help[Print help information]' /
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" /
'*::subcommand -- The subcommand whose help message to display:' /
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_my-app_commands] )) ||
_my-app_commands() {
    local commands; commands=(
'cmd-single-quotes:Can be '/''always'/'', '/''auto'/'', or '/''never'/''' /
'cmd-double-quotes:Can be "always", "auto", or "never"' /
'cmd-backticks:For more information see `echo test`' /
'cmd-backslash:Avoid '/''//n'/''' /
'cmd-brackets:List packages /[filter/]' /
'cmd-expansions:Execute the shell command with $SHELL' /
'help:Print this message or the help of the given subcommand(s)' /
    )
    _describe -t commands 'my-app commands' commands "$@"
}
(( $+functions[_my-app__cmd-backslash_commands] )) ||
_my-app__cmd-backslash_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-backslash commands' commands "$@"
}
(( $+functions[_my-app__cmd-backticks_commands] )) ||
_my-app__cmd-backticks_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-backticks commands' commands "$@"
}
(( $+functions[_my-app__cmd-brackets_commands] )) ||
_my-app__cmd-brackets_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-brackets commands' commands "$@"
}
(( $+functions[_my-app__cmd-double-quotes_commands] )) ||
_my-app__cmd-double-quotes_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-double-quotes commands' commands "$@"
}
(( $+functions[_my-app__cmd-expansions_commands] )) ||
_my-app__cmd-expansions_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-expansions commands' commands "$@"
}
(( $+functions[_my-app__cmd-single-quotes_commands] )) ||
_my-app__cmd-single-quotes_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-single-quotes commands' commands "$@"
}
(( $+functions[_my-app__help_commands] )) ||
_my-app__help_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help commands' commands "$@"
}

_my-app "$@"
