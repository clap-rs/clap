#![crate_type= "lib"]

#![feature(collections, libc, exit_status)]

//! A simply library for parsing command line arguments when writing 
//! command line and console applications.
//!
//!
//! You can use `clap` to lay out a list of possible valid command line arguments and let `clap` parse the string given by the user at runtime.
//! When using `clap` you define a set of parameters and rules for your arguments and at runtime `clap` will determine their validity.
//! Also, `clap` provides the traditional version and help switches 'for free' by parsing the list of possible valid arguments lazily at runtime.
//! i.e. only when it's been determined that the user wants or needs to see the help and version information.
//! 
//! After defining a list of possible valid arguments you get a list of matches that the user supplied at runtime. You can then use this list to
//! determine the functioning of your program.
//!
//! Example:
//! 
//! ```no_run
//! use clap::{Arg, App, SubCommand};
//!
//! // ...
//! 
//! let matches = App::new("MyApp")
//!                        .version("1.0")
//!                        .author("Kevin K. <kbknapp@gmail.com>")
//!                        .about("Does awesome things")
//!                        .arg(Arg::new("config")
//!                             .short("c")
//!                             .long("config")
//!                             .help("Sets a custom config file")
//!                             .takes_value(true))
//!                        .arg(Arg::new("output")
//!                             .help("Sets an optional output file")
//!                             .index(1))
//!                        .arg(Arg::new("debug")
//!                             .short("d")
//!                             .multiple(true)
//!                             .help("Turn debugging information on"))
//!                        .subcommand(SubCommand::new("test")
//!                                    .about("Has test sub functionality")
//!                                    .arg(Arg::new("verbose")
//!                                         .short("v")
//!                                         .help("Display verbose information")))
//!                        .get_matches();
//!
//!    if let Some(o) = matches.value_of("output") {
//!        println!("Value for output: {}", o);
//!    }
//! 
//!    if let Some(c) = matches.value_of("config") {
//!        println!("Value for config: {}", c);
//!    }
//!
//! match matches.occurrences_of("debug") {
//!     0 => println!("Debug mode is off"),
//!        1 => println!("Debug mode is kind of on"),
//!        2 => println!("Debug mode is on"),
//!        3 | _ => println!("Don't be crazy"),
//! }
//!
//! if let Some(ref matches) = matches.subcommand_matches("test") {
//!     if matches.is_present("verbose") {
//!            println!("Printing verbose test info...");
//!        } else {
//!            println!("Not printing regular test info...");
//!        }
//!    }
//!
//! // more porgram logic goes here...
//! ```
//!
//! If you were to compile the above program and run it with the flag `--help` or `-h` the following output woud be presented
//!
//! ```sh
//! $ myprog --help
//! MyApp 1.0
//! Kevin K. <kbknapp@gmail.com>
//! Does awesome things
//! 
//! USAGE:
//!     MyApp [FLAGS] [OPTIONS] [POSITIONAL] [SUBCOMMANDS]
//! 
//! FLAGS:
//!     -d               Turn debugging information on
//!     -h,--help        Prints this message
//!     -v,--version     Prints version information
//! 
//! OPTIONS:
//!     -c,--config <config>        Sets a custom config file
//!
//! POSITIONAL ARGUMENTS:
//!     output            Sets an optional output file
//!
//! SUBCOMMANDS:
//!     help             Prints this message
//!     test             Has test sub-functionality
//! ```

pub use argmatches::ArgMatches;
pub use arg::Arg;
pub use app::App;
pub use subcommand::SubCommand;

mod app;
mod argmatches;
mod arg;
mod args;
mod subcommand;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn unique_arg_names(){
        App::new("some").args(vec![
            Arg::new("arg").short("a"),
            Arg::new("arg").short("b")
        ]);
    }
    #[test]
    #[should_panic]
    fn unique_arg_shorts(){
        App::new("some").args(vec![
            Arg::new("arg1").short("a"),
            Arg::new("arg2").short("a")
        ]);
    }
    #[test]
    #[should_panic]
    fn unique_arg_longs(){
        App::new("some").args(vec![
            Arg::new("arg1").long("long"),
            Arg::new("arg2").long("long")
        ]);
    }
    #[test]
    fn create_app(){
        App::new("some").about("about").author("author").version("1.0");
    }
    #[test]
    fn create_arg_flag(){
        Arg::new("some").short("a").long("long").help("help with some arg").multiple(true);
    }
    #[test]
    fn create_arg_pos(){
        Arg::new("some").index(1).help("help with some arg").required(true);
    }
    #[test]
    fn create_arg_opt(){
        Arg::new("some").short("s").long("some").takes_value(true).help("help with some arg").required(true);
    }
}
