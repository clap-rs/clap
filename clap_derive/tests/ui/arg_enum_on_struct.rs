use clap::ArgEnum;

#[derive(ArgEnum, Clone, Debug)]
struct Opt {}

fn main() {
    println!("{:?}", Opt::value_variants());
}
