//! How to add `no-thing` flag which is `true` by default and
//! `false` if passed.

use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(long = "no-verbose", parse(from_flag = std::ops::Not::not))]
    verbose: bool,
}

fn main() {
    let cmd = Opt::parse();
    println!("{:#?}", cmd);
}
