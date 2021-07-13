use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "basic")]
struct Opt {
    #[clap(short, arg_enum)]
    opts: bool,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
