use clap::{Parser, ValueEnum, builder::Resettable};

#[derive(Parser)]
#[clap(author, version, about, long_about = Resettable::Reset)]
struct Cli {
    /// What mode to run the program in
    #[clap(value_enum)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Fast,
    Slow,
}

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Fast => {
            println!("Hare");
        }
        Mode::Slow => {
            println!("Tortoise");
        }
    }
}
