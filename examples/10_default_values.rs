use clap::{App, Arg};

fn main() {
    let matches = App::new("myapp")
        .about("does awesome things")
        .arg(
            Arg::new("INPUT")
                .help("The input file to use")
                .default_value("input.txt")
                .index(1),
        )
        // Next we'll use the Option::unwrap_or method on this "CONFIG" option
        .arg(
            Arg::new("CONFIG")
                .help("The config file to use")
                .short('c')
                .takes_value(true),
        )
        .get_matches();

    let input = matches
        .value_of("INPUT")
        .expect("'INPUT' is default and parsing will ensure there always is a value");

    // Using Option::unwrap_or we get the same effect, but without the added help text injection
    let config_file = matches.value_of("CONFIG").unwrap_or("config.json");

    println!("The input file is: {}", input);
    println!("The config file is: {}", config_file);
}
