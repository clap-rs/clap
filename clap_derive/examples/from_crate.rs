//! How to derive the author, description, and version from Cargo.toml

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, about, version)]
//     ^^^^^^                   <- derive author from Cargo.toml
//             ^^^^^            <- derive description from Cargo.toml
//                    ^^^^^^^   <- derive version from Cargo.toml
struct Opt {}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
