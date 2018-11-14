extern crate clap;

use clap::{App, Arg};

fn main() {
    // There are two ways in which to get a default value, one is to use claps Arg::default_value
    // method, and the other is to use Rust's built in Option::unwrap_or method.
    //
    // I'll demo both here.
    //
    // First, we'll use clap's Arg::default_value with an "INPUT" file.
    let matches = App::new("myapp")
        .about("does awesome things")
        .arg(
            Arg::with_name("INPUT")
                .help("The input file to use") // Note, we don't need to specify
                // anything like, "Defaults to..."
                // because clap will automatically
                // generate that for us, and place
                // it in the help text
                .default_value("input.txt")
                .index(1),
        )
        // Next we'll use the Option::unwrap_or method on this "CONFIG" option
        .arg(
            Arg::with_name("CONFIG")
                // Note that we have to manaully include some verbage to the user
                // telling them what the default will be.
                .help("The config file to use (default is \"config.json\")")
                .short('c')
                .takes_value(true),
        )
        .get_matches();

    // It's safe to call unwrap because the value with either be what the user input at runtime
    // or "input.txt"
    let input = matches.value_of("INPUT").unwrap();

    // Using Option::unwrap_or we get the same affect, but without the added help text injection
    let config_file = matches.value_of("CONFIG").unwrap_or("config.json");

    println!("The input file is: {}", input);
    println!("The config file is: {}", config_file);
}
