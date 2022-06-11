use clap::error::Error;
use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches, Parser};

#[derive(Debug)]
struct CliArgs {
    foo: bool,
    bar: bool,
    quuz: Option<String>,
}

impl FromArgMatches for CliArgs {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let mut matches = matches.clone();
        Self::from_arg_matches_mut(&mut matches)
    }
    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            foo: *matches.get_one::<bool>("foo").expect("defaulted by clap"),
            bar: *matches.get_one::<bool>("bar").expect("defaulted by clap"),
            quuz: matches.remove_one::<String>("quuz"),
        })
    }
    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        let mut matches = matches.clone();
        self.update_from_arg_matches_mut(&mut matches)
    }
    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        self.foo |= *matches.get_one::<bool>("foo").expect("defaulted by clap");
        self.bar |= *matches.get_one::<bool>("bar").expect("defaulted by clap");
        if let Some(quuz) = matches.remove_one::<String>("quuz") {
            self.quuz = Some(quuz);
        }
        Ok(())
    }
}

impl Args for CliArgs {
    fn augment_args(cmd: Command<'_>) -> Command<'_> {
        cmd.arg(
            Arg::new("foo")
                .short('f')
                .long("foo")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bar")
                .short('b')
                .long("bar")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("quuz").short('q').long("quuz").takes_value(true))
    }
    fn augment_args_for_update(cmd: Command<'_>) -> Command<'_> {
        cmd.arg(
            Arg::new("foo")
                .short('f')
                .long("foo")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bar")
                .short('b')
                .long("bar")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("quuz").short('q').long("quuz").takes_value(true))
    }
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long, action)]
    top_level: bool,
    #[clap(flatten)]
    more_args: CliArgs,
}

fn main() {
    let args = Cli::parse();
    println!("{:#?}", args);
}
