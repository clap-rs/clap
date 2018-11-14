#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    App::new("myapp")
        .about("does awesome things")
        // use crate_authors! to pull the author(s) names from the Cargo.toml
        .author(crate_authors!())
        .get_matches();

    // running this app with -h will display whatever author(s) are in your
    // Cargo.toml
}
