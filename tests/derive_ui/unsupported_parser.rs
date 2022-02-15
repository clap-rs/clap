use clap::Parser;

#[derive(Parser, Clone, Debug)]
struct Opt {
    #[clap(parse(not_a_valid_parser))]
    value: i8,
}

fn main() {
    println!("{:?}", Opt::parse());
}
