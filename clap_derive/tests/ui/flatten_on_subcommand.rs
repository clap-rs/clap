#[derive(clap::Clap)]
struct Opt {
    #[clap(flatten)]
    sub: SubCmd,
}

#[derive(clap::Clap)]
enum SubCmd {}

fn main() {}
