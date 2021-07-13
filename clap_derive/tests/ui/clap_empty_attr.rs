use clap::Parser;

#[derive(Parser, Debug)]
#[clap]
struct Opt {}

#[derive(Parser, Debug)]
struct Opt1 {
    #[clap = "short"]
    foo: u32,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
