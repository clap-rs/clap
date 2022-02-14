use clap::PossibleValue;

use super::*;

fn build_app() -> App<'static> {
    build_app_with_name("myapp")
}

fn build_app_with_name(s: &'static str) -> App<'static> {
    App::new(s)
        .version("3.0")
        .propagate_version(true)
        .about("Tests completions")
        .arg(
            Arg::new("file")
                .value_hint(ValueHint::FilePath)
                .help("some input file"),
        )
        .subcommand(
            App::new("test").about("tests things").arg(
                Arg::new("case")
                    .long("case")
                    .takes_value(true)
                    .help("the case to test"),
            ),
        )
}

#[test]
fn fish() {
    let mut app = build_app();
    common(Fish, &mut app, "myapp", FISH);
}

static FISH: &str = r#"complete -c myapp -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c myapp -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c myapp -n "__fish_use_subcommand" -f -a "test" -d 'tests things'
complete -c myapp -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c myapp -n "__fish_seen_subcommand_from test" -l case -d 'the case to test' -r
complete -c myapp -n "__fish_seen_subcommand_from test" -s h -l help -d 'Print help information'
complete -c myapp -n "__fish_seen_subcommand_from test" -s V -l version -d 'Print version information'
"#;

#[test]
fn fish_with_special_commands() {
    let mut app = build_app_special_commands();
    common(Fish, &mut app, "my_app", FISH_SPECIAL_CMDS);
}

fn build_app_special_commands() -> App<'static> {
    build_app_with_name("my_app")
        .subcommand(
            App::new("some_cmd").about("tests other things").arg(
                Arg::new("config")
                    .long("--config")
                    .takes_value(true)
                    .help("the other case to test"),
            ),
        )
        .subcommand(App::new("some-cmd-with-hyphens").alias("hyphen"))
}

static FISH_SPECIAL_CMDS: &str = r#"complete -c my_app -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c my_app -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c my_app -n "__fish_use_subcommand" -f -a "test" -d 'tests things'
complete -c my_app -n "__fish_use_subcommand" -f -a "some_cmd" -d 'tests other things'
complete -c my_app -n "__fish_use_subcommand" -f -a "some-cmd-with-hyphens"
complete -c my_app -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my_app -n "__fish_seen_subcommand_from test" -l case -d 'the case to test' -r
complete -c my_app -n "__fish_seen_subcommand_from test" -s h -l help -d 'Print help information'
complete -c my_app -n "__fish_seen_subcommand_from test" -s V -l version -d 'Print version information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd" -l config -d 'the other case to test' -r
complete -c my_app -n "__fish_seen_subcommand_from some_cmd" -s h -l help -d 'Print help information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd" -s V -l version -d 'Print version information'
complete -c my_app -n "__fish_seen_subcommand_from some-cmd-with-hyphens" -s h -l help -d 'Print help information'
complete -c my_app -n "__fish_seen_subcommand_from some-cmd-with-hyphens" -s V -l version -d 'Print version information'
"#;

#[test]
fn fish_with_special_help() {
    let mut app = build_app_special_help();
    common(Fish, &mut app, "my_app", FISH_SPECIAL_HELP);
}

fn build_app_special_help() -> App<'static> {
    App::new("my_app")
        .version("3.0")
        .arg(
            Arg::new("single-quotes")
                .long("single-quotes")
                .help("Can be 'always', 'auto', or 'never'"),
        )
        .arg(
            Arg::new("double-quotes")
                .long("double-quotes")
                .help("Can be \"always\", \"auto\", or \"never\""),
        )
        .arg(
            Arg::new("backticks")
                .long("backticks")
                .help("For more information see `echo test`"),
        )
        .arg(Arg::new("backslash").long("backslash").help("Avoid '\\n'"))
        .arg(
            Arg::new("brackets")
                .long("brackets")
                .help("List packages [filter]"),
        )
        .arg(
            Arg::new("expansions")
                .long("expansions")
                .help("Execute the shell command with $SHELL"),
        )
}

static FISH_SPECIAL_HELP: &str = r#"complete -c my_app -s h -l help -d 'Print help information'
complete -c my_app -s V -l version -d 'Print version information'
complete -c my_app -l single-quotes -d 'Can be \'always\', \'auto\', or \'never\''
complete -c my_app -l double-quotes -d 'Can be "always", "auto", or "never"'
complete -c my_app -l backticks -d 'For more information see `echo test`'
complete -c my_app -l backslash -d 'Avoid \'\\n\''
complete -c my_app -l brackets -d 'List packages [filter]'
complete -c my_app -l expansions -d 'Execute the shell command with $SHELL'
"#;

#[test]
fn fish_with_aliases() {
    let mut app = build_app_with_aliases();
    common(Fish, &mut app, "cmd", FISH_ALIASES);
}

fn build_app_with_aliases() -> App<'static> {
    App::new("cmd")
        .version("3.0")
        .about("testing bash completions")
        .arg(
            Arg::new("flag")
                .short('f')
                .visible_short_alias('F')
                .long("flag")
                .visible_alias("flg")
                .help("cmd flag"),
        )
        .arg(
            Arg::new("option")
                .short('o')
                .visible_short_alias('O')
                .long("option")
                .visible_alias("opt")
                .help("cmd option")
                .takes_value(true),
        )
        .arg(Arg::new("positional"))
}

static FISH_ALIASES: &str = r#"complete -c cmd -s o -s O -l option -l opt -d 'cmd option' -r
complete -c cmd -s h -l help -d 'Print help information'
complete -c cmd -s V -l version -d 'Print version information'
complete -c cmd -s f -s F -l flag -l flg -d 'cmd flag'
"#;

#[test]
fn fish_with_sub_subcommands() {
    let mut app = build_app_sub_subcommands();
    common(Fish, &mut app, "my_app", FISH_SUB_SUBCMDS);
}

fn build_app_sub_subcommands() -> App<'static> {
    build_app_with_name("my_app").subcommand(
        App::new("some_cmd")
            .about("top level subcommand")
            .subcommand(
                App::new("sub_cmd").about("sub-subcommand").arg(
                    Arg::new("config")
                        .long("--config")
                        .takes_value(true)
                        .possible_values([PossibleValue::new("Lest quotes aren't escaped.")])
                        .help("the other case to test"),
                ),
            ),
    )
}

static FISH_SUB_SUBCMDS: &str = r#"complete -c my_app -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c my_app -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c my_app -n "__fish_use_subcommand" -f -a "test" -d 'tests things'
complete -c my_app -n "__fish_use_subcommand" -f -a "some_cmd" -d 'top level subcommand'
complete -c my_app -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my_app -n "__fish_seen_subcommand_from test" -l case -d 'the case to test' -r
complete -c my_app -n "__fish_seen_subcommand_from test" -s h -l help -d 'Print help information'
complete -c my_app -n "__fish_seen_subcommand_from test" -s V -l version -d 'Print version information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -s h -l help -d 'Print help information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -s V -l version -d 'Print version information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -f -a "sub_cmd" -d 'sub-subcommand'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd; and not __fish_seen_subcommand_from sub_cmd; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd; and __fish_seen_subcommand_from sub_cmd" -l config -d 'the other case to test' -r -f -a "{Lest quotes aren\'t escaped.	}"
complete -c my_app -n "__fish_seen_subcommand_from some_cmd; and __fish_seen_subcommand_from sub_cmd" -s h -l help -d 'Print help information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd; and __fish_seen_subcommand_from sub_cmd" -s V -l version -d 'Print version information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd; and __fish_seen_subcommand_from help" -s h -l help -d 'Print help information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd; and __fish_seen_subcommand_from help" -s V -l version -d 'Print version information'
"#;
