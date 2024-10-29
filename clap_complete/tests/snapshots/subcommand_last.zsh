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
    _arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
'::free:_default' \
":: :_my-app_commands" \
"*::: :->my-app" \
&& ret=0
    case $state in
    (my-app)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-command-$line[2]:"
        case $line[2] in
            (foo)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(bar)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_my-app__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-help-command-$line[1]:"
        case $line[1] in
            (foo)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(bar)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
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
'foo:' \
'bar:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'my-app commands' commands "$@"
}
(( $+functions[_my-app__bar_commands] )) ||
_my-app__bar_commands() {
    local commands; commands=()
    _describe -t commands 'my-app bar commands' commands "$@"
}
(( $+functions[_my-app__foo_commands] )) ||
_my-app__foo_commands() {
    local commands; commands=()
    _describe -t commands 'my-app foo commands' commands "$@"
}
(( $+functions[_my-app__help_commands] )) ||
_my-app__help_commands() {
    local commands; commands=(
'foo:' \
'bar:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'my-app help commands' commands "$@"
}
(( $+functions[_my-app__help__bar_commands] )) ||
_my-app__help__bar_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help bar commands' commands "$@"
}
(( $+functions[_my-app__help__foo_commands] )) ||
_my-app__help__foo_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help foo commands' commands "$@"
}
(( $+functions[_my-app__help__help_commands] )) ||
_my-app__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help help commands' commands "$@"
}

if [ "$funcstack[1]" = "_my-app" ]; then
    _my-app "$@"
else
    compdef _my-app my-app
fi
