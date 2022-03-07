// Note: this requires the `cargo` feature

use clap::{arg, command};

fn main() {
    let matches = command!().arg(arg!([NAME])).get_matches();

    println!("NAME: {:?}", matches.value_of("NAME"));
}
