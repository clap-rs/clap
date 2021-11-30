use clap::Parser;

#[derive(Parser)]
#[clap(about, version, author)] // Pull these from `Cargo.toml`
struct Cli {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.toml", value_name = "PATH")]
    config: std::path::PathBuf,
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn main() {
    let args = Cli::parse();

    println!("Value for config: {}", args.config.display());
    println!("Using input file: {}", args.input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match args.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be ridiculous"),
    }

    // more program logic goes here...
}
