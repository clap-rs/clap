use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "test")]
pub struct Opt {
    #[arg(long)]
    a: u32,
    #[arg(skip, long)]
    b: u32,
}

fn main() {
    let opt = Opt::parse();
    println!("{opt:?}");
}
