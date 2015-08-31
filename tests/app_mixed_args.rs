extern crate clap;

use clap::{App, Arg};

#[test]
fn mixed_positional_and_options() {
    App::new("mixed_positional_and_options")
        .arg(Arg::with_name("feed")
             .short("f")
             .long("feed")
             .takes_value(true)
             .multiple(true))
        .arg(Arg::with_name("config")
             .required(true)
             .index(1))
        .get_matches_from(vec!["", "--feed", "1", "config.toml"]);
}
