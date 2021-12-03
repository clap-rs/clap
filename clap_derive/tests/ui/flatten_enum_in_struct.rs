#[derive(clap::Parser)]
struct Opt {
    #[clap(flatten)]
    sub: SubCmd,
}

#[derive(clap::Parser)]
enum SubCmd {}

fn main() {}
