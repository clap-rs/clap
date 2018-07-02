#[macro_use]
extern crate clap;

use clap::Clap;

#[derive(Clap, Debug)]
struct Opt {
    #[clap(raw(required = "true", min_values = "2"))]
    foos: Vec<String>,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
