use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
enum Opt {
    #[clap(fallback)]
    First(String),
    #[clap(fallback)]
    Second(String),
}

fn main() {}
