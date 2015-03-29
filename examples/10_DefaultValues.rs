extern crate clap;

use clap::{App, Arg};

fn main() {
    // You can get a "default value" like feature by using Option<T>'s .unwrap_or() method
    //
    // Let's assume you have -c <config> argument to allow users to specify a configuration file
    // but you also want to support a default file, if none is specified.
    let matches = App::new("myapp").about("does awesome things")
                        .arg(Arg::new("CONFIG")
                                .help("The config file to use (default is \"config.json\")")
                                .short("c")
                                .takes_value(true))
                        .get_matches();

    let config_file = matches.value_of("CONFIG").unwrap_or("config.json");

    // use config_file here...
}