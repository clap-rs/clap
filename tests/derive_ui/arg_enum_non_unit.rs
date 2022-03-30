use clap::ArgEnum;

#[derive(ArgEnum, Clone, Debug)]
enum Opt {
    Foo(usize),
}

fn main() {
    println!("{:?}", Opt::Foo(42));
}
