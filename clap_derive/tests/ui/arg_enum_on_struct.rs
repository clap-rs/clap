use clap::ArgEnum;

#[derive(ArgEnum, Debug)]
struct Opt {}

fn main() {
    println!("{:?}", Opt::arg_values());
}
