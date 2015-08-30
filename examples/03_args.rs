extern crate clap;

use clap::{App, Arg};

#[allow(unused_variables)]
fn main() {
    // Args describe a possible valid argument which may be supplied by the user at runtime. There
    // are three different types of arguments (flags, options, and positional) as well as a fourth
    // special type of argument, called SubCommands (which will be discussed separately).
    //
    // Args are described in the same manner as Apps using the "builder pattern" with multiple
    // methods describing various settings for the individual arguments. Or by supplying a "usage"
    // string. Both methods have their pros and cons.
    //
    // Arguments can be added to applications in two manners, one at a time with the arg(), and
    // arg_from_usage() method, or multiple arguments at once via a Vec<Arg> inside the args() method,
    // or a single &str describing multiple Args (one per line) supplied to args_from_usage().
    //
    // There are various options which can be set for a given argument, some apply to any of the
    // three types of arguments, some only apply one or two of the types. *NOTE* if you set 
    // incompatible options on a single argument, clap will panic! at runtime. This is by design,
    // so that you know right away an error was made by the developer, not the end user. 
    //
    // # Help and Version
    // clap automatically generates a help and version flag for you, unless you specificy your
    // own. By default help uses "-h" and "--help", and version uses "-V" and "--version". You can
    // safely overide "-V" and "-h" to your own arguments, and "--help" and "--version" will stil
    // be automatically generated for you.
    let matches = App::new("MyApp")
                        // All application settings go here...
                        
                        // A simple "Flag" argument example (i.e. "-d") using the builder pattern
                        .arg(Arg::with_name("debug")
                                    .help("turn on debugging information")
                                    .short("d"))

                        // Two arguments, one "Option" argument (i.e. one that takes a value) such
                        // as "-c some", and one positional argument (i.e. "myapp some_file")
                        .args( vec![
                            Arg::with_name("config")
                                    .help("sets the config file to use")
                                    .short("c")
                                    .long("config"),
                            Arg::with_name("input")
                                    .help("the input file to use")
                                    .index(1)
                                    .required(true)
                        ])

                        // *Note* the following two examples are convienience methods, if you wish
                        // to still get the full configurability of Arg::with_name() and the readability
                        // of arg_from_usage(), you can instantiate a new Arg with Arg::from_usage() and
                        // still be able to set all the additional properties, just like Arg::with_name()
                        //
                        //
                        // One "Flag" using a usage string
                        .arg_from_usage("--license 'display the license file'")

                        // Two args, one "Positional", and one "Option" using a usage string
                        .args_from_usage("[output] 'Supply an output file to use'
                                          -i --int=[interface] 'Set an interface to use'")
                        .get_matches();
     
    // Continued program logic goes here...
}
