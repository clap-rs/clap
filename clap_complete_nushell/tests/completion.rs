mod common;

#[test]
fn register_completion() {
    common::register_example("test", completest::Shell::Nu);
}

#[test]
fn completion() {
    let term = completest::Term::new();
    let mut runtime = common::load_runtime("test", completest::Shell::Nu);

    let input = "test -\t";
    let expected = r#"% test -
--generate    generate
--global    everywhere
--help    Print help
--version    Print version
-V    Print version
-h    Print help
"#;
    let actual = runtime.complete(input, &term).unwrap();
    snapbox::assert_eq(expected, actual);

    let input = "test action -\t";
    let expected = r#"% test action -
--choice    enum
--count    number
--global    everywhere
--help    Print help
--set    value
--set-true    bool
--version    Print version
-V    Print version
-h    Print help
"#;
    let actual = runtime.complete(input, &term).unwrap();
    snapbox::assert_eq(expected, actual);
}

#[test]
fn completion_value_hint() {
    let term = completest::Term::new();
    let mut runtime = common::load_runtime("test", completest::Shell::Nu);

    let input = "test hint -\t";
    let expected = r#"% test hint -
--choice
--cmd
--cmd-name
--dir
--email
--exe
--file
--global    everywhere
--help    Print help
--host
--other
--path
--unknown
--url
--user
--version    Print version
-H
-V    Print version
-c
-d
-e
-f
-h    Print help
-p
-u
"#;
    let actual = runtime.complete(input, &term).unwrap();
    snapbox::assert_eq(expected, actual);

    let input = "test hint --choice \t";
    let expected = r#"% test hint --choice 
bash
fish
zsh
"#;
    let actual = runtime.complete(input, &term).unwrap();
    snapbox::assert_eq(expected, actual);
}
