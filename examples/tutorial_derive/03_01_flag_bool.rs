use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("verbose: {:?}", cli.verbose);
}
