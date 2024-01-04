use clap::builder::PossibleValue;
use clap::FromArgMatches;
use clap::Subcommand;

fn command() -> clap::Command {
    let cmd = clap::Command::new("dynamic")
        .subcommand(
            clap::Command::new("hidden")
                .about("Hidden subcommand")
                .hide(true),
        )
        .arg(
            clap::Arg::new("input")
                .long("input")
                .short('i')
                .value_hint(clap::ValueHint::FilePath),
        )
        .arg(
            clap::Arg::new("output")
                .long("output")
                .short('o')
                .hide(true)
                .value_hint(clap::ValueHint::FilePath),
        )
        .arg(
            clap::Arg::new("format")
                .long("format")
                .short('F')
                .value_parser([
                    PossibleValue::new("json"),
                    PossibleValue::new("yaml"),
                    PossibleValue::new("toml").hide(true),
                ]),
        )
        .args_conflicts_with_subcommands(true);
    clap_complete::dynamic::shells::CompleteCommand::augment_subcommands(cmd)
}

fn main() {
    let cmd = command();
    let matches = cmd.get_matches();
    if let Ok(completions) =
        clap_complete::dynamic::shells::CompleteCommand::from_arg_matches(&matches)
    {
        completions.complete(&mut command());
    } else {
        println!("{matches:#?}");
    }
}

#[test]
fn verify_cli() {
    command().debug_assert();
}
