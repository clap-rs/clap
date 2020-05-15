use clap::ArgEnum;

#[derive(ArgEnum, Debug)]
struct Opt {}

fn main() {
    println!("{:?}", Opt::VARIANTS);
}
