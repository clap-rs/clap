use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "basic", rename_all = "snake_case".to_owned())]
struct Opt {
    #[arg(short)]
    s: String,
}

fn main() {
    let opt = Opt::parse();
    println!("{opt:?}");
}
