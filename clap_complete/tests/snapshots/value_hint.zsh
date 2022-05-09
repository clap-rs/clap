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
'--choice=[]: :(bash fish zsh)' /
'--unknown=[]: : ' /
'--other=[]: :( )' /
'-p+[]: :_files' /
'--path=[]: :_files' /
'-f+[]: :_files' /
'--file=[]: :_files' /
'-d+[]: :_files -/' /
'--dir=[]: :_files -/' /
'-e+[]: :_absolute_command_paths' /
'--exe=[]: :_absolute_command_paths' /
'--cmd-name=[]: :_command_names -e' /
'-c+[]: :_cmdstring' /
'--cmd=[]: :_cmdstring' /
'-u+[]: :_users' /
'--user=[]: :_users' /
'-h+[]: :_hosts' /
'--host=[]: :_hosts' /
'--url=[]: :_urls' /
'--email=[]: :_email_addresses' /
'--help[Print help information]' /
':positional:(stable nightly)' /
'*:::command_with_args:_cmdambivalent' /
&& ret=0
}

(( $+functions[_my-app_commands] )) ||
_my-app_commands() {
    local commands; commands=()
    _describe -t commands 'my-app commands' commands "$@"
}

_my-app "$@"
