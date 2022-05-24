use clap::FromArgMatches;
use clap::Subcommand;

fn command() -> clap::Command<'static> {
    let cmd = clap::Command::new("dynamic")
        .arg(
            clap::Arg::new("input")
                .long("--input")
                .short('i')
                .value_hint(clap::ValueHint::FilePath),
        )
        .arg(
            clap::Arg::new("format")
                .long("--format")
                .short('F')
                .value_parser(["json", "yaml", "toml"]),
        )
        .args_conflicts_with_subcommands(true);
    let cmd = clap_complete::dynamic::bash::CompleteCommand::augment_subcommands(cmd);
    cmd
}

fn main() {
    let cmd = command();
    let matches = cmd.get_matches();
    if let Ok(completions) =
        clap_complete::dynamic::bash::CompleteCommand::from_arg_matches(&matches)
    {
        completions.complete(&mut command());
    } else {
        println!("{:#?}", matches);
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    command().command().debug_assert();
}
