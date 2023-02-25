#[test]
fn infer_value_hint_for_path_buf() {
    let mut cmd = clap::Command::new("completer")
        .arg(clap::Arg::new("input").value_parser(clap::value_parser!(std::path::PathBuf)));
    cmd.build();
    let input = cmd
        .get_arguments()
        .find(|arg| arg.get_id() == "input")
        .unwrap();
    assert_eq!(input.get_value_hint(), clap::builder::ValueHint::AnyPath);
}

#[cfg(windows)]
#[test]
fn shell_absolute_path() {
    use clap_complete::Shell;

    let path = std::path::Path::new(&std::env::var("PSHOME").unwrap()).join("powershell.exe");

    let matches = clap::Command::new("test")
        .arg(clap::Arg::new("shell").value_parser(clap::value_parser!(Shell)))
        .get_matches_from(["myprog", path.to_str().unwrap()]);

    assert_eq!(
        matches.get_one::<Shell>("shell").unwrap(),
        &Shell::PowerShell
    );
}

#[cfg(not(windows))]
#[test]
fn shell_absolute_path() {
    use clap_complete::Shell;

    let matches = clap::Command::new("test")
        .arg(clap::Arg::new("shell").value_parser(clap::value_parser!(Shell)))
        .get_matches_from(["myprog", "/bin/bash"]);

    assert_eq!(matches.get_one::<Shell>("shell").unwrap(), &Shell::Bash);
}
