use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}

fn main() {
    let cli = Cli::parse();

    println!("verbose: {:?}", cli.verbose);
}
