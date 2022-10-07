use std::path::PathBuf;

use clap::Parser;
use clap::ValueEnum;
use clap::ValueHint;
use clap_complete::dynamic::Completeable;

#[derive(Parser, Debug)]
struct Opts {
    #[arg(long, short, value_hint = ValueHint::FilePath)]
    input: PathBuf,
    #[arg(long, short)]
    format: Format,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Json,
    Yaml,
    Toml,
}

fn main() {
    println!("{:#?}", Opts::complete_or_parse());
}
