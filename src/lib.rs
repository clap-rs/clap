#![crate_id = "clap"]
#![crate_type= "lib"]

#![feature(collections, core, libc, env)]

//! A simply library for parsing command line arguments when writing 
//! command line and console applications.

pub use argmatches::ArgMatches;
pub use arg::Arg;
pub use app::App;

mod app;
mod argmatches;
mod arg;
mod args;
