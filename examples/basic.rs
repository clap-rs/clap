#[macro_use]
extern crate clap;
#[macro_use]
extern crate clap_derive;

use clap::{App, Arg};

#[derive(ArgEnum, Debug)]
enum ArgChoice {
    Foo,
    Bar,
    Baz,
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
            .arg(Arg::with_name("arg")
                .required(true)
                .takes_value(true)
                .possible_values(&ArgChoice::variants())
            ).get_matches();
    
    let t = value_t!(matches.value_of("arg"), ArgChoice)
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", t);
}