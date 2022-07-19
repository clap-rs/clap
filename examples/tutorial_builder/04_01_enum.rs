use clap::{arg, command, value_parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)] // requires `derive` feature
enum Mode {
    Fast,
    Slow,
}

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(<MODE>)
                .help("What mode to run the program in")
                .value_parser(value_parser!(Mode)),
        )
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    match matches
        .get_one::<Mode>("MODE")
        .expect("'MODE' is required and parsing will fail if its missing")
    {
        Mode::Fast => {
            println!("Hare");
        }
        Mode::Slow => {
            println!("Tortoise");
        }
    }
}
