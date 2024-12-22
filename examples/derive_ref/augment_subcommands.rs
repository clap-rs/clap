use clap::{text_provider::DEFAULT_TEXT_PROVIDER, Command, FromArgMatches as _, Parser, Subcommand as _};

#[derive(Parser, Debug)]
enum Subcommands {
    Derived {
        #[arg(short, long)]
        derived_flag: bool,
    },
}

fn main() {
    let cli = Command::new("Built CLI");
    // Augment with derived subcommands
    let cli = Subcommands::augment_subcommands(cli);

    let matches = cli.get_matches(&*DEFAULT_TEXT_PROVIDER);
    let derived_subcommands = Subcommands::from_arg_matches(&matches)
        .map_err(|err| err.exit(&*DEFAULT_TEXT_PROVIDER))
        .unwrap();
    println!("Derived subcommands: {derived_subcommands:#?}");
}
