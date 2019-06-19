#[cfg(not(feature = "no_cargo"))]
fn main() {
    use clap::{App, crate_version};

    // You can have clap pull the application version directly from your Cargo.toml starting with
    // clap v0.4.14 on crates.io (or master#a81f915 on github). Using Rust's env! macro like this:
    //
    // let version = format!("{}.{}.{}{}",
    //                  env!("CARGO_PKG_VERSION_MAJOR"),
    //                  env!("CARGO_PKG_VERSION_MINOR"),
    //                  env!("CARGO_PKG_VERSION_PATCH"),
    //                  option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));
    //
    // Starting from v0.6.6 on crates.io you can also use the crate_version!() macro instead of
    // manually using the env!() macros. Under the hood, the macro uses this exact method to get
    // the version.
    //
    // Thanks to https://github.com/jhelwig for pointing this out
    App::new("myapp")
        .about("does awesome things")
        // use crate_version! to pull the version number
        .version(crate_version!())
        .get_matches();

    // running this app with the -V or --version will display whatever version is in your
    // Cargo.toml, the default being: myapp 0.0.1
}

#[cfg(feature = "no_cargo")]
fn main() {
    // As stated above, if clap is compiled with the no_cargo feature, it is disabled.
    println!("no_cargo feature is enabled.");
    println!("Remove --features no_Cargo to cargo when trying this example.");
}
