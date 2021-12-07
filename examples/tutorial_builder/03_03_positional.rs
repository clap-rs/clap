use clap::{app_from_crate, arg};

fn main() {
    let matches = app_from_crate!().arg(arg!([NAME])).get_matches();

    println!("NAME: {:?}", matches.value_of("NAME"));
}
