use clap::Parser;

#[derive(Parser, Debug)]
struct Opt {
    verbose: bool,
}

fn main() {
    Opt::parse();
}
