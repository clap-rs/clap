use clap::Clap;

#[derive(Clap, Debug)]
struct Opt {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Clap, Debug)]
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
