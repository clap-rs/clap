//! How to append a postscript to the help message generated.

use clap::Parser;

/// I am a program and I do things.
///
/// Sometimes they even work.
#[derive(Parser, Debug)]
#[clap(after_help = "Beware `-d`, dragons be here")]
struct Opt {
    /// Release the dragon.
    #[clap(short)]
    dragon: bool,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
