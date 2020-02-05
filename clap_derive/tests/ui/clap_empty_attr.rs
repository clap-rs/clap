use clap::Clap;

#[derive(Clap, Debug)]
#[clap]
struct Opt {}

#[derive(Clap, Debug)]
struct Opt1 {
    #[clap = "short"]
    foo: u32
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
