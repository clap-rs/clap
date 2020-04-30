use clap::Clap;
use std::ffi::CString;

#[derive(Clap, Debug)]
struct Opt {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Clap, Debug)]
enum Command {
    #[clap(external_subcommand)]
    Other(Vec<CString>),
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
