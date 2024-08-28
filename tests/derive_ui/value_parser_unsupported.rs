use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    foo: Custom,
}

#[derive(Clone, Debug)]
struct Custom;

fn main() {
    Cli::parse();
}
