#compdef bin-name

autoload -U is-at-least

_bin-name() {
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
'-c[]' \
'(-c)-v[]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_bin-name_commands" \
"*::: :->my-app" \
&& ret=0
    case $state in
    (my-app)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:bin-name-command-$line[1]:"
        case $line[1] in
            (test)
_arguments "${_arguments_options[@]}" : \
'*-d[]' \
'-c[]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_bin-name__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:bin-name-help-command-$line[1]:"
        case $line[1] in
            (test)
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

(( $+functions[_bin-name_commands] )) ||
_bin-name_commands() {
    local commands; commands=(
'test:Subcommand with a second line' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'bin-name commands' commands "$@"
}
(( $+functions[_bin-name__help_commands] )) ||
_bin-name__help_commands() {
    local commands; commands=(
'test:Subcommand with a second line' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'bin-name help commands' commands "$@"
}
(( $+functions[_bin-name__help__help_commands] )) ||
_bin-name__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'bin-name help help commands' commands "$@"
}
(( $+functions[_bin-name__help__test_commands] )) ||
_bin-name__help__test_commands() {
    local commands; commands=()
    _describe -t commands 'bin-name help test commands' commands "$@"
}
(( $+functions[_bin-name__test_commands] )) ||
_bin-name__test_commands() {
    local commands; commands=()
    _describe -t commands 'bin-name test commands' commands "$@"
}

if [ "$funcstack[1]" = "_bin-name" ]; then
    _bin-name "$@"
else
    compdef _bin-name bin-name
fi
