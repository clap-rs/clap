#[macro_use]
extern crate clap;

use clap::App;

fn main() {
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
