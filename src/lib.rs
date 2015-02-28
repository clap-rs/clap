#![crate_type= "lib"]

#![feature(collections, core, libc, env)]

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
//! ```rust.example
//! extern crate clap;
//! use clap::{Arg, App};
//!
//! // ...
//! 
//! let matches = App::new("MyApp")
//!						.version("1.0")
//!						.author("Kevin K. <kbknapp@gmail.com>")
//!						.about("Does awesome things")
//!						.arg(Arg::new("config")
//!									.short("c")
//!									.long("config")
//!									.help("Sets a custom config file")
//!									.takes_value(true))
//!						.arg(Arg::new("output")
//!									.help("Sets an optional output file")
//!									.index(1)
//!						.arg(Arg::new("debug")
//!									.short("d")
//! 								.multiple(true)
//!									.help("Turn debugging information on"))
//!						.get_matches();
//!
//!	if let Some(o) = matches.value_of("output") {
//!		println!("Value for output: {}", o);
//!	}
//! 
//!	if let Some(c) = matches.value_of("config") {
//!		println!("Value for config: {}", c);
//!	}
//!
//! match matches.occurrences_of("debug") {
//! 	0 => println!("Debug mode is off"),
//!		1 => println!("Debug mode is kind of on"),
//!		2 => println!("Debug mode is on"),
//!		3 | _ => println!("Don't be crazy"),
//! }
//! 
//! // more porgram logic goes here...
//! ```

pub use argmatches::ArgMatches;
pub use arg::Arg;
pub use app::App;

mod app;
mod argmatches;
mod arg;
mod args;
