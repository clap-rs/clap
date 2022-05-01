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
'*-c[some config file]' /
'*-C[some config file]' /
'*--config[some config file]' /
'*--conf[some config file]' /
'::file -- some input file:_files' /
'::choice:(first second)' /
":: :_my-app_commands" /
"*::: :->my-app" /
&& ret=0
    case $state in
    (my-app)
        words=($line[3] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-command-$line[3]:"
        case $line[3] in
            (test)
_arguments "${_arguments_options[@]}" /
'--case=[the case to test]: : ' /
'-h[Print help information]' /
'--help[Print help information]' /
'-V[Print version information]' /
'--version[Print version information]' /
&& ret=0
;;
(some_cmd)
_arguments "${_arguments_options[@]}" /
'-h[Print help information]' /
'--help[Print help information]' /
'-V[Print version information]' /
'--version[Print version information]' /
":: :_my-app__some_cmd_commands" /
"*::: :->some_cmd" /
&& ret=0

    case $state in
    (some_cmd)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my-app-some_cmd-command-$line[1]:"
        case $line[1] in
            (sub_cmd)
_arguments "${_arguments_options[@]}" /
'--config=[the other case to test]: :(Lest quotes aren't escaped.)' /
'-h[Print help information]' /
'--help[Print help information]' /
'-V[Print version information]' /
'--version[Print version information]' /
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
'test:tests things' /
'some_cmd:top level subcommand' /
'help:Print this message or the help of the given subcommand(s)' /
    )
    _describe -t commands 'my-app commands' commands "$@"
}
(( $+functions[_my-app__help_commands] )) ||
_my-app__help_commands() {
    local commands; commands=()
    _describe -t commands 'my-app help commands' commands "$@"
}
(( $+functions[_my-app__some_cmd__help_commands] )) ||
_my-app__some_cmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'my-app some_cmd help commands' commands "$@"
}
(( $+functions[_my-app__some_cmd_commands] )) ||
_my-app__some_cmd_commands() {
    local commands; commands=(
'sub_cmd:sub-subcommand' /
'help:Print this message or the help of the given subcommand(s)' /
    )
    _describe -t commands 'my-app some_cmd commands' commands "$@"
}
(( $+functions[_my-app__some_cmd__sub_cmd_commands] )) ||
_my-app__some_cmd__sub_cmd_commands() {
    local commands; commands=()
    _describe -t commands 'my-app some_cmd sub_cmd commands' commands "$@"
}
(( $+functions[_my-app__test_commands] )) ||
_my-app__test_commands() {
    local commands; commands=()
    _describe -t commands 'my-app test commands' commands "$@"
}

_my-app "$@"
