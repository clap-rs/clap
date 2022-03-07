// Note: this requires the `cargo` feature

use clap::{arg, command};
use std::ops::RangeInclusive;
use std::str::FromStr;

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn main() {
    let matches = command!()
        .arg(arg!(<PORT>).help("Network port to use").validator(|s| {
            usize::from_str(s)
                .map(|port| PORT_RANGE.contains(&port))
                .map_err(|e| e.to_string())
                .and_then(|result| match result {
                    true => Ok(()),
                    false => Err(format!(
                        "Port not in range {}-{}",
                        PORT_RANGE.start(),
                        PORT_RANGE.end()
                    )),
                })
        }))
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    let port: usize = matches
        .value_of_t("PORT")
        .expect("'PORT' is required and parsing will fail if its missing");
    println!("PORT = {}", port);
}
