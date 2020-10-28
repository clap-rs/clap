use clap::{App, AppSettings, Arg, ValueHint};
use clap_generate::generators::*;
use completions::common;

mod completions;

pub fn build_app_with_value_hints() -> App<'static> {
    App::new("my_app")
        .setting(AppSettings::DisableVersionFlag)
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::new("choice")
                .long("choice")
                .possible_values(&["bash", "fish", "zsh"]),
        )
        .arg(
            Arg::new("unknown")
                .long("unknown")
                .value_hint(ValueHint::Unknown),
        )
        .arg(Arg::new("other").long("other").value_hint(ValueHint::Other))
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .value_hint(ValueHint::AnyPath),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .value_hint(ValueHint::DirPath),
        )
        .arg(
            Arg::new("exe")
                .long("exe")
                .short('e')
                .value_hint(ValueHint::ExecutablePath),
        )
        .arg(
            Arg::new("cmd_name")
                .long("cmd-name")
                .value_hint(ValueHint::CommandName),
        )
        .arg(
            Arg::new("cmd")
                .long("cmd")
                .short('c')
                .value_hint(ValueHint::CommandString),
        )
        .arg(
            Arg::new("command_with_args")
                .multiple_values(true)
                .value_hint(ValueHint::CommandWithArguments),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .value_hint(ValueHint::Username),
        )
        .arg(
            Arg::new("host")
                .short('h')
                .long("host")
                .value_hint(ValueHint::Hostname),
        )
        .arg(Arg::new("url").long("url").value_hint(ValueHint::Url))
        .arg(
            Arg::new("email")
                .long("email")
                .value_hint(ValueHint::EmailAddress),
        )
}

static ZSH_VALUE_HINTS: &str = r#"#compdef my_app

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
'--choice=[]: :(bash fish zsh)' \
'--unknown=[]' \
'--other=[]: :( )' \
'-p+[]: :_files' \
'--path=[]: :_files' \
'-f+[]: :_files' \
'--file=[]: :_files' \
'-d+[]: :_files -/' \
'--dir=[]: :_files -/' \
'-e+[]: :_absolute_command_paths' \
'--exe=[]: :_absolute_command_paths' \
'--cmd-name=[]: :_command_names -e' \
'-c+[]: :_cmdstring' \
'--cmd=[]: :_cmdstring' \
'-u+[]: :_users' \
'--user=[]: :_users' \
'-h+[]: :_hosts' \
'--host=[]: :_hosts' \
'--url=[]: :_urls' \
'--email=[]: :_email_addresses' \
'--help[Prints help information]' \
'*::command_with_args:_cmdambivalent' \
&& ret=0
    
}

(( $+functions[_my_app_commands] )) ||
_my_app_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'my_app commands' commands "$@"
}

_my_app "$@""#;

static FISH_VALUE_HINTS: &str = r#"complete -c my_app -n "__fish_use_subcommand" -l choice -r -f -a "bash fish zsh"
complete -c my_app -n "__fish_use_subcommand" -l unknown -r
complete -c my_app -n "__fish_use_subcommand" -l other -r -f
complete -c my_app -n "__fish_use_subcommand" -s p -l path -r -F
complete -c my_app -n "__fish_use_subcommand" -s f -l file -r -F
complete -c my_app -n "__fish_use_subcommand" -s d -l dir -r -f -a "(__fish_complete_directories)"
complete -c my_app -n "__fish_use_subcommand" -s e -l exe -r -F
complete -c my_app -n "__fish_use_subcommand" -l cmd-name -r -f -a "(__fish_complete_command)"
complete -c my_app -n "__fish_use_subcommand" -s c -l cmd -r -f -a "(__fish_complete_command)"
complete -c my_app -n "__fish_use_subcommand" -s u -l user -r -f -a "(__fish_complete_users)"
complete -c my_app -n "__fish_use_subcommand" -s h -l host -r -f -a "(__fish_print_hostnames)"
complete -c my_app -n "__fish_use_subcommand" -l url -r -f
complete -c my_app -n "__fish_use_subcommand" -l email -r -f
complete -c my_app -n "__fish_use_subcommand" -l help -d 'Prints help information'
"#;

#[test]
fn zsh_with_value_hints() {
    let mut app = build_app_with_value_hints();
    common::<Zsh>(&mut app, "my_app", ZSH_VALUE_HINTS);
}

#[test]
fn fish_with_value_hints() {
    let mut app = build_app_with_value_hints();
    common::<Fish>(&mut app, "my_app", FISH_VALUE_HINTS);
}
