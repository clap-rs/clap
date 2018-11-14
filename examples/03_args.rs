extern crate clap;

use clap::{App, Arg};

fn main() {
    // Args describe a possible valid argument which may be supplied by the user at runtime. There
    // are three different types of arguments (flags, options, and positional) as well as a fourth
    // special type of argument, called s (which will be discussed separately).
    //
    // Args are described in the same manner as Apps using the "builder pattern" with multiple
    // methods describing various settings for the individual arguments. Or by supplying a "usage"
    // string. Both methods have their pros and cons.
    //
    // Arguments can be added to applications in two manners, one at a time with the arg(), and
    // arg() method, or multiple arguments at once via a Vec<Arg> inside the args() method,
    // or a single &str describing multiple Args (one per line) supplied to args_from_usage().
    //
    // There are various options which can be set for a given argument, some apply to any of the
    // three types of arguments, some only apply one or two of the types. *NOTE* if you set
    // incompatible options on a single argument, clap will panic! at runtime. This is by design,
    // so that you know right away an error was made by the developer, not the end user.
    //
    // # Help and Version
    // clap automatically generates a help and version flag for you, unless you specify your
    // own. By default help uses "-h" and "--help", and version uses "-V" and "--version". You can
    // safely override "-V" and "-h" to your own arguments, and "--help" and "--version" will still
    // be automatically generated for you.
    let matches = App::new("MyApp")
        // All application settings go here...
        // A simple "Flag" argument example (i.e. "-d") using the builder pattern
        .arg(
            Arg::with_name("debug")
                .help("turn on debugging information")
                .short('d'),
        )
        // Two arguments, one "Option" argument (i.e. one that takes a value) such
        // as "-c some", and one positional argument (i.e. "myapp some_file")
        .args(&[
            Arg::with_name("config")
                .help("sets the config file to use")
                .takes_value(true)
                .short('c')
                .long("config"),
            Arg::with_name("input")
                .help("the input file to use")
                .index(1)
                .required(true),
        ])
        // *Note* the following two examples are convenience methods, if you wish
        // to still get the full configurability of Arg::with_name() and the readability
        // of arg(), you can instantiate a new Arg with Arg::from() and
        // still be able to set all the additional properties, just like Arg::with_name()
        //
        //
        // One "Flag" using a usage string
        .arg("--license 'display the license file'")
        // Two args, one "Positional", and one "Option" using a usage string
        .arg("[output] 'Supply an output file to use'")
        .arg("-i, --int=[IFACE] 'Set an interface to use'")
        .get_matches();

    // Here are some examples of using the arguments defined above. Keep in mind that this is only
    // an example, and may be somewhat contrived
    //
    // First we check if debugging should be on or not
    println!(
        "Debugging mode is: {}",
        if matches.is_present("debug") {
            "ON"
        } else {
            "OFF"
        }
    );

    // Next we print the config file we're using, if any was defined with either -c <file> or
    // --config <file>
    if let Some(config) = matches.value_of("config") {
        println!("A config file was passed in: {}", config);
    }

    // Let's print the <INPUT> file the user passed in. We can use .unwrap() here becase the arg is
    // required, and parsing would have failed if the user forgot it
    println!("Using input file: {}", matches.value_of("input").unwrap());

    // We could continue checking for and using arguments in this manner, such as "license",
    // "output", and "interface". Keep in mind that "output" and "interface" are optional, so you
    // shouldn't call .unwrap(). Instead, prefer using an 'if let' expression as we did with
    // "config"
}
