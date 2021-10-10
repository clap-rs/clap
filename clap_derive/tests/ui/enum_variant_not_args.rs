#[derive(clap::Parser)]
enum Opt {
    Sub(SubCmd),
}

#[derive(clap::Parser)]
enum SubCmd {}

fn main() {}
