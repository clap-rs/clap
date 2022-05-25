// Note: this requires the `cargo` feature

use clap::{arg, command, AppSettings};

fn main() {
    let matches = command!()
        .args_override_self(true)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .allow_negative_numbers(true)
        .arg(arg!(--two <VALUE>))
        .arg(arg!(--one <VALUE>))
        .get_matches();

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );
}
