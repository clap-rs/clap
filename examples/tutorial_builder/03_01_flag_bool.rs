use clap::{app_from_crate, arg};

fn main() {
    let matches = app_from_crate!().arg(arg!(-v - -verbose)).get_matches();

    println!("verbose: {:?}", matches.is_present("verbose"));
}
