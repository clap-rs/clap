use super::*;

#[test]
fn fish() {
    let mut app = build_app();
    common::<Fish>(&mut app, "myapp", FISH);
}

static FISH: &str = r#"complete -c myapp -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c myapp -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
complete -c myapp -n "__fish_use_subcommand" -f -a "test" -d 'tests things'
complete -c myapp -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c myapp -n "__fish_seen_subcommand_from test" -l case -d 'the case to test' -r
complete -c myapp -n "__fish_seen_subcommand_from test" -s h -l help -d 'Prints help information'
complete -c myapp -n "__fish_seen_subcommand_from test" -s V -l version -d 'Prints version information'
complete -c myapp -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c myapp -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
"#;

#[test]
fn fish_with_special_commands() {
    let mut app = build_app_special_commands();
    common::<Fish>(&mut app, "my_app", FISH_SPECIAL_CMDS);
}

static FISH_SPECIAL_CMDS: &str = r#"complete -c my_app -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
complete -c my_app -n "__fish_use_subcommand" -f -a "test" -d 'tests things'
complete -c my_app -n "__fish_use_subcommand" -f -a "some_cmd" -d 'tests other things'
complete -c my_app -n "__fish_use_subcommand" -f -a "some-cmd-with-hypens"
complete -c my_app -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c my_app -n "__fish_seen_subcommand_from test" -l case -d 'the case to test' -r
complete -c my_app -n "__fish_seen_subcommand_from test" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_seen_subcommand_from test" -s V -l version -d 'Prints version information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd" -l config -d 'the other case to test' -r
complete -c my_app -n "__fish_seen_subcommand_from some_cmd" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_seen_subcommand_from some_cmd" -s V -l version -d 'Prints version information'
complete -c my_app -n "__fish_seen_subcommand_from some-cmd-with-hypens" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_seen_subcommand_from some-cmd-with-hypens" -s V -l version -d 'Prints version information'
complete -c my_app -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
"#;

#[test]
fn fish_with_special_help() {
    let mut app = build_app_special_help();
    common::<Fish>(&mut app, "my_app", FISH_SPECIAL_HELP);
}

static FISH_SPECIAL_HELP: &str = r#"complete -c my_app -n "__fish_use_subcommand" -l single-quotes -d 'Can be \'always\', \'auto\', or \'never\''
complete -c my_app -n "__fish_use_subcommand" -l double-quotes -d 'Can be "always", "auto", or "never"'
complete -c my_app -n "__fish_use_subcommand" -l backticks -d 'For more information see `echo test`'
complete -c my_app -n "__fish_use_subcommand" -l backslash -d 'Avoid \'\\n\''
complete -c my_app -n "__fish_use_subcommand" -l brackets -d 'List packages [filter]'
complete -c my_app -n "__fish_use_subcommand" -l expansions -d 'Execute the shell command with $SHELL'
complete -c my_app -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c my_app -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
"#;
