use clap::{ArgEnum, Parser};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// What mode to run the program in
    #[clap(arg_enum)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    Fast,
    Slow,
}

fn main() {
    let cli = Cli::parse();

    // Note, it's safe to call unwrap() because the arg is required
    match cli.mode {
        Mode::Fast => {
            println!("Hare");
        }
        Mode::Slow => {
            println!("Tortoise");
        }
    }
}
