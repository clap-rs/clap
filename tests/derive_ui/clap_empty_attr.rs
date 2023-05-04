use clap::Parser;

#[derive(Parser, Debug)]
#[command]
struct Opt {}

#[derive(Parser, Debug)]
struct Opt1 {
    #[arg = "short"]
    foo: u32,
}

fn main() {
    let opt = Opt::parse();
    println!("{opt:?}");
}
