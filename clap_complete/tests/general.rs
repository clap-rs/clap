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
