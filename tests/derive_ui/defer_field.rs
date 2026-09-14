#[derive(clap::Parser)]
struct Cli {
    #[arg(defer = true)]
    flag: bool,
}

fn main() {}
