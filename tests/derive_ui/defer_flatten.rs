#[derive(clap::Parser)]
struct Cli {
    #[command(flatten, defer = true)]
    options: Options,
}

#[derive(clap::Args)]
struct Options {
    flag: bool,
}

fn main() {}
