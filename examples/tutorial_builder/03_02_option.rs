use clap::{app_from_crate, arg};

fn main() {
    let matches = app_from_crate!()
        .arg(arg!(-n --name <NAME>).required(false))
        .get_matches();

    println!("name: {:?}", matches.value_of("name"));
}
