extern crate clap;

use clap::App;

fn main() {
    // You can have clap pull the application version directly from your Cargo.toml starting with
    // clap v0.4.14 on crates.io (or master#a81f915 on github)
    //
    // Thanks to https://github.com/jhelwig for pointing this out
    let version = format!("{}.{}.{}{}",
                          env!("CARGO_PKG_VERSION_MAJOR"),
                          env!("CARGO_PKG_VERSION_MINOR"),
                          env!("CARGO_PKG_VERSION_PATCH"),
                          option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));

    let matches = App::new("myapp").about("does awesome things").version(&version[..]).get_matches();

    // running the this app with the -v or --version will display whatever version is in your
    // Cargo.toml, the default being: myapp 0.0.1
}