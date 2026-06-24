use clap::Parser;

#[derive(Parser, Debug)]
struct Opt {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Parser, Debug)]
enum Command {
    #[command(external_subcommand)]
    Run(Vec<String>),

    #[command(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let opt = Opt::parse();
    println!("{opt:?}");
}
