use clap::{Parser, builder::Resettable};

#[derive(Parser)]
#[clap(author, version, about, long_about = Resettable::Reset)]
struct Cli {
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("verbose: {:?}", cli.verbose);
}
