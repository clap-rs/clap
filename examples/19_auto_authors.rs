#[macro_use]
extern crate clap;

#[cfg(feature = "unstable")]
fn main() {
    use clap::App;
    App::new("myapp")
        .about("does awesome things")
       // use crate_authors! to pull the author(s) names from the Cargo.toml
       .author(crate_authors!())
       .get_matches();

    // running the this app with the -h will display whatever author(s) are in your
    // Cargo.toml
}

#[cfg(not(feature = "unstable"))]
fn main() {
    // if clap is not compiled with the unstable feature, it is disabled.
    println!("unstable feature disabled.");
    println!("Pass --features unstable to cargo when trying this example.");
}
