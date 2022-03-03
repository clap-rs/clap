use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Command, FromArgMatches, Parser, Subcommand};

#[derive(Debug)]
enum CliSub {
    Add,
    Remove,
}

impl FromArgMatches for CliSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("add", _)) => Ok(Self::Add),
            Some(("remove", _)) => Ok(Self::Remove),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::UnrecognizedSubcommand,
                "Valid subcommands are `add` and `remove`",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `add` and `remove`",
            )),
        }
    }
    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("add", _)) => *self = Self::Add,
            Some(("remove", _)) => *self = Self::Remove,
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::UnrecognizedSubcommand,
                    "Valid subcommands are `add` and `remove`",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl Subcommand for CliSub {
    fn augment_subcommands(cmd: Command<'_>) -> Command<'_> {
        cmd.subcommand(Command::new("add"))
            .subcommand(Command::new("remove"))
            .subcommand_required(true)
    }
    fn augment_subcommands_for_update(cmd: Command<'_>) -> Command<'_> {
        cmd.subcommand(Command::new("add"))
            .subcommand(Command::new("remove"))
            .subcommand_required(true)
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(name, "add" | "remove")
    }
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long)]
    top_level: bool,
    #[clap(subcommand)]
    subcommand: CliSub,
}

fn main() {
    let args = Cli::parse();
    println!("{:#?}", args);
}
