use clap::{Arg, Command};

#[test]
fn borrowed_args() {
    let arg = Arg::new("some").short('s').long("some").help("other help");
    let arg2 = Arg::new("some2")
        .short('S')
        .long("some-thing")
        .help("other help");
    let result = Command::new("sub_command_negate")
        .arg(Arg::new("test").index(1))
        .arg(&arg)
        .arg(&arg2)
        .subcommand(Command::new("sub1").arg(&arg))
        .try_get_matches_from(vec!["prog"]);
    assert!(result.is_ok(), "{}", result.unwrap_err());
}
