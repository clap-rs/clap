use clap::Clap;

#[derive(Clap, Debug)]
struct Opt {
    verbose: bool,
}

fn main() {
    Opt::parse();
}
