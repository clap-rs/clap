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
fn elvish() {
    let mut app = build_app();
    common::<Elvish>(&mut app, "my_app", ELVISH);
}

static ELVISH: &str = r#"
use str;
edit:completion:arg-completer[my_app] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'my_app'
    for word $words[1:-1] {
        if (str:has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'my_app'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand test 'tests things'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'my_app;test'= {
            cand --case 'the case to test'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'my_app;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
"#;

#[test]
fn elvish_with_special_commands() {
    let mut app = build_app_special_commands();
    common::<Elvish>(&mut app, "my_app", ELVISH_SPECIAL_CMDS);
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
}

static ELVISH_SPECIAL_CMDS: &str = r#"
use str;
edit:completion:arg-completer[my_app] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'my_app'
    for word $words[1:-1] {
        if (str:has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'my_app'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand test 'tests things'
            cand some_cmd 'tests other things'
            cand some-cmd-with-hypens 'some-cmd-with-hypens'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'my_app;test'= {
            cand --case 'the case to test'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'my_app;some_cmd'= {
            cand --config 'the other case to test'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'my_app;some-cmd-with-hypens'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'my_app;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
"#;

#[test]
fn elvish_with_aliases() {
    let mut app = build_app_with_aliases();
    common::<Elvish>(&mut app, "cmd", ELVISH_ALIASES);
}

fn build_app_with_aliases() -> App<'static> {
    App::new("cmd")
        .about("testing bash completions")
        .arg(
            Arg::new("flag")
                .short('f')
                .visible_short_alias('F')
                .long("flag")
                .visible_alias("flg")
                .about("cmd flag"),
        )
        .arg(
            Arg::new("option")
                .short('o')
                .visible_short_alias('O')
                .long("option")
                .visible_alias("opt")
                .about("cmd option")
                .takes_value(true),
        )
        .arg(Arg::new("positional"))
}

static ELVISH_ALIASES: &str = r#"
use str;
edit:completion:arg-completer[cmd] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'cmd'
    for word $words[1:-1] {
        if (str:has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'cmd'= {
            cand -o 'cmd option'
            cand -O 'cmd option'
            cand --option 'cmd option'
            cand --opt 'cmd option'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand -f 'cmd flag'
            cand -F 'cmd flag'
            cand --flag 'cmd flag'
            cand --flg 'cmd flag'
        }
    ]
    $completions[$command]
}
"#;
