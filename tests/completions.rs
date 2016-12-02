extern crate clap;

use clap::{App, Arg, SubCommand, Shell};

#[test]
fn test_generation() {
   let mut app = App::new("myapp")
        .about("Tests completions")
        .arg(Arg::with_name("file")
            .help("some input file"))
        .subcommand(SubCommand::with_name("test")
            .about("tests things")
            .arg(Arg::with_name("case")
                .long("case")
                .takes_value(true)
                .help("the case to test")));
    let mut buf = vec![];
    app.gen_completions_to("myapp", Shell::Bash, &mut buf);
    let string = String::from_utf8(buf).unwrap();
    let first_line = string.lines().nth(0).unwrap();
    let last_line = string.lines().rev().nth(0).unwrap();

    assert_eq!(first_line, "_myapp() {");
    assert_eq!(last_line, "complete -F _myapp -o bashdefault -o default myapp");
}
