use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "basic")]
struct Opt {
    #[clap(default_values_t = [1, 2, 3])]
    value: u32,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
