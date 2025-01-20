use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
enum Opt {
    #[clap(fallback)]
    First(String, String),
}

fn main() {}

