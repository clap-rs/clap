use clap::{app_from_crate, arg};

fn main() {
    let matches = app_from_crate!()
        .arg(arg!([NAME]).default_value("alice"))
        .get_matches();

    println!(
        "NAME: {:?}",
        matches
            .value_of("NAME")
            .expect("default ensures there is always a value")
    );
}
