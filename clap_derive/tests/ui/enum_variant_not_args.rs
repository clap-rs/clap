#[derive(clap::Clap)]
enum Opt {
    Sub(SubCmd),
}

#[derive(clap::Clap)]
enum SubCmd {}

fn main() {}
