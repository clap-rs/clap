#[macro_use]
extern crate clap;

use clap::App;

#[cfg(feature = "unstable")]
fn main() {
    App::new("myapp")
        .about("does awesome things")
       // use crate_authors! to pull the author(s) names from the Cargo.toml
       .author(crate_authors!())
       .get_matches();

    // running the this app with the -h will display whatever author(s) are in your
    // Cargo.toml
}
