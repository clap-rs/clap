use super::*;

#[test]
fn elvish() {
    let mut app = build_app();
    common::<Elvish>(&mut app, "my_app", ELVISH);
}

static ELVISH: &str = r#"
edit:completion:arg-completer[my_app] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'my_app'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
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

static ELVISH_SPECIAL_CMDS: &str = r#"
edit:completion:arg-completer[my_app] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'my_app'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
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
