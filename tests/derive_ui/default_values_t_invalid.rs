use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "basic")]
struct Opt {
    #[arg(default_values_t = [1, 2, 3])]
    value: u32,
}

fn main() {
    let opt = Opt::parse();
    println!("{opt:?}");
}
