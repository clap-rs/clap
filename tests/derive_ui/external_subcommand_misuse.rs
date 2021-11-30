use clap::Parser;

#[derive(Parser, Debug)]
struct Opt {
    #[clap(external_subcommand)]
    field: String,
}

fn main() {
    let _ = Opt::parse();
}
