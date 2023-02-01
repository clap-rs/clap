use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "basic")]
struct Opt {
    #[command(flatten)]
    source: Source,
}

#[derive(clap::Args, Debug)]
#[group(required = true, name = "src")]
struct Source {
    #[arg(short)]
    git: String,

    #[arg(short)]
    path: String,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
