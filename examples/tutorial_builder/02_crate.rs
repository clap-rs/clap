use clap::{app_from_crate, arg};

fn main() {
    let matches = app_from_crate!()
        .arg(arg!(--two <VALUE>))
        .arg(arg!(--one <VALUE>))
        .get_matches();

    println!("two: {:?}", matches.value_of("two").expect("required"));
    println!("one: {:?}", matches.value_of("one").expect("required"));
}
