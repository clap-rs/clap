use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "basic")]
struct Opt {
    #[arg(short, value_enum, default_value_t)]
    opts: bool,
}

fn main() {
    let opt = Opt::parse();
    println!("{opt:?}");
}
