use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(allow_negative_numbers = true)]
struct Cli {
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}
