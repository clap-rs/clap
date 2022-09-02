use clap::Parser;

#[derive(Parser, Debug)]
struct Opt {
    #[command(external_subcommand)]
    field: String,
}

fn main() {
    let _ = Opt::parse();
}
