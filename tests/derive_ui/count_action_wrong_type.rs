use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, action = clap::ArgAction::Count)]
    verbose: i32,
}

fn main() {}
