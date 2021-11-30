#[derive(clap::Parser)]
enum Opt {
    #[clap(flatten)]
    Sub(SubCmd),
}

#[derive(clap::Parser)]
struct SubCmd {}

fn main() {}
