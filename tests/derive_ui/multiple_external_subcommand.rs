use clap::Parser;

#[derive(Parser, Debug)]
struct Opt {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser, Debug)]
enum Command {
    #[clap(external_subcommand)]
    Run(Vec<String>),

    #[clap(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
