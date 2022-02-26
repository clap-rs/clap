use clap::{Arg, Command, Parser, Subcommand as _};

#[derive(Parser)]
enum Subcommands {
    Derived {
        #[clap(short, long)]
        derived_flag: bool,
    },
}

fn main() {
    let cli = Command::new("Built CLI").subcommand(
        Command::new("built").arg(Arg::new("built-flag").short('b').long("built-flag")),
    );
    // Augment built subcommands with derived subcommands
    let cli = Subcommands::augment_subcommands(cli);

    let matches = cli.get_matches();
    match matches.subcommand() {
        Some(("built", sub_matches)) => {
            println!(
                "Got built subcommand with flag {:?}",
                sub_matches.is_present("built-flag")
            );
        }
        Some(("derived", sub_matches)) => {
            println!(
                "Got derived subcommand with flag {:?}",
                sub_matches.is_present("derived-flag")
            );
        }
        None => println!("No subcommand"),
        _ => unreachable!(),
    }
}
