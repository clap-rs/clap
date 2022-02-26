use clap::{arg, Args as _, Command, Parser};

#[derive(Parser)]
struct DerivedArgs {
    #[clap(short, long)]
    derived: bool,
}

fn main() {
    let cli = Command::new("CLI").arg(arg!(-b --built));
    // Augment built args with derived args
    let cli = DerivedArgs::augment_args(cli);

    let matches = cli.get_matches();
    println!("Value of built: {:?}", matches.is_present("built"));
    println!("Value of derived: {:?}", matches.is_present("derived"));
}
