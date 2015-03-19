extern crate clap;

use clap::{App, Arg};

fn main() {

    // Args describe a possible valid argument which may be supplied by the user at runtime. There
    // are three different types of arguments (flags, options, and positional) as well as a fourth
    // special type of arguement, called SubCommands (which will be discussed seperately).
    //
    // Args are described in the same manner as Apps using the "builder pattern" with multiple
    // methods describing various options for the individual arguments. 
    //
    // Arguments can be added to applications in two manners, one at a time with the arg() method,
    // or multiple arguments at once via a Vec<Arg> inside the args() method.
    //
    // There are various options which can be set for a given argument, some apply to any of the
    // three types of arguments, some only apply one or two of the types. *NOTE* if you set 
    // incompatible options on a single argument, it will panic! at runtime. This is by design, so
    // that you know right away an error was made. You only need to set the options you care about
    // for each argument. 
    //
    // # Help and Version
    // clap automatically generates a help and version flag for you, unless you specificy your
    // own. By default help uses "-h" and "--help", and version uses "-v" and "--version". You can
    // safely overide "-v" and "-h" to your own arguments, and "--help" and "--version" will stil
    // be automatically generated for you.
    let matches = App::new("MyApp")
                        // All application settings go here...
                        // A simple "Flag" argument (i.e. "-a")
                        .arg(Arg::new("debug"))
                        // A complex "Option" argument (i.e. one that takes a value) such as "-c some"
                        .arg(Arg::new("config")                 // This name will be displayed with the help message
                                                                // and is used to get runtime details of this argument
                                    .help("sets a config file") // A short message displayed with the help message
                                    .short("c")                 // Sets an argument trigger to "-c"
                                    .long("config")             // Sets an argument trigger to "--config"
                                    .takes_value(true)          // Specifies a value *MUST* accompany this argument
                                                                // such as "-c some" (if you provided a short()) or 
                                                                // "--config some" or "--config=some" (if you provided
                                                                // a long())
                                    .multiple(true)             // Allows multiple instances of this argument such as
                                                                // "-c some -c other -c string"
                                    .required(true)             // By default the user *MUST* use this argument
                                    .requires("")
                                    .requires_all()
                                    .mutually_excludes()
                                    .mutually_excludes_all()
                            )
                        .get_matches();
     
    // Continued program logic goes here...
}
