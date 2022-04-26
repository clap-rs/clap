use clap::Parser;

#[derive(Debug)]
struct Foo;

#[derive(Parser, Debug)]
struct Opt {
    #[clap(long)]
    name: [Foo; 2],
}

fn main() {
    println!("{:?}", Opt::parse());
}
