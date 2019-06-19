#[cfg(not(feature = "no_cargo"))]
fn main() {
    use clap::{App, crate_authors};

    App::new("myapp")
        .about("does awesome things")
        // use crate_authors! to pull the author(s) names from the Cargo.toml
        .author(crate_authors!())
        .get_matches();

    // running this app with -h will display whatever author(s) are in your
    // Cargo.toml
}

#[cfg(feature = "no_cargo")]
fn main() {
    // As stated above, if clap is compiled with the no_cargo feature, it is disabled.
    println!("no_cargo feature is enabled.");
    println!("Remove --features no_Cargo to cargo when trying this example.");
}
