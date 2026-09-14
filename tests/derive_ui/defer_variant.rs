#[derive(clap::Parser)]
enum Cli {
    #[command(defer = true)]
    Run,
}

fn main() {}
