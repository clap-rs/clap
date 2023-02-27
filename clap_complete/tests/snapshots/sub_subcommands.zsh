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
'*-c[some config file]' \
'*-C[some config file]' \
'*--config[some config file]' \
'*--conf[some config file]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
'::file -- some input file:_files' \
'::choice:(first second)' \
":: :_my-app_commands" \
"*::: :->my-app" \
&& ret=0
    case $state in
    (my-app)
        words=($line[3] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-command-$line[3]:"
        case $line[3] in
            (test)
_arguments "${_arguments_options[@]}" \
'--case=[the case to test]: : ' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
&& ret=0
;;
(some_cmd)
_arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_my-app__some_cmd_commands" \
"*::: :->some_cmd" \
&& ret=0

    case $state in
    (some_cmd)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-some_cmd-command-$line[1]:"
        case $line[1] in
            (sub_cmd)
_arguments "${_arguments_options[@]}" \
'--config=[the other case to test]: :((Lest\ quotes,\ aren'\''t\ escaped.\:"help,with,comma"
Second\ to\ trigger\ display\ of\ options\:""))' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'-V[Print version]' \
'--version[Print version]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_my-app__some_cmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-some_cmd-help-command-$line[1]:"
        case $line[1] in
            (sub_cmd)
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
            (test)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(some_cmd)
_arguments "${_arguments_options[@]}" \
":: :_my-app__help__some_cmd_commands" \
"*::: :->some_cmd" \
&& ret=0

    case $state in
    (some_cmd)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-help-some_cmd-command-$line[1]:"
        case $line[1] in
            (sub_cmd)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
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
'test:tests things' \
'some_cmd:top level subcommand' \
'some_cmd_alias:top level subcommand' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'my-app commands' commands "$@"
}
(( $+functions[_my-app__help_commands] )) ||
_my-app__help_commands() {
    local commands; commands=(
'test:tests things' \
'some_cmd:top level subcommand' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'my-app help commands' commands "$@"
}
(( $+functions[_my-app__help__help_commands] )) ||
_my-app__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help help commands' commands "$@"
}
(( $+functions[_my-app__some_cmd__help_commands] )) ||
_my-app__some_cmd__help_commands() {
    local commands; commands=(
'sub_cmd:sub-subcommand' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'my-app some_cmd help commands' commands "$@"
}
(( $+functions[_my-app__some_cmd__help__help_commands] )) ||
_my-app__some_cmd__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'my-app some_cmd help help commands' commands "$@"
}
(( $+functions[_my-app__help__some_cmd_commands] )) ||
_my-app__help__some_cmd_commands() {
    local commands; commands=(
'sub_cmd:sub-subcommand' \
    )
    _describe -t commands 'my-app help some_cmd commands' commands "$@"
}
(( $+functions[_my-app__some_cmd_commands] )) ||
_my-app__some_cmd_commands() {
    local commands; commands=(
'sub_cmd:sub-subcommand' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'my-app some_cmd commands' commands "$@"
}
(( $+functions[_my-app__help__some_cmd__sub_cmd_commands] )) ||
_my-app__help__some_cmd__sub_cmd_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help some_cmd sub_cmd commands' commands "$@"
}
(( $+functions[_my-app__some_cmd__help__sub_cmd_commands] )) ||
_my-app__some_cmd__help__sub_cmd_commands() {
    local commands; commands=()
    _describe -t commands 'my-app some_cmd help sub_cmd commands' commands "$@"
}
(( $+functions[_my-app__some_cmd__sub_cmd_commands] )) ||
_my-app__some_cmd__sub_cmd_commands() {
    local commands; commands=()
    _describe -t commands 'my-app some_cmd sub_cmd commands' commands "$@"
}
(( $+functions[_my-app__help__test_commands] )) ||
_my-app__help__test_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help test commands' commands "$@"
}
(( $+functions[_my-app__test_commands] )) ||
_my-app__test_commands() {
    local commands; commands=()
    _describe -t commands 'my-app test commands' commands "$@"
}

if [ "$funcstack[1]" = "_my-app" ]; then
    _my-app "$@"
else
    compdef _my-app my-app
fi
