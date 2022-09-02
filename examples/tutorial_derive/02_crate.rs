use clap::{Parser, builder::Resettable};

#[derive(Parser)]
#[clap(author, version, about, long_about = Resettable::Reset)] // Read from `Cargo.toml`
struct Cli {
    #[clap(long)]
    two: String,
    #[clap(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}
