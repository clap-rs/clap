#[derive(clap::Parser)]
enum Opt {
    #[command(flatten)]
    Sub(SubCmd),
}

#[derive(clap::Parser)]
struct SubCmd {}

fn main() {}
