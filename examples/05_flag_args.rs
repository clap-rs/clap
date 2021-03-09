use clap::{App, Arg};

fn main() {
    // Of the three argument types, flags are the most simple. Flags are simple switches which can
    // be either "on" or "off"
    //
    // clap also supports multiple occurrences of flags, the common example is "verbosity" where a
    // user could want a little information with "-v" or tons of information with "-v -v" or "-vv"
    let matches = App::new("MyApp")
        // Regular App configuration goes here...
        // We'll add a flag that represents an awesome meter...
        //
        // I'll explain each possible setting that "flags" accept. Keep in mind
        // that you DO NOT need to set each of these for every flag, only the ones
        // you want for your individual case.
        .arg(
            Arg::new("awesome")
                .about("turns up the awesome") // Displayed when showing help info
                .short('a') // Trigger this arg with "-a"
                .long("awesome") // Trigger this arg with "--awesome"
                .takes_value(true)
                .multiple(true) // This flag should allow multiple
                // occurrences such as "-aaa" or "-a -a"
                .requires("config") // Says, "If the user uses -a, they MUST
                // also use this other 'config' arg too"
                // Can also specify a list using
                // requires_all(Vec<&str>)
                .conflicts_with("output"), // Opposite of requires(), says "if the
                                           // user uses -a, they CANNOT use 'output'"
                                           // also has a conflicts_with_all(Vec<&str>)
                                           // and an exclusive(true)
        )
        .arg("-c, --config=[FILE] 'sets a custom config file'")
        .arg("<output> 'sets an output file'")
        .get_matches();

    // We can find out whether or not awesome was used
    if matches.is_present("awesome") {
        println!("Awesomeness is turned on");
    }

    // If we set the multiple() option of a flag we can check how many times the user specified
    //
    // Note: if we did not specify the multiple() option, and the user used "awesome" we would get
    // a 1 (no matter how many times they actually used it), or a 0 if they didn't use it at all
    match matches.occurrences_of("awesome") {
        0 => println!("Nothing is awesome"),
        1 => println!("Some things are awesome"),
        2 => println!("Lots of things are awesome"),
        _ => println!("EVERYTHING is awesome!"),
    }

    // Continued program logic goes here...
}
