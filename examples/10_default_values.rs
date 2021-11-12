use clap::{App, Arg};

fn main() {
    // There are two ways in which to get a default value, one is to use clap's Arg::default_value
    // method, and the other is to use Rust's built in Option::unwrap_or method.
    //
    // I'll demo both here.
    //
    // First, we'll use clap's Arg::default_value with an "INPUT" file.
    let matches = App::new("myapp")
        .about("does awesome things")
        .arg(
            Arg::new("INPUT")
                .about("The input file to use")
                .default_value("input.txt")
                .index(1),
        )
        // Next we'll use the Option::unwrap_or method on this "CONFIG" option
        .arg(
            Arg::new("CONFIG")
                .about("The config file to use")
                .short('c')
                .takes_value(true),
        )
        .get_matches();

    // It's safe to call unwrap because the value will either be what the user input at runtime
    // or "input.txt"
    let input = matches.value_of("INPUT").unwrap();

    // Using Option::unwrap_or we get the same effect, but without the added help text injection
    let config_file = matches.value_of("CONFIG").unwrap_or("config.json");

    println!("The input file is: {}", input);
    println!("The config file is: {}", config_file);
}
