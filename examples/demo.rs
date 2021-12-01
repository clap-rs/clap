use clap::{ArgEnum, Parser};

#[derive(Parser)]
#[clap(about, version, author)] // Pull these from `Cargo.toml`
struct Cli {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(
        short,
        long,
        parse(from_os_str),
        default_value = "default.toml",
        value_name = "PATH"
    )]
    config: std::path::PathBuf,
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,
    /// What mode to run the program in
    #[clap(short, long, arg_enum, default_value_t)]
    mode: Mode,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    Fast,
    Slow,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Slow
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

fn main() {
    let cli = Cli::parse();

    println!("Value for config: {}", cli.config.display());
    println!("Using input file: {}", cli.input);
    match cli.mode {
        Mode::Fast => {
            println!("Hare");
        }
        Mode::Slow => {
            println!("Tortoise");
        }
    }

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match cli.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be ridiculous"),
    }

    // more program logic goes here...
}
