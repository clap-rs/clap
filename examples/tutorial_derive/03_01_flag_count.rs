use clap::{builder::Resettable::Reset, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = Reset)]
struct Cli {
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let cli = Cli::parse();

    println!("verbose: {:?}", cli.verbose);
}
