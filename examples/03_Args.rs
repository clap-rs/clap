extern crate clap;

use clap::{App, Arg};

fn main() {

    // Args describe a possible valid argument which may be supplied by the user at runtime. There
    // are three different types of arguments (flags, options, and positional) as well as a fourth
    // special type of arguement, called SubCommands (which will be discussed seperately).
    //
    // Args are described in the same manner as Apps using the "builder pattern" with multiple
    // methods describing various settings for the individual arguments. 
    //
    // Arguments can be added to applications in two manners, one at a time with the arg() method,
    // or multiple arguments at once via a Vec<Arg> inside the args() method.
    //
    // There are various options which can be set for a given argument, some apply to any of the
    // three types of arguments, some only apply one or two of the types. *NOTE* if you set 
    // incompatible options on a single argument, clap will panic! at runtime. This is by design,
    // so that you know right away an error was made. 
    //
    // # Help and Version
    // clap automatically generates a help and version flag for you, unless you specificy your
    // own. By default help uses "-h" and "--help", and version uses "-v" and "--version". You can
    // safely overide "-v" and "-h" to your own arguments, and "--help" and "--version" will stil
    // be automatically generated for you.
    let matches = App::new("MyApp")
                        // All application settings go here...
                        
                        // A simple "Flag" argument example (i.e. "-d")
                        .arg(Arg::new("debug")
                                    .help("turn on debugging information")
                                    .short("d"))

                        // Two arguments, one "Option" argument (i.e. one that takes a value) such
                        // as "-c some", and one positional argument (i.e. "myapp some_file")
                        .args( vec![
                            Arg::new("config")
                                    .help("sets the config file to use")
                                    .short("c")
                                    .long("config"),
                            Arg::new("input")
                                    .help("the input file to use")
                                    .index(1)
                                    .required(true)
                        ])
                        .get_matches();
     
    // Continued program logic goes here...
}
