extern crate clap;

use clap::{App, Arg};

fn main() {
    // Option arguments are those that take an additional value, such as "-c value". In clap they
    // support three types of specification, those with short() as "-o some", or those with long()
    // as "--option value" or "--option=value"
    //
    // Options also support a multiple setting, which is discussed in the example below.
    let matches = App::new("MyApp")
        // Regular App configuration goes here...
        // Assume we have an application that accepts an input file via the "-i file"
        // or the "--input file" (as well as "--input=file").
        // Below every setting supported by option arguments is discussed.
        // NOTE: You DO NOT need to specify each setting, only those which apply
        // to your particular case.
        .arg(
            Arg::with_name("input")
                .help("the input file to use") // Displayed when showing help info
                .takes_value(true) // MUST be set to true in order to be an "option" argument
                .short('i') // This argument is triggered with "-i"
                .long("input") // This argument is triggered with "--input"
                .multiple(true) // Set to true if you wish to allow multiple occurrences
                // such as "-i file -i other_file -i third_file"
                .required(true) // By default this argument MUST be present
                // NOTE: mutual exclusions take precedence over
                // required arguments
                .requires("config") // Says, "If the user uses "input", they MUST
                // also use this other 'config' arg too"
                // Can also specifiy a list using
                // requires_all(Vec<&str>)
                .conflicts_with("output"), // Opposite of requires(), says "if the
                                           // user uses -a, they CANNOT use 'output'"
                                           // also has a conflicts_with_all(Vec<&str>)
        )
        // NOTE: In order to compile this example, comment out conflicts_with()
        // and requires() because we have not defined an "output" or "config"
        // argument.
        .get_matches();

    // We can find out whether or not "input" was used
    if matches.is_present("input") {
        println!("An input file was specified");
    }

    // We can also get the value for "input"
    //
    // NOTE: If we specified multiple(), this will only return the _FIRST_
    // occurrence
    if let Some(ref in_file) = matches.value_of("input") {
        println!("An input file: {}", in_file);
    }

    // If we specified the multiple() setting we can get all the values
    if let Some(in_v) = matches.values_of("input") {
        for in_file in in_v {
            println!("An input file: {}", in_file);
        }
    }

    // We can see how many times the option was used with the occurrences_of() method
    //
    // NOTE: Just like with flags, if we did not specify the multiple() setting this will only
    // return 1 no matter how many times the argument was used (unless it wasn't used at all, in
    // in which case 0 is returned)
    println!(
        "The \"input\" argument was used {} times",
        matches.occurrences_of("input")
    );

    // Continued program logic goes here...
}
