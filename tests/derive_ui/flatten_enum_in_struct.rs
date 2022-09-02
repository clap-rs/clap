#[derive(clap::Parser)]
struct Opt {
    #[command(flatten)]
    sub: SubCmd,
}

#[derive(clap::Parser)]
enum SubCmd {}

fn main() {}
