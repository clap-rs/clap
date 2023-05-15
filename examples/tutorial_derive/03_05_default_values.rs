use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value_t = 2020)]
    port: u16,

    #[arg(long, default_value = "example.com")]
    host: String,

    #[arg(long, default_value_os_t = PathBuf::from("config.json"))]
    config: PathBuf,

    #[arg(long, default_values_t = [1, 2, 3])]
    seed: Vec<u32>,

    #[arg(long, default_values_os_t = [PathBuf::from("a"), PathBuf::from("b")])]
    source: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    println!("{:#?}", cli);
}
