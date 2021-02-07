use super::*;

fn build_app() -> App<'static> {
    build_app_with_name("myapp")
}

fn build_app_with_name(s: &'static str) -> App<'static> {
    App::new(s)
        .about("Tests completions")
        .arg(
            Arg::new("file")
                .value_hint(ValueHint::FilePath)
                .about("some input file"),
        )
        .subcommand(
            App::new("test").about("tests things").arg(
                Arg::new("case")
                    .long("case")
                    .takes_value(true)
                    .about("the case to test"),
            ),
        )
}

#[test]
fn zsh() {
    let mut app = build_app();
    common::<Zsh>(&mut app, "myapp", ZSH);
}

static ZSH: &str = r#"#compdef myapp

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
'test:tests things' \
'help:Prints this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'myapp commands' commands "$@"
}
(( $+functions[_myapp__help_commands] )) ||
_myapp__help_commands() {
    local commands; commands=()
    _describe -t commands 'myapp help commands' commands "$@"
}
(( $+functions[_myapp__test_commands] )) ||
_myapp__test_commands() {
    local commands; commands=()
    _describe -t commands 'myapp test commands' commands "$@"
}

_myapp "$@""#;

#[test]
fn zsh_with_special_commands() {
    let mut app = build_app_special_commands();
    common::<Zsh>(&mut app, "my_app", ZSH_SPECIAL_CMDS);
}

fn build_app_special_commands() -> App<'static> {
    build_app_with_name("my_app")
        .subcommand(
            App::new("some_cmd").about("tests other things").arg(
                Arg::new("config")
                    .long("--config")
                    .takes_value(true)
                    .about("the other case to test"),
            ),
        )
        .subcommand(App::new("some-cmd-with-hypens").alias("hyphen"))
        .subcommand(
            App::new("some_cmd_with_special_characters")
                .about("This 'is' a \"special\" [character] string \\"),
        )
}

static ZSH_SPECIAL_CMDS: &str = r#"#compdef my_app

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
(some_cmd_with_special_characters)
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
'test:tests things' \
'some_cmd:tests other things' \
'some-cmd-with-hypens:' \
'some_cmd_with_special_characters:This '\''is'\'' a "special" \[character\] string \\' \
'help:Prints this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'my_app commands' commands "$@"
}
(( $+functions[_my_app__help_commands] )) ||
_my_app__help_commands() {
    local commands; commands=()
    _describe -t commands 'my_app help commands' commands "$@"
}
(( $+functions[_my_app__some-cmd-with-hypens_commands] )) ||
_my_app__some-cmd-with-hypens_commands() {
    local commands; commands=()
    _describe -t commands 'my_app some-cmd-with-hypens commands' commands "$@"
}
(( $+functions[_my_app__some_cmd_commands] )) ||
_my_app__some_cmd_commands() {
    local commands; commands=()
    _describe -t commands 'my_app some_cmd commands' commands "$@"
}
(( $+functions[_my_app__some_cmd_with_special_characters_commands] )) ||
_my_app__some_cmd_with_special_characters_commands() {
    local commands; commands=()
    _describe -t commands 'my_app some_cmd_with_special_characters commands' commands "$@"
}
(( $+functions[_my_app__test_commands] )) ||
_my_app__test_commands() {
    local commands; commands=()
    _describe -t commands 'my_app test commands' commands "$@"
}

_my_app "$@""#;

#[test]
fn zsh_with_special_help() {
    let mut app = build_app_special_help();
    common::<Zsh>(&mut app, "my_app", ZSH_SPECIAL_HELP);
}

fn build_app_special_help() -> App<'static> {
    App::new("my_app")
        .arg(
            Arg::new("single-quotes")
                .long("single-quotes")
                .about("Can be 'always', 'auto', or 'never'"),
        )
        .arg(
            Arg::new("double-quotes")
                .long("double-quotes")
                .about("Can be \"always\", \"auto\", or \"never\""),
        )
        .arg(
            Arg::new("backticks")
                .long("backticks")
                .about("For more information see `echo test`"),
        )
        .arg(Arg::new("backslash").long("backslash").about("Avoid '\\n'"))
        .arg(
            Arg::new("brackets")
                .long("brackets")
                .about("List packages [filter]"),
        )
        .arg(
            Arg::new("expansions")
                .long("expansions")
                .about("Execute the shell command with $SHELL"),
        )
}

static ZSH_SPECIAL_HELP: &str = r#"#compdef my_app

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
'--single-quotes[Can be '\''always'\'', '\''auto'\'', or '\''never'\'']' \
'--double-quotes[Can be "always", "auto", or "never"]' \
'--backticks[For more information see `echo test`]' \
'--backslash[Avoid '\''\\n'\'']' \
'--brackets[List packages \[filter\]]' \
'--expansions[Execute the shell command with $SHELL]' \
&& ret=0
    
}

(( $+functions[_my_app_commands] )) ||
_my_app_commands() {
    local commands; commands=()
    _describe -t commands 'my_app commands' commands "$@"
}

_my_app "$@""#;

#[test]
fn zsh_with_nested_subcommands() {
    let mut app = build_app_nested_subcommands();
    common::<Zsh>(&mut app, "my_app", ZSH_NESTED_SUBCOMMANDS);
}

fn build_app_nested_subcommands() -> App<'static> {
    App::new("first").subcommand(App::new("second").subcommand(App::new("third")))
}

static ZSH_NESTED_SUBCOMMANDS: &str = r#"#compdef my_app

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
":: :_my_app_commands" \
"*::: :->first" \
&& ret=0
    case $state in
    (first)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my_app-command-$line[1]:"
        case $line[1] in
            (second)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_my_app__second_commands" \
"*::: :->second" \
&& ret=0
case $state in
    (second)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:my_app-second-command-$line[1]:"
        case $line[1] in
            (third)
_arguments "${_arguments_options[@]}" \
'--help[Prints help information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
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
'second:' \
'help:Prints this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'my_app commands' commands "$@"
}
(( $+functions[_my_app__help_commands] )) ||
_my_app__help_commands() {
    local commands; commands=()
    _describe -t commands 'my_app help commands' commands "$@"
}
(( $+functions[_my_app__second_commands] )) ||
_my_app__second_commands() {
    local commands; commands=(
'third:' \
    )
    _describe -t commands 'my_app second commands' commands "$@"
}
(( $+functions[_my_app__second__third_commands] )) ||
_my_app__second__third_commands() {
    local commands; commands=()
    _describe -t commands 'my_app second third commands' commands "$@"
}

_my_app "$@""#;
