use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(allow_negative_numbers = true)]
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
