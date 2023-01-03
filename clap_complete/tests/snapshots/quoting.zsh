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
    _arguments "${_arguments_options[@]}" \
'--single-quotes[Can be '\''always'\'', '\''auto'\'', or '\''never'\'']' \
'--double-quotes[Can be "always", "auto", or "never"]' \
'--backticks[For more information see `echo test`]' \
'--backslash[Avoid '\''\\n'\'']' \
'--brackets[List packages \[filter\]]' \
'--expansions[Execute the shell command with $SHELL]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_my-app_commands" \
"*::: :->my-app" \
&& ret=0
    case $state in
    (my-app)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-command-$line[1]:"
        case $line[1] in
            (cmd-single-quotes)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cmd-double-quotes)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cmd-backticks)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cmd-backslash)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cmd-brackets)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cmd-expansions)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_my-app__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-help-command-$line[1]:"
        case $line[1] in
            (cmd-single-quotes)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(cmd-double-quotes)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(cmd-backticks)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(cmd-backslash)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(cmd-brackets)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(cmd-expansions)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_my-app_commands] )) ||
_my-app_commands() {
    local commands; commands=(
'cmd-single-quotes:Can be '\''always'\'', '\''auto'\'', or '\''never'\''' \
'cmd-double-quotes:Can be "always", "auto", or "never"' \
'cmd-backticks:For more information see `echo test`' \
'cmd-backslash:Avoid '\''\\n'\''' \
'cmd-brackets:List packages \[filter\]' \
'cmd-expansions:Execute the shell command with $SHELL' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'my-app commands' commands "$@"
}
(( $+functions[_my-app__cmd-backslash_commands] )) ||
_my-app__cmd-backslash_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-backslash commands' commands "$@"
}
(( $+functions[_my-app__help__cmd-backslash_commands] )) ||
_my-app__help__cmd-backslash_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help cmd-backslash commands' commands "$@"
}
(( $+functions[_my-app__cmd-backticks_commands] )) ||
_my-app__cmd-backticks_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-backticks commands' commands "$@"
}
(( $+functions[_my-app__help__cmd-backticks_commands] )) ||
_my-app__help__cmd-backticks_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help cmd-backticks commands' commands "$@"
}
(( $+functions[_my-app__cmd-brackets_commands] )) ||
_my-app__cmd-brackets_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-brackets commands' commands "$@"
}
(( $+functions[_my-app__help__cmd-brackets_commands] )) ||
_my-app__help__cmd-brackets_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help cmd-brackets commands' commands "$@"
}
(( $+functions[_my-app__cmd-double-quotes_commands] )) ||
_my-app__cmd-double-quotes_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-double-quotes commands' commands "$@"
}
(( $+functions[_my-app__help__cmd-double-quotes_commands] )) ||
_my-app__help__cmd-double-quotes_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help cmd-double-quotes commands' commands "$@"
}
(( $+functions[_my-app__cmd-expansions_commands] )) ||
_my-app__cmd-expansions_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-expansions commands' commands "$@"
}
(( $+functions[_my-app__help__cmd-expansions_commands] )) ||
_my-app__help__cmd-expansions_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help cmd-expansions commands' commands "$@"
}
(( $+functions[_my-app__cmd-single-quotes_commands] )) ||
_my-app__cmd-single-quotes_commands() {
    local commands; commands=()
    _describe -t commands 'my-app cmd-single-quotes commands' commands "$@"
}
(( $+functions[_my-app__help__cmd-single-quotes_commands] )) ||
_my-app__help__cmd-single-quotes_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help cmd-single-quotes commands' commands "$@"
}
(( $+functions[_my-app__help_commands] )) ||
_my-app__help_commands() {
    local commands; commands=(
'cmd-single-quotes:Can be '\''always'\'', '\''auto'\'', or '\''never'\''' \
'cmd-double-quotes:Can be "always", "auto", or "never"' \
'cmd-backticks:For more information see `echo test`' \
'cmd-backslash:Avoid '\''\\n'\''' \
'cmd-brackets:List packages \[filter\]' \
'cmd-expansions:Execute the shell command with $SHELL' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'my-app help commands' commands "$@"
}
(( $+functions[_my-app__help__help_commands] )) ||
_my-app__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help help commands' commands "$@"
}

_my-app "$@"
