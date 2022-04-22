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
'-o+[cmd option]: : ' /
'-O+[cmd option]: : ' /
'--option=[cmd option]: : ' /
'--opt=[cmd option]: : ' /
'-h[Print help information]' /
'--help[Print help information]' /
'-V[Print version information]' /
'--version[Print version information]' /
'-f[cmd flag]' /
'-F[cmd flag]' /
'--flag[cmd flag]' /
'--flg[cmd flag]' /
'::positional:' /
&& ret=0
}

(( $+functions[_my-app_commands] )) ||
_my-app_commands() {
    local commands; commands=()
    _describe -t commands 'my-app commands' commands "$@"
}

_my-app "$@"
