use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
struct Opt {}

fn main() {
    println!("{:?}", Opt::value_variants());
}
