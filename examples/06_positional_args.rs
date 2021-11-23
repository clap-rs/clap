use clap::{App, Arg};

fn main() {
    let matches = App::new("MyApp")
        // Regular App configuration goes here...
        // We'll add two positional arguments, an input file, and a config file.
        //
        // I'll explain each possible setting that "positionals" accept. Keep in
        // mind that you DO NOT need to set each of these for every flag, only the
        // ones that apply to your individual case.
        .arg(
            Arg::new("input")
                .help("the input file to use") // Displayed when showing help info
                .index(1) // Set the order in which the user must
                // specify this argument (Starts at 1)
                .requires("config") // Says, "If the user uses "input", they MUST
                // also use this other 'config' arg too"
                // Can also specify a list using
                // requires_all(Vec<&str>)
                .required(true), // By default this argument MUST be present
                                 // NOTE: mutual exclusions take precedence over
                                 // required arguments
        )
        .arg(Arg::new("config").help("the config file to use").index(2)) // Note, we do not need to specify required(true)
        // if we don't want to, because "input" already
        // requires "config"
        // Note, we also do not need to specify requires("input")
        // because requires lists are automatically two-way
        .get_matches();

    // We can find out whether or not "input" or "config" were used
    if matches.is_present("input") {
        println!("An input file was specified");
    }

    // We can also get the values for those arguments
    if let Some(ref in_file) = matches.value_of("input") {
        println!(
            "Doing work with {} and {}",
            in_file,
            matches
                .value_of("config")
                .expect("'config' is required and parsing will fail if its missing")
        );
    }
    // Continued program logic goes here...
}
